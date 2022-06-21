use crate::*;
use near_sdk::{CryptoHash};
use std::mem::size_of;

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

impl NFTAuctions {

     //add a auction to the set of tokens an owner has
     pub(crate) fn internal_add_bid_to_auction(
        &mut self,
        auction_id: auctionId,
        Bid:&Bidst,
    ) {
        //get the set of tokens for the given auction
        let mut bids_set = self.bids_by_auction_id.get(&auction_id).unwrap_or_else(|| {
            //if the account doesn't have any tokens, we create a new unordered set
            UnorderedSet::new(
                StorageKey::bidsByAuctionInner {
                    //we get a new unique prefix for the collection
                    auction_id: (auction_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //we insert the token ID into the set
        bids_set.insert(Bid);

        //we insert that set for the given account ID. 
        self.bids_by_auction_id.insert(&auction_id, &bids_set);
    }




    //add a auction to the set of tokens an owner has
    pub(crate) fn internal_add_auction_to_owner(
        &mut self,
        account_id: &AccountId,
        auction_id: &auctionId,
    ) {
        //get the set of tokens for the given account
        let mut auctions_set = self.auctions_per_owner.get(account_id).unwrap_or_else(|| {
            //if the account doesn't have any tokens, we create a new unordered set
            UnorderedSet::new(
                StorageKey::auctionPerOwnerInner {
                    //we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //we insert the token ID into the set
        auctions_set.insert(auction_id);

        //we insert that set for the given account ID. 
        self.auctions_per_owner.insert(account_id, &auctions_set);
    }

    //remove a token from an owner (internal method and can't be called directly via CLI).
    pub(crate) fn internal_remove_auction_from_owner(
        &mut self,
        account_id: &AccountId,
        auction_id: &auctionId,
    ) {
        //we get the set of tokens that the owner has
        let mut auctions_set = self
            .auctions_per_owner
            .get(account_id)
            //if there is no set of tokens for the owner, we panic with the following message:
            .expect("auction should be owned by the sender");

        //we remove the the token_id from the set of tokens
        auctions_set.remove(auction_id);

        //if the token set is now empty, we remove the owner from the tokens_per_owner collection
        if auctions_set.is_empty() {
            self.auctions_per_owner.remove(account_id);
        } else {
        //if the token set is not empty, we simply insert it back for the account ID. 
            self.auctions_per_owner.insert(account_id, &auctions_set);
        }
    }

    pub(crate) fn internal_add_auction_to_Bidder(
        &mut self,
        account_id: &AccountId,
        auction_id: &auctionId,
    ) {
        //get the set of tokens for the given account
        let mut auctions_set = self.auctions_per_Bidder.get(account_id).unwrap_or_else(|| {
            //if the account doesn't have any tokens, we create a new unordered set
            UnorderedSet::new(
                StorageKey::auctionPerBidderInner {
                    //we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //we insert the token ID into the set
        auctions_set.insert(auction_id);

        //we insert that set for the given account ID. 
        self.auctions_per_Bidder.insert(account_id, &auctions_set);
    }

    //remove a token from an owner (internal method and can't be called directly via CLI).
    pub(crate) fn internal_remove_auction_from_Bidder(
        &mut self,
        account_id: &AccountId,
        auction_id: &auctionId,
    ) {
        //we get the set of tokens that the owner has
        let mut auctions_set = self
            .auctions_per_Bidder
            .get(account_id)
            //if there is no set of tokens for the owner, we panic with the following message:
            .expect("auction should be lended by the sender");

        //we remove the the token_id from the set of tokens
        auctions_set.remove(auction_id);

        //if the token set is now empty, we remove the owner from the tokens_per_owner collection
        if auctions_set.is_empty() {
            self.auctions_per_Bidder.remove(account_id);
        } else {
        //if the token set is not empty, we simply insert it back for the account ID. 
            self.auctions_per_Bidder.insert(account_id, &auctions_set);
        }
    }
}