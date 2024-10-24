import { StyleSheet, View } from "react-native";
import { LinearGradient } from "expo-linear-gradient";

interface Props {
  children?: React.ReactNode;
  mt?: number;
  flexDirection?: "row" | "column";
  items?: "";
  px?: number;
  py?: number;
  pb?: number;
}

export default function Card(props: Props) {
  return (
    <LinearGradient
      colors={["#ffffff1A", "#ffffff00"]}
      style={{
        ...styles.bg,
        marginTop: props.mt ?? 0,
        flexDirection: props.flexDirection ?? "column",
        // alignItems:
        paddingHorizontal: props.px ?? 0,
        paddingVertical: props.py ?? 0,
        paddingBottom: props.pb ?? 0,
      }}
      start={{ x: 0, y: 0 }}
      end={{ x: 1, y: 1 }}
    >
      {props.children && props.children}
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: {},
  bg: {
    alignItems: "center",
    borderWidth: 2,
    // borderRightWidth: 2,
    // borderBottomWidth: 2,
    borderColor: "#ffffff0A",
    borderRadius: 16,
  },
});
