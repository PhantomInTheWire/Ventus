import { Text, View, SafeAreaView } from "react-native";
import { MenuIcon, NotificationIcon } from "@/icons";

export default function TitleBar() {
  return (
    <SafeAreaView>
      <View className="flex-row items-center justify-between px-4 py-2">
        <MenuIcon />
        {/* <Text className="text-4xl text-red-400">Hello!</Text> */}
        <NotificationIcon />
      </View>
    </SafeAreaView>
  );
}
