import { Platform } from "react-native";
import * as FileSystem from "expo-file-system";
import * as MediaLibrary from "expo-media-library";
import { useState, useEffect } from "react";
import * as DocumentPicker from "expo-document-picker";

const FolderAccess = () => {
  const [files, setFiles] = useState([]);
  const [permissionStatus, setPermissionStatus] = useState(null);

  // Request necessary permissions
  const requestPermissions = async () => {
    try {
      // For Android, we need storage permissions
      if (Platform.OS === "android") {
        const { status } = await MediaLibrary.requestPermissionsAsync();
        setPermissionStatus(status);
        return status === "granted";
      }
      // For iOS, we'll use document picker which handles its own permissions
      return true;
    } catch (error) {
      console.error("Error requesting permissions:", error);
      return false;
    }
  };

  // Get downloads on Android using MediaLibrary
  const getAndroidDownloads = async () => {
    try {
      // Get all downloads
      const media = await MediaLibrary.getAssetsAsync({
        mediaType: ["photo", "video", "audio", "unknown"],
        first: 100, // Limit the number of files to retrieve
      });

      // Filter for files in the Downloads directory
      const downloads = media.assets.filter((asset) => {
        return (
          asset.uri.toLowerCase().includes("downloads") ||
          asset.albumId === "downloads"
        );
      });

      return downloads.map((file) => ({
        name: file.filename,
        uri: file.uri,
        size: file.fileSize,
        type: file.mediaType,
        modificationTime: file.modificationTime,
      }));
    } catch (error) {
      console.error("Error getting Android downloads:", error);
      return [];
    }
  };

  // Get downloads on iOS using DocumentPicker
  const getIOSDownloads = async () => {
    try {
      // On iOS, we can't directly access the downloads folder
      // Instead, we'll let the user pick files using DocumentPicker
      const result = await DocumentPicker.getDocumentAsync({
        type: "*/*", // All file types
        multiple: true,
      });

      if (result.canceled) {
        return [];
      }

      return result.assets.map((file) => ({
        name: file.name,
        uri: file.uri,
        size: file.size,
        type: file.mimeType,
      }));
    } catch (error) {
      console.error("Error getting iOS files:", error);
      return [];
    }
  };

  // Main function to get downloads
  const getDownloads = async () => {
    const hasPermission = await requestPermissions();
    if (!hasPermission) {
      console.log("Permission not granted");
      return;
    }

    try {
      const downloadedFiles =
        Platform.OS === "ios"
          ? await getIOSDownloads()
          : await getAndroidDownloads();

      setFiles(downloadedFiles);
    } catch (error) {
      console.error("Error getting downloads:", error);
    }
  };

  // Example usage in a component
  useEffect(() => {
    getDownloads();
  }, []);

  return {
    files,
    permissionStatus,
    refreshFiles: getDownloads,
  };
};

export default FolderAccess;
