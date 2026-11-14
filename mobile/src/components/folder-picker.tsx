import Feather from "@expo/vector-icons/Feather";
import { useState } from "react";
import {
  ActivityIndicator,
  Pressable,
  StyleSheet,
  Text,
  View,
} from "react-native";

import { listCategories } from "@/api/categories";
import { listSections } from "@/api/sections";
import { SheetHeader } from "@/components/sheet-header";
import { useRequest } from "@/hooks/use-request";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type Folder = { id: string; name: string };

/** The category a link will be saved in, and the collection it belongs to, for display */
export type Destination = { categoryId: string; name: string; path: string };

type FolderPickerProps = {
  onBack: () => void;
  onChoose: (destination: Destination) => void;
};

// Two steps: a collection, then one of its categories. A link is saved inside a category.
export function FolderPicker({ onBack, onChoose }: FolderPickerProps) {
  // The collection opened so far, or null while choosing one
  const [collection, setCollection] = useState<Folder | null>(null);

  const level = useRequest(
    async (): Promise<Folder[]> =>
      collection ? listCategories(collection.id) : listSections(),
    collection?.id ?? "",
  );

  const goBack = () => (collection ? setCollection(null) : onBack());

  const choose = (folder: Folder) => {
    if (collection) {
      onChoose({
        categoryId: folder.id,
        name: folder.name,
        path: collection.name,
      });
    } else {
      setCollection(folder);
    }
  };

  return (
    <>
      <SheetHeader title="Save to" onBack={goBack} />

      <Text numberOfLines={1} style={[fonts.sans, styles.breadcrumb]}>
        {collection ? collection.name : "Choose a collection"}
      </Text>

      {level.loading ? (
        <ActivityIndicator
          color={colors.mutedForeground}
          style={styles.spinner}
        />
      ) : level.error ? (
        <View style={styles.status}>
          <Text style={[fonts.sans, styles.note]}>{level.error}</Text>
          <Pressable
            accessibilityRole="button"
            onPress={level.reload}
            style={styles.retry}
          >
            <Text style={[fonts.sans, styles.retryLabel]}>Try again</Text>
          </Pressable>
        </View>
      ) : level.data && level.data.length > 0 ? (
        <View style={styles.list}>
          {level.data.map((folder) => (
            <Pressable
              key={folder.id}
              accessibilityRole="button"
              onPress={() => choose(folder)}
              style={({ pressed }) => [
                styles.row,
                pressed && styles.rowPressed,
              ]}
            >
              <Feather name="folder" size={20} color={colors.foreground} />
              <Text numberOfLines={1} style={[fonts.sans, styles.rowLabel]}>
                {folder.name}
              </Text>
              {collection ? null : (
                <Feather
                  name="chevron-right"
                  size={20}
                  color={colors.mutedStrong}
                />
              )}
            </Pressable>
          ))}
        </View>
      ) : (
        <Text style={[fonts.sans, styles.note]}>
          {collection
            ? "No categories in this collection yet. Create one on its page first."
            : "No collections yet. Create one on the Collections page first."}
        </Text>
      )}

      {collection ? null : (
        <Text style={[fonts.sans, styles.note]}>
          Links are saved inside a category. Open a collection, then choose a
          category.
        </Text>
      )}
    </>
  );
}

const styles = StyleSheet.create({
  breadcrumb: { fontSize: 13, fontWeight: "700", color: colors.foreground },
  spinner: { paddingVertical: 24 },
  status: { alignItems: "center", gap: 12, paddingVertical: 12 },
  note: { fontSize: 14, lineHeight: 20, color: colors.mutedStrong },
  retry: {
    height: 44,
    paddingHorizontal: 20,
    borderRadius: 22,
    backgroundColor: colors.soft,
    alignItems: "center",
    justifyContent: "center",
  },
  retryLabel: { fontSize: 14, fontWeight: "700", color: colors.foreground },
  list: { gap: 8 },
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
  rowPressed: { backgroundColor: colors.soft },
  rowLabel: { flex: 1, fontSize: 15, color: colors.foreground },
});
