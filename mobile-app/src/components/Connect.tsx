import { ImageBackground, Text, View } from "react-native";
import TitleBar from "./TitleBar";
import Card from "./ui/Card";
import { CustomButton, HorizontalLine } from "./ui";
import { InputIcon, QrScanIcon } from "@/icons";

const bgImage = require("@/assets/images/bg.png");

export default function Connect() {
  return (
    <View style={{ flex: 1 }}>
      <ImageBackground source={bgImage} resizeMode="cover" style={{ flex: 1 }}>
        <TitleBar />
        <View className="mt-4 space-y-4 px-4">
          <Text
            className="text-2xl text-[#dadada]"
            style={{ fontFamily: "MMedium" }}
          >
            Connect
          </Text>
          <Card>
            <CustomButton
              onPress={() => alert("Connecting!")}
              active={true}
              icon={<QrScanIcon />}
            >
              Connect
            </CustomButton>
            <HorizontalLine />
            <CustomButton
              onPress={() => alert("Processing code!")}
              icon={<InputIcon />}
            >
              Connect
            </CustomButton>
          </Card>
        </View>
      </ImageBackground>
    </View>
  );
}
