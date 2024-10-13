import React from "react";
import { View } from "react-native";
import Svg, { Path } from "react-native-svg";

interface Props {
    active?: boolean;
    className?: string;
    color?: string;
}

export default function InputIcon(props: Props) {
    const [className, color] = [
        props.className ?? "mx-auto",
        props.color ?? "#A1A1A1"
    ];

    return (
        <View>
            <Svg className={className} width="24" height="24" viewBox="0 0 24 24" fill="none">
                <Path d="M14.831 12L7.29826 4.46725C7.09959 4.26858 7.00284 4.03208 7.00801 3.75775C7.01318 3.48342 7.11509 3.24683 7.31376 3.048C7.51243 2.84933 7.74893 2.75 8.02326 2.75C8.29759 2.75 8.53418 2.84933 8.73301 3.048L16.3888 10.7193C16.5694 10.9001 16.7033 11.1027 16.7905 11.327C16.8778 11.5513 16.9215 11.7757 16.9215 12C16.9215 12.2243 16.8778 12.4487 16.7905 12.673C16.7033 12.8973 16.5694 13.0999 16.3888 13.2808L8.71751 20.952C8.51884 21.1507 8.28493 21.2474 8.01576 21.2423C7.74643 21.2371 7.51243 21.1352 7.31376 20.9365C7.11509 20.7378 7.01576 20.5013 7.01576 20.227C7.01576 19.9527 7.11509 19.7161 7.31376 19.5173L14.831 12Z" fill={color}/>
            </Svg>
        </View>
    );
}
