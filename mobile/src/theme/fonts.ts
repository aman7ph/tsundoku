import type { TextStyle } from "react-native";

export const fonts = {
  sans: {},
  accent: { fontFamily: "serif", fontStyle: "italic" },
} satisfies Record<string, TextStyle>;
