import { AppShell, Burger, Button, Flex, Text } from "@mantine/core";
import { useNavigate } from "react-router-dom";
import { UserDropdown } from "./user-dropdown";
import { ThemeToggle } from "./theme-toggle";

export const Topbar = ({
  opened,
  toggle,
}: {
  opened: boolean;
  toggle: () => void;
}) => {
  const nav = useNavigate();
  return (
    <AppShell.Header
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
      }}
      px="1rem"
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
        <ThemeToggle />
        <UserDropdown />
      </Flex>
    </AppShell.Header>
  );
};
