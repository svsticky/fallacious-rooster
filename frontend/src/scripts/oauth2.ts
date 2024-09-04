import {server} from "@/main";
import {ApiError} from "@/scripts/core/error";
import {fetch1} from "@/scripts/core/fetch1";
import {Result} from "@/scripts/core/result";

export class OAuth2Client {
  static async ok(): Promise<Result<boolean, ApiError>> {
    const r = await fetch1(`${server}/api/oauth/ok`);
    return r.map1(async (response) => {
      interface Json {
        is_admin: boolean;
      }

      const json: Json = await response.json();
      return json.is_admin;
    })
  }

  static async login(): Promise<Result<string, ApiError>> {
    const r = await fetch1(`${server}/api/oauth/login`);
    return r.map1(async (response) => {
      interface Json {
        location: string;
      }

      const json: Json = await response.json();
      return json.location;
    });
  }
}