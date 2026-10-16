import { StyleSheet, Text, View } from "react-native";

import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

export default function Home() {
  return (
    <View style={styles.container}>
      <Text style={[fonts.sans, styles.text]}>Tsundoku</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, alignItems: "center", justifyContent: "center" },
  text: { fontSize: 16, color: colors.foreground },
});
