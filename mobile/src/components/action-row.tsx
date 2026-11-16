import Feather from "@expo/vector-icons/Feather";
import type { ComponentProps } from "react";
import { Pressable, StyleSheet, Text } from "react-native";

import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type ActionRowProps = {
  icon: ComponentProps<typeof Feather>["name"];
  label: string;
  onPress: () => void;
};

// One row of a "…" menu
export function ActionRow({ icon, label, onPress }: ActionRowProps) {
  return (
    <Pressable
      accessibilityRole="button"
      onPress={onPress}
      style={({ pressed }) => [styles.row, pressed && styles.pressed]}
    >
      <Feather name={icon} size={20} color={colors.foreground} />
      <Text style={[fonts.sans, styles.label]}>{label}</Text>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  row: {
    flexDirection: "row",
    alignItems: "center",
    gap: 12,
    height: 52,
    paddingHorizontal: 14,
    borderWidth: 1,
    borderColor: colors.border,
    borderRadius: 14,
    backgroundColor: colors.background,
  },
  pressed: { backgroundColor: colors.soft },
  label: { fontSize: 15, color: colors.foreground },
});
