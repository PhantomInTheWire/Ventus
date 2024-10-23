import * as FileSystem from "expo-file-system";
import { useState } from "react";

const FolderManager = () => {
  const [currentPath, setCurrentPath] = useState(FileSystem.documentDirectory);
  const [folderContents, setFolderContents] = useState([]);
  const [error, setError] = useState(null);

  // Create a new folder
  const createFolder = async (folderName) => {
    try {
      // Ensure folder name is valid
      if (!folderName || folderName.trim() === "") {
        throw new Error("Please provide a valid folder name");
      }

      // Create the full path
      const newFolderPath = `${currentPath}${folderName}`;

      // Check if folder already exists
      const folderInfo = await FileSystem.getInfoAsync(newFolderPath);
      if (folderInfo.exists) {
        throw new Error("A folder with this name already exists");
      }

      // Create the directory
      await FileSystem.makeDirectoryAsync(newFolderPath, {
        intermediates: true, // Create parent directories if they don't exist
      });

      // Refresh folder contents
      await listFolderContents();

      return newFolderPath;
    } catch (err) {
      setError(err.message);
      throw err;
    }
  };

  // Delete a folder
  const deleteFolder = async (folderName) => {
    try {
      const folderPath = `${currentPath}${folderName}`;

      // Check if folder exists
      const folderInfo = await FileSystem.getInfoAsync(folderPath);
      if (!folderInfo.exists) {
        throw new Error("Folder does not exist");
      }

      // Delete the directory and its contents
      await FileSystem.deleteAsync(folderPath, { idempotent: true });

      // Refresh folder contents
      await listFolderContents();
    } catch (err) {
      setError(err.message);
      throw err;
    }
  };

  // List contents of current folder
  const listFolderContents = async () => {
    try {
      const contents = await FileSystem.readDirectoryAsync(currentPath);

      // Get detailed info for each item
      const contentsWithInfo = await Promise.all(
        contents.map(async (item) => {
          const info = await FileSystem.getInfoAsync(`${currentPath}${item}`);
          return {
            name: item,
            ...info,
          };
        })
      );

      setFolderContents(contentsWithInfo);
      return contentsWithInfo;
    } catch (err) {
      setError(err.message);
      throw err;
    }
  };

  // Navigate into a folder
  const navigateToFolder = async (folderName) => {
    try {
      const newPath = `${currentPath}${folderName}/`;

      // Check if folder exists
      const folderInfo = await FileSystem.getInfoAsync(newPath);
      if (!folderInfo.exists || !folderInfo.isDirectory) {
        throw new Error("Invalid folder");
      }

      setCurrentPath(newPath);
      await listFolderContents();
    } catch (err) {
      setError(err.message);
      throw err;
    }
  };

  // Navigate up one level
  const navigateUp = async () => {
    try {
      if (currentPath === FileSystem.documentDirectory) {
        throw new Error("Already at root directory");
      }

      // Remove trailing slash and get parent path
      const parentPath =
        currentPath.slice(0, -1).split("/").slice(0, -1).join("/") + "/";
      setCurrentPath(parentPath);
      await listFolderContents();
    } catch (err) {
      setError(err.message);
      throw err;
    }
  };

  // Get the current directory path relative to app root
  const getRelativePath = () => {
    return currentPath.replace(FileSystem.documentDirectory, "");
  };

  // Example usage
  const exampleUsage = async () => {
    try {
      // Create a new folder
      // await createFolder("MyDocuments");

      // Navigate into the folder
      await navigateToFolder("/storage/emulated/0/Download/");

      // Create a subfolder
      await createFolder("Ventus");

      // List contents
      const contents = await listFolderContents();
      console.log("Current folder contents:", contents);

      // Navigate back up
      await navigateUp();
    } catch (err) {
      console.error("Error:", err);
    }
  };

  return {
    currentPath,
    folderContents,
    error,
    createFolder,
    deleteFolder,
    listFolderContents,
    navigateToFolder,
    navigateUp,
    getRelativePath,
    exampleUsage,
  };
};

export default FolderManager;
