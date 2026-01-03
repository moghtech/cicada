import { lazy, Suspense } from "react";
import { BrowserRouter, Outlet, Route, Routes } from "react-router-dom";
import { AppShell, Burger, Flex, Loader } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";

// Lazy import pages
const Filesystems = lazy(() => import("@/pages/filesystems"));
const Filesystem = lazy(() => import("@/pages/filesystem"));

export const Router = () => {
  return (
    <Suspense
      fallback={
        <Flex justify="center" align="center" w="100vw" h="100vh">
          <Loader />
        </Flex>
      }
    >
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Layout />}>
            <Route path="" element={<Filesystems />} />
            <Route path="filesystems/:id/:parent" element={<Filesystem />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </Suspense>
  );
};

const Layout = () => {
  const [opened, { toggle }] = useDisclosure();
  return (
    <AppShell
      padding="md"
      header={{ height: 60 }}
      navbar={{
        width: 300,
        breakpoint: "sm",
        collapsed: { mobile: !opened },
      }}
    >
      <AppShell.Header>
        <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm" />

        <div>Logo</div>
      </AppShell.Header>

      <AppShell.Navbar>Navbar</AppShell.Navbar>

      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
};
