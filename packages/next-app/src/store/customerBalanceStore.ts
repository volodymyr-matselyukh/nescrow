import { create } from "zustand";


interface CustomerBalanceStore {
  usdtWalletBalance: number;
  setUsdtWalletBalance: (value: number) => void;

  usdtDepositBalance: number;
  setUsdtDepositBalance: (value: number) => void;

  usdtWithdrawableBalance: number;
  setUsdtWithdrawableBalance: (value: number) => void;
}

const useCustomerBalanceStore = create<CustomerBalanceStore>()((set) => ({
  usdtWalletBalance: 0,
  setUsdtWalletBalance: (newValue) => { set({ usdtWalletBalance: newValue }) },

  usdtDepositBalance: 0,
  setUsdtDepositBalance: (newValue) => { set({ usdtDepositBalance: newValue }) },

  usdtWithdrawableBalance: 0,
  setUsdtWithdrawableBalance: (newValue) => { set({ usdtWithdrawableBalance: newValue }) },
}));

export default useCustomerBalanceStore;
