# Bicycle Borrowing Shop

- Alice and Bob are two users 
- One Fungible token bift is used as medium of exchange for the shop
- Both Alice and Bob already have 10000 tokens for both 
- Alice wants to lend her bicycle, so she mints an NFT with info(type: bicycle, model: 2016, owner:alice)
- Alice put this NFT into shop as an offer with rent per second: 1, guarantee: 5000, max_expiry(in seconds)
- Bob gets the list of NFTs in the shop and put buys it by sending the guarantee money
- Bicycle shop add the borrowerID, change the status  and expiry to the NFT, and save the guarantee into escrew
- NFT will have two status: OFFER/ONRENT
- The owner can reclaim the escrow, after expiry period of NFT.(liquidation)  
- The borrower can withdraw_guarantee. Bob returns the bicycle  and is charged for the amount equal to days*rent. Pending amount of guarantee is returned to him.
  

# Contracts Involved

1. Create an FT token bift
2. Create an NFT contract binft
3. Create a shop contract bi
4. Setup frontend to mint NFT and exchange NFT

## Use Makefile to setup contracts and test the execution

### test scenario: `make test`
	## Creating and setting up ft accounts 
	## Minting token on nft
	## Putting offer on shop
	## Modifying offer 
	## borrowing on shop 
	## Withdraw Guarantee and return on shop 
	## Liquidation on shop 


## Online test
- Use `npm build:web && npm start` to setup basic frontend and try out the same
- Change the Makefile to add appropriate accounts and call `make setupOnline`
### Actions
- MintAndOffer
  -  Alice mints an NFT token after providing the details and put it on offer
- ViewOffers
  - Bob view the available offers
- BorrowsToken
  - Bob applies for one of the offer providing NFT tokenID
- Liquidation
  - Alice tries to liquidate it providing NFT tokenID
- WithDrawGuarantee
  - Bob withdraws_guarantee by providing NFT tokenID

### Corresponding JSON
Minting: {"token_id": "hero2017","receiver_id":"alice1.testnet", "metadata": {"title": "Hero-2016 model"} }
Approval: {"token_id": "hero2017","account_id":"shop.testkrishu.testnet", "msg": "{\"rent\": 1, \"guarantee\": 5000, \"max_duration\": 2500}" }
Borrow: {"receiver_id":"shop.testkrishu.testnet","amount":"5000","msg": "{\"nft_contract\":\"nft.testkrishu.testnet\",\"token_id\":\"hero2017\",\"expiry\":100}","memo":"rent a bicycle"}
Liquidation: {"nft_contract_id":"nft.testkrishu.testnet", "token_id":"hero2017"}
Withdraw: {"nft_contract_id":"nft.testkrishu.testnet", "token_id":"hero2017"}
