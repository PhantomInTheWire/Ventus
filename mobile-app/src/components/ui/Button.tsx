import React from "react";
import { Pressable, StyleSheet, Text, View } from "react-native";
import { Card } from "./";

interface Props {
  active?: boolean;
  children?: React.ReactNode;
  icon?: React.ReactNode;
  marginVertical?: number;
  size?: "sm" | "lg";
  onPress: () => void;
}

export default function CustomButton(props: Props) {
  const [active, size] = [props.active ?? false, props.size ?? "lg"];
  return (
    <View
      style={{
        ...styles.container,
        marginVertical: props.marginVertical ?? 0,
      }}
    >
      {active ? (
        <Pressable
          style={{
            ...styles.btn,
            ...styles.btnActive,
            ...(size === "sm" ? styles.btnSm : styles.btnLg),
          }}
          onPress={props.onPress}
        >
          {props.icon && <View style={{ ...styles.icon }}>{props.icon}</View>}
          {props.children && (
            <Text
              style={{
                ...styles.text,
                ...styles.textActive,
              }}
            >
              {props.children}
            </Text>
          )}
        </Pressable>
      ) : (
        <Card>
          <Pressable
            onPress={props.onPress}
            style={{
              ...styles.btn,
              ...(size === "sm" ? styles.btnSm : styles.btnLg),
            }}
          >
            {props.icon && <View style={{ ...styles.icon }}>{props.icon}</View>}
            <Text style={{ ...styles.text }}>
              {props.children && props.children}
            </Text>
          </Pressable>
        </Card>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {},
  btn: {
    flexDirection: "row",
    alignItems: "center",
    borderRadius: 12,
  },
  btnSm: {
    paddingHorizontal: 8,
    paddingVertical: 6,
  },
  btnLg: {
    paddingHorizontal: 20,
    paddingVertical: 10,
  },
  btnActive: {
    gap: 10,
    backgroundColor: "#dadada",
    fontFamily: "MSemiBold",
    fontWeight: 700,
  },
  icon: {},
  text: {
    color: "#dadada",
    fontSize: 20,
  },
  textActive: {
    fontFamily: "MMedium",
    fontSize: 25,
    fontWeight: 600,
    color: "#1A1D21",
  },
});
