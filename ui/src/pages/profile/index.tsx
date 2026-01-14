import { useLoginOptions, useManageAuth, useUser } from "@/lib/hooks";
import {
  ActionIcon,
  Badge,
  Button,
  Center,
  Fieldset,
  Group,
  Loader,
  PasswordInput,
  Text,
  TextInput,
} from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { MoghAuth, Types } from "cicada_client";
import { CirclePlus, Save } from "lucide-react";
import { useMemo, useState } from "react";
import { EnrollPasskey } from "./passkey";
import { EnrollTotp } from "./totp";
import { CICADA_BASE_URL } from "@/main";
import { DataTable } from "@/components/data-table";
import ConfirmDelete from "@/components/confirm-delete";
import { ICONS } from "@/lib/icons";
import { Page } from "@/layout/page";
import { EnableSwitch } from "@/components/enable-switch";

const ProfilePage = () => {
  const user = useUser().data;

  if (!user) {
    return (
      <Center>
        <Loader size="xl" />
      </Center>
    );
  }

  return <ProfileInner user={user} />;
};

const useLinkWithOauth = () => {
  const { mutateAsync } = useManageAuth("BeginExternalLoginLink");
  return (provider: MoghAuth.Types.ExternalLoginProvider) =>
    mutateAsync({}).then(() =>
      location.replace(`${CICADA_BASE_URL}/auth/${provider.toLowerCase()}/link`)
    );
};

const ProfileInner = ({ user }: { user: Types.UserEntity }) => {
  const { refetch: refetchUser } = useUser();
  const options = useLoginOptions().data;
  const [username, setUsername] = useState(user.username);
  const [password, setPassword] = useState("");
  const { mutate: updateUsername } = useManageAuth("UpdateUsername", {
    onSuccess: () => {
      notifications.show({ message: "Username updated." });
      refetchUser();
    },
  });
  const { mutate: updatePassword } = useManageAuth("UpdatePassword", {
    onSuccess: () => {
      notifications.show({ message: "Password updated." });
      setPassword("");
      refetchUser();
    },
  });
  const { mutate: updateExternalSkip2fa } = useManageAuth(
    "UpdateExternalSkip2fa",
    {
      onSuccess: () => {
        notifications.show({
          message: "External login skip 2fa mode updated.",
        });
        refetchUser();
      },
    }
  );

  const { mutateAsync: unlink } = useManageAuth("UnlinkLogin", {
    onSuccess: () => {
      notifications.show({ message: "Unlinked login." });
      refetchUser();
    },
  });
  const loginProviders: Array<{
    provider: MoghAuth.Types.LoginProvider;
    enabled: boolean;
    linked: boolean;
  }> = useMemo(() => {
    const externalLoginKinds = user?.external_logins.map((login) => login.kind);
    return [
      {
        provider: "Local" as MoghAuth.Types.LoginProvider,
        enabled: !!options?.local,
        linked: !!user?.password,
      },
      {
        provider: "Oidc" as MoghAuth.Types.LoginProvider,
        enabled: !!options?.oidc,
        linked: externalLoginKinds.includes(Types.ExternalLoginKind.Oidc),
      },
      {
        provider: "Github" as MoghAuth.Types.LoginProvider,
        enabled: !!options?.github,
        linked: externalLoginKinds.includes(Types.ExternalLoginKind.Github),
      },
      {
        provider: "Google" as MoghAuth.Types.LoginProvider,
        enabled: !!options?.google,
        linked: externalLoginKinds.includes(Types.ExternalLoginKind.Google),
      },
    ].filter(({ enabled }) => enabled);
  }, [user, options]);
  const linkedCount = loginProviders.filter(({ linked }) => linked).length;
  const linkWithOauth = useLinkWithOauth();

  return (
    <Page title="Profile" icon={ICONS.User}>
      <Fieldset legend={<Text size="lg">Login</Text>}>
        <Group>
          <Text ff="monospace">Username:</Text>

          <TextInput
            placeholder="Update username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            w={250}
          />

          <ActionIcon
            onClick={() => updateUsername({ username })}
            disabled={!username || username === user.username}
          >
            <Save size="1rem" />
          </ActionIcon>
        </Group>

        {options?.local && (
          <Group mt="sm">
            <Text ff="monospace">Password:</Text>

            <PasswordInput
              placeholder="Update password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              w={250}
            />

            <ActionIcon
              onClick={() => updatePassword({ password })}
              disabled={!password}
            >
              <Save size="1rem" />
            </ActionIcon>
          </Group>
        )}
      </Fieldset>

      {!!loginProviders.length && (
        <Fieldset legend={<Text size="lg">Providers</Text>}>
          <DataTable
            tableKey="login-providers-v1"
            data={loginProviders}
            columns={[
              { header: "Provider", accessorKey: "provider" },
              {
                header: "Linked",
                cell: ({
                  row: {
                    original: { linked },
                  },
                }) => (
                  <Badge color={linked ? "green" : "red"}>
                    {linked ? "Linked" : "Unlinked"}
                  </Badge>
                ),
              },
              {
                header: "Link",
                cell: ({
                  row: {
                    original: { provider, linked },
                  },
                }) =>
                  linked ? (
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
                      variant="default"
                      onClick={() => linkWithOauth(provider as any)}
                      leftSection={<CirclePlus size="1rem" />}
                    >
                      Link {provider}
                    </Button>
                  ),
              },
            ]}
          />
        </Fieldset>
      )}

      <Fieldset legend={<Text size="lg">2FA</Text>}>
        <Group>
          <EnrollPasskey user={user} />
          <EnrollTotp user={user} />
          {(user.totp || user.passkey) && (
            <EnableSwitch
              label="Skip 2FA for external logins"
              checked={user.external_skip_2fa}
              onCheckedChange={(external_skip_2fa) =>
                updateExternalSkip2fa({ external_skip_2fa })
              }
            />
          )}
        </Group>
      </Fieldset>
    </Page>
  );
};

export default ProfilePage;
