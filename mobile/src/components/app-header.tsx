import Feather from "@expo/vector-icons/Feather";
import { router } from "expo-router";
import { Pressable, StyleSheet, Text, View } from "react-native";

import { Logo } from "@/components/logo";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type AppHeaderProps = { onMenuPress: () => void };
export function AppHeader({ onMenuPress }: AppHeaderProps) {
  return (
    <View style={styles.header}>
      <Pressable
        accessibilityRole="button"
        accessibilityLabel="Open menu"
        hitSlop={8}
        onPress={onMenuPress}
        style={styles.side}
      >
        <Feather name="menu" size={24} color={colors.foreground} />
      </Pressable>
      <Logo />
    </View>
  );
}
const styles = StyleSheet.create({
  header: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    paddingHorizontal: 24,
    paddingVertical: 12,
  },
  // Both sides have the same size, so the logo stays exactly centered
  side: {
    width: 36,
    height: 36,
    alignItems: "center",
    justifyContent: "center",
  },
  avatar: { borderRadius: 18, backgroundColor: colors.coral },
  avatarText: { color: colors.paper, fontSize: 14, fontWeight: "700" },
});
