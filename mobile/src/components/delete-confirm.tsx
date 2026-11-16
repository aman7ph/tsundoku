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
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type DeleteConfirmProps = {
  title: string;
  message: string;
  onDelete: () => Promise<unknown>;
  onBack: () => void;
  onDone: () => void;
};

export function DeleteConfirm({
  title,
  message,
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
      await onDelete();
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
        {title}
      </Text>
      <Text style={[fonts.sans, styles.message]}>{message}</Text>

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
  title: { fontSize: 24, letterSpacing: -0.5, color: colors.foreground },
  message: { fontSize: 14, lineHeight: 20, color: colors.mutedStrong },
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
