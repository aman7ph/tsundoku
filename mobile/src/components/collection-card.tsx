import Feather from "@expo/vector-icons/Feather";
import { StyleSheet, Text, View } from "react-native";

import type { Section } from "@/api/sections";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type CollectionCardProps = { section: Section; width: number };

function emojiOf(icon: string | null): string | null {
  return icon?.startsWith("emoji:") ? icon.slice("emoji:".length) : null;
}

export function CollectionCard({ section, width }: CollectionCardProps) {
  const emoji = emojiOf(section.icon);

  return (
    <View style={[styles.card, { width }]}>
      <View style={styles.tile}>
        {emoji ? (
          <Text style={styles.emoji}>{emoji}</Text>
        ) : (
          <Feather name="folder" size={20} color={colors.foreground} />
        )}
      </View>
      <Text numberOfLines={2} style={[fonts.sans, styles.name]}>
        {section.name}
      </Text>
    </View>
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
