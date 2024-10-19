import { Dimensions, Text, View } from "react-native";
import {
  BarChart,
  LineChart,
  PieChart,
  PopulationPyramid,
} from "react-native-gifted-charts";

const data = [{ value: 50 }, { value: 80 }, { value: 90 }, { value: 70 }];

export default function Chart() {
  return (
    <PieChart
      data={data}
      donut
      //   showGradient={true}
      //   backgroundColor=""
      semiCircle={true}
    />
  );
}
