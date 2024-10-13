import { Text, View } from "react-native";
import { LinearGradient } from "expo-linear-gradient";

export default function Card() {
  return (
    <View className="w-full flex space-y-4 py-4">
      <LinearGradient
        className="w-full h-full py-20 rounded-2xl"
        colors={["#ffffff0A", "#ffffff00"]}
        style={{ flex: 1 }}
      >
        <Text
          className="text-2xl text-[#dadada]"
          style={{ fontFamily: "MMedium" }}
        >
          Connect
        </Text>
      </LinearGradient>
    </View>
  );
}
