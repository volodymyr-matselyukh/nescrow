// React
import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';

// NEAR
import { GuestBook } from './near-interface';
import { Wallet } from './near-wallet';
 
export const APP_CONTRACT_NAME = "malicious-basketball.testnet"

//volodymyr_matseliukh1.testnet

//tired-cobweb.testnet

// When creating the wallet you can choose to create an access key, so the user
// can skip signing non-payable methods when talking wth the contract
const wallet = new Wallet({ createAccessKeyFor: APP_CONTRACT_NAME})

// Abstract the logic of interacting with the contract to simplify your flow
const guestBook = new GuestBook({ walletToUse: wallet });

// Setup on page load
window.onload = async () => {
  const isSignedIn = await wallet.startUp()
 
  ReactDOM.render(
    <App isSignedIn={isSignedIn} guestBook={guestBook} wallet={wallet} />,
    document.getElementById('root')
  );
}