import React, { useEffect, useState } from "react";
import { View, Text, StyleSheet, Button, TouchableOpacity } from "react-native";
import { CameraView, CameraType, useCameraPermissions } from "expo-camera";
import { useRouter } from "expo-router";
import useAuthStore from "@/store/auth";
import { BackIcon, SwitchCamera } from "@/icons";

export default function Scan() {
  const [facing, setFacing] = useState<CameraType>("back");
  const [permission, requestPermission] = useCameraPermissions();
  const [scannedData, setScannedData] = useState<string | null>(null);
  const router = useRouter();
  const connect = useAuthStore((state) => state.connect);

  if (!permission) {
    return <View />;
  }

  function toggleCameraFacing() {
    setFacing((current) => (current === "back" ? "front" : "back"));
  }
  const handleScan = ({ data }: { data: string }) => {
    if (scannedData) return;

    router.push("./home");
    connect(data);
    setScannedData(data);
  };

  return (
    <View style={styles.container}>
      {permission.granted ? (
        <CameraView
          facing={facing}
          style={styles.camera}
          barcodeScannerSettings={{
            barcodeTypes: ["qr"],
          }}
          onBarcodeScanned={handleScan}
        >
          <View
            style={{
              ...styles.buttonContainer,
              flex: 0,
              marginHorizontal: 0,
            }}
          >
            <TouchableOpacity
              style={{
                ...styles.button,
                flex: 0,
                alignSelf: "flex-start",
                marginLeft: 20,
              }}
              onPress={router.back}
            >
              <BackIcon />
            </TouchableOpacity>
          </View>
          <View style={styles.buttonContainer}>
            <TouchableOpacity
              style={styles.button}
              onPress={toggleCameraFacing}
            >
              <SwitchCamera />
            </TouchableOpacity>
          </View>
        </CameraView>
      ) : (
        <>
          <Text style={styles.message}>
            We need your permission to show the camera
          </Text>
          <Button onPress={requestPermission} title="grant permission" />
        </>
      )}
      {/* <TitleBar icons={["back"]} /> */}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: "center",
  },
  message: {
    textAlign: "center",
    paddingBottom: 10,
  },
  camera: {
    flex: 1,
  },
  buttonContainer: {
    flex: 1,
    flexDirection: "row",
    backgroundColor: "transparent",
    margin: 64,
  },
  button: {
    flex: 1,
    alignSelf: "flex-end",
    alignItems: "center",
  },
  text: {
    fontSize: 24,
    fontWeight: "bold",
    color: "white",
  },
});
