import { StyleSheet, View } from "react-native";

export default function HorizontalLine() {
  return <View style={styles.borderElem}></View>;
}
const styles = StyleSheet.create({
  borderElem: {
    width: "70%",
    borderWidth: 1,
    borderColor: "#FFFFFF08",
    marginVertical: 30,
  },
});
