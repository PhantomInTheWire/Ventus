import { useEffect, useState } from "react";
import {
  ImageBackground,
  StyleSheet,
  Text,
  ActivityIndicator,
  NativeModules,
} from "react-native";
// import * as FileSystem from 'expo-file-system';
import TitleBar from "./TitleBar";
import { Card, HorizontalLine } from "./ui";
import useAuthStore from "@/store/auth";
import FileAccess from "@/app/files";
import { hello, ftpSync } from "modules/my-rust-module";

const bgImg = require("@/assets/images/bg.png");

export default function Home() {
  const [value, setValue] = useState<null | boolean>(null);
  const code = useAuthStore((state) => state.port);

  useEffect(() => {
    const fetchSum = async () => {
      // const newSum = await rustAdd(69, 420);
      // setSum(newSum);
      const [host, port, local_dir, remote_dir] = [
        "192.168.1.5",
        1234,
        "/sdcard/Download",
        // "/sdk_gphone64_x86_64/Download",
        "~",
      ];
      ftpSync(host, port, local_dir, remote_dir);
      // setValue(result);
    };
    fetchSum();
  }, []);
  // FileAccess

  return (
    <ImageBackground source={bgImg} resizeMode="cover" style={styles.container}>
      <TitleBar icons={["settings"]} justify="flex-end" />
      <Text style={styles.title}>Status</Text>
      <Card py={25} pb={25} px={30} mt={10}>
        <ActivityIndicator size="large" color={"#dadada"} />
        <HorizontalLine />
        <Text style={styles.text}>{code}</Text>
        <Text style={styles.text}>{hello()}</Text>
        {/* {rustAdd(10, 20)} */}
        <Text style={styles.text}>{value ?? "Loading"}</Text>
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
