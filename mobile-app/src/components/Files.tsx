import * as FileSystem from "expo-file-system";
import { useState, useEffect } from "react";
import { StyleSheet, Text, View, ScrollView } from "react-native";
import TitleBar from "./TitleBar";
// import FolderAccess from "./Files2";
// import FolderManager from "@/actions/folderManager";
// import DownloadsFolderManager from "@/actions/downloadsFolderManager";
// import VentusFolderManager from "@/actions/VentusFolderManager";
// import {
//   setupVentusFolder,
//   initializeVentus,
//   verifyVentusSetup,
// } from "@/actions/SetupVentusFolder";

export default function Files() {
  // const [fileContent, setFileContent] = useState("");
  // const [ventusFiles, setVentusFiles] = useState<any>();
  // const [fileNames, setFileNames] = useState<string[]>();
  // const {
  //   // files,
  //   error,
  //   folderExists,
  //   // refreshFiles,
  //   addFileToVentus,
  //   initialize,
  //   listVentusFiles,
  // } = VentusFolderManager();

  // const { files, refreshFiles } = FolderAccess();
  // const {setupVentusFolder, initializeVentus, verifyVentusSetup} = SetupVentusFolder();
  //   const {
  //     currentPath,
  //     folderContents,
  //     error,
  //     createFolder,
  //     deleteFolder,
  //     listFolderContents,
  //     navigateToFolder,
  //     navigateUp,
  //     getRelativePath,
  //   } = FolderManager();
  //   const {
  //     createFolderInDownloads,
  //     checkFolderExists,
  //     permissionStatus,
  //     //   error,
  //     exampleUsage,
  //   } = DownloadsFolderManager();

  // Function to write to a file
  const writeToFile = async (filename: string, content: any) => {
    try {
      const path = `${FileSystem.documentDirectory}${filename}`;
      await FileSystem.writeAsStringAsync(path, content);
      console.log("File written successfully");
      return path;
    } catch (error) {
      console.error("Error writing file:", error);
      throw error;
    }
  };

  // Function to read from a file
  const readFromFile = async (filename: string) => {
    try {
      const path = `${FileSystem.documentDirectory}${filename}`;
      const content = await FileSystem.readAsStringAsync(path);
      return content;
    } catch (error) {
      console.error("Error reading file:", error);
      throw error;
    }
  };
  //
  // Function to list directory contents
  const listDirectory = async (directory = FileSystem.documentDirectory) => {
    try {
      const contents = await FileSystem.readDirectoryAsync(
        directory ?? "./phone"
      );
      return contents;
    } catch (error) {
      console.error("Error listing directory:", error);
      throw error;
    }
  };

  // Function to delete a file
  const deleteFile = async (filename: string) => {
    try {
      const path = `${FileSystem.documentDirectory}${filename}`;
      await FileSystem.deleteAsync(path);
      console.log("File deleted successfully");
    } catch (error) {
      console.error("Error deleting file:", error);
      throw error;
    }
  };

  // Example usage
  useEffect(() => {
    const demonstrateFileOperations = async () => {
      try {
        // initialize();
        // setVentusFiles(await listVentusFiles());
        // Write a file
        // await writeToFile("example.txt", "Hello, Expo!");
        // Read the file
        // const content = await readFromFile("example.txt");
        // setFileContent(content);
        // List directory contents
        // const newFiles = await listDirectory("/phone");
        // setFileNames(newFiles);
        // console.log("Files in directory:", newFiles);
        // Delete file (uncomment to test)
        // await deleteFile('example.txt');
        // await navigateToFolder("storage/emulated/0/Download/");
        // await createFolder("Ventus");
        // exampleUsage();
        // if (!(await checkFolderExists("/Ventus"))) {
        //   createFolderInDownloads("/Ventus");
        //   console.info(checkFolderExists("Ventus"));
        // }
        // initializeVentus();
        // refreshFiles();
      } catch (error) {
        console.error("File operation failed:", error);
      }
    };

    demonstrateFileOperations();
  }, []);

  return (
    <ScrollView style={styles.container}>
      <TitleBar icons={["back"]} />
      <View style={{ marginTop: 20 }}>
        {/* {files?.length ? (
          files.map((file, index) => (
            <Text style={styles.text} key={index}>
              {file.uri}
            </Text>
          ))
        ) : (
          <Text style={styles.text}>No files</Text>
          )} */}
        <Text style={styles.text}>No files</Text>
      </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    paddingHorizontal: 20,
  },
  text: {
    color: "#dadada",
    fontSize: 20,
    marginTop: 10,
  },
});
