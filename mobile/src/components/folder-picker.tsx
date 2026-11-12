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

export type Destination = { categoryId: string; name: string; path: string };

type FolderPickerProps = {
  onBack: () => void;
  onChoose: (destination: Destination) => void;
};

export function FolderPicker({ onBack, onChoose }: FolderPickerProps) {
  const [path, setPath] = useState<Folder[]>([]);
  const current = path.length > 0 ? path[path.length - 1] : null;

  const level = useRequest(
    async (): Promise<Folder[]> => {
      if (current === null) {
        return listSections();
      }
      return listCategories(
        path[0].id,
        path.length > 1 ? current.id : undefined,
      );
    },
    path.map((folder) => folder.id).join("/"),
  );

  const canChoose = path.length >= 2;
  const goBack = () =>
    path.length === 0 ? onBack() : setPath(path.slice(0, -1));

  const names = path.map((folder) => folder.name);
  const before = names.slice(Math.max(0, names.length - 2), -1);

  return (
    <>
      <SheetHeader title="Save to" onBack={goBack} />

      <Text numberOfLines={1} style={[fonts.sans, styles.breadcrumb]}>
        {current === null ? (
          "Choose a collection"
        ) : (
          <>
            {names.length > 2 ? "… › " : ""}
            {before.map((name) => `${name} › `)}
            <Text style={styles.current}>{current.name}</Text>
          </>
        )}
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
              onPress={() => setPath([...path, folder])}
              style={({ pressed }) => [
                styles.row,
                pressed && styles.rowPressed,
              ]}
            >
              <Feather name="folder" size={20} color={colors.foreground} />
              <Text numberOfLines={1} style={[fonts.sans, styles.rowLabel]}>
                {folder.name}
              </Text>
              <Feather
                name="chevron-right"
                size={20}
                color={colors.mutedStrong}
              />
            </Pressable>
          ))}
        </View>
      ) : (
        <Text style={[fonts.sans, styles.note]}>
          {path.length === 0
            ? "No collections yet. Create one on the Collections page first."
            : path.length === 1
              ? "No categories in this collection yet. Create one on its page first."
              : "No subfolders here."}
        </Text>
      )}

      {canChoose && current ? (
        <Pressable
          accessibilityRole="button"
          onPress={() =>
            onChoose({
              categoryId: current.id,
              name: current.name,
              path: names.slice(0, -1).join(" › "),
            })
          }
          style={({ pressed }) => [styles.choose, pressed && styles.pressed]}
        >
          <Text numberOfLines={1} style={[fonts.sans, styles.chooseLabel]}>
            Save to {current.name}
          </Text>
        </Pressable>
      ) : (
        <Text style={[fonts.sans, styles.note]}>
          Links are saved inside a category. Open a collection, then a category.
        </Text>
      )}
    </>
  );
}

const styles = StyleSheet.create({
  breadcrumb: { fontSize: 13, color: colors.mutedStrong },
  current: { fontWeight: "700", color: colors.foreground },
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
  choose: {
    height: 52,
    borderRadius: 16,
    backgroundColor: colors.coral,
    alignItems: "center",
    justifyContent: "center",
    paddingHorizontal: 16,
  },
  chooseLabel: { fontSize: 16, fontWeight: "700", color: colors.foreground },
  pressed: { opacity: 0.85 },
});
