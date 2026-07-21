use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum OAuthError {
    #[error("Unauthorized or invalid token")]
    Unauthorized,
    #[error("HTTP error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Internal OAuth error: {0}")]
    Internal(String),
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct OAuthConfig {
    #[serde(
        alias = "koala_host",
        alias = "oidc_host",
        alias = "issuer_url",
        alias = "issuer"
    )]
    pub server_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub auth_url: Option<String>,
    pub token_url: Option<String>,
    pub userinfo_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: Option<String>,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub is_admin: bool,
    pub sub: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub raw: Value,
}

#[derive(Debug, Deserialize)]
struct OpenIdConfig {
    authorization_endpoint: Option<String>,
    token_endpoint: Option<String>,
    userinfo_endpoint: Option<String>,
}

impl OAuthConfig {
    pub async fn discover_endpoints(&self) -> (String, String, String) {
        let base = self.server_url.trim_end_matches('/');

        // 1. If explicit endpoints are provided in config, use them.
        let mut auth_ep = self.auth_url.clone();
        let mut token_ep = self.token_url.clone();
        let mut userinfo_ep = self.userinfo_url.clone();

        // 2. If any endpoint is missing, try OIDC discovery
        if auth_ep.is_none() || token_ep.is_none() || userinfo_ep.is_none() {
            let discovery_url = format!("{base}/.well-known/openid-configuration");
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build();

            if let Ok(client) = client {
                if let Ok(resp) = client.get(&discovery_url).send().await {
                    if resp.status().is_success() {
                        if let Ok(disc) = resp.json::<OpenIdConfig>().await {
                            if auth_ep.is_none() {
                                auth_ep = disc.authorization_endpoint;
                            }
                            if token_ep.is_none() {
                                token_ep = disc.token_endpoint;
                            }
                            if userinfo_ep.is_none() {
                                userinfo_ep = disc.userinfo_endpoint;
                            }
                        }
                    }
                }
            }
        }

        // 3. Fallbacks if discovery wasn't used/successful or for non-discovery standard Keycloak / OIDC servers
        let auth_final = auth_ep.unwrap_or_else(|| format!("{base}/protocol/openid-connect/auth"));
        let token_final = token_ep.unwrap_or_else(|| format!("{base}/protocol/openid-connect/token"));
        let userinfo_final = userinfo_ep.unwrap_or_else(|| format!("{base}/protocol/openid-connect/userinfo"));

        (auth_final, token_final, userinfo_final)
    }

    pub async fn get_authorization_url(&self) -> Result<String, OAuthError> {
        let (auth_ep, _, _) = self.discover_endpoints().await;

        let scope = self.scope.as_deref().unwrap_or("openid profile email");

        let mut url = reqwest::Url::parse(&auth_ep)
            .map_err(|e| OAuthError::Internal(format!("Invalid auth URL '{auth_ep}': {e}")))?;

        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("scope", scope);

        Ok(url.to_string())
    }

    pub async fn exchange_code(&self, code: &str) -> Result<TokenResponse, OAuthError> {
        let (_, token_ep, _) = self.discover_endpoints().await;

        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &self.redirect_uri),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
        ];

        let resp = client.post(&token_ep).form(&params).send().await?;

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED || resp.status() == reqwest::StatusCode::BAD_REQUEST {
            return Err(OAuthError::Unauthorized);
        }

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            warn!("Token endpoint returned status {status}: {body}");
            return Err(OAuthError::Internal(format!("Token exchange failed ({status})")));
        }

        let tokens = resp.json::<TokenResponse>().await?;
        Ok(tokens)
    }

    pub async fn get_userinfo(&self, token: &str) -> Result<UserInfo, OAuthError> {
        let (_, _, userinfo_ep) = self.discover_endpoints().await;

        let client = reqwest::Client::new();
        let resp = client
            .get(&userinfo_ep)
            .bearer_auth(token)
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED || resp.status() == reqwest::StatusCode::FORBIDDEN {
            return Err(OAuthError::Unauthorized);
        }

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            warn!("Userinfo endpoint returned status {status}: {body}");
            return Err(OAuthError::Internal(format!("Failed to fetch userinfo ({status})")));
        }

        let json_val: Value = resp.json().await?;

        let sub = json_val.get("sub").and_then(|v| v.as_str()).map(String::from);
        let name = json_val
            .get("name")
            .or_else(|| json_val.get("preferred_username"))
            .and_then(|v| v.as_str())
            .map(String::from);
        let email = json_val.get("email").and_then(|v| v.as_str()).map(String::from);

        let is_admin = check_is_admin(&json_val, &self.client_id);

        Ok(UserInfo {
            is_admin,
            sub,
            name,
            email,
            raw: json_val,
        })
    }
}

