import { StyleSheet, View } from "react-native";
import { LinearGradient } from "expo-linear-gradient";

interface Props {
  children?: React.ReactNode;
}

export default function Card(props: Props) {
  return (
    <View className="w-full flex py-4">
      <LinearGradient
        className="w-full flex items-center py-20 rounded-2xl"
        colors={["#ffffff0A", "#ffffff00"]}
        style={{ ...styles.bg }}
      >
        {props.children && props.children}
      </LinearGradient>
    </View>
  );
}

const styles = StyleSheet.create({
  bg: {
    width: "100%",
  },
});
