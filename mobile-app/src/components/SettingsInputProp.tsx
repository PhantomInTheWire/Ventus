import { useState } from "react";
import { View, Text, StyleSheet, Pressable, TextInput } from "react-native";
import { CustomButton } from "./ui";
import CustomInput from "./ui/Input";
import { type Settings } from "@/store/settings";

interface Props {
  title: string;
  icon: React.ReactNode;
  propName: keyof Settings;
  value: string | number; // Receive the current value from Settings
  onUpdate: <K extends keyof Settings>(key: K, value: Settings[K]) => void; // Receive the update function
}

export default function SettingsInputProp(props: Props) {
  const [inputValue, setInputValue] = useState(props.value); // Input value is now local

  const handleInputChange = (newText: string) => {
    setInputValue(newText);
  };

  const handleConfirm = () => {
    props.onUpdate(props.propName, inputValue); // Update the store through the passed function
  };

  return (
    <View style={styles.container}>
      {props.icon}
      <View style={styles.text}>
        <Text style={styles.title}>{props.title}</Text>
        <TextInput
          style={{ ...styles.subtitle, fontSize: 18, width: 180 }}
          onChangeText={handleInputChange}
          placeholder={"" + props.value} // Use the passed value
          cursorColor={"#dadada"}
          placeholderTextColor={"#dadada3A"}
          value={"" + inputValue}
          onBlur={handleConfirm}
        />
        {/* <CustomButton onPress={handleConfirm}>
          <Text>Confirm</Text>
        </CustomButton> */}
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    // flex: 1,
    flexDirection: "row",
    // backgroundColor: "red",
    marginTop: 20,
    alignSelf: "baseline",
  },
  text: {
    marginLeft: 10,
  },
  title: {
    color: "#dadada",
    fontSize: 22,
    fontFamily: "MMedium",
    // fontWeight: 700,
  },
  subtitle: {
    color: "#8D959FD3",
  },
});
