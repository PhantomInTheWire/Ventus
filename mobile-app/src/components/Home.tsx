import {
  ImageBackground,
  StyleSheet,
  Text,
  ActivityIndicator,
} from "react-native";
import TitleBar from "./TitleBar";
import { Card, HorizontalLine } from "./ui";
import useAuthStore from "@/store/auth";

const bgImg = require("@/assets/images/bg.png");

export default function Home() {
  const port = useAuthStore((state) => state.port);
  return (
    <ImageBackground source={bgImg} resizeMode="cover" style={styles.container}>
      <TitleBar icons={["settings"]} justify="flex-end" />
      <Text style={styles.title}>Status</Text>
      <Card py={25} pb={25} px={30} mt={10}>
        <ActivityIndicator size="large" color={"#dadada"} />
        <HorizontalLine />
        <Text style={styles.text}>{port}</Text>
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
    fontFamily: "MMedium",
  },
  text: {
    color: "#dadada",
    fontSize: 25,
  },
});
