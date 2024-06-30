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

const Page = () => {
  const [signInType, setSignInType] = useState(SignInType.SIGN_IN);

  const onSignInTypeChange = ({ target: { value } }: RadioChangeEvent) => {
    console.log('radio4 checked', value);
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
    <div className="flex flex-col gap-5 text-center">
      <Radio.Group
        options={options}
        onChange={onSignInTypeChange}
        value={signInType}
        optionType="button"
        buttonStyle="solid"
      />

      {getSignInComponent()}
    </div>
  );
};

export default Page;
