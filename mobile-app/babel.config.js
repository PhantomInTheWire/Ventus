module.exports = function (api) {
    api.cache(true);
    return {
        presets: ["babel-preset-expo"],
        // presets: [["babel-preset-expo", { jsxImportSource: "nativewind" }]],
        plugins: [
            // Required for expo-router
            // "expo-router/babel",
            "nativewind/babel",
            // "react-native-reanimated/plugin",
        ],
    };
};
