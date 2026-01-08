import { useLogin, useLoginOptions, useUserInvalidate } from "@/lib/hooks";
import { sanitize_query } from "@/lib/utils";
import {
  Button,
  Center,
  Fieldset,
  PasswordInput,
  TextInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { MoghAuth } from "cicada_client";
import { useRef, useState } from "react";

export default function Login({
  passkeyIsPending: _passkeyIsPending,
  totpIsPending: _totpIsPending,
}: {
  passkeyIsPending?: boolean;
  totpIsPending?: boolean;
}) {
  const options = useLoginOptions().data;
  const userInvalidate = useUserInvalidate();
  const formRef = useRef<HTMLFormElement>(null);
  const totpFormRef = useRef<HTMLFormElement>(null);
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

  const { mutate: completeTotpLogin, isPending: totpPending } = useLogin(
    "CompleteTotpLogin",
    {
      onSuccess: secondFactorOnSuccess,
    }
  );

  const { mutate: completePasskeyLogin } = useLogin("CompletePasskeyLogin", {
    onSuccess: secondFactorOnSuccess,
  });

  const { mutate: login, isPending: loginPending } = useLogin("LoginLocalUser", {
    onSuccess: ({ type, data }) => {
      switch (type) {
        case "Jwt":
          return onSuccess(data);
        case "Passkey":
          setPasskeyPending(true);
          return navigator.credentials
            .get(MoghAuth.Passkey.preparePasskeyCredential(data))
            .then((credential) => completePasskeyLogin({ credential }))
            .catch((e) => {
              console.error(e);
              // toast({
              //   title: "Failed to select passkey",
              //   description: "See console for details",
              //   variant: "destructive",
              // });
            });
        case "Totp":
          return setTotpPending(true);
      }
    },
  });

  const getFormCredentials = () => {
    if (!formRef.current) return undefined;
    const fd = new FormData(formRef.current);
    const username = String(fd.get("username") ?? "");
    const password = String(fd.get("password") ?? "");
    return { username, password };
  };

  const handleLogin = () => {
    const creds = getFormCredentials();
    if (!creds) return;
    login(creds);
  };

  const handleSubmit = (e: any) => {
    e.preventDefault();
    handleLogin();
  };

  const handleSignUp = () => {
    const creds = getFormCredentials();
    if (!creds) return;
    signup(creds);
  };

  const getTotpFormCredentials = () => {
    if (!totpFormRef.current) return undefined;
    const fd = new FormData(totpFormRef.current);
    const code = String(fd.get("code") ?? "");
    return { code };
  };

  const handleTotpSubmit = (e: any) => {
    e.preventDefault();
    const creds = getTotpFormCredentials();
    if (!creds || !totpIsPending) return;
    completeTotpLogin({ code: creds.code });
  };

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

  return (
    <Center h="80vh">
      <Fieldset
        legend="Login"
        component="form"
        onSubmit={form.onSubmit((form) => login(form)) as any}
        style={{ display: "flex", flexDirection: "column", gap: "1rem" }}
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
        <Button type="submit" loading={loginPending}>
          Login
        </Button>
      </Fieldset>
    </Center>
  );
}
