accountid=testkrishu.testnet
nftaccountid=nft.testkrishu.testnet
ftaccountid=ft1.testkrishu.testnet
shopaccountid=shop.testkrishu.testnet
alice=alice1.testkrishu.testnet
bob=bob1.testkrishu.testnet
CARGO_NET_GIT_FETCH_WITH_CLI=true

aliceOnline = alice1.testnet
bobOnline = bob111.testnet

.PHONY: all
all:
	echo "this is working" $(option_one)

.PHONY: test
test:
	echo "testing ft" 
	## Creating and setting up ft accounts 
	# near call $(ftaccountid) storage_deposit '{"account_id": "$(alice)"}' --accountId $(accountid) --amount 0.00125
	# near call $(ftaccountid) storage_deposit '{"account_id": "$(bob)"}' --accountId $(accountid) --amount 0.00125
	# near call $(ftaccountid) storage_deposit '{"account_id": "$(shopaccountid)"}' --accountId $(accountid) --amount 0.00125
	# near call $(ftaccountid) ft_transfer '{"receiver_id": "$(alice)", "amount": "20000", "memo": "initial money deposited" }' --accountId $(accountid) --depositYocto 1
	# near call $(ftaccountid) ft_transfer '{"receiver_id": "$(bob)", "amount": "15000", "memo": "initial money deposited" }' --accountId $(accountid) --depositYocto 1
	near view $(ftaccountid) ft_balance_of '{"account_id": "$(alice)"}'
	# near view $(ftaccountid) ft_balance_of '{"account_id": "$(bob)"}'
	# near view $(ftaccountid) ft_balance_of '{"account_id": "$(accountid)"}'
	
	## Minting token on nft
	# near call $(nftaccountid) nft_mint '{"token_id": "hero2016","receiver_id":"$(alice)", "metadata": {"title": "Hero-2016 model"} }' --accountId $(alice) --deposit 1
	# near view $(nftaccountid) nft_tokens '{"from_index": "0", "limit": 5}'

	## Putting offer on shop
	# near call $(shopaccountid) storage_deposit '{"account_id": "$(alice)"}' --accountId $(accountid) --amount 0.01
	# near view $(shopaccountid) get_offers_by_nft_contract_id '{"nft_contract_id":"$(nftaccountid)", "from_index":"0", "limit": 5}'
	# near call $(nftaccountid) nft_approve '{"token_id": "hero2016","account_id":"$(shopaccountid)", "msg": "{\"rent\": 1, \"guarantee\": 5000, \"max_duration\": 2500}" }' --accountId $(alice) --deposit 1
	# near view $(shopaccountid) get_offers_by_nft_contract_id '{"nft_contract_id":"$(nftaccountid)", "from_index":"0", "limit": 5}'

	## Modifying offer 
	# near call $(shopaccountid) update_offer '{"nft_contract_id":"$(nftaccountid)", "token_id":"hero2016","rent": 2, "guarantee": 1000, "max_duration": 200}' --accountId $(alice) --depositYocto 1 --gas 300000000000000
	# near view $(shopaccountid) get_offers_by_nft_contract_id '{"nft_contract_id":"$(nftaccountid)", "from_index":"0", "limit": 5}'
	# near call $(shopaccountid) remove_offer '{"nft_contract_id":"$(nftaccountid)", "token_id":"hero2016"}' --accountId $(alice) --depositYocto 1 --gas 300000000000000
	# near view $(shopaccountid) get_offers_by_nft_contract_id '{"nft_contract_id":"$(nftaccountid)", "from_index":"0", "limit": 5}'
	
	## borrowing on shop 
	# near call $(ftaccountid) ft_transfer_call '{"receiver_id":"$(shopaccountid)","amount":"1000","msg": "{\"nft_contract\":\"$(nftaccountid)\",\"token_id\":\"hero2016\",\"expiry\":100}","memo":"rent a bicycle"}' --accountId $(bob) --depositYocto 1 --gas 300000000000000
	# near view $(shopaccountid) get_offers_by_nft_contract_id '{"nft_contract_id":"$(nftaccountid)", "from_index":"0", "limit": 5}'
	# near view $(ftaccountid) ft_balance_of '{"account_id": "$(bob)"}'
	# near view $(ftaccountid) ft_balance_of '{"account_id": "$(shopaccountid)"}'

	## Withdraw Guarantee and return on shop 
	# near call $(shopaccountid) withdraw_guarantee '{"nft_contract_id":"$(nftaccountid)", "token_id":"hero2016"}' --accountId $(bob) --depositYocto 1 --gas 300000000000000
	# near view $(shopaccountid) get_offers_by_nft_contract_id '{"nft_contract_id":"$(nftaccountid)", "from_index":"0", "limit": 5}'
	# near view $(ftaccountid) ft_balance_of '{"account_id": "$(bob)"}'
	# near view $(ftaccountid) ft_balance_of '{"account_id": "$(shopaccountid)"}'
	# near view $(ftaccountid) ft_balance_of '{"account_id": "$(alice)"}'

	## Liquidation on shop 
	# near call $(shopaccountid) liquidate '{"nft_contract_id":"$(nftaccountid)", "token_id":"hero2016"}' --accountId $(alice) --depositYocto 1 --gas 300000000000000
	# near view $(shopaccountid) get_offers_by_nft_contract_id '{"nft_contract_id":"$(nftaccountid)", "from_index":"0", "limit": 5}'
	# near view $(ftaccountid) ft_balance_of '{"account_id": "$(bob)"}'
	# near view $(ftaccountid) ft_balance_of '{"account_id": "$(shopaccountid)"}'
	# near view $(ftaccountid) ft_balance_of '{"account_id": "$(alice)"}'

