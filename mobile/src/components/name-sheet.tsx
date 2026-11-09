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

import { describeError } from "@/api/errors";
import { BottomSheet } from "@/components/bottom-sheet";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type NameSheetProps = {
  visible: boolean;
  title: string;
  placeholder: string;
  submitLabel: string;
  initialValue?: string;
  onSubmit: (name: string) => Promise<unknown>;
  onClose: () => void;
  onDone: () => void;
};

export function NameSheet({ visible, onClose, ...form }: NameSheetProps) {
  return (
    <BottomSheet visible={visible} onClose={onClose}>
      <NameForm onClose={onClose} {...form} />
    </BottomSheet>
  );
}

function NameForm({
  title,
  placeholder,
  submitLabel,
  initialValue = "",
  onSubmit,
  onClose,
  onDone,
}: Omit<NameSheetProps, "visible">) {
  const [name, setName] = useState(initialValue);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const canSave = name.trim().length > 0 && !busy;

  const save = async () => {
    if (!canSave) {
      return;
    }

    setBusy(true);
    setError(null);

    try {
      await onSubmit(name.trim());
      onDone();
    } catch (err) {
      setError(describeError(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <>
      <View style={styles.titleRow}>
        <Text style={[fonts.sans, styles.title]}>{title}</Text>
        <Pressable
          accessibilityRole="button"
          accessibilityLabel="Close"
          onPress={onClose}
          style={styles.close}
        >
          <Feather name="x" size={20} color={colors.foreground} />
        </Pressable>
      </View>

      <View>
        <Text style={[fonts.sans, styles.label]}>NAME</Text>
        <TextInput
          autoFocus
          value={name}
          onChangeText={setName}
          onSubmitEditing={save}
          placeholder={placeholder}
          placeholderTextColor={colors.mutedStrong}
          maxLength={60}
          returnKeyType="done"
          style={[fonts.sans, styles.input]}
        />
      </View>

      {error ? <Text style={[fonts.sans, styles.error]}>{error}</Text> : null}

      <Pressable
        accessibilityRole="button"
        disabled={!canSave}
        onPress={save}
        style={({ pressed }) => [
          styles.button,
          !canSave && styles.buttonDisabled,
          pressed && styles.buttonPressed,
        ]}
      >
        {busy ? (
          <ActivityIndicator color={colors.foreground} />
        ) : (
          <Text style={[fonts.sans, styles.buttonLabel]}>{submitLabel}</Text>
        )}
      </Pressable>
    </>
  );
}

const styles = StyleSheet.create({
  titleRow: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
  },
  title: { fontSize: 24, letterSpacing: -0.5, color: colors.foreground },
  close: {
    width: 44,
    height: 44,
    borderRadius: 22,
    backgroundColor: colors.soft,
    alignItems: "center",
    justifyContent: "center",
  },
  label: {
    fontSize: 11,
    fontWeight: "700",
    letterSpacing: 1.5,
    color: colors.mutedStrong,
    marginBottom: 6,
  },
  input: {
    height: 48,
    borderWidth: 1,
    borderColor: colors.border,
    borderRadius: 14,
    backgroundColor: colors.background,
    paddingHorizontal: 14,
    fontSize: 15,
    color: colors.foreground,
  },
  error: { fontSize: 13, lineHeight: 18, color: colors.mutedStrong },
  button: {
    height: 52,
    borderRadius: 16,
    backgroundColor: colors.coral,
    alignItems: "center",
    justifyContent: "center",
  },
  buttonDisabled: { opacity: 0.45 },
  buttonPressed: { opacity: 0.85 },
  buttonLabel: { fontSize: 16, fontWeight: "700", color: colors.foreground },
});
