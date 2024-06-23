import { getDeposit, getWalletBalance } from '@/actions/nearActions';
import useWalletSelectorStore from '@/store/walletSelectorStore';
import { Spin } from 'antd';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';

const Deposit = () => {
  const [currentDeposit, setcurrentDeposit] = useState<number | null>(null);
  const [walletBalance, setwalletBalance] = useState<number | null>(null);
  const { walletSelector } = useWalletSelectorStore();

  useEffect(() => {
    if (!walletSelector) {
      return;
    }

    getDeposit(walletSelector)
      .then((amount) => setcurrentDeposit(amount))
      .catch((e) => toast.error('Error getting deposit'));

    getWalletBalance(walletSelector)
      .then((amount) => setwalletBalance(amount))
      .catch((e) => toast.error('Error getting balance'));
  }, [walletSelector]);

  const getCurrencyString = (amount: number) => {
    const formatter = new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    });

    return formatter.format(amount);
  };

  return (
    <div className='flex flex-col items-end'>
      <div className="flex gap-2">
        <span>Deposit:</span>
        <span className="font-medium">
          {currentDeposit ? (
            getCurrencyString(currentDeposit)
          ) : (
            <Spin className="flex" size="small" />
          )}
        </span>
      </div>
      <div className="flex gap-2">
        <span>Wallet balance:</span>
        <span className="font-medium">
          {walletBalance ? (
            getCurrencyString(walletBalance)
          ) : (
            <Spin className="flex" size="small" />
          )}
        </span>
      </div>
    </div>
  );
};

export default Deposit;
