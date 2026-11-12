import Feather from "@expo/vector-icons/Feather";
import { Pressable, StyleSheet, Text, View } from "react-native";

import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type SheetHeaderProps = {
  title: string;
  onBack?: () => void;
  onClose?: () => void;
};

function RoundButton({
  icon,
  label,
  onPress,
}: {
  icon: "chevron-left" | "x";
  label: string;
  onPress: () => void;
}) {
  return (
    <Pressable
      accessibilityRole="button"
      accessibilityLabel={label}
      onPress={onPress}
      style={styles.round}
    >
      <Feather name={icon} size={20} color={colors.foreground} />
    </Pressable>
  );
}

export function SheetHeader({ title, onBack, onClose }: SheetHeaderProps) {
  return (
    <View style={styles.row}>
      {onBack ? (
        <RoundButton icon="chevron-left" label="Back" onPress={onBack} />
      ) : null}
      <Text numberOfLines={1} style={[fonts.sans, styles.title]}>
        {title}
      </Text>
      {onClose ? (
        <RoundButton icon="x" label="Close" onPress={onClose} />
      ) : null}
    </View>
  );
}

const styles = StyleSheet.create({
  row: { flexDirection: "row", alignItems: "center", gap: 12 },
  title: {
    flex: 1,
    fontSize: 24,
    letterSpacing: -0.5,
    color: colors.foreground,
  },
  round: {
    width: 44,
    height: 44,
    borderRadius: 22,
    backgroundColor: colors.soft,
    alignItems: "center",
    justifyContent: "center",
  },
});
