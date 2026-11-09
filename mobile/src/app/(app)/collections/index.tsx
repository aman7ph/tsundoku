import { router } from "expo-router";
import { useState } from "react";
import { RefreshControl, ScrollView, StyleSheet } from "react-native";

import { createSection, listSections } from "@/api/sections";
import { FolderGrid, PAGE_PADDING } from "@/components/folder-grid";
import { NameSheet } from "@/components/name-sheet";
import { ScreenStatus } from "@/components/screen-status";
import { useRequest } from "@/hooks/use-request";

export default function Collections() {
  const sections = useRequest(listSections);
  const [creating, setCreating] = useState(false);

  if (sections.data === null) {
    return <ScreenStatus error={sections.error} onRetry={sections.reload} />;
  }

  return (
    <>
      <ScrollView
        contentContainerStyle={styles.page}
        refreshControl={
          <RefreshControl
            refreshing={sections.loading}
            onRefresh={sections.reload}
          />
        }
      >
        <FolderGrid
          items={sections.data}
          newLabel="New collection"
          onOpen={(section) =>
            router.push({
              pathname: "/collections/[sectionId]",
              params: { sectionId: section.id, name: section.name },
            })
          }
          onNew={() => setCreating(true)}
        />
      </ScrollView>

      <NameSheet
        visible={creating}
        title="New collection"
        placeholder="For example: Learning"
        submitLabel="Create collection"
        onSubmit={createSection}
        onClose={() => setCreating(false)}
        onDone={() => {
          setCreating(false);
          void sections.reload();
        }}
      />
    </>
  );
}

const styles = StyleSheet.create({
  page: { padding: PAGE_PADDING },
});
