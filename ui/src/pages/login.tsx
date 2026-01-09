import { useLogin, useLoginOptions, useUserInvalidate } from "@/lib/hooks";
import { sanitize_query } from "@/lib/utils";
import {
  Button,
  Center,
  Fieldset,
  Flex,
  PasswordInput,
  SimpleGrid,
  Text,
  TextInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { notifications } from "@mantine/notifications";
import { MoghAuth } from "cicada_client";
import { useState } from "react";

export default function Login({
  passkeyIsPending: _passkeyIsPending,
  totpIsPending: _totpIsPending,
}: {
  passkeyIsPending?: boolean;
  totpIsPending?: boolean;
}) {
  const options = useLoginOptions().data;
  const userInvalidate = useUserInvalidate();
  const [passkeyIsPending, setPasskeyPending] = useState(
    _passkeyIsPending ?? false
  );
  const [totpIsPending, setTotpPending] = useState(_totpIsPending ?? false);
  const secondFactorPending = passkeyIsPending || totpIsPending;

  // If signing in another user, need to redirect away from /login manually
  const maybeNavigate = location.pathname.startsWith("/login")
    ? () =>
        location.replace(
          new URLSearchParams(location.search).get("backto") ?? "/"
        )
    : undefined;

  const onSuccess = ({ jwt }: MoghAuth.Types.JwtResponse) => {
    MoghAuth.LOGIN_TOKENS.add_and_change(jwt);
    userInvalidate();
    maybeNavigate?.();
  };

  const secondFactorOnSuccess = (res: MoghAuth.Types.JwtResponse) => {
    sanitize_query();
    onSuccess(res);
  };

  const { mutate: signup, isPending: signupPending } = useLogin(
    "SignUpLocalUser",
    {
      onSuccess,
    }
  );

  const { mutate: completePasskeyLogin } = useLogin("CompletePasskeyLogin", {
    onSuccess: secondFactorOnSuccess,
  });

  // const { mutate: completeTotpLogin, isPending: totpPending } = useLogin(
  //   "CompleteTotpLogin",
  //   {
  //     onSuccess: secondFactorOnSuccess,
  //   }
  // );

  const { mutate: login, isPending: loginPending } = useLogin(
    "LoginLocalUser",
    {
      onSuccess: ({ type, data }) => {
        switch (type) {
          case "Jwt":
            return onSuccess(data);
          case "Passkey":
            setPasskeyPending(true);
            return navigator.credentials
              .get(MoghAuth.Passkey.prepareRequestChallengeResponse(data))
              .then((credential) => completePasskeyLogin({ credential }))
              .catch((e) => {
                console.error(e);
                notifications.show({
                  title: "Failed to select passkey",
                  message: "See console for details",
                  color: "red",
                });
              });
          case "Totp":
            return setTotpPending(true);
        }
      },
    }
  );

  const noAuthConfigured =
    options !== undefined &&
    Object.values(options).every((value) => value === false);

  const showSignUp = options !== undefined && !options.registration_disabled;

  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      username: "",
      password: "",
    },
    validate: {
      username: (username) =>
        username.length ? null : "Username cannot be empty",
      password: (password) =>
        password.length ? null : "Password cannot be empty",
    },
  });

  const registration_disabled = options?.registration_disabled ?? true;

  return (
    <Center h="80vh">
      <Fieldset
        legend={
          <Flex gap="sm" align="center">
            <img
              src="/mogh-512x512.png"
              width={32}
              height={32}
              alt="moghtech"
            />
            <Flex direction="column">
              <Text size="xl" fw="bold">
                Cicada
              </Text>
              <Text size="sm">Log In</Text>
            </Flex>
          </Flex>
        }
        component="form"
        onSubmit={form.onSubmit((form) => login(form)) as any}
        style={{ display: "flex", flexDirection: "column", gap: "1rem" }}
        w={400}
      >
        <TextInput
          {...form.getInputProps("username")}
          autoFocus
          label="Username"
          placeholder="Enter username"
          key={form.key("username")}
        />
        <PasswordInput
          {...form.getInputProps("password")}
          label="Password"
          placeholder="Enter password"
          key={form.key("password")}
        />
        <SimpleGrid cols={registration_disabled ? 1 : 2} mt="sm">
          {!registration_disabled && (
            <Button
              variant="default"
              onClick={form.onSubmit((form) => signup(form)) as any}
              loading={signupPending}
            >
              Sign Up
            </Button>
          )}
          <Button variant="filled" type="submit" loading={loginPending}>
            Login
          </Button>
        </SimpleGrid>
      </Fieldset>
    </Center>
  );
}
