import * as MediaLibrary from "expo-media-library";
import * as FileSystem from "expo-file-system";
import { Platform } from "react-native";

const SetupVentusFolder = async () => {
  try {
    // First, request permissions
    const { status: mediaStatus } =
      await MediaLibrary.requestPermissionsAsync();
    if (mediaStatus !== "granted") {
      throw new Error("Media library permission not granted");
    }

    // Create Ventus directory in app's document directory
    const ventusPath = `${FileSystem.documentDirectory}Ventus`;

    // Check if directory exists
    const dirInfo = await FileSystem.getInfoAsync(ventusPath);

    if (!dirInfo.exists) {
      await FileSystem.makeDirectoryAsync(ventusPath, {
        intermediates: true,
      });
    }

    // Create .nomedia file only on Android
    if (Platform.OS === "android") {
      const nomediaPath = `${ventusPath}/.nomedia`;
      const nomediaInfo = await FileSystem.getInfoAsync(nomediaPath);

      if (!nomediaInfo.exists) {
        // FileSystem.
        let file = await FileSystem.writeAsStringAsync(nomediaPath, "", {
          encoding: FileSystem.EncodingType.UTF8,
        });
      }

      // Save the directory to MediaLibrary to ensure it's recognized
      // await MediaLibrary.createAlbumAsync("Ventus", file, false);
      await MediaLibrary.makeDirectoryAsync("/storage/emulated/0/download", {
        intermediates: true,
      });
    }

    return ventusPath;
  } catch (error) {
    console.error("Error in setupVentusFolder:", error);
    throw error;
  }
};

// Usage example with error handling
const initializeVentus = async () => {
  try {
    const ventusPath = await SetupVentusFolder();
    console.log("Ventus folder setup successfully at:", ventusPath);
    return ventusPath;
  } catch (error) {
    // Handle specific error types
    if (error.message.includes("permission")) {
      console.error("Permission denied:", error.message);
      // Handle permission error
    } else if (error.message.includes("Could not create asset")) {
      console.error("Asset creation failed:", error.message);
      // Handle asset creation error - might need to check storage space or permissions
    } else {
      console.error("Unknown error:", error);
    }
    throw error;
  }
};

// Additional helper to verify the setup
const verifyVentusSetup = async () => {
  try {
    const ventusPath = `${FileSystem.documentDirectory}Ventus`;
    const dirInfo = await FileSystem.getInfoAsync(ventusPath);

    if (!dirInfo.exists) {
      return false;
    }

    if (Platform.OS === "android") {
      const nomediaPath = `${ventusPath}/.nomedia`;
      const nomediaInfo = await FileSystem.getInfoAsync(nomediaPath);
      return nomediaInfo.exists;
    }

    return true;
  } catch (error) {
    console.error("Error verifying Ventus setup:", error);
    return false;
  }
};

export {
  SetupVentusFolder as setupVentusFolder,
  initializeVentus,
  verifyVentusSetup,
};
