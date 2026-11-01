import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";
import {
  ActivityIndicator,
  Pressable,
  StyleSheet,
  Text,
  View,
} from "react-native";

type ScreenStatusProps = { error: string | null; onRetry: () => void };

export function ScreenStatus({ error, onRetry }: ScreenStatusProps) {
  return (
    <View style={styles.center}>
      {error ? (
        <>
          <Text style={[fonts.sans, styles.message]}>{error}</Text>
          <Pressable
            accessibilityRole="button"
            onPress={onRetry}
            style={styles.retry}
          >
            <Text style={[fonts.sans, styles.retryLabel]}>Try agin</Text>
          </Pressable>
        </>
      ) : (
        <ActivityIndicator color={colors.mutedForeground} />
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  center: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
    gap: 12,
    padding: 24,
  },
  message: {
    textAlign: "center",
    fontSize: 14,
    lineHeight: 20,
    color: colors.mutedStrong,
  },
  retry: {
    height: 44,
    paddingHorizontal: 20,
    borderRadius: 22,
    backgroundColor: colors.soft,
    alignItems: "center",
    justifyContent: "center",
  },
  retryLabel: { fontSize: 14, fontWeight: "700", color: colors.foreground },
});
