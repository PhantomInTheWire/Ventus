import { Text, View } from "react-native";
import TitleBar from "./TitleBar";
import Card from "./Card";

export default function Connect() {
  return (
    <View>
      <TitleBar />
      <View className="mt-4 space-y-4 px-4">
        <Text
          className="text-2xl text-[#dadada]"
          style={{ fontFamily: "MMedium" }}
        >
          Connect
        </Text>
        <Card />
      </View>
    </View>
  );
}
