import "./globals.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { ThemeProvider } from "@/ui/theme";
import { Toaster } from "@/ui/sonner";
import { useRead } from "./lib/hooks";

export const CICADA_BASE_URL =
  import.meta.env.VITE_CICADA_HOST ?? location.origin;
export const UPDATE_WS_URL =
  CICADA_BASE_URL.replace("http", "ws") + "/ws/update";
const client = new QueryClient({
  defaultOptions: { queries: { retry: false } },
});

const Testo = () => {
  const { data } = useRead("GetVersion", {});
  return <>{data?.version}</>;
};

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ThemeProvider>
      <QueryClientProvider client={client}>
        <Testo />
        <Toaster />
      </QueryClientProvider>
    </ThemeProvider>
  </StrictMode>
);
