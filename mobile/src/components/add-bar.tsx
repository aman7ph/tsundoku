import Feather from "@expo/vector-icons/Feather";
import { Pressable, StyleSheet, View } from "react-native";

import { colors } from "@/theme/colors";

export function AddBar() {
  return (
    <View style={styles.bar}>
      <Pressable
        accessibilityRole="button"
        accessibilityLabel="Add a link"
        style={({ pressed }) => [styles.button, pressed && styles.pressed]}
      >
        <Feather name="plus" size={24} color={colors.paper} />
      </Pressable>
    </View>
  );
}

const styles = StyleSheet.create({
  bar: {
    alignItems: "center",
    backgroundColor: colors.paper,
    borderTopWidth: 1,
    borderTopColor: colors.border,
    paddingVertical: 5,
  },
  button: {
    width: 38,
    height: 38,
    borderRadius: 16,
    backgroundColor: colors.coral,
    alignItems: "center",
    justifyContent: "center",
    elevation: 6,
    shadowColor: colors.coral,
  },
  pressed: { opacity: 0.85 },
});
