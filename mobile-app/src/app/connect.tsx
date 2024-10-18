import { Connect } from "@/components";
import { useNavigation } from "expo-router";
import { useEffect } from "react";

export default function ConnectPage() {
  const navigation = useNavigation();
  useEffect(() => {
    navigation.setOptions({ headerShown: false });
  }, [navigation]);
  return <Connect />;
}
