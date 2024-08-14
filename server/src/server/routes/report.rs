use crate::email::send_report;
use crate::email::template::{
    render_report_board, render_report_confidential_advisors, ReportTemplate,
};
use crate::server::types::{Authorization, Empty, Error, WConfig, WResult, WStorage};
use actix_web::web;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReportRequest {
    to_board: bool,
    to_confidential_advisors: Vec<String>,
    message: String,
    want_contact: bool,
}

pub async fn report(
    wconfig: WConfig,
    wstorage: WStorage,
    auth: Authorization,
    payload: web::Json<ReportRequest>,
) -> WResult<Empty> {
    let payload = payload.into_inner();

    // Verify all provided advisors actually exist
    let storage = wstorage.0.read().await;

    let advisors_exist = payload
        .to_confidential_advisors
        .iter()
        .all(|advisor| storage.confidential_advisors.contains(advisor));

    if !advisors_exist {
        // Request contains advisors who aren't on the list
        return Err(Error::AdvisorDoesNotExist);
    }

    let template = ReportTemplate {
        message: payload.message,
        full_name: payload.want_contact.then(|| auth.full_name),
        want_contact: payload.want_contact,
    };

    // Send to confidential advisors if needed
    if !payload.to_confidential_advisors.is_empty() {
        let body = render_report_confidential_advisors(&template)?;
        send_report(&wconfig.email, payload.to_confidential_advisors, body).await?;
    }

    // Send to board if needed
    if payload.to_board {
        let body = render_report_board(&template)?;
        send_report(&wconfig.email, vec![storage.board.clone()], body).await?;
    }

    Ok(Empty)
}
