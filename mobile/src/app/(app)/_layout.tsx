import { Slot } from "expo-router";
import { useState } from "react";
import { StyleSheet, View } from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

import { AddBar } from "@/components/add-bar";
import { AppHeader } from "@/components/app-header";
import { Sidebar } from "@/components/sidebar";

// Shared frame for every page after the welcome screen
export default function AppLayout() {
  const [menuOpen, setMenuOpen] = useState(false);

  return (
    <SafeAreaView style={styles.screen}>
      <AppHeader onMenuPress={() => setMenuOpen(true)} />
      <View style={styles.content}>
        <Slot />
      </View>
      <AddBar />
      <Sidebar visible={menuOpen} onClose={() => setMenuOpen(false)} />
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  screen: { flex: 1 },
  content: { flex: 1 },
});
