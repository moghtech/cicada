import { MoghAuthClient } from "mogh_auth_client";
import { ReadResponses, WriteResponses } from "./responses";
import { ReadRequest, WriteRequest } from "./types";

export * as MoghAuth from "mogh_auth_client";
export * as Types from "./types.js";
export type { ReadResponses, WriteResponses } from "./responses";

export type InitOptions =
  | { type: "jwt"; params: { jwt: string } }
  | { type: "api-key"; params: { key: string; secret: string } };

export type ClientState = {
  jwt: string | undefined;
  key: string | undefined;
  secret: string | undefined;
};

export function CicadaClient(url: string, options: InitOptions) {
  const state: ClientState = {
    jwt: options.type === "jwt" ? options.params.jwt : undefined,
    key: options.type === "api-key" ? options.params.key : undefined,
    secret: options.type === "api-key" ? options.params.secret : undefined,
  };

  const auth = MoghAuthClient(url + "/auth", state.jwt);

  const request = <Params, Res>(
    path: "/user" | "/read" | "/write",
    type: string,
    params: Params
  ): Promise<Res> =>
    new Promise(async (res, rej) => {
      try {
        let response = await fetch(`${url}${path}/${type}`, {
          method: "POST",
          body: JSON.stringify(params),
          headers: {
            ...(state.jwt
              ? {
                  authorization: state.jwt,
                }
              : state.key && state.secret
              ? {
                  "x-api-key": state.key,
                  "x-api-secret": state.secret,
                }
              : {}),
            "content-type": "application/json",
          },
          credentials: "include",
        });
        if (response.status === 200) {
          const body: Res = await response.json();
          res(body);
        } else {
          try {
            const result = await response.json();
            rej({ status: response.status, result });
          } catch (error) {
            rej({
              status: response.status,
              result: {
                error: "Failed to get response body",
                trace: [JSON.stringify(error)],
              },
              error,
            });
          }
        }
      } catch (error) {
        rej({
          status: 1,
          result: {
            error: "Request failed with error",
            trace: [JSON.stringify(error)],
          },
          error,
        });
      }
    });

  const read = async <
    T extends ReadRequest["type"],
    Req extends Extract<ReadRequest, { type: T }>
  >(
    type: T,
    params: Req["params"]
  ) =>
    await request<Req["params"], ReadResponses[Req["type"]]>(
      "/read",
      type,
      params
    );

  const write = async <
    T extends WriteRequest["type"],
    Req extends Extract<WriteRequest, { type: T }>
  >(
    type: T,
    params: Req["params"]
  ) =>
    await request<Req["params"], WriteResponses[Req["type"]]>(
      "/write",
      type,
      params
    );

  return {
    /**
     * Call the `/auth` api.
     *
     * ```
     * const { jwt } = await cicada.auth.login("LoginLocalUser", {
     *   username: "test-user",
     *   password: "test-pass"
     * });
     * ```
     *
     * https://docs.rs/mogh_auth_client/latest/mogh_auth_client/api/index.html
     */
    auth,
    /**
     * Call the `/read` api.
     *
     * ```
     * const stack = await cicada.read("GetNode", {
     *   id: "asdfasdfasdf"
     * });
     * ```
     *
     * https://docs.rs/cicada_client/latest/cicada_client/api/read/index.html
     */
    read,
    /**
     * Call the `/write` api.
     *
     * ```
     * const node = await komodo.write("UpdateNode", {
     *   id: "asdfasdfdsa",
     *   parent: 3,
     * });
     * ```
     *
     * https://docs.rs/cicada_client/latest/cicada_client/api/write/index.html
     */
    write,
  };
}
