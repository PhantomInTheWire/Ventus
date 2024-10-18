import { StyleSheet, View } from "react-native";

export default function HorizontalLine() {
  return <View style={styles.borderElem}></View>;
}
const styles = StyleSheet.create({
  borderElem: {
    width: "80%",
    borderWidth: 2,
    borderColor: "#FFFFFF08",
  },
});
