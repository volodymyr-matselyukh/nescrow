'use client';

import { type WalletSelector, setupWalletSelector } from '@near-wallet-selector/core';
import { setupHereWallet } from '@near-wallet-selector/here-wallet';
import { setupMeteorWallet } from '@near-wallet-selector/meteor-wallet';
import { setupModal } from '@near-wallet-selector/modal-ui';
import { setupMyNearWallet } from '@near-wallet-selector/my-near-wallet';
import { Button } from 'antd';
import { useEffect, useRef, useState } from 'react';
import { toast } from 'react-toastify';
import '@near-wallet-selector/modal-ui/styles.css';

const SignIn = () => {
  const walletSelectorRef = useRef<WalletSelector>();
  const [isLoading, setIsLoading] = useState(true);
  const [accounts, setAccounts] = useState<string[]>([]);

  useEffect(() => {
    setIsLoading(true);
    initSelector()
      .then(async () => {
        await checkIfWalletConnected();
      })
      .catch(() => toast('Error initing selector'))
      .finally(() => {
        setIsLoading(false);
      });
  }, []);

  const initSelector = async () => {
    if (walletSelectorRef.current !== undefined) {
      return;
    }

    const selector = await setupWalletSelector({
      network: 'testnet',
      modules: [setupMyNearWallet(), setupHereWallet(), setupMeteorWallet()],
    });

    walletSelectorRef.current = selector;
  };

  const checkIfWalletConnected = async () => {
    if (walletSelectorRef.current !== undefined) {
      const wallet = await walletSelectorRef.current.wallet();

      const accounts = await wallet.getAccounts();

      setAccounts(accounts.map((acc) => acc.accountId));
    }
  };

  const signIn = async () => {
    try {
      if (walletSelectorRef.current === undefined) {
        toast.error("Modal isn't initialized");
        return;
      }

      const modal = setupModal(walletSelectorRef.current, {
        contractId: 'malicious-basketball.testnet',
      });

      modal.show();
    } catch (error) {
      toast.error('Error connecting to wallet');
    }
  };

  return (
    <>
      {accounts.length > 0 ? (
        <span>{accounts[0]}</span>
      ) : (
        <Button type="primary" onClick={signIn} loading={isLoading}>
          Sign In
        </Button>
      )}
    </>
  );
};

export default SignIn;
