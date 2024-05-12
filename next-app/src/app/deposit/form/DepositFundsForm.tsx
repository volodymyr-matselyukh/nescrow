import { Button, InputNumber } from 'antd';
import { useState } from 'react';

const DepositFundsForm = () => {
  const [amount, setAmount] = useState(500);

  return (
    <>
      <label>
        Amount
        <InputNumber
          size="large"
          className="block w-full pr-5 [&>.ant-input-number-input]:text-right"
          type="number"
          value={amount}
        />
      </label>

      <Button type="primary" size="large" className="mt-3 w-40 justify-end self-end">
        Deposit
      </Button>
    </>
  );
};

export default DepositFundsForm;
