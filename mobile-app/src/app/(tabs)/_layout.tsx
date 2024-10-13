import { Tabs } from "expo-router";
import React from "react";
import { ScrollView, Text } from "react-native";

import { TabBarIcon } from "@/components/navigation/TabBarIcon";
import { Colors } from "@/constants/Colors";
import { useColorScheme } from "@/hooks/useColorScheme";
import { SafeAreaView } from "react-native-safe-area-context";
import HomeScreen from ".";

export default function TabLayout() {
  const colorScheme = useColorScheme();

  return (
    <SafeAreaView className="h-full">
      <ScrollView className="bg-[#1A1D21]">
        <HomeScreen />
      </ScrollView>
    </SafeAreaView>
    // <Tabs
    //     screenOptions={{
    //         tabBarActiveTintColor: Colors[colorScheme ?? "light"].tint,
    //         headerShown: false,
    //     }}
    // >
    //     <Tabs.Screen
    //         name="index"
    //         options={{
    //             title: "Home",
    //             tabBarIcon: ({ color, focused }) => (
    //                 <TabBarIcon
    //                     name={focused ? "home" : "home-outline"}
    //                     color={color}
    //                 />
    //             ),
    //         }}
    //     />
    //     <Tabs.Screen
    //         name="explore"
    //         options={{
    //             title: "Explore",
    //             tabBarIcon: ({ color, focused }) => (
    //                 <TabBarIcon
    //                     name={focused ? "code-slash" : "code-slash-outline"}
    //                     color={color}
    //                 />
    //             ),
    //         }}
    //     />
    // </Tabs>
  );
}
