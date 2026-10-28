import Feather from "@expo/vector-icons/Feather";
import { useState } from "react";
import {
  ActivityIndicator,
  Pressable,
  RefreshControl,
  ScrollView,
  StyleSheet,
  Text,
  useWindowDimensions,
  View,
} from "react-native";

import { listSections } from "@/api/sections";
import { CollectionCard } from "@/components/collection-card";
import { NewCollectionSheet } from "@/components/new-collection-sheet";
import { useRequest } from "@/hooks/use-request";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

const PADDING = 24;
const GAP = 12;

export default function Collections() {
  const { width } = useWindowDimensions();
  const cardWidth = (width - PADDING * 2 - GAP) / 2;

  const sections = useRequest(listSections);
  const [creating, setCreating] = useState(false);

  if (sections.data === null) {
    return (
      <View style={styles.center}>
        {sections.error ? (
          <>
            <Text style={[fonts.sans, styles.message]}>{sections.error}</Text>
            <Pressable
              accessibilityRole="button"
              onPress={sections.reload}
              style={styles.retry}
            >
              <Text style={[fonts.sans, styles.retryLabel]}>Try again</Text>
            </Pressable>
          </>
        ) : (
          <ActivityIndicator color={colors.mutedForeground} />
        )}
      </View>
    );
  }

  return (
    <>
      <ScrollView
        contentContainerStyle={styles.grid}
        refreshControl={
          <RefreshControl
            refreshing={sections.loading}
            onRefresh={sections.reload}
          />
        }
      >
        {sections.data.map((section) => (
          <CollectionCard
            key={section.id}
            section={section}
            width={cardWidth}
          />
        ))}

        <Pressable
          accessibilityRole="button"
          accessibilityLabel="New collection"
          onPress={() => setCreating(true)}
          style={({ pressed }) => [
            styles.newCard,
            { width: cardWidth },
            pressed && styles.newCardPressed,
          ]}
        >
          <Feather name="plus" size={22} color={colors.mutedStrong} />
          <Text style={[fonts.sans, styles.newLabel]}>New collection</Text>
        </Pressable>
      </ScrollView>

      <NewCollectionSheet
        visible={creating}
        onClose={() => setCreating(false)}
        onCreated={() => {
          setCreating(false);
          void sections.reload();
        }}
      />
    </>
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
  grid: { flexDirection: "row", flexWrap: "wrap", gap: GAP, padding: PADDING },
  newCard: {
    minHeight: 124,
    alignItems: "center",
    justifyContent: "center",
    gap: 8,
    borderWidth: 1,
    borderStyle: "dashed",
    borderColor: colors.mutedForeground,
    borderRadius: 20,
  },
  newCardPressed: { backgroundColor: colors.soft },
  newLabel: { fontSize: 14, color: colors.mutedStrong },
});
