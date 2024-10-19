import { Settings } from "@/components";
import { useNavigation } from "expo-router";
import { useEffect } from "react";

export default function SettingsPage() {
  const navigation = useNavigation();
  useEffect(() => {
    navigation.setOptions({ headerShown: false });
  }, [navigation]);
  return <Settings />;
}
