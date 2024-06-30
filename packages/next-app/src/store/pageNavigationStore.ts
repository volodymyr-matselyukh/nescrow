import { create } from 'zustand'

interface PageNavigationStore {
  isNavigating: boolean;
  setIsNavigating: (value: boolean) => void;
}

const usePageNavigationStore = create<PageNavigationStore>()((set) => ({
  isNavigating: false,
  setIsNavigating: (newValue) => { set({ isNavigating: newValue }) },
}));

export default usePageNavigationStore;