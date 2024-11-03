import { create } from "zustand";

export interface Settings {
  deviceName: string;
  targetFolder: string;
  maxConn: number;
  maxRate: number;
  sizeLimit: number;
}

interface SettingsStore {
  settings: Settings;
  updateSetting: <K extends keyof Settings>(key: K, value: Settings[K]) => void;
}

const useSettingsStore = create<SettingsStore>((set) => ({
  settings: {
    deviceName: "Mobile",
    targetFolder: "Phone/Downloads/Ventus",
    maxConn: 5,
    maxRate: 1,
    sizeLimit: 10,
  },
  updateSetting: (key, value) => {
    set((state) => ({
      ...state,
      settings: {
        ...state.settings,
        [key]: value,
      },
    }));
    console.log("Updated!");
  },
}));

export default useSettingsStore;
