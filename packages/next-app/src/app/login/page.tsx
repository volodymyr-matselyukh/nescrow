'use client';

import { Radio, RadioChangeEvent } from 'antd';
import { useState } from 'react';
import SignIn from './components/SignIn';
import SignUp from './components/SignUp';

enum SignInType {
  SIGN_IN,
  SIGN_UP,
}

const options = [
  { label: 'SignIn', value: SignInType.SIGN_IN },
  { label: 'SignUp', value: SignInType.SIGN_UP },
];

const Page = ({ searchParams }: { searchParams: { message: string } }) => {
  const [signInType, setSignInType] = useState(SignInType.SIGN_IN);

  const onSignInTypeChange = ({ target: { value } }: RadioChangeEvent) => {
    setSignInType(value);
  };

  const getSignInComponent = () => {
    switch (signInType) {
      case SignInType.SIGN_IN:
        return <SignIn></SignIn>;
      case SignInType.SIGN_UP:
        return <SignUp></SignUp>;
    }
  };

  return (
    <div className="flex w-[500px] flex-col gap-5">
      <Radio.Group
        options={options}
        onChange={onSignInTypeChange}
        value={signInType}
        optionType="button"
        buttonStyle="solid"
        className="text-center"
      />

      {getSignInComponent()}

      {searchParams?.message && (
        <p className="text-foreground rounded-lg bg-gray-300 p-4 text-center">
          {searchParams.message}
        </p>
      )}
    </div>
  );
};

export default Page;
