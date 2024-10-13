import { getDefaultConfig } from "expo/metro-config";
const { withNativeWind } = require("nativewind/metro");

const config = getDefaultConfig(__dirname, { isCSSEnabled: true });

module.exports = withNativeWind(config, { input: "./global.css" });
