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
    set((state: AuthStore) => {
      if (newPort === state.port) return state;
      console.info(`Connected to ${newPort}!`);
      return {
        ...state,
        isConnected: true,
        port: newPort,
      };
    });
  },
  disconnect: () => {
    set((state: AuthStore) => ({
      ...state,
      isConnected: false,
      port: null,
    }));
    console.warn("Disconnected!");
  },
}));

export default useAuthStore;
