import {Result} from "@/scripts/core/result";
import {ApiError} from "@/scripts/core/error";
import {fetch1} from "@/scripts/core/fetch1";
import {server} from "@/main";
import {ConfidentialAdvisor} from "@/scripts/config";

export class Report {
  static async report(
    message: string,
    toBoard: boolean,
    toAdvisors: ConfidentialAdvisor[],
    contactEmail: string | null
  ): Promise<Result<[], ApiError>> {
    const r = await fetch1(`${server}/api/report`, {
      method: "POST",
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        to_board: toBoard,
        to_confidential_advisors: toAdvisors.map(v => v.email),
        message: message,
        contact_address: contactEmail,
      })
    });
    
    if(r.isOk()) {
      return Result.ok([]);
    } else {
      return Result.err(r.unwrapErr());
    }
  }
}