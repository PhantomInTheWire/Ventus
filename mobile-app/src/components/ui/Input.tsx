import { useState } from "react";
import { Pressable, StyleSheet, Text, TextInput, View } from "react-native";
import { LinearGradient } from "expo-linear-gradient";

interface Props {
  children?: React.ReactNode;
  className?: string;
  icon?: React.ReactNode;
  placeholder?: string;
  size?: "sm" | "lg";
}

export default function CustomInput(props: Props) {
  const [textInput, setTextInput] = useState<string | null>(null);
  return (
    <LinearGradient
      className="w-full h-full flex items-center py-20 rounded-2xl"
      colors={["#ffffff0A", "#ffffff00"]}
      style={{ flex: 1 }}
    >
      {props.icon && <View style={{ ...styles.icon }}>{props.icon}</View>}
      <TextInput
        style={styles.input}
        onChangeText={(newText) => setTextInput(newText)}
        placeholder={props.placeholder}
        cursorColor={"#dadada"}
        placeholderTextColor={"#dadada3A"}
      />
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  input: {
    flexDirection: "row",
    alignItems: "center",
    borderRadius: 12,
    paddingHorizontal: 20,
    paddingVertical: 10,
    color: "#dadada",
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
