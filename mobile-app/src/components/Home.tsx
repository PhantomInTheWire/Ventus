import { ImageBackground, StyleSheet, View, Text } from "react-native";
import TitleBar from "./TitleBar";
import { Link } from "expo-router";
import { Card, HorizontalLine } from "./ui";
import Chart from "./Chart";
import CustomPieChart from "./PieChart";
import Arc from "./Arc";

const bgImg = require("@/assets/images/bg.png");

export default function Home() {
  return (
    // <View style={{ flex: 1 }}>
    <ImageBackground source={bgImg} resizeMode="cover" style={styles.container}>
      <TitleBar icons={["back", "settings"]} />
      <Text style={styles.title}>Sync</Text>
      <Card py={20} mt={20}>
        <Chart />
        {/* <CustomPieChart /> */}
        {/* <Arc /> */}
        <HorizontalLine />
        <Text style={styles.text}>192.168.124.20:1234</Text>
      </Card>
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
  text: {
    color: "#dadada",
    fontSize: 25,
  },
});
