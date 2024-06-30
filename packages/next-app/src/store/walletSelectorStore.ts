import { WalletSelector } from '@near-wallet-selector/core';
import { create } from 'zustand';

interface WalletSelectorStore {
  walletSelector: WalletSelector | null;
  setWalletSelector: (value: WalletSelector | null) => void;
}

const useWalletSelectorStore = create<WalletSelectorStore>()((set) => ({
  walletSelector: null,
  setWalletSelector: (newValue) => { set({ walletSelector: newValue }) },
}));

export default useWalletSelectorStore;
