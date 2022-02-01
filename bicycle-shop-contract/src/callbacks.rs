use crate::*;
use near_sdk::{log,PromiseOrValue};
/// approval callbacks from NFT Contracts

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OfferArgs {
    pub rent: u64,
    pub guarantee: u64,
    pub max_duration: u64

}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BorrowArgs {
    pub nft_contract: String,
    pub token_id: String, 
    pub expiry: u64, // in seconds
}

trait FungibleTokenReceiver {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        let ft_contract_id = env::predecessor_account_id();
        // let signer_id = env::signer_account_id();
        log!{"ft:{}, msg:{}",ft_contract_id,msg}
        let BorrowArgs { nft_contract, token_id, expiry } = near_sdk::serde_json::from_str(&msg).expect("Not valid OfferArgs");
        
        // assert_ne!(
        //     nft_contract,
        //     signer_id,
        //     "nft_on_approve should only be called via cross-contract call"
        // );
        
        let contract_and_token_id = format!("{}{}{}", nft_contract, DELIMETER, token_id);
        log!("offerID is {}, expiry:{}",contract_and_token_id, expiry);
        let mut offer = self.offers.get(&contract_and_token_id).expect("offer not available");
        assert!(offer.status == "OFFER","Offer state not valid");
        assert!(expiry < offer.max_duration, "expiry not valid");
        assert!(u128::from(offer.guarantee) == amount.0,"money transferred not valid");
        offer.status = "ONRENT".into();
        offer.lending_start = Some(env::block_timestamp());
        offer.lending_expiry = Some(env::block_timestamp() + expiry*1000_000_000);
        offer.user_rights = Some(sender_id);
        self.offers.insert(&contract_and_token_id, &offer);

        PromiseOrValue::Value(U128(0))

    }
}
/*
    trait that will be used as the callback from the NFT contract. When nft_approve is
    called, it will fire a cross contract call to this marketplace and this is the function
    that is invoked. 
*/
trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

//implementation of the trait
#[near_bindgen]
impl NonFungibleTokenApprovalsReceiver for Contract {
    
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) {
        // get the contract ID which is the predecessor
        let nft_contract_id = env::predecessor_account_id();
        //get the signer which is the person who initiated the transaction
        let signer_id = env::signer_account_id();

        //make sure that the signer isn't the predecessor. This is so that we're sure
        //this was called via a cross-contract call
        assert_ne!(
            nft_contract_id,
            signer_id,
            "nft_on_approve should only be called via cross-contract call"
        );
        //make sure the owner ID is the signer. 
        assert_eq!(
            owner_id,
            signer_id,
            "owner_id should be signer_id"
        );

        let storage_amount = self.storage_minimum_balance().0;
        //get the total storage paid by the owner
        let owner_paid_storage = self.storage_deposits.get(&signer_id).unwrap_or(0);
        let signer_storage_required = (self.get_supply_by_owner_id(signer_id).0 + 1) as u128 * storage_amount;
        
        //make sure that the total paid is >= the required storage
        assert!(
            owner_paid_storage >= signer_storage_required,
            "Insufficient storage paid: {}, for {} offers at {} rate of per offer",
            owner_paid_storage, signer_storage_required / STORAGE_PER_OFFER, STORAGE_PER_OFFER
        );

        let OfferArgs { rent, guarantee, max_duration } = near_sdk::serde_json::from_str(&msg).expect("Not valid OfferArgs");
        log!("Got OfferArgs {}, {}, {}",rent, guarantee, max_duration);
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        if self.offers.get(&contract_and_token_id).is_some() {
            std::panic!("Offer already available");
        }
        
        self.offers.insert(
            &contract_and_token_id,
            &Offer {
                owner_id: owner_id.clone(), 
                approval_id, //approval ID for that token that was given to the market
                nft_contract_id: nft_contract_id.to_string(), //NFT contract the token was minted on
                token_id: token_id.clone(), //the actual token ID
                status:"OFFER".into(),
                rent, 
                guarantee,
                max_duration,

                lending_start: None,
                lending_expiry: None,
                user_rights: None
           },
        );

        //Extra functionality that populates collections necessary for the view calls 

        let mut by_owner_id = self.by_owner_id.get(&owner_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByOwnerIdInner {
                    //we get a new unique prefix for the collection by hashing the owner
                    account_id_hash: hash_account_id(&owner_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        
        by_owner_id.insert(&contract_and_token_id);
        //insert that set back into the collection for the owner
        self.by_owner_id.insert(&owner_id, &by_owner_id);

        //get the token IDs for the given nft contract ID. If there are none, we create a new empty set
        let mut by_nft_contract_id = self
            .by_nft_contract_id
            .get(&nft_contract_id)
            .unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::ByNFTContractIdInner {
                        //we get a new unique prefix for the collection by hashing the owner
                        account_id_hash: hash_account_id(&nft_contract_id),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });
        
        //insert the token ID into the set
        by_nft_contract_id.insert(&token_id);
        //insert the set back into the collection for the given nft contract ID
        self.by_nft_contract_id
            .insert(&nft_contract_id, &by_nft_contract_id);
    }
}
