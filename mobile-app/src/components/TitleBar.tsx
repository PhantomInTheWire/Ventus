import { View, SafeAreaView, StatusBar, Pressable } from "react-native";
import { BackIcon, MenuIcon, NotificationIcon, SettingsIcon } from "@/icons";
import { Link, useRouter } from "expo-router";

interface Props {
  justify?: "flex-end";
  icons?: string[];
}

export default function TitleBar(props: Props) {
  const router = useRouter();
  return (
    <SafeAreaView>
      <View
        className="flex-row items-center justify-between py-2"
        style={{
          paddingTop: 50,
          justifyContent: props.justify ?? "space-between",
        }}
      >
        <StatusBar barStyle={"light-content"} />

        {props.icons?.includes("back") && (
          <Pressable onPress={router.back}>
            <BackIcon />
          </Pressable>
        )}
        {props.icons?.includes("menu") && <MenuIcon />}
        {props.icons?.includes("settings") && (
          <Link href="settings">
            <SettingsIcon />
          </Link>
        )}
        {/* {props.icons?.includes('notification') && <MenuIcon />} */}
        {/* <Text className="text-4xl text-red-400">Hello!</Text> */}
      </View>
    </SafeAreaView>
  );
}
