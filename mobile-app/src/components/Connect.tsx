import { ImageBackground, StyleSheet, Text } from "react-native";
import TitleBar from "./TitleBar";
import Card from "./ui/Card";
import { CustomButton, HorizontalLine } from "./ui";
import { InputIcon, QrScanIcon } from "@/icons";
import CustomInput from "./ui/Input";
import { useRouter } from "expo-router";

const bgImage = require("@/assets/images/bg.png");

export default function Connect() {
  const router = useRouter();
  const navigateToHome = () => {
    router.push("/home");
  };
  return (
    <ImageBackground
      source={bgImage}
      resizeMode="cover"
      style={{
        ...styles.container,
      }}
    >
      <TitleBar />

      <Text className="text-2xl text-[#dadada]" style={styles.title}>
        Connect
      </Text>
      <Card mt={20} py={30}>
        <CustomButton
          onPress={navigateToHome}
          active={true}
          icon={<QrScanIcon />}
        >
          Scan
        </CustomButton>
        <HorizontalLine />
        <CustomInput icon={<InputIcon />} placeholder="Enter code" />
      </Card>
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
    fontFamily: "MSemiBold",
    fontWeight: 700,
  },
});
