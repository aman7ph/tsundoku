import { Stack } from "expo-router";

import { colors } from "@/theme/colors";

export default function CollectionsLayout() {
  return (
    <Stack
      screenOptions={{
        headerShown: false,
        contentStyle: { backgroundColor: colors.background },
      }}
    />
  );
}
