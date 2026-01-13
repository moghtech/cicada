import { AppShell, Center, Loader } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { Outlet } from "react-router-dom";
import { Topbar } from "./topbar";
import { Sidebar } from "./sidebar";
import { Suspense } from "react";

export const App = () => {
  const [opened, { toggle, close }] = useDisclosure();
  return (
    <AppShell
      padding="lg"
      header={{ height: 70 }}
      navbar={{
        width: 300,
        breakpoint: "sm",
        collapsed: { mobile: !opened },
      }}
    >
      <Topbar opened={opened} toggle={toggle} />

      <AppShell.Navbar>
        <Sidebar close={close} />
      </AppShell.Navbar>

      <AppShell.Main>
        <Suspense
          fallback={
            <Center h="70vh">
              <Loader size={60} />
            </Center>
          }
        >
          <Outlet />
        </Suspense>
      </AppShell.Main>
    </AppShell>
  );
};
