import { StyleSheet, Text, View, type ViewStyle } from "react-native";

import { colors } from "@/theme/colors";
import { fonts } from "@/theme/fonts";

const SIZE = 92;
const NOTE_SHADOW = "#d9c3a5";

type NoteProps = {
  character: string;
  background: string;
  textColor: string;
  rotate: string;
  position: ViewStyle;
};

function Note({
  character,
  background,
  textColor,
  rotate,
  position,
}: NoteProps) {
  return (
    <View style={[styles.note, position, { transform: [{ rotate }] }]}>
      <View style={styles.shadow} />
      <View style={[styles.face, { backgroundColor: background }]}>
        <Text style={[fonts.sans, styles.character, { color: textColor }]}>
          {character}
        </Text>
      </View>
    </View>
  );
}

export function PaperNotes() {
  return (
    <View style={styles.container}>
      <Note
        character="積"
        background={colors.yellow}
        textColor={colors.foreground}
        rotate="7deg"
        position={{ top: 0, left: 64 }}
      />
      <Note
        character="読"
        background={colors.coral}
        textColor={colors.paper}
        rotate="-9deg"
        position={{ top: 44, left: 0 }}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: { width: 180, height: 150 },
  note: { position: "absolute", width: SIZE, height: SIZE },
  shadow: {
    position: "absolute",
    top: 6,
    left: 6,
    width: SIZE,
    height: SIZE,
    borderRadius: 4,
    backgroundColor: NOTE_SHADOW,
  },
  face: {
    width: SIZE,
    height: SIZE,
    borderRadius: 4,
    alignItems: "center",
    justifyContent: "center",
  },
  character: { fontSize: 46 },
});
