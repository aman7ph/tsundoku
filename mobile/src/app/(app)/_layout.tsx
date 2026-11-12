import { Redirect, Slot } from "expo-router";
import { useState } from "react";
import { StyleSheet, View } from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

import { AddBar } from "@/components/add-bar";
import { AddLinkSheet } from "@/components/add-link-sheet";
import { AppHeader } from "@/components/app-header";
import { Sidebar } from "@/components/sidebar";
import { useSessionStore } from "@/stores/session";

export default function AppLayout() {
  const status = useSessionStore((state) => state.status);
  const [menuOpen, setMenuOpen] = useState(false);
  const [adding, setAdding] = useState(false);

  if (status === "signedOut") {
    return <Redirect href="/" />;
  }

  return (
    <SafeAreaView style={styles.screen}>
      <AppHeader onMenuPress={() => setMenuOpen(true)} />
      <View style={styles.content}>
        <Slot />
      </View>
      <AddBar onPress={() => setAdding(true)} />
      <Sidebar visible={menuOpen} onClose={() => setMenuOpen(false)} />
      <AddLinkSheet visible={adding} onClose={() => setAdding(false)} />
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  screen: { flex: 1 },
  content: { flex: 1 },
});
