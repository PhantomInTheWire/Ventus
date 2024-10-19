import { ImageBackground, StyleSheet, View } from "react-native";
import TitleBar from "./TitleBar";
import { Link } from "expo-router";
import { Card } from "./ui";

const bgImg = require("@/assets/images/bg.png");

export default function Home() {
  return (
    // <View style={{ flex: 1 }}>
    <ImageBackground source={bgImg} resizeMode="cover" style={styles.container}>
      <TitleBar />
      <Link style={styles.title} href="./connect">
        Sync
      </Link>
      <Card></Card>
    </ImageBackground>
    // </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    paddingHorizontal: 20,
  },
  title: {
    color: "#dadada",
    fontSize: 30,
    marginTop: 10,
  },
});
