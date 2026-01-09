import { useLoginOptions, useManageAuth, useUser } from "@/lib/hooks";
import {
  ActionIcon,
  Center,
  Fieldset,
  Flex,
  Group,
  Loader,
  PasswordInput,
  Text,
  TextInput,
} from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { Types } from "cicada_client";
import { Save } from "lucide-react";
import { useState } from "react";
import { EnrollPasskey } from "./passkey";
import { EnrollTotp } from "./totp";

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
  return (
    <Flex direction="column" gap="lg">
      <Fieldset legend="Login">
        <Group>
          <Text ff="monospace">Update Username:</Text>
          <TextInput
            placeholder="Input username"
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
            <Text ff="monospace">Update Password:</Text>
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

      <Fieldset legend="2FA">
        <Group>
          <EnrollPasskey user={user} />
          <EnrollTotp user={user} />
        </Group>
      </Fieldset>
    </Flex>
  );
};

export default ProfilePage;
