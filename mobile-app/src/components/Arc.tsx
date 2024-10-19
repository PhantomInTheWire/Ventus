import { View } from "react-native";
import { Svg, Path, Defs, Stop, RadialGradient } from "react-native-svg";

export default function Arc() {
  return (
    <Svg width="126" height="202" viewBox="0 0 126 202" fill="none">
      <Path
        d="M11 11.7276C31.7018 11.7276 51.9258 17.9617 69.0479 29.6209C86.1699 41.2801 99.3997 57.8262 107.021 77.1124C114.641 96.3986 116.302 117.535 111.786 137.778C107.27 158.021 96.786 176.437 81.695 190.637"
        stroke="url(#paint0_angular_2900_443)"
        stroke-width="50"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
      <Defs>
        <RadialGradient
          id="paint0_angular_2900_443"
          cx="0"
          cy="0"
          r="1"
          gradientUnits="userSpaceOnUse"
          gradientTransform="translate(11 114.5) rotate(-103.064) scale(92.9045 92.9045)"
        >
          <Stop stop-color="#7B55D5" />
          <Stop offset="1" stop-color="#FF71A8" />
        </RadialGradient>
      </Defs>
    </Svg>
  );
}
