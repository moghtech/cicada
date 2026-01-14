import ConfirmDelete from "@/components/confirm-delete";
import { DataTable } from "@/components/data-table";
import { useLoginOptions, useManageAuth } from "@/lib/hooks";
import { CICADA_BASE_URL } from "@/main";
import { Badge, Button, Fieldset, Text } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { MoghAuth, Types } from "cicada_client";
import { CirclePlus } from "lucide-react";
import { useMemo } from "react";

const useLinkWithExternalLogin = () => {
  const { mutateAsync } = useManageAuth("BeginExternalLoginLink");
  return (provider: MoghAuth.Types.ExternalLoginProvider) =>
    mutateAsync({}).then(() =>
      location.replace(`${CICADA_BASE_URL}/auth/${provider.toLowerCase()}/link`)
    );
};

export const LinkedLogins = ({
  user,
  refetchUser,
}: {
  user: Types.UserEntity;
  refetchUser: () => void;
}) => {
  const options = useLoginOptions().data;
  const loginProviders: Array<{
    provider: MoghAuth.Types.LoginProvider;
    enabled: boolean;
    data: string | undefined;
  }> = useMemo(() => {
    return [
      {
        provider: "Local" as MoghAuth.Types.LoginProvider,
        enabled: !!options?.local,
        data: user.password ? "########" : undefined,
      },
      {
        provider: "Oidc" as MoghAuth.Types.LoginProvider,
        enabled: !!options?.oidc,
        data: user.external_logins.find(
          (login) => login.kind === Types.ExternalLoginKind.Oidc
        )?.external_id,
      },
      {
        provider: "Github" as MoghAuth.Types.LoginProvider,
        enabled: !!options?.github,
        data: user.external_logins.find(
          (login) => login.kind === Types.ExternalLoginKind.Github
        )?.external_id,
      },
      {
        provider: "Google" as MoghAuth.Types.LoginProvider,
        enabled: !!options?.google,
        data: user.external_logins.find(
          (login) => login.kind === Types.ExternalLoginKind.Google
        )?.external_id,
      },
    ].filter(({ enabled }) => enabled);
  }, [user, options]);
  const linkedCount = loginProviders.filter(({ data }) => data).length;
  const linkWithExternalLogin = useLinkWithExternalLogin();
  const { mutateAsync: unlink } = useManageAuth("UnlinkLogin", {
    onSuccess: () => {
      notifications.show({ message: "Unlinked login." });
      refetchUser();
    },
  });

  if (!loginProviders.length) {
    return null;
  }

  return (
    <Fieldset legend={<Text size="lg">Providers</Text>}>
      <DataTable
        tableKey="login-providers-v1"
        data={loginProviders}
        columns={[
          {
            header: "Provider",
            accessorKey: "provider",
            cell: ({ row }) => <Text fw="bold">{row.original.provider}</Text>,
          },
          {
            header: "Linked",
            cell: ({
              row: {
                original: { data },
              },
            }) => (
              <Badge color={data ? "green.8" : "red"}>
                {data ? "Linked" : "Unlinked"}
              </Badge>
            ),
          },
          {
            header: "Data",
            cell: ({
              row: {
                original: { data },
              },
            }) =>
              data && (
                <Text
                  maw="20vw"
                  size="sm"
                  style={{
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                    textWrap: "nowrap",
                  }}
                >
                  {data}
                </Text>
              ),
          },
          {
            header: "Link",
            cell: ({
              row: {
                original: { provider, data },
              },
            }) =>
              data ? (
                linkedCount < 2 ? (
                  <>Must have at least 1 login linked.</>
                ) : (
                  <ConfirmDelete
                    action="Unlink"
                    name={provider}
                    entityType="Login"
                    onConfirm={() => unlink({ provider })}
                  />
                )
              ) : provider === "Local" ? (
                <>Set password above to enable.</>
              ) : (
                <Button
                  color="green.8"
                  onClick={() => linkWithExternalLogin(provider as any)}
                  leftSection={<CirclePlus size="1rem" />}
                >
                  Link {provider}
                </Button>
              ),
          },
        ]}
      />
    </Fieldset>
  );
};
