import { CICADA_BASE_URL } from "@/main";
import {
  useMutation,
  useQuery,
  useQueryClient,
  type UseMutationOptions,
  type UseQueryOptions,
} from "@tanstack/react-query";
import {
  CicadaClient,
  Types,
  MoghAuth,
  type ReadResponses,
  type WriteResponses,
} from "cicada_client";
import { useEffect } from "react";
import { notifications } from "@mantine/notifications";

export function cicada_client() {
  return CicadaClient(CICADA_BASE_URL, {
    type: "jwt",
    params: { jwt: MoghAuth.LOGIN_TOKENS.jwt() },
  });
}

export function useUser() {
  const userReset = useUserReset();
  const hasJwt = !!MoghAuth.LOGIN_TOKENS.jwt();

  const query = useQuery({
    queryKey: ["GetUser"],
    queryFn: () => cicada_client().getUser(),
    refetchInterval: 30_000,
    enabled: hasJwt,
  });

  useEffect(() => {
    if (query.data && query.error) {
      userReset();
    }
  }, [query.data, query.error]);

  return query;
}

export function useUserInvalidate() {
  const qc = useQueryClient();
  return () => {
    qc.invalidateQueries({ queryKey: ["GetUser"] });
  };
}

export function useUserReset() {
  const qc = useQueryClient();
  return () => {
    qc.resetQueries({ queryKey: ["GetUser"] });
  };
}

//

export function useRead<
  T extends Types.ReadRequest["type"],
  R extends Extract<Types.ReadRequest, { type: T }>,
  P extends R["params"],
  C extends Omit<
    UseQueryOptions<
      ReadResponses[R["type"]],
      unknown,
      ReadResponses[R["type"]],
      (T | P)[]
    >,
    "queryFn" | "queryKey"
  >,
>(type: T, params: P, config?: C) {
  const hasJwt = !!MoghAuth.LOGIN_TOKENS.jwt();
  return useQuery({
    queryKey: [type, params],
    queryFn: () => cicada_client().read<T, R>(type, params),
    enabled: hasJwt && config?.enabled !== false,
    ...config,
  });
}

export function useInvalidate() {
  const qc = useQueryClient();
  return <
    Type extends Types.ReadRequest["type"],
    Params extends Extract<Types.ReadRequest, { type: Type }>["params"],
  >(
    ...keys: Array<[Type] | [Type, Params]>
  ) => keys.forEach((key) => qc.invalidateQueries({ queryKey: key }));
}

//

export function useWrite<
  T extends Types.WriteRequest["type"],
  R extends Extract<Types.WriteRequest, { type: T }>,
  P extends R["params"],
  C extends Omit<
    UseMutationOptions<WriteResponses[R["type"]], unknown, P, unknown>,
    "mutationKey" | "mutationFn"
  >,
>(type: T, config?: C) {
  return useMutation({
    mutationKey: [type],
    mutationFn: (params: P) => cicada_client().write<T, R>(type, params),
    onError: (e: { result: { error?: string; trace?: string[] } }, v, r, c) => {
      console.log("Write error:", e);
      const msg = e.result.error ?? "Unknown error. See console.";
      const detail = e.result?.trace
        ?.map((msg) => msg[0].toUpperCase() + msg.slice(1))
        .join(" | ");
      let msg_log = msg ? msg[0].toUpperCase() + msg.slice(1) + " | " : "";
      if (detail) {
        msg_log += detail + " | ";
      }
      notifications.show({
        title: `Write request ${type} failed`,
        message: `${msg_log}See console for details`,
        color: "red",
      });
      config?.onError && config.onError(e, v, r, c);
    },
    ...config,
  });
}

export function useSetTitle(more?: string) {
  const info = useRead("GetCoreInfo", {}).data;
  const title = more ? `${more} | ${info?.title}` : info?.title;
  useEffect(() => {
    if (title) {
      document.title = title;
    }
  }, [title]);
}
