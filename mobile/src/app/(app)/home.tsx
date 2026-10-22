import { useEffect, useState } from "react";
import { StyleSheet, Text, View } from "react-native";

import { api } from "@/api/client";
import { describeError } from "@/api/errors";
import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

export default function Home() {
  const [message, setMessage] = useState("Loading…");

  useEffect(() => {
    api
      .get<{ email: string }>("/auth/me")
      .then(({ data }) => setMessage(`Signed in as ${data.email}`))
      .catch((error) => setMessage(describeError(error)));
  }, []);

  return (
    <View style={styles.container}>
      <Text style={[fonts.sans, styles.title]}>Recent</Text>
      <Text style={[fonts.sans, styles.message]}>{message}</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
    gap: 8,
  },
  title: { fontSize: 18, color: colors.foreground },
  message: { fontSize: 14, color: colors.mutedForeground },
});
