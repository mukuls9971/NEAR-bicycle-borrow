use crate::*;

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

impl Contract {
    pub(crate) fn internal_remove_offer(
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
    ) -> Offer {
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id);
        let offer = self.offers.remove(&contract_and_token_id).expect("No offer");

        let mut by_owner_id = self.by_owner_id.get(&offer.owner_id).expect("No offer by_owner_id");
        by_owner_id.remove(&contract_and_token_id);
        
        if by_owner_id.is_empty() {
            self.by_owner_id.remove(&offer.owner_id);
        } else {
            self.by_owner_id.insert(&offer.owner_id, &by_owner_id);
        }

        let mut by_nft_contract_id = self
            .by_nft_contract_id
            .get(&nft_contract_id)
            .expect("No offfer by nft_contract_id");
        
        //remove the token ID from the set 
        by_nft_contract_id.remove(&token_id);
        
        //if the set is now empty after removing the token ID, we remove that nft contract ID from the map
        if by_nft_contract_id.is_empty() {
            self.by_nft_contract_id.remove(&nft_contract_id);
        //if the set is not empty after removing, we insert the set back into the map for the nft contract ID
        } else {
            self.by_nft_contract_id
                .insert(&nft_contract_id, &by_nft_contract_id);
        }

        
        offer
    }
}
