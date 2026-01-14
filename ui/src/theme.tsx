import { colorsTuple, createTheme, MantineColorScheme, virtualColor } from "@mantine/core";

export const DEFAULT_COLOR_SCHEME: MantineColorScheme = "auto";

export const theme = createTheme({
  cursorType: "pointer",
  primaryColor: "cyan",
  colors: {
    // dark.0
    lightMain: colorsTuple("#ffffff"),
    // dark.8
    darkMain: colorsTuple("#1f1f1f"),
    main: virtualColor({
      name: "main",
      light: "lightMain",
      dark: "darkMain",
    }),
  },
});
