import { Pressable, View, Text, StyleSheet } from "react-native";
import { router } from "expo-router";
import Feather from "@expo/vector-icons/Feather";

import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type PageHeaderProps = { title: string; backLabel: string };

export function PageHeader({ title, backLabel }: PageHeaderProps) {
  return (
    <View style={styles.header}>
      <Pressable
        accessibilityRole="button"
        accessibilityLabel={`Back to ${backLabel}`}
        hitSlop={10}
        onPress={() => router.back()}
        style={styles.back}
      >
        <Feather name="arrow-left" size={16} color={colors.coral} />
        <Text style={[fonts.sans, styles.backLabel]}>{backLabel}</Text>
      </Pressable>
      <Text numberOfLines={2} style={[fonts.sans, styles.title]}>
        {title}
      </Text>
    </View>
  );
}
const styles = StyleSheet.create({
  header: { gap: 10 },
  back: {
    flexDirection: "row",
    alignItems: "center",
    gap: 6,
    alignSelf: "flex-start",
  },
  backLabel: { fontSize: 14, color: colors.coral },
  title: { fontSize: 28, letterSpacing: -0.5, color: colors.foreground },
});
