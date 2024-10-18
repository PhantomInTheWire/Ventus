import React from "react";
import { Pressable, StyleSheet, Text, View } from "react-native";
import { LinearGradient } from "expo-linear-gradient";

interface Props {
  active?: boolean;
  children: React.ReactNode;
  icon?: React.ReactNode;
  size?: "sm" | "lg";
  onPress: () => void;
}

export default function CustomButton(props: Props) {
  const [active, size] = [props.active ?? false, props.size ?? "sm"];
  return (
    <View className="flex-1">
      {active ? (
        <Pressable
          style={{ ...styles.btn, ...styles.active, backgroundColor: "red" }}
          onPress={props.onPress}
        >
          {props.icon && <View style={{ ...styles.icon }}>{props.icon}</View>}
          <Text
            className="text-2xl"
            style={{
              ...styles.text,
              fontFamily: "MMedium",
              fontSize: 25,
              // color: "#dadada",
            }}
          >
            {props.children}
          </Text>
        </Pressable>
      ) : (
        <LinearGradient
          className="w-full h-full flex items-center py-20 rounded-2xl"
          colors={["#ffffff0A", "#ffffff00"]}
          style={{}}
        >
          <Pressable onPress={props.onPress} style={styles.btn}>
            {props.icon && <View style={{ ...styles.icon }}>{props.icon}</View>}
            <Text style={{ ...styles.text }}>{props.children}</Text>
          </Pressable>
        </LinearGradient>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  btn: {
    flexDirection: "row",
    alignItems: "center",
    borderRadius: 12,
    paddingHorizontal: 20,
    paddingVertical: 10,
    backgroundColor: "red",
  },
  active: {
    backgroundColor: "#dadada",
  },
  icon: {},
  text: {
    // color: "#dadada",
    marginLeft: 10,
    alignItems: "center",
    fontSize: 20,
    fontWeight: 800,
    fontFamily: "MBold",
  },
});
