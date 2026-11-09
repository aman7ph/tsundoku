import { router, useLocalSearchParams } from "expo-router";
import { useState } from "react";
import { RefreshControl, ScrollView, StyleSheet } from "react-native";

import { createCategory, listCategories } from "@/api/categories";
import { FolderGrid, PAGE_PADDING } from "@/components/folder-grid";
import { NameSheet } from "@/components/name-sheet";
import { PageHeader } from "@/components/page-header";
import { ScreenStatus } from "@/components/screen-status";
import { useRequest } from "@/hooks/use-request";

export default function CollectionPage() {
  const { sectionId, name } = useLocalSearchParams<{
    sectionId: string;
    name: string;
  }>();

  const categories = useRequest(() => listCategories(sectionId));
  const [creating, setCreating] = useState(false);

  if (categories.data === null) {
    return (
      <ScreenStatus error={categories.error} onRetry={categories.reload} />
    );
  }

  return (
    <>
      <ScrollView
        contentContainerStyle={styles.page}
        refreshControl={
          <RefreshControl
            refreshing={categories.loading}
            onRefresh={categories.reload}
          />
        }
      >
        <PageHeader title={name} backLabel="All collections" />

        <FolderGrid
          items={categories.data}
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
      </ScrollView>

      <NameSheet
        visible={creating}
        title="New category"
        placeholder="For example: Rust"
        submitLabel="Create category"
        onSubmit={(categoryName) => createCategory(sectionId, categoryName)}
        onClose={() => setCreating(false)}
        onDone={() => {
          setCreating(false);
          void categories.reload();
        }}
      />
    </>
  );
}

const styles = StyleSheet.create({
  page: { padding: PAGE_PADDING, gap: 20 },
});
