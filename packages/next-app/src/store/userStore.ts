import { User } from '@supabase/supabase-js';
import { create } from 'zustand';

interface UserStore {
  user: User | null;
  setUser: (value: User | null) => void;
}

const useUserStore = create<UserStore>()((set) => ({
  user: null,
  setUser: (newValue) => {
    set({ user: newValue });
  },
}));

export default useUserStore;
