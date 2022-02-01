use crate::*;
const GAS_FOR_FT_TRANSFER: Gas = Gas(50_000_000_000_000);
use near_sdk::{log};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Offer {
    //owner of the offer
    pub owner_id: AccountId,
    //market contract's approval ID to transfer the token on behalf of the owner
    pub approval_id: u64,
    //nft contract where the token was minted
    pub nft_contract_id: String,
    //actual token ID for offer
    pub token_id: String,
    pub status:String,
    pub rent: u64, 
    pub guarantee: u64,
    pub max_duration: u64, // in seconds
    
    pub lending_start: Option<u64>,
    pub  lending_expiry: Option<u64>,
    pub user_rights: Option<AccountId>,

}

#[near_bindgen]
impl Contract {
    
    #[payable]
    pub fn remove_offer(&mut self, nft_contract_id: AccountId, token_id: String) {
        //assert that the user has attached exactly 1 yoctoNEAR (for security reasons)
        assert_one_yocto();
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        let offer = self.offers.get(&contract_and_token_id).expect("No offer");
        assert!(offer.status == "OFFER","Offer state not valid");
        let offer = self.internal_remove_offer(nft_contract_id.into(), token_id);
        let owner_id = env::predecessor_account_id();
        assert_eq!(owner_id, offer.owner_id, "Must be offer owner");
    }


    #[payable]
    pub fn update_offer(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        rent: u64, 
        guarantee: u64,
        max_duration: u64,
    ) {
        //assert that the user has attached exactly 1 yoctoNEAR (for security reasons)
        assert_one_yocto();
        
        // let contract_id: AccountId = nft_contract_id.into();
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        
        let mut offer = self.offers.get(&contract_and_token_id).expect("No offer");

        assert_eq!(
            env::predecessor_account_id(),
            offer.owner_id,
            "Must be offer owner"
        );
        assert!(offer.status == "OFFER","Offer state not valid");
        offer.rent = rent;
        offer.guarantee =guarantee;
        offer.max_duration = max_duration;
        self.offers.insert(&contract_and_token_id, &offer);
    }
    #[payable]
    pub fn withdraw_guarantee(&mut self, nft_contract_id: AccountId, token_id: String) -> Promise {
        assert_one_yocto();
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        let mut offer = self.offers.get(&contract_and_token_id).expect("No offer");
        assert!(offer.status == "ONRENT","Offer state not valid");
        let offer_user = offer.user_rights.unwrap();
        assert!(env::predecessor_account_id()==offer_user, "Only user with user right can call this contract");
        assert!(env::block_timestamp()<offer.lending_expiry.unwrap(), "contract expired ");
        let days = (env::block_timestamp() - offer.lending_start.unwrap())/1000_000_000; //24*3600*1000_000_000;
        let payable_to_owner = if offer.rent*days > offer.guarantee {
            offer.guarantee
        } else {
            offer.rent*days 
        };
        let payable_to_user = offer.guarantee - payable_to_owner;
        log!("timestamp: {}, owner part:{} borrower part:{}",env::block_timestamp(), payable_to_owner,payable_to_user );
        offer.status = "OFFER".into();
        offer.lending_start = None;
        offer.lending_expiry = None;
        offer.user_rights = None;
        self.offers.insert(&contract_and_token_id, &offer);
        ext_contract_ft::ft_transfer(offer.owner_id, U128::from(u128::from(payable_to_owner)), Some("withdraw guarantee".into()), 
        self.ft_contract_id.clone(), 1, GAS_FOR_FT_TRANSFER)
        .then(ext_contract_ft::ft_transfer(offer_user, U128::from(u128::from(payable_to_user)), Some("withdraw guarantee".into()), 
        self.ft_contract_id.clone(), 1, GAS_FOR_FT_TRANSFER))
        
    }

    #[payable]
    pub fn liquidate(&mut self, nft_contract_id: AccountId, token_id: String) -> Promise{
        assert_one_yocto();
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        let mut offer = self.offers.get(&contract_and_token_id).expect("No offer");
        assert!(offer.status == "ONRENT","Offer state not valid");
        assert!(env::predecessor_account_id()==offer.owner_id, "only owner can call this contract");
        assert!(env::block_timestamp()>offer.lending_expiry.unwrap(), "contract not expired yet, {}",env::block_timestamp());
        offer.status = "OFFER".into();
        offer.lending_start = None;
        offer.lending_expiry = None;
        offer.user_rights = None;
        self.offers.insert(&contract_and_token_id, &offer);
        ext_contract_ft::ft_transfer(offer.owner_id, U128::from(u128::from(offer.guarantee)), Some("liquidate".into()), 
        self.ft_contract_id.clone(), 1, GAS_FOR_FT_TRANSFER)

    }
  
}
