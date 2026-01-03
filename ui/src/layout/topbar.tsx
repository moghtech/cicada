import {
  ActionIcon,
  AppShell,
  Burger,
  Flex,
  MantineColorScheme,
  Menu,
  useComputedColorScheme,
  useMantineColorScheme,
  Text,
} from "@mantine/core";
import { CheckCircle, Moon, Sun } from "lucide-react";
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

const ThemeToggle = () => {
  const { colorScheme, setColorScheme } = useMantineColorScheme();
  return (
    <Menu position="bottom-end">
      <Menu.Target>
        <ActionIcon
          aria-label="ThemeToggle"
          variant="outline"
          size="lg"
          style={{ cursor: "pointer" }}
        >
          <ThemeIcon />
        </ActionIcon>
      </Menu.Target>
      <Menu.Dropdown>
        {["light", "dark", "auto"].map((theme) => (
          <Menu.Item
            key={theme}
            onClick={() => setColorScheme(theme as MantineColorScheme)}
            style={{ cursor: "pointer", textTransform: "capitalize" }}
            rightSection={
              colorScheme === theme ? <CheckCircle size="0.8rem" /> : undefined
            }
          >
            {theme}
          </Menu.Item>
        ))}
      </Menu.Dropdown>
    </Menu>
  );
};

const ThemeIcon = () => {
  const computedColorScheme = useComputedColorScheme();
  const dark = computedColorScheme === "dark";
  return (
    <>
      <Sun
        color="black"
        size="1.3rem"
        style={{
          display: dark ? "none" : undefined,
          cursor: "pointer",
        }}
      />
      <Moon
        color="white"
        size="1.3rem"
        style={{
          display: dark ? undefined : "none",
          cursor: "pointer",
        }}
      />
    </>
  );
};
