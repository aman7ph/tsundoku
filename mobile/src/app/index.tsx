import AntDesign from "@expo/vector-icons/AntDesign";
import Feather from "@expo/vector-icons/Feather";
import { router } from "expo-router";
import { Pressable, StyleSheet, Text, View } from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

import { PaperNotes } from "@/components/paper-notes";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

const GOOGLE_BLUE = "#4285F4";

export default function Welcome() {
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
            <Text style={[styles.headlineAccent]}>mind piled up.</Text>
          </Text>
          <Text style={[fonts.sans, styles.body]}>
            Save the things you want to remember, and find them when you're
            ready.
          </Text>
        </View>

        <Pressable
          accessibilityRole="button"
          accessibilityLabel="Continue with Google"
          onPress={() => router.replace("/home")}
          style={({ pressed }) => [
            styles.button,
            pressed && styles.buttonPressed,
          ]}
        >
          <View style={styles.googleBadge}>
            <AntDesign name="google" size={16} color={GOOGLE_BLUE} />
          </View>
          <Text style={[fonts.sans, styles.buttonLabel]}>
            Continue with Google
          </Text>
          <Feather
            name="chevron-right"
            size={18}
            color={colors.mutedForeground}
          />
        </Pressable>

        <Text style={[fonts.sans, styles.footnote]}>
          Demo mode · Google sign-in will be connected later
        </Text>
      </View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  screen: { flex: 1 },
  content: { flex: 1, paddingHorizontal: 24, paddingVertical: 16 },
  logo: {
    alignSelf: "flex-start",
    backgroundColor: colors.primary,
    borderRadius: 8,
    paddingHorizontal: 10,
    paddingVertical: 5,
  },
  logoText: { color: colors.paper, fontSize: 15, fontWeight: "700" },
  illustration: { flex: 1, alignItems: "center", justifyContent: "center" },
  copy: { marginBottom: 28 },
  eyebrow: {
    fontSize: 11,
    fontWeight: "700",
    letterSpacing: 2,
    color: colors.mutedForeground,
    marginBottom: 12,
  },
  headline: {
    fontSize: 38,
    lineHeight: 42,
    letterSpacing: -1,
    color: colors.foreground,
  },
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
});
