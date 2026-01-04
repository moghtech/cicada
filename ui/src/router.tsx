import { lazy } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Layout } from "./layout";

// Lazy import pages
const Filesystems = lazy(() => import("@/pages/filesystems"));
const Node = lazy(() => import("@/pages/node"));

export const Router = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route path="" element={<Filesystems />} />
          <Route path="filesystems/:filesystem" element={<Node />} />
          <Route path="filesystems/:filesystem/:inode" element={<Node />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
};
