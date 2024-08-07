'use client';

import useWalletSelectorStore from '@/store/walletSelectorStore';
import { setupWalletSelector } from '@near-wallet-selector/core';
import { setupHereWallet } from '@near-wallet-selector/here-wallet';
import { setupMeteorWallet } from '@near-wallet-selector/meteor-wallet';
import { setupModal } from '@near-wallet-selector/modal-ui';
import '@near-wallet-selector/modal-ui/styles.css';
import { setupMyNearWallet } from '@near-wallet-selector/my-near-wallet';

import { BACK_END_CONTRACT } from '@/actions/nearActions';
import { DisconnectOutlined, WalletOutlined } from '@ant-design/icons';
import { Button, Dropdown, message, type MenuProps } from 'antd';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import Deposit from './CustomerDeposit/Deposit';
import useCustomerBalanceStore from '@/store/customerBalanceStore';

const NETWORK = 'testnet';

const items: MenuProps['items'] = [
  {
    label: 'Disconnect',
    key: '1',
    icon: <DisconnectOutlined />,
  },
];

const ConnectedWallet = () => {
  const [isLoading, setIsLoading] = useState(true);
  const [accounts, setAccounts] = useState<string[]>([]);
  const { walletSelector, setWalletSelector } = useWalletSelectorStore();
  const { setUsdtWalletBalance } = useCustomerBalanceStore();

  const signOut: MenuProps['onClick'] = async (e) => {
    if (walletSelector !== null) {
      const wallet = await walletSelector.wallet();
      await wallet.signOut();
      refreshWalletAccounts();
      setUsdtWalletBalance(0);

      await message.info('Wallet disconnected');
    } else {
      await message.warning('No wallet connected');
    }
  };

  const menuProps = {
    items,
    onClick: signOut,
  };

  useEffect(() => {
    setIsLoading(true);
    initAndGetSelector()
      .then((selector) => {
        if (selector != null) {
          setWalletSelector(selector);
        }
      })
      .catch((e) => toast.error('Error initializing wallet'))
      .finally(() => {
        setIsLoading(false);
      });
  }, []);

  useEffect(() => {
    refreshWalletAccounts();
  }, [walletSelector]);

  const initAndGetSelector = async () => {
    if (walletSelector !== null) {
      return;
    }

    const selector = await setupWalletSelector({
      network: NETWORK,
      modules: [setupMyNearWallet(), setupHereWallet(), setupMeteorWallet()],
    });

    return selector;
  };

  const refreshWalletAccounts = () => {
    getWalletAccounts()
      .then((accounts) => {
        setAccounts(accounts);
      })
      .catch((e) => {
        toast.error('Error retrieving accounts');
      });
  };

  const getWalletAccounts = async () => {
    if (walletSelector !== null && walletSelector.isSignedIn()) {
      const wallet = await walletSelector.wallet();

      const accounts = await wallet.getAccounts();

      return accounts.map((acc) => acc.accountId);
    } else {
      return [];
    }
  };

  const connectWallet = async () => {
    try {
      if (walletSelector === null) {
        toast.error("Modal isn't initialized");
        return;
      }

      const modal = setupModal(walletSelector, {
        contractId: BACK_END_CONTRACT,
      });

      modal.show();
    } catch (error) {
      toast.error('Error connecting to wallet');
    }
  };

  const copyToClipBoard = async () => {
    await navigator.clipboard.writeText(accounts[0]);

    await message.info('Address copied to clipboard');
  };

  return (
    <div className="flex flex-col items-end gap-2">
      {accounts.length > 0 ? (
        <Dropdown.Button
          menu={menuProps}
          placement="bottomRight"
          icon={<WalletOutlined />}
          className="w-max"
          onClick={copyToClipBoard}
        >
          {`Wallet account: ${accounts[0]}`}
        </Dropdown.Button>
      ) : (
        <Button type="primary" onClick={connectWallet} loading={isLoading}>
          {isLoading ? "Checking wallet" : "Connect wallet" } 
        </Button>
      )}

      <Deposit />
    </div>
  );
};

export default ConnectedWallet;
