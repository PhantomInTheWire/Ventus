import { create } from "zustand";

type AuthStore = {
  isConnected: boolean;
  port: string | null;
  connect: (newPort: string) => void;
  disconnect: () => void;
};

const useAuthStore = create<AuthStore>((set) => ({
  isConnected: false,
  port: null,
  connect: (newPort: string) => {
    set((state) => ({
      ...state,
      isConnected: true,
      port: newPort,
    }));
    console.log(`Connected to ${newPort}!`);
  },
  disconnect: () => {
    set((state) => ({
      ...state,
      isConnected: false,
      port: null,
    }));
    console.log("Disconnected!");
  },
}));

export default useAuthStore;
