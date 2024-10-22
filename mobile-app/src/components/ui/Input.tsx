import { useState } from "react";
import { StyleSheet, TextInput, View } from "react-native";
import { CustomButton, Card } from "./";
import { LinkIcon } from "@/icons";

interface Props {
  children?: React.ReactNode;
  className?: string;
  icon?: React.ReactNode;
  placeholder?: string;
  onPress: () => void;
}

export default function CustomInput(props: Props) {
  const [textInput, setTextInput] = useState<string | null>(null);
  return (
    <View style={styles.container}>
      <Card flexDirection="row" px={10} py={5}>
        {props.icon && <View style={{ ...styles.icon }}>{props.icon}</View>}
        <TextInput
          style={{ ...styles.input, fontSize: 20, width: 180 }}
          onChangeText={(newText) => setTextInput(newText)}
          placeholder={props.placeholder}
          cursorColor={"#dadada"}
          placeholderTextColor={"#dadada3A"}
          maxLength={15}
        />
        <CustomButton
          active={true}
          size="sm"
          onPress={props.onPress}
          icon={<LinkIcon />}
          marginVertical={0}
        />
      </Card>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    marginVertical: 0,
  },
  input: {
    borderRadius: 12,
    paddingHorizontal: 10,
    paddingVertical: 10,
    color: "#dadada",
  },
  active: {
    backgroundColor: "#dadada",
  },
  icon: {},
  text: {
    alignItems: "center",
    fontSize: 250,
    fontWeight: 800,
    fontFamily: "MBold",
  },
});