.PHONY: setupOnline
setupOnline:
	echo "setting up online accounts"
	# near call $(ftaccountid) storage_deposit '{"account_id": "$(aliceOnline)"}' --accountId $(accountid) --amount 0.00125
	# near call $(ftaccountid) storage_deposit '{"account_id": "$(bobOnline)"}' --accountId $(accountid) --amount 0.00125
	# near call $(shopaccountid) storage_deposit '{"account_id": "$(aliceOnline)"}' --accountId $(accountid) --amount 0.02
	near call $(ftaccountid) ft_transfer '{"receiver_id": "$(aliceOnline)", "amount": "20000", "memo": "initial money deposited" }' --accountId $(accountid) --depositYocto 1
	near call $(ftaccountid) ft_transfer '{"receiver_id": "$(bobOnline)", "amount": "50000", "memo": "initial money deposited" }' --accountId $(accountid) --depositYocto 1

.PHONY: deploy
deploy:  ft nft shop 
	echo "creating users"
	near create-account $(alice) --masterAccount $(accountid) --initialBalance 10
	near create-account $(bob) --masterAccount $(accountid) --initialBalance 10

.PHONY: deleteAll
deleteAll: deleteft deletenft deleteshop 
	echo "deleting ft, nft, shop, users"	
	echo "deleting users"
	near delete $(alice) $(accountid) 
	near delete $(bob) $(accountid) 

.PHONY: shop
shop: 
	echo "building shop" 
	cd bicycle-shop-contract && bash build.sh
	# near create-account $(shopaccountid) --masterAccount $(accountid) --initialBalance 10
	near deploy --accountId $(shopaccountid) --wasmFile out/shop.wasm
	# near call $(shopaccountid) new '{"owner_id": "$(accountid)", "ft_contract_id": "$(ftaccountid)"}' --accountId $(accountid) 

.PHONY: deleteshop
deleteshop:
	echo "deleting shop"
	near delete $(shopaccountid) $(accountid) 

.PHONY: nft
nft:
	echo "building nft" 
	cd nft-contract && bash build.sh
	near create-account $(nftaccountid) --masterAccount $(accountid) --initialBalance 10
	near deploy --accountId $(nftaccountid) --wasmFile out/nft.wasm
	near call $(nftaccountid) new '{"owner_id": "$(accountid)", "metadata": {"spec": "nft-1.0.0", "name": "BiCycle NFT","symbol": "Bicycle"} }' --accountId $(accountid) 
	

.PHONY: deletenft
deletenft:
	echo "deleting nft"
	near delete $(nftaccountid) $(accountid) 


.PHONY: ft
ft:
	echo "building ft" 
	cd ft && bash build.sh
	near create-account $(ftaccountid) --masterAccount $(accountid) --initialBalance 10
	near deploy --accountId $(ftaccountid) --wasmFile out/ft.wasm
	near call $(ftaccountid) new '{"owner_id": "$(accountid)", "total_supply": "1000000000","version":"ft-1.0.0",  "name": "BIFT","symbol": "Bicycle","decimals": 24 }' --accountId $(accountid) 
	

.PHONY: deleteft
deleteft:
	echo "deleting ft"
	near delete $(ftaccountid) $(accountid) 




