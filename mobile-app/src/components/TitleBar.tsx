import { Text, View, SafeAreaView, StatusBar } from "react-native";
import { MenuIcon, NotificationIcon } from "@/icons";

export default function TitleBar() {
  return (
    <SafeAreaView>
      <View
        className="flex-row items-center justify-between px-4 py-2"
        style={{ paddingTop: 50 }}
      >
        <StatusBar barStyle={"light-content"} />
        <MenuIcon />
        {/* <Text className="text-4xl text-red-400">Hello!</Text> */}
        <NotificationIcon />
      </View>
    </SafeAreaView>
  );
}
