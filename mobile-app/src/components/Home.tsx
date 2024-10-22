import {
  ImageBackground,
  StyleSheet,
  View,
  Text,
  ActivityIndicator,
} from "react-native";
import TitleBar from "./TitleBar";
import { Link, useRouter } from "expo-router";
import { Card, HorizontalLine } from "./ui";
import Chart from "./Chart";
import CustomPieChart from "./PieChart";
import Arc from "./Arc";
import { useEffect } from "react";

const bgImg = require("@/assets/images/bg.png");

export default function Home() {
  const router = useRouter();
  // useEffect(() => {
  //   router.canGoBack();
  // })
  return (
    // <View style={{ flex: 1 }}>
    <ImageBackground source={bgImg} resizeMode="cover" style={styles.container}>
      <TitleBar icons={["settings"]} justify="flex-end" />
      <Text style={styles.title}>Sync</Text>
      <Card py={25} pb={25} px={30} mt={10}>
        {/* <Chart /> */}
        <ActivityIndicator size="large" color={"#dadada"} />
        {/* <CustomPieChart /> */}
        {/* <Arc /> */}
        <HorizontalLine />
        <Text style={styles.text}>192.168.124.20:1234</Text>
      </Card>
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
    fontSize: 25,
    marginTop: 10,
    fontFamily: "MMedium",
  },
  text: {
    color: "#dadada",
    fontSize: 25,
  },
});
