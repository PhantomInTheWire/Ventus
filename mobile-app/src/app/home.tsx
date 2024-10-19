import { Home } from "@/components";
import { useNavigation } from "expo-router";
import { useEffect } from "react";

export default function HomePage() {
  const navigation = useNavigation();
  useEffect(() => {
    navigation.setOptions({ headerShown: false });
  }, [navigation]);
  return <Home />;
}
