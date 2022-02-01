import "regenerator-runtime/runtime";
import React, { useState, useEffect } from "react";
import PropTypes from "prop-types";
import Big from "big.js";
import Form from "./components/Form";
const nft_contract_id = "nft.testkrishu.testnet";
const BOATLOAD_OF_GAS = Big(3).times(10 ** 14).toFixed();

const App = ({ nftContract, ftContract, shopContract, currentUser, nearConfig, wallet }) => {
  const [offers, setOffers] = useState([]);
  const [balance, setBalance] = useState('0');
  
  useEffect(async () => {
    if (currentUser) {
      // get_offers_by_nft_contract_id '{"nft_contract_id":"$(nftaccountid)", "from_index":"0", "limit": 5}
      const offers = await shopContract.get_offers_by_nft_contract_id({
        nft_contract_id: nft_contract_id, from_index: "0", limit: 5
      });
      // console.log("offers",offers);
      setOffers(offers);
      const balance = await ftContract.ft_balance_of({
        account_id: currentUser.accountId
      });
      setBalance(balance);
    }
    
  });
  const viewSubmit = async event => {
    event.preventDefault();
    if (currentUser) {
      // get_offers_by_nft_contract_id '{"nft_contract_id":"$(nftaccountid)", "from_index":"0", "limit": 5}
      const offers = await shopContract.get_offers_by_nft_contract_id({
        nft_contract_id: nft_contract_id, from_index: "0", limit: 5
      });
      console.log("offers",offers);
      setOffers(offers);
    }
  }
  const mintSubmit = async event => {
    event.preventDefault();
    const { fieldset, message } = event.target.elements;
    fieldset.disabled = true;
    const jsonMessage = JSON.parse(message.value);
    console.log(jsonMessage );
    res1 = await nftContract.nft_mint(
      jsonMessage,
      BOATLOAD_OF_GAS,
      "1000000000000000000000000"
    );
    console.log(res1);
    fieldset.disabled = false;
  }
  const approveSubmit = async event => {
    event.preventDefault();
    const { fieldset, message } = event.target.elements;
    fieldset.disabled = true;
    const jsonMessage = JSON.parse(message.value);
    console.log(jsonMessage );
    res1 = await nftContract.nft_approve(
      jsonMessage,
      BOATLOAD_OF_GAS,
      "1000000000000000000000000"
    );
    console.log(res1);
    fieldset.disabled = true;
  }
  const borrowSubmit = async event => {
    event.preventDefault();
    const { fieldset, message } = event.target.elements;
    fieldset.disabled = true;
    const jsonMessage = JSON.parse(message.value);
    console.log(jsonMessage );
    res1 = await ftContract.ft_transfer_call(
      jsonMessage,
      BOATLOAD_OF_GAS,
      1
    );
    console.log(res1);
    fieldset.disabled = true;
  }
  const liqSubmit = async event => {
    event.preventDefault();
    const { fieldset, message } = event.target.elements;
    fieldset.disabled = true;
    const jsonMessage = JSON.parse(message.value);
    console.log(jsonMessage );
    res1 = await shopContract.liquidate(
      jsonMessage,
      BOATLOAD_OF_GAS,
      1
    );
    console.log(res1);
    fieldset.disabled = true;
  }
  const withSubmit = async event => {
    event.preventDefault();
    const { fieldset, message } = event.target.elements;
    fieldset.disabled = true;
    const jsonMessage = JSON.parse(message.value);
    console.log(jsonMessage );
    res1 = await shopContract.withdraw_guarantee(
      jsonMessage,
      BOATLOAD_OF_GAS,
      1
    );
    console.log(res1);
    fieldset.disabled = true;
  }
    

  const signIn = () => {
    wallet.requestSignIn(
      nearConfig.contractName,
      "NEAR Status Message"
    );
  };

  const signOut = () => {
    wallet.signOut();
    window.location.replace(window.location.origin + window.location.pathname);
  };

  return (
    <main>
      <header>
        <h1>Bicycle Shop Experience</h1>

        {currentUser ?
          <p>Currently signed in as: <code>{currentUser.accountId} with bift balance as {balance}</code></p>
        :
          <p>Please login to continue.</p>
        }

        { currentUser
          ? <button onClick={signOut}>Log out</button>
          : <button onClick={signIn}>Log in</button>
        }
      </header>

      <section>
      <h1>ViewOffers</h1>
      {currentUser && 
      <button onClick={viewSubmit}>Refresh</button>
      }
      {
        offers.map((data, key) => {
          return (
            <section key={key}>
              <p>owner_id:{data.owner_id}</p>
              <p>nft_contract_id:{data.nft_contract_id}</p>
              <p>status:{data.status}</p>
              <p>rent:{data.rent}</p>
              <p>max_duration:{data.max_duration}</p>
              <p>lending_start:{data.lending_start}</p>
              <p>lending_expiry:{data.lending_expiry}</p>
              <p>user_rights:{data.user_rights}</p>
              <p>********************************</p>
            </section>
          );
        })}
      </section> 
        
      <section>
        <h1>MintAndOffer</h1>
        {currentUser &&
        <Form
          onSubmit={mintSubmit}
          currentUser={currentUser}
          placeholder={`{"token_id": "hero2017","receiver_id":"alice1.testnet", "metadata": {"title": "Hero-2016 model"} }`}
        />
        }
       {currentUser &&
       <Form
       onSubmit={approveSubmit}
       currentUser={currentUser}
       placeholder={`'{"token_id": "hero2017","account_id":"shop.testkrishu.testnet", "msg": "{\"rent\": 1, \"guarantee\": 5000, \"max_duration\": 2500}" }`}
      />

       } 
      </section>
      
      <section>
      <h1>BorrowsToken</h1>
      {currentUser &&
        <Form
          onSubmit={borrowSubmit}
          currentUser={currentUser}
          placeholder={`{"receiver_id":"shop.testkrishu.testnet","amount":"1000","msg": "{\"nft_contract\":\"nft.testkrishu.testnet\",\"token_id\":\"hero2017\",\"expiry\":100}","memo":"rent a bicycle"}`}
        />}
      </section>
      <section>
      <h1>Liquidation</h1>
      {currentUser &&
        <Form
          onSubmit={liqSubmit}
          currentUser={currentUser}
          placeholder={`{"nft_contract_id":"nft.testkrishu.testnet", "token_id":"hero2017"}`}
        />}
      </section>
      <section>
      <h1>WithDrawGuarantee</h1>
      {currentUser &&
        <Form
          onSubmit={withSubmit}
          currentUser={currentUser}
          placeholder={`{"nft_contract_id":"nft.testkrishu.testnet", "token_id":"hero2017"}`}
        />}
      </section>

      
    </main>
  );
};

App.propTypes = {
  // nftCcontract: PropTypes.shape({
  //   set_status: PropTypes.func.isRequired,
  //   get_status: PropTypes.func.isRequired
  // }).isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired,
    balance: PropTypes.string.isRequired
  }),
  // nearConfig: PropTypes.shape({
  //   contractName: PropTypes.string.isRequired
  // }).isRequired,
  wallet: PropTypes.shape({
    requestSignIn: PropTypes.func.isRequired,
    signOut: PropTypes.func.isRequired
  }).isRequired
};

export default App;
