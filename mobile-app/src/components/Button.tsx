import React from "react";
import { Pressable, StyleSheet } from "react-native";
import { Text, View } from "react-native-ui-lib";
import { LinearGradient } from "expo-linear-gradient";

interface Props {
    active?: boolean;
    children: React.ReactNode;
    icon?: React.ReactNode;
    onPress: () => void;
}

export default function CustomButton(props: Props) {
    const [active] = [props.active ?? false];
    return (
        <View width="100%" flex paddingV-120>
            {active ? (
                <LinearGradient
                    className="w-full h-full flex items-center py-20 rounded-2xl"
                    colors={["#ffffff0A", "#ffffff00"]}
                    style={{ flex: 1 }}
                >
                    <Pressable
                        style={{ ...styles.btn, ...styles.active }}
                        onPress={props.onPress}
                    >
                        {props.icon && (
                            <View style={{ ...styles.icon }}>{props.icon}</View>
                        )}
                        <Text
                            className="text-2xl"
                            style={{
                                ...styles.text,
                                fontFamily: "MMedium",
                                fontSize: 25,
                                // color: "#dadada",
                            }}
                        >
                            {props.children}
                        </Text>
                    </Pressable>
                </LinearGradient>
            ) : (
                <Pressable onPress={props.onPress} style={styles.btn}>
                    {props.icon && (
                        <View style={{ ...styles.icon }}>{props.icon}</View>
                    )}
                    <Text style={{ ...styles.text }}>{props.children}</Text>
                </Pressable>
            )}
        </View>
    );
}

const styles = StyleSheet.create({
    btn: {
        flexDirection: "row",
        alignItems: "center",
        borderRadius: 12,
        paddingHorizontal: 20,
        paddingVertical: 10,
    },
    active: {
        backgroundColor: "#dadada",
    },
    icon: {},
    text: {
        // color: "#dadada",
        marginLeft: 10,
        alignItems: "center",
        fontSize: 20,
        fontWeight: 800,
        fontFamily: "MBold",
    },
});
