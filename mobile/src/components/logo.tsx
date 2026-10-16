import { StyleSheet, Text, View } from "react-native";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

export function Logo() {
  return (
    <View style={styles.chip}>
      <Text style={[fonts.sans, styles.text]}>Tsundoku</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  chip: {
    backgroundColor: colors.primary,
    borderRadius: 8,
    paddingHorizontal: 10,
    paddingVertical: 5,
  },
  text: { color: colors.paper, fontSize: 15, fontWeight: "700" },
});
