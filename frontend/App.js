import 'regenerator-runtime/runtime';
import React, { useState, useEffect } from 'react';
import Form from './components/Form';
import SignIn from './components/SignIn';
import Messages from './components/Messages';

const App = ({ isSignedIn, guestBook, wallet }) => {
  const [messages, setMessages] = useState([]);
  const [deposit, setDeposit] = useState(0);
  const [usdtDeposit, setUsdtDeposit] = useState(0);
  const [platformUsdtDeposit, setPlatformUsdtDeposit] = useState(0);

  useEffect(() => {
    guestBook.getMessages().then(setMessages);
    guestBook.getDeposit().then(setDeposit);
    guestBook.getWalletUsdDeposit().then(setUsdtDeposit);
    guestBook.getPlatformUsdDeposit().then(setPlatformUsdtDeposit);
  }, []);

  const depositFunds = () => {
    guestBook.depositFunds();
  }

  onSubmit = async (e) => {
    e.preventDefault();

    const { fieldset, message, donation } = e.target.elements;

    fieldset.disabled = true;

    await guestBook.addMessage(message.value, donation.value)
    const messages = await guestBook.getMessages()

    setMessages(messages);
    message.value = '';
    donation.value = '0';
    fieldset.disabled = false;
    message.focus();
  };

  const signIn = () => { wallet.signIn() }

  const signOut = () => { wallet.signOut() }

  return (
    <main>
      <table>
        <tr>
          <td><h1>📖 NEAR Guest Book</h1></td>
          <td></td>
          <td>{ isSignedIn
          ? <button onClick={signOut}>Log out</button>
          : <button onClick={signIn}>Log in</button>
        }</td>
        </tr>
        <tr>
          <td>My deposit {deposit}</td>
          <td>My usdt deposit {usdtDeposit}</td>
          <td>Platform deposit {platformUsdtDeposit}</td>
        </tr>
      </table>

      <hr />
      { isSignedIn
        ? <Form onSubmit={onSubmit} currentAccountId={wallet.accountId} />
        : <SignIn/>
      }

      <button onClick={depositFunds}>Invest</button>
      <hr />

      { !!messages.length && <Messages messages={messages}/> }

    </main>
  );
};

export default App;