import { useLocalSearchParams } from "expo-router";
import { RefreshControl, ScrollView, StyleSheet, Text } from "react-native";

import {
  listResources,
  updateResource,
  type Resource,
  type ResourceChanges,
} from "@/api/resources";
import { PAGE_PADDING } from "@/components/folder-grid";
import { LinkActions } from "@/components/link-actions";
import { LinkCard } from "@/components/link-card";
import { PageHeader } from "@/components/page-header";
import { ScreenStatus } from "@/components/screen-status";
import { useMenuTarget } from "@/hooks/use-menu-target";
import { useRequest } from "@/hooks/use-request";
import { useRefreshStore } from "@/stores/refresh";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

export default function CategoryPage() {
  const { categoryId, name } = useLocalSearchParams<{
    categoryId: string;
    name: string;
  }>();

  const version = useRefreshStore((state) => state.version);
  const links = useRequest(() => listResources(categoryId), String(version));
  const menu = useMenuTarget<Resource>();

  if (links.data === null) {
    return <ScreenStatus error={links.error} onRetry={links.reload} />;
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

  return (
    <>
      <ScrollView
        contentContainerStyle={styles.page}
        refreshControl={
          <RefreshControl refreshing={links.loading} onRefresh={links.reload} />
        }
      >
        <PageHeader title={name} backLabel="Back" />

        {links.data.length > 0 ? (
          links.data.map((resource) => (
            <LinkCard
              key={resource.id}
              resource={resource}
              onToggle={(changes) => void toggle(resource.id, changes)}
              onMenuPress={() => menu.open(resource)}
            />
          ))
        ) : (
          <Text style={[fonts.sans, styles.empty]}>
            No links yet. Tap + to add one.
          </Text>
        )}
      </ScrollView>

      <LinkActions
        visible={menu.visible}
        link={menu.target}
        onClose={menu.close}
        onChanged={() => {
          menu.close();
          void links.reload();
        }}
      />
    </>
  );
}

const styles = StyleSheet.create({
  page: { padding: PAGE_PADDING, gap: 10 },
  empty: {
    textAlign: "center",
    fontSize: 14,
    color: colors.mutedStrong,
    marginTop: 10,
  },
});
