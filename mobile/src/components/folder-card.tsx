import Feather from "@expo/vector-icons/Feather";
import { Pressable, StyleSheet, Text, View } from "react-native";

import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type FolderCardProps = {
  name: string;
  icon: string | null;
  width: number;
  onPress: () => void;
  onMenuPress: () => void;
};

function emojiOf(icon: string | null): string | null {
  return icon?.startsWith("emoji:") ? icon.slice("emoji:".length) : null;
}

export function FolderCard({
  name,
  icon,
  width,
  onPress,
  onMenuPress,
}: FolderCardProps) {
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
      <View style={styles.top}>
        <View style={styles.tile}>
          {emoji ? (
            <Text style={styles.emoji}>{emoji}</Text>
          ) : (
            <Feather name="folder" size={20} color={colors.foreground} />
          )}
        </View>

        <Pressable
          accessibilityRole="button"
          accessibilityLabel={`More options for ${name}`}
          hitSlop={6}
          onPress={onMenuPress}
          style={styles.menu}
        >
          <Feather
            name="more-horizontal"
            size={20}
            color={colors.mutedStrong}
          />
        </Pressable>
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
  top: {
    flexDirection: "row",
    alignItems: "flex-start",
    justifyContent: "space-between",
  },
  tile: {
    width: 40,
    height: 40,
    borderRadius: 12,
    backgroundColor: colors.yellowSoft,
    alignItems: "center",
    justifyContent: "center",
  },
  emoji: { fontSize: 20 },
  menu: {
    width: 36,
    height: 36,
    borderRadius: 18,
    marginTop: -6,
    marginRight: -8,
    alignItems: "center",
    justifyContent: "center",
  },
  name: { fontSize: 16, fontWeight: "700", color: colors.foreground },
});
