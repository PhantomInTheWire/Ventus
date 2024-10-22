import useAuthStore from "@/store/auth";
import { View, Text, StyleSheet, Pressable, TextInput } from "react-native";

interface Props {
  title: string;
  subtitle: string;
  icon: React.ReactNode;
  // onUpdate: <K extends keyof Settings>(key: K, value: Settings[K]) => void;
  onPress: () => void;
}

export default function SettingsBtnProp(props: Props) {
  // const disconnect = useAuthStore((state) => state.disconnect);
  return (
    <Pressable style={styles.container} onPress={props.onPress}>
      {props.icon}
      <View style={styles.text}>
        <Text style={styles.title}>{props.title}</Text>
        <Text style={styles.subtitle}>{props.subtitle}</Text>
      </View>
    </Pressable>
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
  },
  subtitle: {
    color: "#8D959FD3",
  },
});
