import { ThemeToggle } from "@/theme";
import { AppShell, Burger, Flex, Text } from "@mantine/core";
import { Link } from "react-router-dom";

export const Topbar = ({
  opened,
  toggle,
}: {
  opened: boolean;
  toggle: () => void;
}) => {
  return (
    <AppShell.Header
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        padding: "0rem 2rem",
      }}
    >
      <Flex align="center" gap="md">
        <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm" />
        <Link to="/" style={{ textDecoration: "none", color: "inherit" }}>
          <Flex align="center" gap="sm">
            <img src="/mogh-512x512.png" width={32} alt="moghtech" />
            <Text size="xl">CICADA</Text>
          </Flex>
        </Link>
      </Flex>
      <Flex align="center" gap="md">
        <ThemeToggle />
      </Flex>
    </AppShell.Header>
  );
};
