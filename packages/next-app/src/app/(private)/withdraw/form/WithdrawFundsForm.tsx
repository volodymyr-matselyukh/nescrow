'use client';

import { BACK_END_CONTRACT } from '@/actions/nearActions';
import { useUser } from '@/hooks/useUser';
import useCustomerBalanceStore from '@/store/customerBalanceStore';
import useWalletSelectorStore from '@/store/walletSelectorStore';
import { getCurrencyString } from '@/utils/money';
import { Account, Transaction } from '@near-wallet-selector/core';
import { Button, Form, InputNumber, notification, Typography } from 'antd';

import { useState } from 'react';
const { Text } = Typography;

const WithdrawFundsForm = () => {
  const { usdtWithdrawableBalance } = useCustomerBalanceStore();
  const [amount, setAmount] = useState<number | null>(0);
  const { walletSelector } = useWalletSelectorStore();
  const [api, contextHolder] = notification.useNotification();
  const { user } = useUser();

  const withdrawFunds = async () => {
    if (!walletSelector) {
      api.error({ message: 'Please connect wallet' });
      return;
    }

    if (!amount) {
      api.error({
        message: `Withdraw amount should be bigger than ${getCurrencyString(0)}`,
      });
      return;
    }

    const wallet = await walletSelector.wallet();

    const usdtAmount = amount * Math.pow(10, 6);

    const accounts = await wallet.getAccounts();

    let transactions: Transaction[] = [];

    try {
      transactions = await prepareTransactions(accounts, usdtAmount);
    } catch (error) {
      api.error({ message: 'Preparing transaction issue' });
      return;
    }

    try {
      await wallet.signAndSendTransactions({
        transactions,
      });
    } catch (e) {
      console.log('error occured', e);
      api.error({ message: 'Problem with the withdraw' });
    }
  };

  const prepareTransactions = async (
    accounts: Account[],
    usdtAmount: number,
  ) => {
    if (!walletSelector) {
      throw new Error('Wallet selector is empty');
    }

    if (!user?.email) {
      throw new Error('User email is unknown');
    }

    const gas = new Intl.NumberFormat('us', {
      style: 'decimal',
      useGrouping: false,
    }).format(Math.pow(10, 14)); //100 TGas

    const transactions: Transaction[] = [
      {
        signerId: accounts[0].accountId,
        receiverId: BACK_END_CONTRACT,
        actions: [
          {
            type: 'FunctionCall',
            params: {
              methodName: 'withdraw',
              args: {
                amount: usdtAmount.toString(),
                receiver_email: user.email,
              },
              gas,
              deposit: '1', //1 yoctoNear
            },
          },
        ],
      },
    ];

    return transactions;
  };

  return (
    <Form onFinish={withdrawFunds} className="flex flex-col">
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
          max={usdtWithdrawableBalance}
        />
      </label>

      <Text type="secondary" className="self-end">
        The amount will be withdrawn to connected wallet.
      </Text>
      <Text type="warning" className="self-end">
        Maximum amount to withdraw {getCurrencyString(usdtWithdrawableBalance)}
      </Text>

      <Button
        type="primary"
        htmlType="submit"
        size="large"
        className="mt-3 w-20 self-end"
      >
        Withdraw
      </Button>
    </Form>
  );
};

export default WithdrawFundsForm;
