import * as FileSystem from "expo-file-system";
import * as MediaLibrary from "expo-media-library";
import { Platform } from "react-native";
import { useState, useEffect } from "react";

const VentusFolderManager = () => {
  const [files, setFiles] = useState([]);
  const [error, setError] = useState(null);
  const [folderExists, setFolderExists] = useState(false);
  const VENTUS_FOLDER = "Ventusss";

  // Request permissions and initialize
  const initialize = async () => {
    try {
      const { status } = await MediaLibrary.requestPermissionsAsync();
      if (status !== "granted") {
        throw new Error("Storage permission is required");
      }

      await ensureVentusFolder();
      await listVentusFiles();
    } catch (error) {
      setError(error.message);
      console.error("Initialization error:", error);
    }
  };

  // Check if Ventus folder exists and create if it doesn't
  const ensureVentusFolder = async () => {
    try {
      if (Platform.OS === "ios") {
        throw new Error("Direct downloads folder access not available on iOS");
      }

      const { status } = await MediaLibrary.requestPermissionsAsync();
      if (status !== "granted") {
        console.error("Permission to access photos is not granted");
        return;
      }

      // Check if folder exists
      const albums = await MediaLibrary.getAlbumsAsync();
      const ventusAlbum = albums.find((album) => album.title === VENTUS_FOLDER);

      if (!ventusAlbum) {
        // Create the Ventus folder
        // First create a temporary file to ensure folder creation
        const tempPath = `${FileSystem.documentDirectory}${VENTUS_FOLDER}`;
        // const tempPath = `/phone/emulated/0/${VENTUS_FOLDER}`;
        await FileSystem.makeDirectoryAsync(tempPath, {
          intermediates: true,
        });
        // console.log("Creating .nomedia file");
        // Create .nomedia file to ensure folder shows up
        const placeholderFile = `${tempPath}/.nomedia`;
        await FileSystem.writeAsStringAsync(placeholderFile, "");
        console.log("Created .nomedia file");

        // Move to downloads using MediaLibrary
        const asset = await MediaLibrary.createAssetAsync(placeholderFile);
        await MediaLibrary.createAlbumAsync(VENTUS_FOLDER, asset, false);
        console.log("Moved .nomedia file to Ventus");

        // Clean up temp directory
        await FileSystem.deleteAsync(tempPath, { idempotent: true });

        console.log("Ventus folder created successfully");
      }

      setFolderExists(true);
      return true;
    } catch (error) {
      console.error("Error ensuring Ventus folder:", error);
      setError(error.message);
      return false;
    }
  };

  // List all files in the Ventus folder
  const listVentusFiles = async () => {
    try {
      if (Platform.OS === "ios") {
        throw new Error("Direct downloads folder access not available on iOS");
      }

      // Get the Ventus album
      const albums = await MediaLibrary.getAlbumsAsync();
      const ventusAlbum = albums.find((album) => album.title === VENTUS_FOLDER);

      if (!ventusAlbum) {
        setFiles([]);
        return [];
      }

      // Get all assets in the Ventus album
      const assets = await MediaLibrary.getAssetsAsync({
        album: ventusAlbum,
        mediaType: ["photo", "video", "audio", "unknown"],
        first: 1000, // Adjust this number based on your needs
      });

      const fileList = assets.assets.map((asset) => ({
        id: asset.id,
        filename: asset.filename,
        uri: asset.uri,
        mediaType: asset.mediaType,
        creationTime: asset.creationTime,
        modificationTime: asset.modificationTime,
        duration: asset.duration,
        width: asset.width,
        height: asset.height,
        size: asset.fileSize,
      }));

      setFiles(fileList);
      return fileList;
    } catch (error) {
      console.error("Error listing Ventus files:", error);
      setError(error.message);
      return [];
    }
  };

  // Refresh the file list
  const refreshFiles = async () => {
    await listVentusFiles();
  };

  // Initialize on component mount
  useEffect(() => {
    initialize();
  }, []);

  // Add a new file to Ventus folder (example function)
  const addFileToVentus = async (fileUri) => {
    try {
      if (!folderExists) {
        await ensureVentusFolder();
      }

      const asset = await MediaLibrary.createAssetAsync(fileUri);
      const albums = await MediaLibrary.getAlbumsAsync();
      const ventusAlbum = albums.find((album) => album.title === VENTUS_FOLDER);

      if (ventusAlbum) {
        await MediaLibrary.addAssetsToAlbumAsync([asset], ventusAlbum, false);
        await refreshFiles();
        return true;
      }

      return false;
    } catch (error) {
      console.error("Error adding file to Ventus:", error);
      setError(error.message);
      return false;
    }
  };

  return {
    files,
    error,
    folderExists,
    refreshFiles,
    addFileToVentus,
    initialize,
    listVentusFiles,
  };
};

export default VentusFolderManager;
