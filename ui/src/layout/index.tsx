import { AppShell } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { Outlet } from "react-router-dom";
import { Topbar } from "./topbar";
import { Sidebar } from "./sidebar";

export const Layout = () => {
  const [opened, { toggle }] = useDisclosure();
  return (
    <AppShell
      padding="md"
      header={{ height: 80 }}
      navbar={{
        width: 300,
        breakpoint: "sm",
        collapsed: { mobile: !opened },
      }}
    >
      <Topbar opened={opened} toggle={toggle} />

      <AppShell.Navbar>
        <Sidebar />
      </AppShell.Navbar>

      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
};
