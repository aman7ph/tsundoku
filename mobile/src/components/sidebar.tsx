import Feather from "@expo/vector-icons/Feather";
import { router, usePathname } from "expo-router";
import { useEffect, useState } from "react";
import {
  Animated,
  Easing,
  Modal,
  Pressable,
  StyleSheet,
  Text,
  useWindowDimensions,
  View,
} from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { Logo } from "@/components/logo";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

const items = [
  { label: "Recent", href: "/home", icon: "home" },
  { label: "Collections", href: "/collections", icon: "folder" },
  { label: "Social", href: "/social", icon: "share-2" },
  { label: "Search", href: "/search", icon: "search" },
  { label: "Settings", href: "/settings", icon: "settings" },
] as const;

type SidebarProps = { visible: boolean; onClose: () => void };

export function Sidebar({ visible, onClose }: SidebarProps) {
  const { width } = useWindowDimensions();
  const insets = useSafeAreaInsets();
  const pathname = usePathname();

  const panelWidth = Math.min(230, width * 0.78);

  const [mounted, setMounted] = useState(false);
  const [progress] = useState(() => new Animated.Value(0));

  useEffect(() => {
    if (visible) {
      setMounted(true);
    }
  }, [visible]);

  useEffect(() => {
    if (!mounted) {
      return;
    }

    const animation = Animated.timing(progress, {
      toValue: visible ? 1 : 0,
      duration: visible ? 240 : 180,
      easing: visible ? Easing.out(Easing.cubic) : Easing.in(Easing.cubic),
      useNativeDriver: true,
    });

    animation.start(({ finished }) => {
      if (finished && !visible) {
        setMounted(false);
      }
    });

    return () => animation.stop();
  }, [mounted, visible, progress]);

  const translateX = progress.interpolate({
    inputRange: [0, 1],
    outputRange: [-panelWidth, 0],
  });
  const backdropOpacity = progress.interpolate({
    inputRange: [0, 1],
    outputRange: [0, 0.4],
  });

  const go = (href: (typeof items)[number]["href"]) => {
    onClose();
    router.replace(href);
  };

  return (
    <Modal
      visible={mounted}
      transparent
      animationType="none"
      statusBarTranslucent
      navigationBarTranslucent
      onRequestClose={onClose}
    >
      <View style={styles.root}>
        <Pressable
          accessibilityLabel="Close menu"
          style={StyleSheet.absoluteFill}
          onPress={onClose}
        >
          <Animated.View
            style={[styles.backdrop, { opacity: backdropOpacity }]}
          />
        </Pressable>

        <Animated.View
          style={[
            styles.panel,
            {
              width: panelWidth,
              paddingTop: insets.top + 16,
              paddingBottom: insets.bottom + 16,
              transform: [{ translateX }],
            },
          ]}
        >
          <View style={styles.logo}>
            <Logo />
          </View>
          <View style={styles.divider} />

          {items.map((item) => {
            const active = pathname === item.href;

            return (
              <Pressable
                key={item.href}
                accessibilityRole="button"
                onPress={() => go(item.href)}
                style={[styles.item, active && styles.itemActive]}
              >
                <Feather
                  name={item.icon}
                  size={20}
                  color={active ? colors.coral : colors.mutedForeground}
                />
                <Text
                  style={[
                    fonts.sans,
                    styles.label,
                    active && styles.labelActive,
                  ]}
                >
                  {item.label}
                </Text>
              </Pressable>
            );
          })}
        </Animated.View>
      </View>
    </Modal>
  );
}

const styles = StyleSheet.create({
  root: { flex: 1 },
  backdrop: { flex: 1, backgroundColor: colors.foreground },
  panel: {
    position: "absolute",
    top: 0,
    bottom: 0,
    left: 0,
    backgroundColor: colors.paper,
    paddingHorizontal: 20,
    elevation: 8,
  },
  logo: { alignSelf: "flex-start" },
  divider: {
    height: 1,
    backgroundColor: colors.border,
    marginTop: 20,
    marginBottom: 16,
  },
  item: {
    flexDirection: "row",
    alignItems: "center",
    gap: 14,
    borderRadius: 14,
    paddingHorizontal: 16,
    paddingVertical: 15,
  },
  itemActive: { backgroundColor: colors.coralSoft },
  label: { fontSize: 16, color: colors.mutedForeground },
  labelActive: { color: colors.coral, fontWeight: "700" },
});
