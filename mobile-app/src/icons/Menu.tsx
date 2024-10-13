import React from "react";
import { View } from "react-native";
import Svg, { Path } from "react-native-svg";

interface Props {
    active?: boolean;
    className?: string;
    color?: string;
}

export default function MenuIcon(props: Props) {
    const [className, color] = [
        props.className ?? "mx-auto",
        props.color ?? "#DADADA"
    ];

    return (
        <View>
            <Svg className={className} width="50" height="50" viewBox="0 0 50 50" fill="none">
                <Path d="M8.5625 31.301C8.11979 31.301 7.74879 31.1512 7.44948 30.8516C7.14983 30.5523 7 30.1811 7 29.738C7 29.2953 7.14983 28.9243 7.44948 28.625C7.74879 28.326 8.11979 28.1766 8.5625 28.1766H25.8542C26.2969 28.1766 26.6679 28.3262 26.9672 28.6255C27.2668 28.9252 27.4167 29.2965 27.4167 29.7396C27.4167 30.1823 27.2668 30.5533 26.9672 30.8526C26.6679 31.1516 26.2969 31.301 25.8542 31.301H8.5625ZM8.5625 21.125C8.11979 21.125 7.74879 20.9752 7.44948 20.6755C7.14983 20.3759 7 20.0047 7 19.562C7 19.1189 7.14983 18.7479 7.44948 18.449C7.74879 18.1497 8.11979 18 8.5625 18H40.8542C41.2969 18 41.6679 18.1498 41.9672 18.4495C42.2668 18.7491 42.4167 19.1203 42.4167 19.563C42.4167 20.0061 42.2668 20.3771 41.9672 20.676C41.6679 20.9753 41.2969 21.125 40.8542 21.125H8.5625Z" fill={color}/>
            </Svg>
        </View>
    );
}
