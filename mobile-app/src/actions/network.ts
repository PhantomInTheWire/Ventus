import * as FileSystem from "expo-file-system";

async function downloadFileFromFTP(ftpUrl: string, filename: string) {
  try {
    const downloadPath = FileSystem.documentDirectory + filename; // Choose download path
    const response = await FileSystem.downloadAsync(ftpUrl, downloadPath);
    console.log("Download complete:", response.uri);
    return response.uri; // Return the downloaded file path
  } catch (error) {
    console.error("Download error:", error);
  }
}

// Example usage:
const ftpUrl = "ftp://username:password@ftp.example.com/path/to/file.txt";
const filename = "file.txt";

downloadFileFromFTP(ftpUrl, filename).then((filePath) => {
  console.log("Downloaded file:", filePath);
  // Now you can use the downloaded file
});
