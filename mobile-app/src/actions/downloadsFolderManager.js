import * as FileSystem from "expo-file-system";
import * as MediaLibrary from "expo-media-library";
import * as Permissions from "expo-permissions";
import { Platform } from "react-native";
import { useState } from "react";

const DownloadsFolderManager = () => {
  const [error, setError] = useState(null);
  const [permissionStatus, setPermissionStatus] = useState(null);

  // Request necessary permissions
  const requestPermissions = async () => {
    try {
      if (Platform.OS === "android") {
        const { status } = await MediaLibrary.requestPermissionsAsync();
        setPermissionStatus(status);
        return status === "granted";
      }
      return true; // iOS handles permissions differently
    } catch (error) {
      console.error("Error requesting permissions:", error);
      return false;
    }
  };

  // Get downloads directory path (Android only)
  const getDownloadsPath = async () => {
    if (Platform.OS === "android") {
      // This is a common path for downloads on Android
      return FileSystem.documentDirectory + "../Download/";
    } else {
      throw new Error("Direct downloads folder access not available on iOS");
    }
  };

  // Create folder in downloads (Android)
  const createFolderInAndroidDownloads = async (folderName) => {
    try {
      const hasPermission = await requestPermissions();
      if (!hasPermission) {
        throw new Error("Storage permission not granted");
      }

      // Create a temporary file in app directory first
      const tempPath = `${FileSystem.documentDirectory}${folderName}`;
      await FileSystem.makeDirectoryAsync(tempPath, {
        intermediates: true,
      });

      // Create empty file to ensure folder shows up in MediaLibrary
      const placeholderFile = `${tempPath}/.nomedia`;
      await FileSystem.writeAsStringAsync(placeholderFile, "");

      // Move to downloads using MediaLibrary
      const asset = await MediaLibrary.createAssetAsync(placeholderFile);
      await MediaLibrary.createAlbumAsync(folderName, asset, false);

      // Clean up temp directory
      await FileSystem.deleteAsync(tempPath, { idempotent: true });

      return true;
    } catch (error) {
      console.error("Error creating folder in downloads:", error);
      throw error;
    }
  };

  // Main function to create folder in downloads
  const createFolderInDownloads = async (folderName) => {
    try {
      if (!folderName || folderName.trim() === "") {
        throw new Error("Please provide a valid folder name");
      }

      if (Platform.OS === "android") {
        await createFolderInAndroidDownloads(folderName);
        return `Created folder "${folderName}" in Downloads`;
      } else {
        throw new Error(
          "Creating folders in Downloads is not supported on iOS due to system restrictions. " +
            "Consider using the app's document directory instead."
        );
      }
    } catch (err) {
      setError(err.message);
      throw err;
    }
  };

  // Check if a folder exists in downloads (Android only)
  const checkFolderExists = async (folderName) => {
    try {
      if (Platform.OS === "android") {
        const albums = await MediaLibrary.getAlbumsAsync();
        return albums.some((album) => album.title === folderName);
      }
      return false;
    } catch (error) {
      console.error("Error checking folder existence:", error);
      return false;
    }
  };

  // Example usage
  const exampleUsage = async () => {
    try {
      // Create a new folder in downloads
      await createFolderInDownloads("MyExpoFolder");

      // Check if folder exists
      const exists = await checkFolderExists("MyExpoFolder");
      console.log("Folder exists:", exists);
    } catch (err) {
      console.error("Error in example usage:", err);
    }
  };

  return {
    createFolderInDownloads,
    checkFolderExists,
    permissionStatus,
    error,
    exampleUsage,
  };
};

export default DownloadsFolderManager;
