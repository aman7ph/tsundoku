import Feather from "@expo/vector-icons/Feather";
import { useState } from "react";
import {
  ActivityIndicator,
  Pressable,
  StyleSheet,
  Text,
  TextInput,
  View,
} from "react-native";
import * as Clipboard from "expo-clipboard";
import { describeError } from "@/api/errors";
import { createResource } from "@/api/resources";
import { BottomSheet } from "@/components/bottom-sheet";
import { FolderPicker, type Destination } from "@/components/folder-picker";
import { SheetHeader } from "@/components/sheet-header";
import { useRefreshStore } from "@/stores/refresh";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type AddLinkSheetProps = { visible: boolean; onClose: () => void };

export function AddLinkSheet({ visible, onClose }: AddLinkSheetProps) {
  return (
    <BottomSheet visible={visible} onClose={onClose}>
      <AddLinkContent onClose={onClose} />
    </BottomSheet>
  );
}

const looksLikeLink = (value: string) => /^https?:\/\/\S+$/i.test(value.trim());

type Tab = "resources" | "social";

function AddLinkContent({ onClose }: { onClose: () => void }) {
  const [view, setView] = useState<"form" | "picker">("form");
  const [tab, setTab] = useState<Tab>("resources");
  const [url, setUrl] = useState("");
  const [description, setDescription] = useState("");
  const [destination, setDestination] = useState<Destination | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const bump = useRefreshStore((state) => state.bump);

  // The form's text lives here, not in the form, so it survives a visit to the picker
  if (view === "picker") {
    return (
      <FolderPicker
        onBack={() => setView("form")}
        onChoose={(chosen) => {
          setDestination(chosen);
          setView("form");
        }}
      />
    );
  }

  const paste = async () => {
    try {
      const text = (await Clipboard.getStringAsync()).trim();
      if (text) {
        setUrl(text);
      }
    } catch (err) {
      console.warn("Could not read the clipboard", err);
    }
  };

  const canSave =
    tab === "resources" && destination !== null && looksLikeLink(url) && !busy;

  const save = async () => {
    if (!canSave || !destination) {
      return;
    }

    setBusy(true);
    setError(null);

    try {
      await createResource(destination.categoryId, {
        url: url.trim(),
        description: description.trim() || undefined,
      });
      bump();
      onClose();
    } catch (err) {
      setError(describeError(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <>
      <SheetHeader title="Add a link" onClose={onClose} />

      <View style={styles.tabs}>
        <TabButton
          label="Resources"
          active={tab === "resources"}
          onPress={() => setTab("resources")}
        />
        <TabButton
          label="Social Links"
          active={tab === "social"}
          onPress={() => setTab("social")}
        />
      </View>

      {tab === "social" ? (
        <Text style={[fonts.sans, styles.note]}>
          Saving to Social Links is not built yet. It arrives with the Social
          page.
        </Text>
      ) : null}

      <View>
        <Text style={[fonts.sans, styles.label]}>LINK</Text>
        <View>
          <TextInput
            autoFocus
            autoCapitalize="none"
            autoCorrect={false}
            keyboardType="url"
            value={url}
            onChangeText={setUrl}
            placeholder="https://"
            placeholderTextColor={colors.mutedStrong}
            style={[fonts.sans, styles.input, styles.linkInput]}
          />
          <Pressable
            accessibilityRole="button"
            accessibilityLabel="Paste from clipboard"
            onPress={paste}
            style={styles.paste}
          >
            <Text style={[fonts.sans, styles.pasteLabel]}>Paste</Text>
          </Pressable>
        </View>
      </View>

      {tab === "resources" ? (
        <View>
          <Text style={[fonts.sans, styles.label]}>SAVE TO</Text>
          <Pressable
            accessibilityRole="button"
            onPress={() => setView("picker")}
            style={styles.destination}
          >
            <Feather name="folder" size={20} color={colors.foreground} />
            <View style={styles.destinationText}>
              {destination ? (
                <>
                  <Text
                    numberOfLines={1}
                    style={[fonts.sans, styles.destinationName]}
                  >
                    {destination.name}
                  </Text>
                  <Text
                    numberOfLines={1}
                    style={[fonts.sans, styles.destinationPath]}
                  >
                    {destination.path}
                  </Text>
                </>
              ) : (
                <Text style={[fonts.sans, styles.destinationEmpty]}>
                  Choose a folder
                </Text>
              )}
            </View>
            <Feather
              name="chevron-right"
              size={20}
              color={colors.mutedStrong}
            />
          </Pressable>
        </View>
      ) : null}

      <View>
        <Text style={[fonts.sans, styles.label]}>DESCRIPTION (OPTIONAL)</Text>
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
          <Text style={[fonts.sans, styles.saveLabel]}>Save link</Text>
        )}
      </Pressable>
    </>
  );
}

function TabButton({
  label,
  active,
  onPress,
}: {
  label: string;
  active: boolean;
  onPress: () => void;
}) {
  return (
    <Pressable
      accessibilityRole="button"
      accessibilityState={{ selected: active }}
      onPress={onPress}
      style={[styles.tab, active && styles.tabActive]}
    >
      <Text
        style={[fonts.sans, styles.tabLabel, active && styles.tabLabelActive]}
      >
        {label}
      </Text>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  tabs: { flexDirection: "row", gap: 8 },
  tab: {
    flex: 1,
    height: 44,
    borderRadius: 22,
    backgroundColor: colors.soft,
    alignItems: "center",
    justifyContent: "center",
  },
  tabActive: { backgroundColor: colors.primary },
  tabLabel: { fontSize: 14, fontWeight: "700", color: colors.mutedStrong },
  tabLabelActive: { color: colors.paper },
  note: { fontSize: 14, lineHeight: 20, color: colors.mutedStrong },
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
  description: { height: 76, paddingTop: 12 },
  destination: {
    flexDirection: "row",
    alignItems: "center",
    gap: 12,
    minHeight: 60,
    paddingHorizontal: 14,
    borderWidth: 1,
    borderColor: colors.border,
    borderRadius: 14,
    backgroundColor: colors.background,
  },
  destinationText: { flex: 1 },
  destinationName: {
    fontSize: 15,
    fontWeight: "700",
    color: colors.foreground,
  },
  destinationPath: { fontSize: 12, color: colors.mutedStrong },
  destinationEmpty: { fontSize: 15, color: colors.mutedStrong },
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
  linkInput: { paddingRight: 84 },
  paste: {
    position: "absolute",
    right: 8,
    top: 8,
    height: 32,
    paddingHorizontal: 12,
    borderRadius: 16,
    backgroundColor: colors.soft,
    alignItems: "center",
    justifyContent: "center",
  },
  pasteLabel: { fontSize: 13, fontWeight: "700", color: colors.foreground },
});
