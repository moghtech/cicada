import React from "react";
import ReactDOM from "react-dom/client";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Router } from "@/router";
import { Notifications } from "@mantine/notifications";
import { setAuthUrl, ThemeProvider } from "mogh_ui";

import "@mantine/core/styles.css";
// ‼️ import notifications styles after core package styles
import "@mantine/notifications/styles.css";
// Import local css after to avoid mantine default body color flash.
import "./index.scss";
// Import mogh_ui scss
import "mogh_ui/index.scss";

export const CICADA_BASE_URL =
  import.meta.env.VITE_CICADA_HOST ?? location.origin;
const client = new QueryClient({
  defaultOptions: { queries: { retry: false } },
});

setAuthUrl(CICADA_BASE_URL + "/auth");

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={client}>
      <ThemeProvider>
        <Router />
        <Notifications />
      </ThemeProvider>
    </QueryClientProvider>
  </React.StrictMode>,
);
