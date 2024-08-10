import { WalletSelector } from '@near-wallet-selector/core';
import { query } from './common';

export const BACK_END_CONTRACT = 'shiny-crook.testnet';
const USDT_TESTNET_CONTRACT = 'usdt.fakes.testnet';

export const getUsdtContract = () => {
  return USDT_TESTNET_CONTRACT;
};

export const getDeposit = async (
  walletSelector: WalletSelector,
  senderEmail: string,
): Promise<number> => {
  const stringArguments = JSON.stringify({ sender_email: senderEmail });

  const queryResult = await query(
    walletSelector,
    'get_my_deposit',
    stringArguments,
  );

  const usdtAmount = JSON.parse(queryResult);

  return usdtAmount / Math.pow(10, 6);
};

export const getWalletBalance = async (
  walletSelector: WalletSelector,
): Promise<number> => {
  const wallet = await walletSelector.wallet();
  const accounts = await wallet.getAccounts();
  const stringArguments = JSON.stringify({ account_id: accounts[0].accountId });

  const queryResult = await query(
    walletSelector,
    'ft_balance_of',
    stringArguments,
    getUsdtContract(),
  );

  const usdtAmount = JSON.parse(queryResult);

  return usdtAmount / Math.pow(10, 6);
};

export const getWithdrawableAmount = async (
  walletSelector: WalletSelector,
  senderEmail: string,
): Promise<number> => {
  const wallet = await walletSelector.wallet();
  const accounts = await wallet.getAccounts();
  const stringArguments = JSON.stringify({
    sender_email: senderEmail,
    account_id: accounts[0].accountId,
  });

  const queryResult = await query(
    walletSelector,
    'get_withdrawable_amount',
    stringArguments,
  );

  const usdtAmount = JSON.parse(queryResult);

  return usdtAmount / Math.pow(10, 6);
};

export const getIsEmailRegistered = async (
  senderEmail: string,
  walletSelector: WalletSelector,
): Promise<boolean> => {
  const stringArguments = JSON.stringify({ sender_email: senderEmail });

  const queryResult = await query(
    walletSelector,
    'is_registered',
    stringArguments,
  );

  const isEmailRegistered = JSON.parse(queryResult);

  return isEmailRegistered;
};
