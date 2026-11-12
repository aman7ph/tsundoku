import Feather from "@expo/vector-icons/Feather";
import { Pressable, StyleSheet, View } from "react-native";

import { colors } from "@/theme/colors";

type AddBarProps = { onPress: () => void };

export function AddBar({ onPress }: AddBarProps) {
  return (
    <View style={styles.bar}>
      <Pressable
        accessibilityRole="button"
        accessibilityLabel="Add a link"
        onPress={onPress}
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
    paddingVertical: 10,
  },
  button: {
    width: 48,
    height: 48,
    borderRadius: 16,
    backgroundColor: colors.coral,
    alignItems: "center",
    justifyContent: "center",
    elevation: 6,
    shadowColor: colors.coral,
  },
  pressed: { opacity: 0.85 },
});
