import { useState } from "react";
import {
  ActivityIndicator,
  Pressable,
  StyleSheet,
  Text,
  TextInput,
  View,
} from "react-native";

import { describeError } from "@/api/errors";
import { deleteResource, updateResource, type Resource } from "@/api/resources";
import { ActionRow } from "@/components/action-row";
import { BottomSheet } from "@/components/bottom-sheet";
import { DeleteConfirm } from "@/components/delete-confirm";
import { SheetHeader } from "@/components/sheet-header";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type LinkActionsProps = {
  visible: boolean;
  link: Resource | null;
  onClose: () => void;
  onChanged: () => void;
};

export function LinkActions({
  visible,
  link,
  onClose,
  onChanged,
}: LinkActionsProps) {
  return (
    <BottomSheet visible={visible} onClose={onClose}>
      {link ? (
        <Actions link={link} onClose={onClose} onChanged={onChanged} />
      ) : null}
    </BottomSheet>
  );
}

type ActionsProps = Omit<LinkActionsProps, "visible" | "link"> & {
  link: Resource;
};

function Actions({ link, onClose, onChanged }: ActionsProps) {
  const [view, setView] = useState<"menu" | "edit" | "delete">("menu");

  if (view === "edit") {
    return (
      <EditForm
        link={link}
        onBack={() => setView("menu")}
        onClose={onClose}
        onDone={onChanged}
      />
    );
  }

  if (view === "delete") {
    return (
      <DeleteConfirm
        title="Delete this link?"
        message={`${link.title ?? link.url}\n\nThis can't be undone.`}
        onDelete={() => deleteResource(link.id)}
        onBack={() => setView("menu")}
        onDone={onChanged}
      />
    );
  }

  return (
    <>
      <SheetHeader title={link.title ?? link.url} onClose={onClose} />
      <ActionRow
        icon="edit-2"
        label="Edit title and description"
        onPress={() => setView("edit")}
      />
      <ActionRow
        icon="trash-2"
        label="Delete"
        onPress={() => setView("delete")}
      />
    </>
  );
}

type EditFormProps = {
  link: Resource;
  onBack: () => void;
  onClose: () => void;
  onDone: () => void;
};

function EditForm({ link, onBack, onClose, onDone }: EditFormProps) {
  const [title, setTitle] = useState(link.title ?? "");
  const [description, setDescription] = useState(link.description ?? "");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const changed =
    title.trim() !== (link.title ?? "") ||
    description.trim() !== (link.description ?? "");
  const canSave = changed && !busy;

  const save = async () => {
    if (!canSave) {
      return;
    }

    setBusy(true);
    setError(null);

    try {
      // Empty text clears the field on the server
      await updateResource(link.id, {
        title: title.trim(),
        description: description.trim(),
      });
      onDone();
    } catch (err) {
      setError(describeError(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <>
      <SheetHeader title="Edit link" onBack={onBack} onClose={onClose} />

      <Text numberOfLines={1} style={[fonts.sans, styles.url]}>
        {link.url}
      </Text>

      <View>
        <Text style={[fonts.sans, styles.label]}>TITLE</Text>
        <TextInput
          autoFocus
          value={title}
          onChangeText={setTitle}
          placeholder="Leave empty to show the web address"
          placeholderTextColor={colors.mutedStrong}
          style={[fonts.sans, styles.input]}
        />
      </View>

      <View>
        <Text style={[fonts.sans, styles.label]}>DESCRIPTION</Text>
        <TextInput
          multiline
          textAlignVertical="top"
          value={description}
          onChangeText={setDescription}
          placeholder="Why is this worth keeping?"
          placeholderTextColor={colors.mutedStrong}
          style={[fonts.sans, styles.input, styles.description]}
        />
      </View>

      {error ? <Text style={[fonts.sans, styles.error]}>{error}</Text> : null}

      <Pressable
        accessibilityRole="button"
        disabled={!canSave}
        onPress={save}
        style={({ pressed }) => [
          styles.save,
          !canSave && styles.saveDisabled,
          pressed && styles.pressed,
        ]}
      >
        {busy ? (
          <ActivityIndicator color={colors.foreground} />
        ) : (
          <Text style={[fonts.sans, styles.saveLabel]}>Save</Text>
        )}
      </Pressable>
    </>
  );
}

const styles = StyleSheet.create({
  url: { fontSize: 13, color: colors.mutedStrong },
  label: {
    fontSize: 11,
    fontWeight: "700",
    letterSpacing: 1.5,
    color: colors.mutedStrong,
    marginBottom: 6,
  },
  input: {
    minHeight: 48,
    borderWidth: 1,
    borderColor: colors.border,
    borderRadius: 14,
    backgroundColor: colors.background,
    paddingHorizontal: 14,
    fontSize: 15,
    color: colors.foreground,
  },
  description: { height: 96, paddingTop: 12 },
  error: { fontSize: 13, lineHeight: 18, color: colors.mutedStrong },
  save: {
    height: 52,
    borderRadius: 16,
    backgroundColor: colors.coral,
    alignItems: "center",
    justifyContent: "center",
  },
  saveDisabled: { opacity: 0.45 },
  pressed: { opacity: 0.85 },
  saveLabel: { fontSize: 16, fontWeight: "700", color: colors.foreground },
});
