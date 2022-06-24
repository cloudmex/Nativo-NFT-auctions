use crate::*;

#[near_bindgen]
impl NFTAuctions {
    #![allow(dead_code, unused_variables,irrefutable_let_patterns)]

    //get the information for a specific token ID
    fn get_nft_auction(&self, auction_id: AuctionId) -> Option<AuctionOutput> {
        //if there is some auction ID in the auctions_by_id collection
        if let auctions = self.auctions_by_id.get(&auction_id).unwrap() {
            //we'll return the data for that auction
            Some(AuctionOutput {
                id:auction_id,
                auction:auctions.into(),
            })
        } else { //if there wasn't a auction ID in the auctions_by_id collection, we return None
            None
        }
    }

    //get the information for a specific token ID
   pub fn get_bid_auction(&self, auction_id: AuctionId) -> Option< Vec<Bid>  >{
        //if there is some auction ID in the auctions_by_id collection
        if let bid = self.bids_by_auction_id.get(&auction_id).unwrap() {
            //we'll return the data for that auction
            Some(bid.to_vec())
        } else { //if there wasn't a auction ID in the auctions_by_id collection, we return None
            None
        }
    }

    // pub fn get_nft_auction(&self, auction_id: u64) -> AuctionOutput {
    //     let auctions = self.auctions.get(&auction_id).expect("ERR_NO_auction");
    //     AuctionOutput {
    //         id:auction_id,
    //         auction: auctions.into(),
    //     }
    // }

    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn get_nfts_for_auction(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<AuctionOutput> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through each token using an iterator
        self.auctions_by_id.keys_as_vector().iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|auction_id| self.get_nft_auction(auction_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
//   //Query for nft tokens on the contract regardless of the owner using pagination
//   pub fn get_bids_for_auction(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Bid> {
//     //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
//     let start = u128::from(from_index.unwrap_or(U128(0)));

//     //iterate through each token using an iterator
//     self.bids_by_auction_id.keys_as_vector().iter()
//         //skip to the index we specified in the start variable
//         .skip(start as usize) 
//         //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
//         .take(limit.unwrap_or(50) as usize) 
//         //we'll map the token IDs which are strings into Json Tokens
//         .map(|auction_id| self.get_bid_auction(auction_id.clone()).unwrap())
//         //since we turned the keys into an iterator, we need to turn it back into a vector to return
//         .collect()
// }
    //View wich NFT are available for auctioning
    // pub fn get_nfts_for_auction(&self, from_index: u64, limit: u64)-> Vec<AuctionOutput> {
    //     (from_index..min(self.last_auction_id, from_index + limit))
    //         .filter_map(|id| {
    //             self.auctions.get(&id).map(|auction| AuctionOutput {
    //                 id,
    //                 auction: auction.into(),
    //             })
    //         })
    //         .collect()
    // }

    //View the auction_id of the last auction
    pub fn get_contract_interest(&self)-> u64 {
        self.contract_interest
    }
    
    //View the auction_id of the last auction
    pub fn get_last_auction(&self)-> u64 {
        self.last_auction_id
    }

    //get the total supply of NFTs for a given owner
    pub fn auction_supply_for_owner(
        &self,
        account_id: AccountId,
    ) -> U128 {
        //get the set of tokens for the passed in owner
        let auctions_for_owner_set = self.auctions_per_owner.get(&account_id);

        //if there is some set of tokens, we'll return the length as a U128
        if let Some(auctions_for_owner_set) = auctions_for_owner_set {
            U128(auctions_for_owner_set.len() as u128)
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            U128(0)
        }
    }

    //Query for all the tokens for an owner
    pub fn auctions_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<AuctionOutput> {
        //get the set of tokens for the passed in owner
        let auctions_for_owner_set = self.auctions_per_owner.get(&account_id);
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let auctions = if let Some(auctions_for_owner_set) = auctions_for_owner_set {
            auctions_for_owner_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector. 
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        auctions.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|auction_id| self.get_nft_auction(auction_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }


    //get the total supply of NFTs for a given owner
    pub fn auction_supply_for_bidder(
        &self,
        account_id: AccountId,
    ) -> U128 {
        //get the set of tokens for the passed in owner
        let auctions_for_bidder_set = self.auctions_per_bidder.get(&account_id);

        //if there is some set of tokens, we'll return the length as a U128
        if let Some(auctions_for_bidder_set) = auctions_for_bidder_set {
            U128(auctions_for_bidder_set.len() as u128)
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            U128(0)
        }
    }

    //Query for all the tokens for an owner
    pub fn auctions_for_bidder(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<AuctionOutput> {
        //get the set of tokens for the passed in owner
        let auctions_for_bidder_set = self.auctions_per_bidder.get(&account_id);
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let auctions = if let Some(auctions_for_bidder_set) = auctions_for_bidder_set {
            auctions_for_bidder_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector. 
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        auctions.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|auction_id| self.get_nft_auction(auction_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}