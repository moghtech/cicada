import { useLoginOptions, useManageAuth, useUser } from "@/lib/hooks";
import {
  ActionIcon,
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

const ProfileInner = ({ user }: { user: Types.UserRecord }) => {
  const { refetch: refetchUser } = useUser();
  const options = useLoginOptions().data;
  const [username, setUsername] = useState(user.name);
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
  }> = useMemo(
    () =>
      [
        {
          provider: "Local" as MoghAuth.Types.LoginProvider,
          enabled: !!options?.local,
          linked: !!user?.password,
        },
        {
          provider: "Oidc" as MoghAuth.Types.LoginProvider,
          enabled: !!options?.oidc,
          linked: !!user?.oidc_subject,
        },
        {
          provider: "Github" as MoghAuth.Types.LoginProvider,
          enabled: !!options?.github,
          linked: !!user?.github_id,
        },
        {
          provider: "Google" as MoghAuth.Types.LoginProvider,
          enabled: !!options?.google,
          linked: !!user?.google_id,
        },
      ].filter(({ enabled }) => enabled),
    [user, options]
  );
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
            disabled={!username || username === user.name}
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
                  <Button color={linked ? "green" : "red"}>
                    {linked ? "Linked" : "Unlinked"}
                  </Button>
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
                      disabled={
                        provider === "Oidc" ? !!user.oidc_subject : false
                      }
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
        </Group>
      </Fieldset>
    </Page>
  );
};

export default ProfilePage;
