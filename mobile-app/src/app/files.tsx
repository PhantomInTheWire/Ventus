import { Files } from "@/components";
import { useNavigation } from "expo-router";
import { useEffect } from "react";

export default function FilesPage() {
  const navigation = useNavigation();
  useEffect(() => {
    navigation.setOptions({ headerShown: false });
  }, [navigation]);
  return <Files />;
}
