import AntDesign from "@expo/vector-icons/AntDesign";
import Feather from "@expo/vector-icons/Feather";
import { Linking, Pressable, StyleSheet, Text, View } from "react-native";

import type { Resource, ResourceChanges } from "@/api/resources";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

type LinkCardProps = {
  resource: Resource;
  onToggle: (changes: ResourceChanges) => void;
};

export function LinkCard({ resource, onToggle }: LinkCardProps) {
  return (
    <View style={[styles.card, resource.visited && styles.cardVisited]}>
      <Pressable
        accessibilityRole="checkbox"
        accessibilityState={{ checked: resource.visited }}
        accessibilityLabel="Visited"
        hitSlop={8}
        onPress={() => onToggle({ visited: !resource.visited })}
        style={[styles.check, resource.visited && styles.checkOn]}
      >
        {resource.visited ? (
          <Feather name="check" size={16} color={colors.paper} />
        ) : null}
      </Pressable>

      <Pressable
        accessibilityRole="link"
        onPress={() =>
          void Linking.openURL(resource.url).catch(() => undefined)
        }
        style={styles.body}
      >
        <Text numberOfLines={2} style={[fonts.sans, styles.title]}>
          {resource.title ?? resource.url}
        </Text>
        {resource.description ? (
          <Text numberOfLines={2} style={[fonts.sans, styles.description]}>
            {resource.description}
          </Text>
        ) : null}
      </Pressable>

      <Pressable
        accessibilityRole="checkbox"
        accessibilityState={{ checked: resource.shareable }}
        accessibilityLabel="Worth sharing"
        hitSlop={12}
        onPress={() => onToggle({ shareable: !resource.shareable })}
      >
        <AntDesign
          name={resource.shareable ? "heart" : "hearto"}
          size={20}
          color={resource.shareable ? colors.coral : colors.mutedStrong}
        />
      </Pressable>
    </View>
  );
}

const styles = StyleSheet.create({
  card: {
    flexDirection: "row",
    alignItems: "center",
    gap: 12,
    backgroundColor: colors.paper,
    borderWidth: 1,
    borderColor: colors.border,
    borderRadius: 18,
    padding: 14,
  },
  cardVisited: { opacity: 0.65 },
  check: {
    width: 28,
    height: 28,
    borderRadius: 14,
    borderWidth: 1.5,
    borderColor: colors.mutedForeground,
    alignItems: "center",
    justifyContent: "center",
  },
  checkOn: { backgroundColor: colors.sage, borderColor: colors.sage },
  body: { flex: 1, gap: 4 },
  title: { fontSize: 15, color: colors.foreground },
  description: { fontSize: 13, lineHeight: 18, color: colors.mutedStrong },
});
