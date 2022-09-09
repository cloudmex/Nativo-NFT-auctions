use crate::*;

#[near_bindgen]
impl NFTAuctions {
    #![allow(dead_code, unused_variables,irrefutable_let_patterns,)]
    #[warn(unconditional_recursion)]
    //get the information for a specific token ID
   pub fn get_nft_auction(&self, auction_id: AuctionId) -> Option<AuctionOutput> {
        //if there is some auction ID in the auctions_by_id collection

        let auction :Option<Auction>= self.auctions_by_id.get(&auction_id);

        if auction.is_some() {
            //we'll return the data for that auction
            Some(AuctionOutput {
                id:auction_id,
                auction:auction.unwrap().into(),
            })
        } else { //if there wasn't a auction ID in the auctions_by_id collection, we return None
            None
        }
    }

    //get the information for a specific token ID
   pub fn get_bid_auction(&self, auction_id: AuctionId) -> Option< Vec<Bid>  >{
        //if there is some auction ID in the auctions_by_id collection
        let bid:Option< UnorderedSet<Bid> >=self.bids_by_auction_id.get(&auction_id);
        if  bid.is_some()  {
            //we'll return the data for that auction
            Some(bid.unwrap().to_vec())
        } else { //if there wasn't a auction ID in the auctions_by_id collection, we return None
            None
        }
    }

 

    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn get_all_nfts_for_auction(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<AuctionOutput> {
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

    //this method return  all the auctions with status published or bidded
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
            .filter_map(|auction_id|{
                        let action_g = self.get_nft_auction(auction_id).unwrap_or( AuctionOutput{
                              id: auction_id,
                              auction: self.new_auction(),
                                                
                         });
                    if action_g.auction.status== AuctionStatus::Published || action_g.auction.status== AuctionStatus::Bidded{
                        Some(self.get_nft_auction(auction_id).unwrap())
                    }else{
                        None
                    }
                }
            )
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    //its al helped method that return a empty auction
    fn new_auction  (&self) -> Auction{
        let new_auction = Auction{
            auction_id:None,
            nft_contract:"null".to_string().try_into().unwrap(),
            nft_id:"null".to_string().try_into().unwrap(),
            nft_owner:"null".to_string().try_into().unwrap() ,
            nft_media:"null".to_string().try_into().unwrap() ,
            description:"null".to_string().try_into().unwrap(),
            auction_base_requested:0.into(),
            auction_payback:0.into(),
            status: AuctionStatus::NotFound,
            submission_time: env::block_timestamp(),
            auction_time:None,
            auction_deadline:None,
            bidder_id:None,
            approved_account_ids:None,
            royalty:None,
            
         };
         new_auction
    }
 
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


    pub fn get_contract_owmer(&self)->AccountId {
        self.owner_account_id.clone()
    }
    pub fn get_contract_treasury(&self)->AccountId {
        self.treasury_account_id.clone()
    }
    pub fn get_payment_period(&self)->u64 {
        self.payment_period.clone()
    }
    
    
    pub fn get_contract_fee(&self)->u64 {
        self.contract_fee.clone()
    }
    
    pub fn is_ntv_enable_minting(&self)->bool {
        self.is_minting_ntv
    }
    pub fn get_auctions_actives(&self)->u128 {
        self.auctions_active
    }

    pub fn get_auctions_amount_sold(&self)->u128 {
        self.auctions_amount_sold
    }
    pub fn get_auctions_current_ath(&self)->u128 {
        self.auctions_current_ath
    }



    pub(crate) fn to_sec(timestamp: Timestamp) -> TimestampSec {
        (timestamp / 10u64.pow(9)) as u32
    }
    pub(crate) fn to_sec_u64(timestamp: Timestamp) -> Timestamp {
        timestamp / 10u64.pow(9) 
    }
}