fn check_is_admin(json: &Value, client_id: &str) -> bool {
    // 1. Direct boolean flags
    if json.get("is_admin").and_then(|v| v.as_bool()).unwrap_or(false) {
        return true;
    }
    if json.get("admin").and_then(|v| v.as_bool()).unwrap_or(false) {
        return true;
    }

    // Helper to check array of roles/groups
    let is_admin_role = |r: &str| {
        let r = r.to_lowercase();
        r == "admin" || r == "administrator" || r == "board" || r.ends_with("/admin")
    };

    // 2. Roles array in root
    if let Some(roles) = json.get("roles").and_then(|v| v.as_array()) {
        if roles.iter().any(|role| role.as_str().map_or(false, is_admin_role)) {
            return true;
        }
    }

    // 3. Keycloak realm_access.roles
    if let Some(roles) = json
        .get("realm_access")
        .and_then(|v| v.get("roles"))
        .and_then(|v| v.as_array())
    {
        if roles.iter().any(|role| role.as_str().map_or(false, is_admin_role)) {
            return true;
        }
    }

    // 4. Keycloak resource_access.<client_id>.roles
    if let Some(roles) = json
        .get("resource_access")
        .and_then(|v| v.get(client_id))
        .and_then(|v| v.get("roles"))
        .and_then(|v| v.as_array())
    {
        if roles.iter().any(|role| role.as_str().map_or(false, is_admin_role)) {
            return true;
        }
    }

    // 5. Groups array
    if let Some(groups) = json.get("groups").and_then(|v| v.as_array()) {
        if groups.iter().any(|group| group.as_str().map_or(false, is_admin_role)) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_check_is_admin_koala_boolean() {
        let val = json!({ "is_admin": true, "sub": "123" });
        assert!(check_is_admin(&val, "client1"));

        let val_false = json!({ "is_admin": false, "sub": "123" });
        assert!(!check_is_admin(&val_false, "client1"));
    }

    #[test]
    fn test_check_is_admin_keycloak_realm_roles() {
        let val = json!({
            "sub": "user-uuid",
            "realm_access": {
                "roles": ["default-roles", "admin", "offline_access"]
            }
        });
        assert!(check_is_admin(&val, "client1"));

        let val_no_admin = json!({
            "sub": "user-uuid",
            "realm_access": {
                "roles": ["default-roles", "user"]
            }
        });
        assert!(!check_is_admin(&val_no_admin, "client1"));
    }

    #[test]
    fn test_check_is_admin_keycloak_resource_roles() {
        let val = json!({
            "sub": "user-uuid",
            "resource_access": {
                "my-client": {
                    "roles": ["administrator"]
                }
            }
        });
        assert!(check_is_admin(&val, "my-client"));
    }

    #[tokio::test]
    async fn test_get_authorization_url() {
        let config = OAuthConfig {
            server_url: "http://localhost:8082/realms/tavern".into(),
            client_id: "test-client".into(),
            client_secret: "secret".into(),
            redirect_uri: "http://localhost:8080/callback".into(),
            scope: None,
            auth_url: Some("http://localhost:8082/realms/tavern/protocol/openid-connect/auth".into()),
            token_url: None,
            userinfo_url: None,
        };

        let url = config.get_authorization_url().await.unwrap();
        assert!(url.contains("http://localhost:8082/realms/tavern/protocol/openid-connect/auth"));
        assert!(url.contains("client_id=test-client"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("scope=openid+profile+email") || url.contains("scope=openid"));
    }
}
