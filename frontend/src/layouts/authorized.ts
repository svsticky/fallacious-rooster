import {OAuth2Client} from "@/scripts/oauth2";
import {ApiError} from "@/scripts/core/error";
import {Result} from "@/scripts/core/result";

export class LoginResult {
  public readonly ok: boolean;
  public readonly isAdmin: boolean;
  
  constructor(ok: boolean, isAdmin: boolean) {
    this.ok = ok;
    this.isAdmin = isAdmin;
  }
}

export async function checkLogin(requireAdmin: boolean = false): Promise<Result<LoginResult, string>> {
  const result = await OAuth2Client.ok();
  if(result.isOk()) {
    
    if(requireAdmin && !result.unwrap()) {
      const loginLocation = await OAuth2Client.login();
      if(loginLocation.isOk()) {
        window.location.href = loginLocation.unwrap();
      } else {
        const error = loginLocation.unwrapErr();
        console.error(`Login redirect failed (${error.status}): ${error.message}`);
        return Result.err("Could not log you in. Please try again later");
      }
    }
    
    return Result.ok(new LoginResult(true, result.unwrap()))
  } else {
    const error: ApiError = result.unwrapErr();
    // noinspection FallThroughInSwitchStatementJS
    switch(error.status!) {
      case 401: {
        const loginLocation = await OAuth2Client.login();
        if(loginLocation.isOk()) {
          window.location.href = loginLocation.unwrap();
        } else {
          const error = loginLocation.unwrapErr();
          console.error(`Login redirect failed (${error.status}): ${error.message}`);
          return Result.err("Could not log you in. Please try again later");
        }
      }
      case 500:
      case 502: {
        console.error(`Login check failed (${error.status}): ${error.message}`);
        return Result.err("The server is having some troubles. Please try again later");
      }
      default: {
        console.error(`Login check failed (${error.status}): ${error.message}`);
        return Result.err("Could not log you in. Please try again later");
      }
    }
  }
}