// import { Text, View } from "react-native";
import { Text, View, Button } from "react-native-ui-lib";
import { LinearGradient } from "expo-linear-gradient";
import CustomButton from "./Button";
import { QrScanIcon } from "@/icons";

export default function Card() {
    return (
        // <View className="w-full flex space-y-4 py-4">
        //     <LinearGradient
        //         className="w-full h-full flex items-center py-20 rounded-2xl"
        //         colors={["#ffffff0A", "#ffffff00"]}
        //         style={{ flex: 1 }}
        //     >
        //         <Text
        //             className="text-2xl text-[#dadada]"
        //             style={{ fontFamily: "MMedium" }}
        //         >
        //             Connect
        //         </Text>
        //     </LinearGradient>
        // </View>
        <View width="100%" paddingV-120>
            <CustomButton
                onPress={() => alert("Connecting!")}
                active={true}
                icon={<QrScanIcon />}
            >
                Connect
            </CustomButton>
            {/* <Button
                label={"Connect"}
                size={Button.sizes.medium}
                backgroundColor="red"
            /> */}
            {/* <Text center text40 animated={true}>
                Hello
            </Text> */}
        </View>
    );
}
