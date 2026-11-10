import Feather from "@expo/vector-icons/Feather";
import { useState } from "react";
import {
  ActivityIndicator,
  Pressable,
  StyleSheet,
  Text,
  View,
} from "react-native";

import { describeError } from "@/api/errors";
import { BottomSheet } from "@/components/bottom-sheet";
import { NameForm } from "@/components/name-sheet";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

export type FolderRef = { id: string; name: string };

type FolderActionsProps = {
  visible: boolean;
  folder: FolderRef | null;
  kind: "collection" | "category";
  onRename: (id: string, name: string) => Promise<unknown>;
  onDelete: (id: string) => Promise<unknown>;
  onClose: () => void;
  onChanged: () => void;
};

export function FolderActions({
  visible,
  folder,
  onClose,
  ...rest
}: FolderActionsProps) {
  return (
    <BottomSheet visible={visible} onClose={onClose}>
      {folder ? <Actions folder={folder} onClose={onClose} {...rest} /> : null}
    </BottomSheet>
  );
}

type ActionsProps = Omit<FolderActionsProps, "visible" | "folder"> & {
  folder: FolderRef;
};

function Actions({
  folder,
  kind,
  onRename,
  onDelete,
  onClose,
  onChanged,
}: ActionsProps) {
  const [view, setView] = useState<"menu" | "rename" | "delete">("menu");

  if (view === "rename") {
    return (
      <NameForm
        title={`Rename ${kind}`}
        placeholder="Name"
        submitLabel="Save"
        initialValue={folder.name}
        onSubmit={(name) => onRename(folder.id, name)}
        onClose={onClose}
        onDone={onChanged}
      />
    );
  }

  if (view === "delete") {
    return (
      <DeleteConfirm
        folder={folder}
        onDelete={onDelete}
        onBack={() => setView("menu")}
        onDone={onChanged}
      />
    );
  }

  return (
    <>
      <View style={styles.titleRow}>
        <Text numberOfLines={1} style={[fonts.sans, styles.title]}>
          {folder.name}
        </Text>
        <Pressable
          accessibilityRole="button"
          accessibilityLabel="Close"
          onPress={onClose}
          style={styles.close}
        >
          <Feather name="x" size={20} color={colors.foreground} />
        </Pressable>
      </View>

      <Pressable
        accessibilityRole="button"
        onPress={() => setView("rename")}
        style={styles.row}
      >
        <Feather name="edit-2" size={20} color={colors.foreground} />
        <Text style={[fonts.sans, styles.rowLabel]}>Rename</Text>
      </Pressable>

      <Pressable
        accessibilityRole="button"
        onPress={() => setView("delete")}
        style={styles.row}
      >
        <Feather name="trash-2" size={20} color={colors.foreground} />
        <Text style={[fonts.sans, styles.rowLabel]}>Delete</Text>
      </Pressable>
    </>
  );
}

type DeleteConfirmProps = {
  folder: FolderRef;
  onDelete: (id: string) => Promise<unknown>;
  onBack: () => void;
  onDone: () => void;
};

function DeleteConfirm({
  folder,
  onDelete,
  onBack,
  onDone,
}: DeleteConfirmProps) {
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const confirm = async () => {
    if (busy) {
      return;
    }

    setBusy(true);
    setError(null);

    try {
      await onDelete(folder.id);
      onDone();
    } catch (err) {
      setError(describeError(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <>
      <Text numberOfLines={2} style={[fonts.sans, styles.title]}>
        Delete “{folder.name}”?
      </Text>
      <Text style={[fonts.sans, styles.warning]}>
        This also deletes everything inside it: all the categories and links.
        This can't be undone.
      </Text>

      {error ? <Text style={[fonts.sans, styles.error]}>{error}</Text> : null}

      <View style={styles.buttons}>
        <Pressable
          accessibilityRole="button"
          disabled={busy}
          onPress={onBack}
          style={[styles.button, styles.cancel]}
        >
          <Text style={[fonts.sans, styles.cancelLabel]}>Cancel</Text>
        </Pressable>

        <Pressable
          accessibilityRole="button"
          disabled={busy}
          onPress={confirm}
          style={({ pressed }) => [
            styles.button,
            styles.danger,
            pressed && styles.pressed,
          ]}
        >
          {busy ? (
            <ActivityIndicator color={colors.paper} />
          ) : (
            <>
              <Feather name="trash-2" size={18} color={colors.paper} />
              <Text style={[fonts.sans, styles.dangerLabel]}>Delete</Text>
            </>
          )}
        </Pressable>
      </View>
    </>
  );
}

const styles = StyleSheet.create({
  titleRow: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    gap: 12,
  },
  title: {
    flex: 1,
    fontSize: 24,
    letterSpacing: -0.5,
    color: colors.foreground,
  },
  close: {
    width: 44,
    height: 44,
    borderRadius: 22,
    backgroundColor: colors.soft,
    alignItems: "center",
    justifyContent: "center",
  },
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
  rowLabel: { fontSize: 15, color: colors.foreground },
  warning: { fontSize: 14, lineHeight: 20, color: colors.mutedStrong },
  error: { fontSize: 13, lineHeight: 18, color: colors.mutedStrong },
  buttons: { flexDirection: "row", gap: 12 },
  button: {
    flex: 1,
    height: 52,
    flexDirection: "row",
    gap: 8,
    borderRadius: 16,
    alignItems: "center",
    justifyContent: "center",
  },
  cancel: { backgroundColor: colors.soft },
  cancelLabel: { fontSize: 16, fontWeight: "700", color: colors.foreground },
  danger: { backgroundColor: colors.primary },
  dangerLabel: { fontSize: 16, fontWeight: "700", color: colors.paper },
  pressed: { opacity: 0.85 },
});
