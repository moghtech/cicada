import { LoginPage } from "mogh_ui";
import { useUserInvalidate } from "@/lib/hooks";

export default function Login(props: {
  passkeyIsPending?: boolean;
  totpIsPending?: boolean;
}) {
  const userInvalidate = useUserInvalidate();
  return (
    <LoginPage
      {...props}
      appName="CICADA"
      iconLink="/mogh-512x512.png"
      iconLinkAlt="moghtech"
      exampleConfigLink="https://github.com/moghtech/cicada/blob/main/config/core.config.toml"
      onLogin={userInvalidate}
    />
  );
}
