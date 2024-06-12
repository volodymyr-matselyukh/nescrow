// Make a read-only call to retrieve information from the network
const viewMethod = async (contractId: string, method: string, ...args: any[]) {
  const { network } = this.walletSelector.options;
  const provider = new providers.JsonRpcProvider({ url: network.nodeUrl });

  let res = await provider.query({
    request_type: 'call_function',
    account_id: contractId,
    method_name: method,
    args_base64: Buffer.from(JSON.stringify(args)).toString('base64'),
    finality: 'optimistic',
  });
  return JSON.parse(Buffer.from(res.result).toString());
}

// Call a method that changes the contract's state
async callMethod({ contractId, method, args = {}, gas = THIRTY_TGAS, deposit = NO_DEPOSIT }) {
  // Sign a transaction with the "FunctionCall" action
  const outcome = await this.wallet.signAndSendTransaction({
    signerId: this.accountId,
    receiverId: contractId,
    actions: [
      {
        type: 'FunctionCall',
        params: {
          methodName: method,
          args,
          gas,
          deposit,
        },
      },
    ],
  });

  return providers.getTransactionLastResult(outcome)
}