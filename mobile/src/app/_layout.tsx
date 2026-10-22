import AntDesign from "@expo/vector-icons/AntDesign";
import Feather from "@expo/vector-icons/Feather";
import { useFonts } from "expo-font";
import { Slot } from "expo-router";
import * as SplashScreen from "expo-splash-screen";
import { StatusBar } from "expo-status-bar";
import { useEffect } from "react";
import { View } from "react-native";

import { useSessionStore } from "@/stores/session";
import { colors } from "@/theme/colors";

SplashScreen.preventAutoHideAsync();

export default function RootLayout() {
  const [fontsLoaded, fontError] = useFonts({
    ...AntDesign.font,
    ...Feather.font,
  });
  const status = useSessionStore((state) => state.status);

  useEffect(() => {
    void useSessionStore.getState().restore();
  }, []);

  const ready = (fontsLoaded || fontError !== null) && status !== "loading";

  useEffect(() => {
    if (ready) {
      void SplashScreen.hideAsync();
    }
  }, [ready]);

  if (!ready) {
    return null;
  }

  return (
    <View style={{ flex: 1, backgroundColor: colors.background }}>
      <Slot />
      <StatusBar style="dark" />
    </View>
  );
}
