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
  
  static async newAdvisor(name: string, email: string): Promise<Result<[], ApiError>> {
    const r = await fetch1(`${server}/api/config/confidential-advisors`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        name: name,
        email: email,
      })
    });
    
    if(r.isOk()) {
      return Result.ok([]);
    } else {
      return Result.err(r.unwrapErr());
    }
  }
  
  async deleteAdvisor(): Promise<Result<[], ApiError>> {
    const r = await fetch1(`${server}/api/config/confidential-advisors`, {
      method: 'DELETE',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        email: this.email,
      })
    });
    
    if(r.isOk()) {
      return Result.ok([]);
    } else {
      return Result.err(r.unwrapErr());
    }
  }
}

export class Board {
  static async getEmail(): Promise<Result<string, ApiError>> {
    const r = await fetch1(`${server}/api/config/board`);
    return r.map1(async (response) => {
      interface Json {
        email: string,
      }
      
      const json: Json = await response.json();
      return json.email;
    });
  }
  
  static async updateEmail(email: string): Promise<Result<[], ApiError>> {
    const r = await fetch1(`${server}/api/config/board`, {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        email: email
      })
    });
    
    if(r.isOk()) {
      return Result.ok([]);
    } else {
      return Result.err(r.unwrapErr());
    }
  }
}