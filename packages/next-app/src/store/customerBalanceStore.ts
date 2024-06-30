import { create } from "zustand";


interface CustomerBalanceStore {
  usdtWalletBalance: number;
  setUsdtWalletBalance: (value: number) => void;

  usdtDepositBalance: number;
  setUsdtDepositBalance: (value: number) => void;
}

const useCustomerBalanceStore = create<CustomerBalanceStore>()((set) => ({
  usdtWalletBalance: 0,
  usdtDepositBalance: 0,
  setUsdtWalletBalance: (newValue) => { set({ usdtWalletBalance: newValue }) },
  setUsdtDepositBalance: (newValue) => { set({ usdtDepositBalance: newValue }) },
}));

export default useCustomerBalanceStore;
