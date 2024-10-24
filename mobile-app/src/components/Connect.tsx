import { useState } from "react";
import { ImageBackground, StyleSheet, Text, TextInput } from "react-native";
import { Link, useRouter } from "expo-router";
import TitleBar from "./TitleBar";
import { Card, CustomButton, HorizontalLine } from "./ui";
import { InputIcon, QrScanIcon, LinkIcon } from "@/icons";
import useAuthStore from "@/store/auth";

const bgImage = require("@/assets/images/bg.png");

export default function Connect() {
  const [textInput, setTextInput] = useState<string>("");
  const router = useRouter();
  const navigateToScan = () => {
    router.push("/scan");
  };

  const connect = useAuthStore((state) => state.connect);
  // const disconnect = useAuthStore((state) => state.disconnect);
  const connectToNetwork = () => {
    connect(textInput);
    router.replace("./home");
  };

  return (
    <ImageBackground
      source={bgImage}
      resizeMode="cover"
      style={{
        ...styles.container,
      }}
    >
      <TitleBar icons={["settings"]} justify="flex-end" />

      <Text className="text-[#dadada]" style={styles.title}>
        Connect
      </Text>
      <Card mt={10} py={25} pb={25}>
        <CustomButton
          onPress={navigateToScan}
          active={true}
          icon={<QrScanIcon />}
        >
          Scan
        </CustomButton>
        <HorizontalLine />
        <Card flexDirection="row" px={10} py={0}>
          <InputIcon />
          <TextInput
            style={{
              borderRadius: 12,
              paddingHorizontal: 10,
              paddingVertical: 10,
              color: "#dadada",
              fontSize: 20,
              width: 180,
            }}
            onChangeText={(newText) => setTextInput(newText)}
            placeholder="Enter code"
            cursorColor={"#dadada"}
            placeholderTextColor={"#dadada3A"}
            maxLength={15}
          />
          <CustomButton
            active={true}
            size="sm"
            onPress={connectToNetwork}
            icon={<LinkIcon />}
            marginVertical={0}
          />
        </Card>
      </Card>
      <Link href={"./files"} style={{ ...styles.title }}>
        Files
      </Link>
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
