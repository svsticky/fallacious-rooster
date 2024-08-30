import {Result} from "@/scripts/result";
import {ApiError} from "@/scripts/error";

export async function fetch1(input: RequestInfo | URL, init?: RequestInit): Promise<Result<Response, ApiError>> {
  if (init) {
    init.credentials = "include";
  } else {
    init = {
      credentials: "include"
    };
  }

  try {
    const r = await fetch(input, init);
    if (!r.ok) {
      return Result.err(ApiError.request(r.status, r.body));
    }

    return Result.ok(r);
  } catch (error) {
    return Result.err(ApiError.request(-1, error.toString()))
  }
}