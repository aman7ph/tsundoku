import { router, useLocalSearchParams } from "expo-router";
import { useState } from "react";
import {
  RefreshControl,
  ScrollView,
  StyleSheet,
  Text,
  View,
} from "react-native";

import { createCategory, listCategories } from "@/api/categories";
import {
  listResources,
  updateResource,
  type ResourceChanges,
} from "@/api/resources";
import { FolderGrid, PAGE_PADDING } from "@/components/folder-grid";
import { LinkCard } from "@/components/link-card";
import { NameSheet } from "@/components/name-sheet";
import { PageHeader } from "@/components/page-header";
import { ScreenStatus } from "@/components/screen-status";
import { useRequest } from "@/hooks/use-request";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

export default function CategoryPage() {
  const { sectionId, categoryId, name } = useLocalSearchParams<{
    sectionId: string;
    categoryId: string;
    name: string;
  }>();

  const subcategories = useRequest(() => listCategories(sectionId, categoryId));
  const links = useRequest(() => listResources(categoryId));
  const [creating, setCreating] = useState(false);

  const reloadAll = () => {
    void subcategories.reload();
    void links.reload();
  };

  if (subcategories.data === null || links.data === null) {
    return (
      <ScreenStatus
        error={subcategories.error ?? links.error}
        onRetry={reloadAll}
      />
    );
  }

  const toggle = async (resourceId: string, changes: ResourceChanges) => {
    try {
      await updateResource(resourceId, changes);
    } catch (error) {
      console.warn("Could not update the link", error);
    } finally {
      await links.reload();
    }
  };

  const empty = subcategories.data.length === 0 && links.data.length === 0;

  return (
    <>
      <ScrollView
        contentContainerStyle={styles.page}
        refreshControl={
          <RefreshControl
            refreshing={subcategories.loading || links.loading}
            onRefresh={reloadAll}
          />
        }
      >
        <PageHeader title={name} backLabel="Back" />

        <FolderGrid
          items={subcategories.data}
          newLabel="New category"
          onOpen={(category) =>
            router.push({
              pathname: "/collections/[sectionId]/[categoryId]",
              params: {
                sectionId,
                categoryId: category.id,
                name: category.name,
              },
            })
          }
          onNew={() => setCreating(true)}
        />

        {links.data.length > 0 ? (
          <View style={styles.links}>
            <Text style={[fonts.sans, styles.eyebrow]}>LINKS</Text>
            {links.data.map((resource) => (
              <LinkCard
                key={resource.id}
                resource={resource}
                onToggle={(changes) => void toggle(resource.id, changes)}
              />
            ))}
          </View>
        ) : null}

        {empty ? (
          <Text style={[fonts.sans, styles.empty]}>Nothing here yet.</Text>
        ) : null}
      </ScrollView>

      <NameSheet
        visible={creating}
        title="New category"
        placeholder="For example: Axum"
        submitLabel="Create category"
        onSubmit={(categoryName) =>
          createCategory(sectionId, categoryName, categoryId)
        }
        onClose={() => setCreating(false)}
        onDone={() => {
          setCreating(false);
          void subcategories.reload();
        }}
      />
    </>
  );
}

const styles = StyleSheet.create({
  page: { padding: PAGE_PADDING, gap: 20 },
  links: { gap: 10 },
  eyebrow: {
    fontSize: 11,
    fontWeight: "700",
    letterSpacing: 1.5,
    color: colors.mutedStrong,
  },
  empty: { textAlign: "center", fontSize: 14, color: colors.mutedStrong },
});
