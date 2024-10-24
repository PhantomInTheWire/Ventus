import "ts-node/register"; // To import TypeScript files
import { ExpoConfig } from "expo/config";

const config: ExpoConfig = {
  name: "Ventus",
  slug: "Ventus",
  scheme: "io.ventus.mobile",
  version: "1.0.0",
  orientation: "portrait",
  icon: "./src/assets/images/icon.png",
  userInterfaceStyle: "automatic",
  splash: {
    image: "./src/assets/images/bg.png",
    resizeMode: "contain",
    backgroundColor: "#1A1D21",
  },
  ios: {
    supportsTablet: true,
  },
  android: {
    adaptiveIcon: {
      foregroundImage: "./src/assets/images/adaptive-icon.png",
      backgroundColor: "#1A1D21",
    },
    package: "io.ventus.mobile",
    permissions: [
      "INTERNET",
      "ACCESS_NETWORK_STATE",
      "READ_EXTERNAL_STORAGE",
      "WRITE_EXTERNAL_STORAGE",
    ],
  },
  web: {
    bundler: "metro",
    output: "static",
    favicon: "./src/assets/images/favicon.png",
  },
  plugins: [
    "expo-router",
    [
      "expo-camera",
      {
        cameraPermission: "Allow $(PRODUCT_NAME) to access your camera",
        microphonePermission: "Allow $(PRODUCT_NAME) to access your microphone",
        recordAudioAndroid: true,
      },
    ],
  ],
  experiments: {
    typedRoutes: true,
  },
  // permissions: ["camera_roll", "write_external_storage"],
  extra: {
    eas: {
      projectId: "e85da362-023b-4f21-b1c1-869d6e1849d3",
    },
  },
};

export default config;
