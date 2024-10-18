import { ImageBackground, View } from "react-native";
import TitleBar from "./TitleBar";
import { Link } from "expo-router";

const bgImg = require("@/assets/images/bg.png");

export default function Home() {
  return (
    <View style={{}}>
      <ImageBackground source={bgImg} resizeMode="cover" style={{ flex: 1 }}>
        <TitleBar />
        <Link href="./connect">Connect</Link>
      </ImageBackground>
    </View>
  );
}
