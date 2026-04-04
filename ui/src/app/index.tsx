import { AppShell, Box } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { Outlet } from "react-router-dom";
import { Topbar } from "./topbar";
import { Sidebar } from "./sidebar";
import { Suspense } from "react";
import { LoadingScreen } from "mogh_ui";

export const TOPBAR_HEIGHT = 62;

export const App = () => {
  const [opened, { toggle, close }] = useDisclosure();
  return (
    <AppShell
      padding={{ base: "lg", sm: "xl" }}
      header={{ height: TOPBAR_HEIGHT }}
      navbar={{
        width: 300,
        breakpoint: "sm",
        collapsed: { mobile: !opened },
      }}
    >
      <Topbar opened={opened} toggle={toggle} />

      <AppShell.Navbar
        style={(theme) => {
          return {
            borderColor: theme.colors["accent-border"][1],
          };
        }}
      >
        <Sidebar close={close} />
      </AppShell.Navbar>

      <AppShell.Main>
        <Suspense fallback={<LoadingScreen />}>
          <Box px={{ xl: "xl" }}>
            <Outlet />
          </Box>
        </Suspense>
      </AppShell.Main>
    </AppShell>
  );
};
