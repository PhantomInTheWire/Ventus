import { View, Text, StyleSheet } from "react-native";
import * as Progress from "react-native-progress";

interface Props {
  title: string;
  icon?: React.ReactNode;
  total: number;
  completed: number;
  unit?: string;
}

export default function ProgressBar(props: Props) {
  return (
    <View style={styles.container}>
      {props.icon && props.icon}

      <View style={styles.content}>
        <View>
          <Text>{props.title}</Text>
          <Text>
            {props.completed}
            {props.unit} / {props.total}
            {props.unit}
          </Text>
        </View>
        <Progress.Bar progress={0.3} width={200} />
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flexDirection: "row",
  },
  content: {
    flex: 1,
  },
});
