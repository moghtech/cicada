import { lazy } from "react";
import {
  BrowserRouter,
  Navigate,
  Outlet,
  Route,
  Routes,
  useLocation,
} from "react-router-dom";
import { Layout } from "./layout";
import { useAuthState, useUser } from "./lib/hooks";
import { Center, Loader } from "@mantine/core";
import { MoghAuth } from "cicada_client";

// Lazy import pages
const Login = lazy(() => import("@/pages/login"));
const Profile = lazy(() => import("@/pages/profile"));
const Filesystems = lazy(() => import("@/pages/filesystems"));
const Node = lazy(() => import("@/pages/node"));
const UserDisabled = lazy(() => import("@/pages/user-disabled"));

export const Router = () => {
  // Handle exchange token loop to avoid showing login flash
  const { jwt_redeem_ready, passkey_pending, totp } = useAuthState();

  if (jwt_redeem_ready) {
    return (
      <Center>
        <Loader size="xl" />
      </Center>
    );
  }

  if (passkey_pending || totp) {
    return <Login passkeyIsPending={passkey_pending} totpIsPending={totp} />;
  }

  return (
    <BrowserRouter>
      <Routes>
        <Route path="login" element={<Login />} />
        <Route element={<RequireAuth />}>
          <Route path="/" element={<Layout />}>
            <Route path="" element={<Filesystems />} />
            <Route path="filesystems/:filesystem" element={<Node />} />
            <Route path="filesystems/:filesystem/:inode" element={<Node />} />
            <Route path="profile" element={<Profile />} />
          </Route>
        </Route>
      </Routes>
    </BrowserRouter>
  );
};

const RequireAuth = () => {
  const { data: user, error } = useUser();
  const location = useLocation();

  if (
    (error as { error?: TypeError } | undefined)?.error?.message?.startsWith(
      "NetworkError"
    )
  ) {
    // Will just show the spinner without navigate to login,
    // which won't help because its not a login issue.
    return (
      <Center>
        <Loader size="xl" />
      </Center>
    );
  }

  if (!MoghAuth.LOGIN_TOKENS.jwt() || error) {
    if (location.pathname === "/") {
      return <Navigate to="/login" replace />;
    }
    const backto = encodeURIComponent(location.pathname + location.search);
    return <Navigate to={`/login?backto=${backto}`} replace />;
  }

  if (!user) {
    return (
      <Center>
        <Loader size="xl" />
      </Center>
    );
  }

  if (!user.enabled) return <UserDisabled />;

  return <Outlet />;
};
