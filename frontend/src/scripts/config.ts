import {Result} from "@/scripts/core/result";
import {ApiError} from "@/scripts/core/error";
import {fetch1} from "@/scripts/core/fetch1";
import {server} from "@/main";

export class ConfidentialAdvisor {
  public readonly name: string;
  public readonly email: string;
  
  constructor(name: string, email: string) {
    this.name = name;
    this.email = email;
  }
  
  static async list(): Promise<Result<ConfidentialAdvisor[], ApiError>> {
    const r = await fetch1(`${server}/api/config/confidential-advisors`);
    return r.map1(async (response) => {
      interface Json {
        advisors: {
          name: string,
          email: string,
        }[]
      }
      
      const json: Json = await response.json();
      return json.advisors.map((adv) => new ConfidentialAdvisor(adv.name, adv.email))
    });
  }
}