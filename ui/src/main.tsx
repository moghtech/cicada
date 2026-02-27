import "@mantine/core/styles.css";
// ‼️ import notifications styles after core package styles
import "@mantine/notifications/styles.css";
import React from "react";
import ReactDOM from "react-dom/client";
import { MantineProvider } from "@mantine/core";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { DEFAULT_COLOR_SCHEME, theme } from "@/theme";
import { Router } from "@/router";
// Run monaco setup
import "@/monaco";
import { init_monaco } from "@/monaco/init";
import { Notifications } from "@mantine/notifications";

export const CICADA_BASE_URL =
  import.meta.env.VITE_CICADA_HOST ?? location.origin;
const client = new QueryClient({
  defaultOptions: { queries: { retry: false } },
});

// Don't need to await this to render.
init_monaco();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <MantineProvider theme={theme} defaultColorScheme={DEFAULT_COLOR_SCHEME}>
      <QueryClientProvider client={client}>
        <Router />
      </QueryClientProvider>
      <Notifications />
    </MantineProvider>
  </React.StrictMode>
);
