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
  type ReadResponses,
  type WriteResponses,
} from "cicada_client";

export const cicada_client = () => CicadaClient(CICADA_BASE_URL);

export const useLoginOptions = () => {
  return useQuery({
    queryKey: ["GetLoginOptions"],
    queryFn: () => cicada_client().auth("GetLoginOptions", {}),
  });
};

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
