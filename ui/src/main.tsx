import "@mantine/core/styles.css";
import React from "react";
import ReactDOM from "react-dom/client";
import { MantineProvider } from "@mantine/core";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { theme } from "@/theme";
import { Router } from "./router";

export const CICADA_BASE_URL =
  import.meta.env.VITE_CICADA_HOST ?? location.origin;
export const UPDATE_WS_URL =
  CICADA_BASE_URL.replace("http", "ws") + "/ws/update";
const client = new QueryClient({
  defaultOptions: { queries: { retry: false } },
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <MantineProvider theme={theme}>
      <QueryClientProvider client={client}>
        <Router />
      </QueryClientProvider>
    </MantineProvider>
  </React.StrictMode>
);
