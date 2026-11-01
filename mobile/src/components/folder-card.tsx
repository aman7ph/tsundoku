import Feather from "@expo/vector-icons/Feather";
import { Pressable, StyleSheet, Text, View } from "react-native";

import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type FolderCardProps = {
  name: string;
  icon: string | null;
  width: number;
  onPress: () => void;
};

function emojiOf(icon: string | null): string | null {
  return icon?.startsWith("emoji:") ? icon.slice("emoji:".length) : null;
}

export function FolderCard({ name, icon, width, onPress }: FolderCardProps) {
  const emoji = emojiOf(icon);
  return (
    <Pressable
      accessibilityRole="button"
      onPress={onPress}
      style={({ pressed }) => [
        styles.card,
        { width },
        pressed && styles.pressed,
      ]}
    >
      <View style={styles.tile}>
        {emoji ? (
          <Text style={styles.emoji}>{emoji}</Text>
        ) : (
          <Feather name="folder" size={20} color={colors.foreground} />
        )}
      </View>
      <Text numberOfLines={2} style={[fonts.sans, styles.name]}>
        {name}
      </Text>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  card: {
    minHeight: 124,
    justifyContent: "space-between",
    backgroundColor: colors.paper,
    borderWidth: 1,
    borderColor: colors.border,
    borderRadius: 20,
    padding: 16,
  },
  pressed: { backgroundColor: colors.soft },
  tile: {
    width: 40,
    height: 40,
    borderRadius: 12,
    backgroundColor: colors.yellowSoft,
    alignItems: "center",
    justifyContent: "center",
  },
  emoji: { fontSize: 20 },
  name: { fontSize: 16, fontWeight: "700", color: colors.foreground },
});
