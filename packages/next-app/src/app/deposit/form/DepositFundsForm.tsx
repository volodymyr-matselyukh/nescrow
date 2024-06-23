'use client';

import { BACK_END_CONTRACT, getUsdtContract } from '@/actions/nearActions';
import useWalletSelectorStore from '@/store/walletSelectorStore';
import { Button, InputNumber, notification } from 'antd';
import { useState } from 'react';

const DepositFundsForm = () => {
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

    const usdtAmount = amount * Math.pow(10, 6);

    try {
      await wallet.signAndSendTransaction({
        receiverId: getUsdtContract(),
        actions: [
          {
            type: 'FunctionCall',
            params: {
              methodName: 'ft_transfer_call',
              args: {
                amount: usdtAmount.toString(),
                receiver_id: BACK_END_CONTRACT,
                msg: ""
              },
              gas,
              deposit: '1', //1 yoctoNear
            },
          },
        ],
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
