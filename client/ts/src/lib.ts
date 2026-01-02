import { ReadResponses, WriteResponses } from "./responses";
import { ReadRequest, WriteRequest } from "./types";

export * as Types from "./types.js";
export type { ReadResponses, WriteResponses } from "./responses";

export function CicadaClient(url: string) {
  const request = <Params, Res>(
    path: "/read" | "/write",
    type: string,
    params: Params
  ): Promise<Res> =>
    new Promise(async (res, rej) => {
      try {
        let response = await fetch(`${url}${path}/${type}`, {
          method: "POST",
          body: JSON.stringify(params),
          headers: {
            // ...(state.jwt
            //   ? {
            //       authorization: state.jwt,
            //     }
            //   : state.key && state.secret
            //   ? {
            //       "x-api-key": state.key,
            //       "x-api-secret": state.secret,
            //     }
            //   : {}),
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
