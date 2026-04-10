import { AppShell, Burger, Button, Flex, Group, Text } from "@mantine/core";
import { useNavigate } from "react-router-dom";
import { ThemeToggle } from "mogh_ui";
import UserDropdown from "./user-dropdown";
import { useRead } from "@/lib/hooks";
import TopbarLink from "./link";

export default function Topbar({
  opened,
  toggle,
}: {
  opened: boolean;
  toggle: () => void;
}) {
  const nav = useNavigate();
  const version = useRead("GetVersion", {}, { refetchInterval: 30_000 }).data
    ?.version;
  return (
    <AppShell.Header
      renderRoot={(props) => (
        <Group justify="space-between" wrap="nowrap" {...props} />
      )}
      style={(theme) => ({
        borderColor: theme.colors["accent-border"][1],
      })}
      bg="accent.1"
      pl="1.3rem"
      pr="2rem"
      py="0rem"
    >
      <Flex align="center" gap="md">
        <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm" />
        <Button
          variant="subtle"
          c="inherit"
          leftSection={
            <img src="/mogh-512x512.png" width={32} alt="moghtech" />
          }
          onClick={() => nav("/")}
        >
          <Text fz="h2" fw="450" lts="0.1rem">
            CICADA
          </Text>
        </Button>
      </Flex>
      <Flex align="center" gap="0.3rem">
        {version && (
          <TopbarLink to="https://github.com/moghtech/cicada/releases">
            v{version}
          </TopbarLink>
        )}
        <ThemeToggle />
        <UserDropdown />
      </Flex>
    </AppShell.Header>
  );
}
