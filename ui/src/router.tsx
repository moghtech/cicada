import { lazy } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Layout } from "./layout";

// Lazy import pages
const Filesystems = lazy(() => import("@/pages/filesystems"));
const Filesystem = lazy(() => import("@/pages/filesystem"));

export const Router = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route path="" element={<Filesystems />} />
          <Route path="filesystems/:filesystem" element={<Filesystem />} />
          <Route
            path="filesystems/:filesystem/:parent"
            element={<Filesystem />}
          />
        </Route>
      </Routes>
    </BrowserRouter>
  );
};
