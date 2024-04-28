import { NearBindgen, near, call, view, Vector, initialize, LookupMap } from 'near-sdk-js'
import { Deposit, POINT_ONE, PostedMessage } from './model'

@NearBindgen({ requireInit: true })
class GuestBook {
  messages: Vector<PostedMessage>;
  deposits: LookupMap<bigint>;

  @initialize({})
  cons(){
    this.messages = new Vector<PostedMessage>("guestmessages");
    this.deposits = new LookupMap<bigint>("deposits");
  }
  
  @call({ payableFunction: true })
  // Public - Adds a new message.
  add_message({ text }: { text: string }) {
    // If the user attaches more than 0.1N the message is premium
    const depositAmount = near.attachedDeposit();
    const premium = depositAmount >= BigInt(POINT_ONE);
    const sender = near.predecessorAccountId();

    if (depositAmount > 0) {
      this.depositFunds(depositAmount, sender);
    }

    const message: PostedMessage = { premium, sender, text };
    this.messages.push(message);
  }

  @view({})
  // Returns an array of messages.
  get_messages({ from_index = 0, limit = 10 }: { from_index: number, limit: number }): PostedMessage[] {
    return this.messages.toArray().slice(from_index, from_index + limit);
  }

  @view({})
  total_messages(): number { return this.messages.length }

  @view({})
  my_deposit(): bigint {
    const callerAccount = near.predecessorAccountId();

    if (this.deposits.containsKey(callerAccount)) {
      return this.deposits.get(callerAccount);
    }

    return BigInt(0);
  }

  depositFunds(amount: bigint, account: string) {
    if (this.deposits.containsKey(account)) {
      let currentDeposit = this.deposits.get(account);
      currentDeposit += amount;

      this.deposits.set(account, currentDeposit);
    }
    else {
      this.deposits.set(account, amount);
    }
  }
}
