/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */

import { utils } from 'near-api-js';

const usdtAccountId = 'usdt.fakes.testnet';

const THIRTY_TGAS = '300000000000000';

//const usdtWallet = new Wallet({ createAccessKeyFor: usdtAccountId });

export class GuestBook {

  constructor({ contractId, walletToUse }) {
    this.contractId = contractId;
    this.wallet = walletToUse
  }

  async getWalletUsdDeposit() {
    console.log('contract id', this.contractId);
    console.log('wallet contract id', this.wallet);

    const deposit = await this.wallet.viewMethod({ contractId: usdtAccountId, method: "ft_balance_of", args: { account_id: this.wallet.accountId } });

    console.log('usdt deposit', deposit);

    return (deposit / 1000_000).toFixed(2);
  }

  async getPlatformUsdDeposit() {
    const deposit = await this.wallet.viewMethod({ contractId: usdtAccountId, method: "ft_balance_of", args: { account_id: this.contractId } });

    return (deposit / 1000_000).toFixed(2);
  }

  async getDeposit() {
    console.log('contract id', this.contractId);
    console.log('wallet contract id', this.wallet);

    const deposit = await this.wallet.viewMethod({ contractId: this.contractId, method: "get_my_deposit", args: { sender: this.wallet.accountId }});

    const nears = deposit / Math.pow(10, 24);

    return nears.toFixed(5);
  }

  async getMessages() {
    const messages = await this.wallet.viewMethod({ contractId: this.contractId, method: "get_messages" })
    console.log(messages)
    return messages
  }

  async depositFunds() {
    const nearDeposit = utils.format.formatNearAmount(1);

    console.log('near deposit', nearDeposit);
    const deposit = await this.wallet.callMethod({ contractId: usdtAccountId, method: "ft_transfer_call", gas: THIRTY_TGAS, args: { receiver_id: this.contractId, amount: "1000000", msg: "invest" }, deposit: "1"});
  }

  async addMessage(message, donation) {
    const deposit = utils.format.parseNearAmount(donation);
    return await this.wallet.callMethod({ contractId: this.contractId, method: "add_message", args: { text: message }, deposit });
  }
}