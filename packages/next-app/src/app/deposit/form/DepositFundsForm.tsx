'use client';

import { BACK_END_CONTRACT, getUsdtContract } from '@/actions/nearActions';
import useCustomerBalanceStore from '@/store/customerBalanceStore';
import useWalletSelectorStore from '@/store/walletSelectorStore';
import { Transaction } from '@near-wallet-selector/core';
import { Button, InputNumber, notification } from 'antd';
import { useState } from 'react';

const DepositFundsForm = () => {
  const { usdtDepositBalance, usdtWalletBalance } = useCustomerBalanceStore();
  const [amount, setAmount] = useState<number | null>(0);
  const { walletSelector } = useWalletSelectorStore();
  const [api, contextHolder] = notification.useNotification();

  const depositFunds = async () => {
    if (!walletSelector) {
      api.error({ message: 'Please connect wallet' });
      return;
    }

    if (!amount) {
      api.error({ message: 'Please specify the deposit amount' });
      return;
    }

    const wallet = await walletSelector.wallet();

    const gas = new Intl.NumberFormat('us', {
      style: 'decimal',
      useGrouping: false,
    }).format(Math.pow(10, 14)); //100 TGas

    const customerRegisterDeposit = new Intl.NumberFormat('us', {
      style: 'decimal',
      useGrouping: false,
    }).format(Math.pow(10, 22)); //0.01 Near

    const usdtAmount = amount * Math.pow(10, 6);

    const accounts = await wallet.getAccounts();

    const transactions: Transaction[] = [
      {
        signerId: accounts[0].accountId,
        receiverId: getUsdtContract(),
        actions: [
          {
            type: 'FunctionCall',
            params: {
              methodName: 'ft_transfer_call',
              args: {
                amount: usdtAmount.toString(),
                receiver_id: BACK_END_CONTRACT,
                msg: '',
              },
              gas,
              deposit: '1', //1 yoctoNear
            },
          },
        ],
      },
    ];

    if (usdtDepositBalance === 0) {
      transactions.unshift({
        signerId: accounts[0].accountId,
        receiverId: BACK_END_CONTRACT,
        actions: [
          {
            type: 'FunctionCall',
            params: {
              methodName: 'register_customer',
              args: {
                customer_id: accounts[0].accountId,
              },
              gas,
              deposit: customerRegisterDeposit,
            },
          },
        ],
      });
    }

    try {
      await wallet.signAndSendTransactions({
        transactions,
      });
    } catch (e) {
      console.log('error occured', e);
      api.error({ message: 'Problem with the deposit' });
    }
  };

  return (
    <>
      {contextHolder}
      <label>
        Amount
        <InputNumber
          onChange={setAmount}
          size="large"
          className="block w-full pr-5 [&_.ant-input-number-input]:text-right"
          type="number"
          value={amount}
          min={0}
          max={usdtWalletBalance}
        />
      </label>

      <Button
        type="primary"
        size="large"
        className="mt-3 w-20 self-end"
        onClick={depositFunds}
      >
        Deposit
      </Button>
    </>
  );
};

export default DepositFundsForm;
