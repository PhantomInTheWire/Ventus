import React from "react";
import { View } from "react-native";
import Svg, { Path } from "react-native-svg";

interface Props {
    active?: boolean;
    className?: string;
    color?: string;
}

export default function FilterIcon(props: Props) {
    const [className, color] = [
        props.className ?? "mx-auto",
        props.color ?? "#DADADA"
    ];

    return (
        <View>
            <Svg className={className} width="42" height="42" viewBox="0 0 42 42" fill="none">
                <Path d="M19.9233 34.125C19.4835 34.125 19.1155 33.9768 18.8195 33.6805C18.5232 33.3845 18.375 33.0165 18.375 32.5767V22.4472L8.5785 10.0021C8.24192 9.55325 8.19306 9.08658 8.43194 8.60212C8.67111 8.11737 9.07448 7.875 9.64207 7.875H32.3579C32.9255 7.875 33.3289 8.11737 33.5681 8.60212C33.8069 9.08658 33.7581 9.55325 33.4215 10.0021L23.625 22.4472V32.5767C23.625 33.0165 23.4768 33.3845 23.1805 33.6805C22.8845 33.9768 22.5165 34.125 22.0767 34.125H19.9233ZM21 21.525L29.6625 10.5H12.3375L21 21.525Z" fill={color}/>
            </Svg>
        </View>
    );
}
