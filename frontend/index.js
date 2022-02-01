import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import getConfig from './config.js';
import * as nearAPI from 'near-api-js';

// Initializing contract
async function initContract() {
  // const nearConfig = getConfig(process.env.NODE_ENV || 'testnet');
  const nearConfig = {
    networkId: 'testnet',
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org',
    nftContract: "nft.testkrishu.testnet",
    ftContract:"ft1.testkrishu.testnet",
    shopContract:"shop.testkrishu.testnet"
  }
  // Initializing connection to the NEAR TestNet
  const near = await nearAPI.connect({
    keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore(),
    ...nearConfig
  });

  // Needed to access wallet
  const walletConnection = new nearAPI.WalletConnection(near);

  // Load in account data
  let currentUser;
  if(walletConnection.getAccountId()) {
    currentUser = {
      accountId: walletConnection.getAccountId(),
      balance: (await walletConnection.account().state()).amount
    };
  }

  // Initializing our contract APIs by contract name and configuration
  const nftContract = await new nearAPI.Contract(walletConnection.account(), nearConfig.nftContract, {
    // View methods are read-only – they don't modify the state, but usually return some value
    viewMethods: [],
    // Change methods can modify the state, but you don't receive the returned value when called
    changeMethods: ['nft_mint','nft_approve'],
    sender: walletConnection.getAccountId()
  });
  const ftContract = await new nearAPI.Contract(walletConnection.account(), nearConfig.ftContract, {
    // View methods are read-only – they don't modify the state, but usually return some value
    viewMethods: ['ft_balance_of'],
    // Change methods can modify the state, but you don't receive the returned value when called
    changeMethods: ['ft_transfer_call'],
    sender: walletConnection.getAccountId()
  });

  const shopContract = await new nearAPI.Contract(walletConnection.account(), nearConfig.shopContract, {
    // View methods are read-only – they don't modify the state, but usually return some value
    viewMethods: ['get_offers_by_nft_contract_id'],
    // Change methods can modify the state, but you don't receive the returned value when called
    changeMethods: ['liquidate', 'withdraw_guarantee'],
    sender: walletConnection.getAccountId()
  });

  return { nftContract, ftContract, shopContract, currentUser, nearConfig, walletConnection };
}

window.nearInitPromise = initContract()
  .then(({ nftContract, ftContract, shopContract, currentUser, nearConfig, walletConnection }) => {
    ReactDOM.render(
      <App
        nftContract={nftContract}
        ftContract={ftContract}
        shopContract={shopContract}
        currentUser={currentUser}
        nearConfig={nearConfig}
        wallet={walletConnection}
      />,
      document.getElementById('root')
    );
  });
