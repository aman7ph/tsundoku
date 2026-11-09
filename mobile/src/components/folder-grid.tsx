import Feather from "@expo/vector-icons/Feather";
import {
  Pressable,
  StyleSheet,
  Text,
  useWindowDimensions,
  View,
} from "react-native";

import { FolderCard } from "@/components/folder-card";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

export const PAGE_PADDING = 24;
const GAP = 12;

type FolderItem = { id: string; name: string; icon: string | null };

type FolderGridProps = {
  items: FolderItem[];
  newLabel: string;
  onOpen: (item: FolderItem) => void;
  onNew: () => void;
};

export function FolderGrid({
  items,
  newLabel,
  onOpen,
  onNew,
}: FolderGridProps) {
  const { width } = useWindowDimensions();
  const cardWidth = (width - PAGE_PADDING * 2 - GAP) / 2;

  return (
    <View style={styles.grid}>
      {items.map((item) => (
        <FolderCard
          key={item.id}
          name={item.name}
          icon={item.icon}
          width={cardWidth}
          onPress={() => onOpen(item)}
        />
      ))}

      <Pressable
        accessibilityRole="button"
        accessibilityLabel={newLabel}
        onPress={onNew}
        style={({ pressed }) => [
          styles.newCard,
          { width: cardWidth },
          pressed && styles.newCardPressed,
        ]}
      >
        <Feather name="plus" size={22} color={colors.mutedStrong} />
        <Text style={[fonts.sans, styles.newLabel]}>{newLabel}</Text>
      </Pressable>
    </View>
  );
}

const styles = StyleSheet.create({
  grid: { flexDirection: "row", flexWrap: "wrap", gap: GAP },
  newCard: {
    minHeight: 124,
    alignItems: "center",
    justifyContent: "center",
    gap: 8,
    borderWidth: 1,
    borderStyle: "dashed",
    borderColor: colors.mutedForeground,
    borderRadius: 20,
  },
  newCardPressed: { backgroundColor: colors.soft },
  newLabel: { fontSize: 14, color: colors.mutedStrong },
});
