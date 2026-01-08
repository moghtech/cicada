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
import { sanitize_query_inner } from "./utils";

export const cicada_client = () =>
  CicadaClient(CICADA_BASE_URL, {
    type: "jwt",
    params: { jwt: MoghAuth.LOGIN_TOKENS.jwt() },
  });

export const useLoginOptions = () => {
  return useQuery({
    queryKey: ["GetLoginOptions"],
    queryFn: () => cicada_client().auth.login("GetLoginOptions", {}),
  });
};

export const useLogin = <
  T extends MoghAuth.Types.LoginRequest["type"],
  R extends Extract<MoghAuth.Types.LoginRequest, { type: T }>,
  P extends R["params"],
  C extends Omit<
    UseMutationOptions<MoghAuth.LoginResponses[T], unknown, P, unknown>,
    "mutationKey" | "mutationFn"
  >,
>(
  type: T,
  config?: C
) => {
  // const { toast } = useToast();
  return useMutation({
    mutationKey: [type],
    mutationFn: (params: P) => cicada_client().auth.login<T, R>(type, params),
    onError: (e: { result: { error?: string; trace?: string[] } }, v, r, c) => {
      console.log("Auth error:", e);
      const msg = e.result.error ?? "Unknown error. See console.";
      const detail = e.result?.trace
        ?.map((msg) => msg[0].toUpperCase() + msg.slice(1))
        .join(" | ");
      let msg_log = msg ? msg[0].toUpperCase() + msg.slice(1) + " | " : "";
      if (detail) {
        msg_log += detail + " | ";
      }
      // toast({
      //   title: `Auth request ${type} failed`,
      //   description: `${msg_log}See console for details`,
      //   variant: "destructive",
      // });
      config?.onError && config.onError(e, v, r, c);
    },
    ...config,
  });
};

let jwt_redeem_sent = false;
let passkey_sent = false;

/// returns whether to show login / loading screen depending on state of exchange token loop
export const useAuthState = () => {
  const onSuccess = ({ jwt }: MoghAuth.Types.JwtResponse) => {
    MoghAuth.LOGIN_TOKENS.add_and_change(jwt);
    sanitize_query_inner(search);
  };
  const { mutate: redeemJwt } = useLogin("ExchangeForJwt", {
    onSuccess,
  });
  const { mutate: completePasskeyLogin } = useLogin("CompletePasskeyLogin", {
    onSuccess,
  });
  const search = new URLSearchParams(location.search);

  const _passkey = search.get("passkey");
  const passkey = _passkey ? JSON.parse(_passkey) : null;

  // guard against multiple reqs sent
  // maybe isPending would do this but not sure about with render loop, this for sure will.
  if (passkey && !passkey_sent) {
    navigator.credentials
      .get(MoghAuth.Passkey.preparePasskeyCredential(passkey))
      .then((credential) => completePasskeyLogin({ credential }))
      .catch((e) => {
        console.error(e);
        // toast({
        //   title: "Failed to select passkey",
        //   description: "See console for details",
        //   variant: "destructive",
        // });
      });
    passkey_sent = true;
  }

  const jwt_redeem_ready = search.get("redeem_ready") === "true";

  // guard against multiple reqs sent
  // maybe isPending would do this but not sure about with render loop, this for sure will.
  if (jwt_redeem_ready && !jwt_redeem_sent) {
    redeemJwt({});
    jwt_redeem_sent = true;
  }

  return {
    jwt_redeem_ready,
    passkey_pending: !!passkey,
    totp: search.get("totp") === "true",
  };
};

export const useUser = () => {
  const userReset = useUserReset();
  const hasJwt = !!MoghAuth.LOGIN_TOKENS.jwt();

  const query = useQuery({
    queryKey: ["GetUser"],
    queryFn: () => cicada_client().read("GetUser", {}),
    refetchInterval: 30_000,
    enabled: hasJwt,
  });

  useEffect(() => {
    if (query.data && query.error) {
      userReset();
    }
  }, [query.data, query.error]);

  return query;
};

export const useUserInvalidate = () => {
  const qc = useQueryClient();
  return () => {
    qc.invalidateQueries({ queryKey: ["GetUser"] });
  };
};

export const useUserReset = () => {
  const qc = useQueryClient();
  return () => {
    qc.resetQueries({ queryKey: ["GetUser"] });
  };
};

//

export const useRead = <
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
>(
  type: T,
  params: P,
  config?: C
) => {
  // const hasJwt = !!LOGIN_TOKENS.jwt();
  return useQuery({
    queryKey: [type, params],
    queryFn: () => cicada_client().read<T, R>(type, params),
    // enabled: hasJwt && config?.enabled !== false,
    enabled: config?.enabled !== false,
    ...config,
  });
};

export const useInvalidate = () => {
  const qc = useQueryClient();
  return <
    Type extends Types.ReadRequest["type"],
    Params extends Extract<Types.ReadRequest, { type: Type }>["params"],
  >(
    ...keys: Array<[Type] | [Type, Params]>
  ) => keys.forEach((key) => qc.invalidateQueries({ queryKey: key }));
};

//

export const useWrite = <
  T extends Types.WriteRequest["type"],
  R extends Extract<Types.WriteRequest, { type: T }>,
  P extends R["params"],
  C extends Omit<
    UseMutationOptions<WriteResponses[R["type"]], unknown, P, unknown>,
    "mutationKey" | "mutationFn"
  >,
>(
  type: T,
  config?: C
) => {
  // const { toast } = useToast();
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
      // toast({
      //   title: `Write request ${type} failed`,
      //   description: `${msg_log}See console for details`,
      //   variant: "destructive",
      // });
      config?.onError && config.onError(e, v, r, c);
    },
    ...config,
  });
};
