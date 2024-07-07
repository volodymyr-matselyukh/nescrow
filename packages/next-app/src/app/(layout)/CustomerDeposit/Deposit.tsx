import { getDeposit, getWalletBalance } from '@/actions/nearActions';
import { useUser } from '@/hooks/useUser';
import useCustomerBalanceStore from '@/store/customerBalanceStore';
import useWalletSelectorStore from '@/store/walletSelectorStore';
import { getCurrencyString } from '@/utils/money';
import { Spin } from 'antd';
import { useEffect } from 'react';
import { toast } from 'react-toastify';

const Deposit = () => {
  const {
    usdtDepositBalance,
    setUsdtDepositBalance,
    usdtWalletBalance,
    setUsdtWalletBalance,
  } = useCustomerBalanceStore();

  const { walletSelector } = useWalletSelectorStore();
  const { user } = useUser();

  useEffect(() => {
    if (!walletSelector) {
      return;
    }

    getWalletBalance(walletSelector)
      .then((amount) => {
        setUsdtWalletBalance(amount);
      })
      .catch((e) => toast.error('Error getting balance'));
  }, [walletSelector]);


  useEffect(() => {
    if (!walletSelector) {
      return;
    }

    if(user?.email)
      {
        getDeposit(user?.email, walletSelector)
        .then((amount) => {
          setUsdtDepositBalance(amount);
        })
        .catch((e) => toast.error('Error getting deposit'));
      }
  }, [walletSelector, user?.email])

  return (
    <div className="flex flex-col items-end">
      <div className="flex gap-2">
        <span>Deposit:</span>
        <span className="font-medium">
          {usdtDepositBalance != null && usdtDepositBalance !== undefined ? (
            getCurrencyString(usdtDepositBalance)
          ) : (
            <Spin className="flex" size="small" />
          )}
        </span>
      </div>
      <div className="flex gap-2">
        <span>Wallet balance:</span>
        <span className="font-medium">
          {usdtWalletBalance != null && usdtWalletBalance !== undefined ? (
            getCurrencyString(usdtWalletBalance)
          ) : (
            <Spin className="flex" size="small" />
          )}
        </span>
      </div>
    </div>
  );
};

export default Deposit;
