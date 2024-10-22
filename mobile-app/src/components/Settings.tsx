import {
  ImageBackground,
  StyleSheet,
  View,
  Text,
  Pressable,
} from "react-native";
import { useRouter } from "expo-router";
import { Card } from "./ui";
import { FolderIcon, IdIcon, LogoutIcon } from "@/icons/settings";
import SettingsInputProp from "./SettingsInputProp";
import TitleBar from "./TitleBar";
import useAuthStore from "@/store/auth";
import useSettingsStore, { type Settings } from "@/store/settings";

const bgImg = require("@/assets/images/bg.png");

export default function Settings() {
  const settings = useSettingsStore((state) => state.settings);
  const updateSetting = useSettingsStore((state) => state.updateSetting);
  const isConnected = useAuthStore((state) => state.isConnected);
  const disconnect = useAuthStore((state) => state.disconnect);
  const router = useRouter();

  type SettingInput = {
    title: string;
    icon: React.ReactNode;
    propName: keyof Settings;
  };

  const settingInputProps: SettingInput[] = [
    {
      title: "Device name",
      icon: <IdIcon />,
      propName: "deviceName",
    },
    {
      title: "Target Folder",
      icon: <FolderIcon />,
      propName: "targetFolder",
    },
  ];

  const handleDisconnect = () => {
    disconnect();
    router.replace("./");
  };

  return (
    <ImageBackground source={bgImg} resizeMode="cover" style={styles.container}>
      <TitleBar icons={["back"]} />
      <Text style={styles.title}>General Settings</Text>
      <Card py={5} pb={25} px={30} mt={10}>
        {settingInputProps.map((inputProp, index) => (
          <SettingsInputProp
            key={index}
            title={inputProp.title}
            icon={inputProp.icon}
            value={settings[inputProp.propName]}
            propName={inputProp.propName}
            onUpdate={(key, value) => updateSetting(key, value)}
          />
        ))}
      </Card>

      {isConnected && (
        <>
          <Text style={{ ...styles.title, marginTop: 40 }}>
            Network Settings
          </Text>
          <Card py={5} pb={25} px={30} mt={10}>
            <Pressable
              style={{
                flexDirection: "row",
                marginTop: 20,
                alignSelf: "baseline",
              }}
              onPress={handleDisconnect}
            >
              <LogoutIcon />
              <View style={{ marginLeft: 10 }}>
                <Text
                  style={{
                    color: "#dadada",
                    fontSize: 22,
                    fontFamily: "MMedium",
                  }}
                >
                  Disconnect
                </Text>
                <Text style={{ color: "#8D959FD3", fontSize: 18, width: 180 }}>
                  Get out of the network
                </Text>
              </View>
            </Pressable>
          </Card>
        </>
      )}
    </ImageBackground>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    paddingHorizontal: 20,
  },
  title: {
    color: "#dadada",
    fontSize: 25,
    marginTop: 10,
    fontFamily: "MMedium",
  },
});
