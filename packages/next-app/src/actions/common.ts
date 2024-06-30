import { WalletSelector } from '@near-wallet-selector/core';
import { providers } from 'near-api-js';
import { CodeResult } from 'near-api-js/lib/providers/provider';
import { BACK_END_CONTRACT } from './nearActions';

export const getBase64 = (argumentsString: string) => {
  return btoa(argumentsString);
};

export const query = async (
  walletSelector: WalletSelector,
  methodName: string,
  argumentsString: string,
  accountId: string = BACK_END_CONTRACT,
) => {
  const { network } = walletSelector.options;
  const provider = new providers.JsonRpcProvider({ url: network.nodeUrl });

  const result = await provider.query<CodeResult>({
    request_type: 'call_function',
    account_id: accountId,
    method_name: methodName,
    finality: 'final',
    args_base64: getBase64(argumentsString),
  });

  return Buffer.from(result.result).toString();
};
