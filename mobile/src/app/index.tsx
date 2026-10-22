import AntDesign from "@expo/vector-icons/AntDesign";
import Feather from "@expo/vector-icons/Feather";
import {
  ActivityIndicator,
  Pressable,
  StyleSheet,
  Text,
  View,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

import { PaperNotes } from "@/components/paper-notes";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";
import { Redirect } from "expo-router";
import { useState } from "react";
import { devLogin } from "@/api/auth";
import { describeError } from "@/api/errors";
import { useSessionStore } from "@/stores/session";

export default function Welcome() {
  const status = useSessionStore((state) => state.status);
  const setTokens = useSessionStore((state) => state.setTokens);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (status === "signedIn") {
    return <Redirect href="/home" />;
  }

  const continueWithGoogle = async () => {
    if (busy) {
      return;
    }

    if (!__DEV__) {
      setError("Google sign-in is not connected yet.");
      return;
    }

    setBusy(true);
    setError(null);

    try {
      await setTokens(await devLogin());
    } catch (err) {
      setError(describeError(err));
    } finally {
      setBusy(false);
    }
  };
  return (
    <SafeAreaView style={styles.screen}>
      <View style={styles.content}>
        <View style={styles.logo}>
          <Text style={[fonts.sans, styles.logoText]}>tsundoku</Text>
        </View>
        <View style={styles.illustration}>
          <PaperNotes />
        </View>
        <View style={styles.copy}>
          <Text style={[fonts.sans, styles.headlineAccent]}>
            Keep your curious{"\n"}
            mind piled up.
          </Text>
        </View>
        <Pressable
          accessibilityRole="button"
          accessibilityLabel="Continue with Google"
          onPress={continueWithGoogle}
          disabled={busy}
          style={({ pressed }) => [
            styles.button,
            pressed && styles.buttonPressed,
          ]}
        >
          <View style={styles.googleBadge}>
            <AntDesign name="google" size={16} color={colors.coral} />
          </View>
          <Text style={[fonts.sans, styles.buttonLabel]}>
            Continue with Google
          </Text>
          {busy ? (
            <ActivityIndicator size="small" color={colors.mutedForeground} />
          ) : (
            <Feather
              name="chevron-right"
              size={18}
              color={colors.mutedForeground}
            />
          )}
        </Pressable>
        {error ? <Text style={[fonts.sans, styles.error]}>{error}</Text> : null}
      </View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  screen: { flex: 1 },
  content: { flex: 1, paddingHorizontal: 24, paddingVertical: 16 },
  logo: {
    alignSelf: "center",
    backgroundColor: colors.primary,
    borderRadius: 8,
    paddingHorizontal: 10,
    paddingVertical: 5,
  },
  logoText: { color: colors.paper, fontSize: 15, fontWeight: "700" },

  illustration: { alignItems: "center", paddingTop: 120 },
  copy: { marginTop: 150, marginBottom: 58 },
  headlineAccent: {
    fontSize: 38,
    lineHeight: 42,
    letterSpacing: -1,
    ...fonts.accent,
    color: colors.coral,
  },
  body: {
    fontSize: 15,
    lineHeight: 22,
    color: colors.mutedForeground,
    marginTop: 14,
    maxWidth: 300,
  },
  button: {
    flexDirection: "row",
    alignItems: "center",
    backgroundColor: colors.paper,
    borderWidth: 1,
    borderColor: colors.border,
    borderRadius: 18,
    paddingVertical: 14,
    paddingHorizontal: 16,
    elevation: 1,
  },
  buttonPressed: { backgroundColor: colors.soft },
  googleBadge: {
    width: 28,
    height: 28,
    borderRadius: 8,
    borderWidth: 1,
    borderColor: colors.border,
    backgroundColor: colors.paper,
    alignItems: "center",
    justifyContent: "center",
    marginRight: 12,
  },
  buttonLabel: {
    flex: 1,
    fontSize: 15,
    fontWeight: "600",
    color: colors.foreground,
  },
  footnote: {
    textAlign: "center",
    fontSize: 11,
    color: colors.mutedForeground,
    marginTop: 16,
  },
  error: {
    color: colors.coral,
    fontSize: 13,
    lineHeight: 18,
    textAlign: "center",
    marginTop: 12,
  },
});
