import { StyleSheet, Text, View } from "react-native";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

export function PlaceholderPage({ title }: { title: string }) {
  return (
    <View style={styles.container}>
      <Text style={[fonts.sans, styles.title]}></Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, alignItems: "center", justifyContent: "center" },
  title: { fontSize: 18, color: colors.foreground },
});
