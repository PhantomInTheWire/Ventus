import { Scan } from "@/components";
import { useNavigation } from "expo-router";
import { useEffect } from "react";

export default function ScanPage() {
  const navigation = useNavigation();
  useEffect(() => {
    navigation.setOptions({ headerShown: false });
  }, [navigation]);
  return <Scan />;
}
