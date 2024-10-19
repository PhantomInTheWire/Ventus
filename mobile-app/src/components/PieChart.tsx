import { StyleSheet, Text, View } from "react-native";
import PieChart from "react-native-pie-chart";

export default function CustomPieChart() {
  const widthAndHeight = 250;
  const series = [123, 321, 123, 789, 537];
  const sliceColor = ["#fbd203", "#ffb300", "#ff9100", "#ff6c00", "#ff3c00"];
  return (
    <View style={styles.container}>
      <Text style={styles.title}>Basic</Text>
      <PieChart
        widthAndHeight={widthAndHeight}
        series={series}
        sliceColor={sliceColor}
      />
      <Text style={styles.title}>Doughnut</Text>
      <PieChart
        widthAndHeight={widthAndHeight}
        series={series}
        sliceColor={sliceColor}
        coverRadius={0.95}
        coverFill={"#1A1D21"}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: "center",
  },
  title: {
    fontSize: 24,
    margin: 10,
    color: "#fff",
  },
});
