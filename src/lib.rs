 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env,ext_contract,Gas, Balance, near_bindgen, AccountId, PromiseOrValue,PanicOnDefault,CryptoHash,serde_json::json};
use near_sdk::serde::{Deserialize, Serialize};

use near_sdk::json_types::{U128};
use near_sdk::serde_json::{from_str};
use near_sdk::Promise;
use uint::construct_uint;

//use std::cmp::min;

//use crate::internal::*;
pub use crate::metadata::*;
pub use crate::migrate::*;
pub use crate::dao::*;

mod enumeration;
mod metadata;
mod migrate;
mod dao;

mod internal;

 
pub type EpochHeight = u64;
pub type SalePriceInYoctoNear = U128;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}
/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    AuctionsPerOwner,
    AuctionPerOwnerInner { account_id_hash: CryptoHash },
    AuctionsPerBidder,
    AuctionPerBidderInner { account_id_hash: CryptoHash },
    AuctionsById,
    BidsById,
    BidsByAuctionInner { auction_id: u128 },

    AuctionsMetadataById,
    NewLookup,
}



//aqui van los nombres de los metodos que mandaremos llamar
#[ext_contract(ext_contract_nft)]
trait NonFungibleToken {

    // change methods
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: String,
        msg: String,
    );

}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountActive {

    pub owner:String,
    pub treasury:String,
    pub nvt:String,


}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NFTAuctions {
    /// Owner's account ID (it will be a DAO on phase II)
    pub owner_account_id: AccountId,
    /// Owner's account ID (it will be a DAO on phase II)
    pub treasury_account_id: AccountId,
    //Index for auctions
    pub last_auction_id: u64,
    // Transaction interest estimated for the NFT payment
    // It is based as 10000=100%
    pub contract_interest: u64,
    //keeps track of the auction struct for a given auction ID
    pub auctions_by_id: UnorderedMap<AuctionId, Auction>,
    //keeps track of all the auction IDs for a given account
    pub auctions_per_owner: LookupMap<AccountId, UnorderedSet<AuctionId>>,
    //keeps track of all the auction IDs for a given account
    pub auctions_per_bidder: LookupMap<AccountId, UnorderedSet<AuctionId>>,

    pub bids_by_auction_id: UnorderedMap<AuctionId, UnorderedSet<Bid>>,
    /// Total token amount deposited.
    pub total_amount: Balance,
    /// Duration of payment period for auctions
    pub payment_period: u64,
    /// Fee payed to Nativo auctions
    pub contract_fee:u64, //200=2%
    // a flag to start/stop the ntv minting
    pub is_minting_ntv:bool,
    //
    pub ntv_multiply:u128,
    //how much auctions are running
    pub auctions_active: u128,
    //how much money has made by auctions
    pub auctions_amount_sold: u128,
    //how much ATH has made by auctions
    pub auctions_current_ath: u128,
    
    pub ntv_token_contract:String,
}


#[ext_contract(ext_nft)]
pub trait ExternsContract {
    fn mint(&self, account_id:AccountId,amount: String) -> String;
 }
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct PrevNFTAuctions {
    /// Owner's account ID (it will be a DAO on phase II)
    pub owner_account_id: AccountId,
    /// Owner's account ID (it will be a DAO on phase II)
    pub treasury_account_id: AccountId,
    //Index for auctions
    pub last_auction_id: u64,
    // Transaction interest estimated for the NFT payment
    // It is based as 10000=100%
    pub contract_interest: u64,
    //keeps track of the auction struct for a given auction ID
    pub auctions_by_id: UnorderedMap<AuctionId, Auction>,
    //keeps track of all the auction IDs for a given account
    pub auctions_per_owner: LookupMap<AccountId, UnorderedSet<AuctionId>>,
    //keeps track of all the auction IDs for a given account
    pub auctions_per_bidder: LookupMap<AccountId, UnorderedSet<AuctionId>>,

    pub bids_by_auction_id: UnorderedMap<AuctionId, UnorderedSet<Bid>>,
    /// Total token amount deposited.
    pub total_amount: Balance,
    /// Duration of payment period for auctions
    pub payment_period: u64,
    /// Fee payed to Nativo auctions
    pub contract_fee:u64, //200=2%
     // a flag to start/stop the ntv minting
     pub is_minting_ntv:bool,

     pub ntv_multiply:u128,

     //how much auctions are running
     pub auctions_active: u128,
     //how much money has made by auctions
     pub auctions_amount_sold: u128,
     //how much ATH has made by auctions
     pub auctions_current_ath: u128,
}



 
#[near_bindgen]
impl NFTAuctions {
    //Initialize the contract
    #![allow(dead_code, unused_variables,irrefutable_let_patterns,unconditional_recursion)]
    
    #[init]
    pub fn new(
        owner_account_id: AccountId,
        treasury_account_id: AccountId,
        contract_interest: u64, //800=8%
        contract_fee: u64, //200=2%
        
    ) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        let result= Self{
            owner_account_id,
            treasury_account_id,
            last_auction_id: 0,
            contract_interest,
            auctions_by_id: UnorderedMap::new(StorageKey::AuctionsById.try_to_vec().unwrap()),
            auctions_per_owner: LookupMap::new(StorageKey::AuctionsPerOwner.try_to_vec().unwrap()),
            auctions_per_bidder: LookupMap::new(StorageKey::AuctionsPerBidder.try_to_vec().unwrap()),
            bids_by_auction_id: UnorderedMap::new(StorageKey::BidsById.try_to_vec().unwrap()),
            total_amount: 0,
            payment_period:    1_000_000_000 * 60 * 60 * 24 * 7, //this is for a week
            contract_fee, //200=2%
            is_minting_ntv:true,
            ntv_multiply:3,
            auctions_active: 0,
            auctions_amount_sold: 0,
            auctions_current_ath: 0,
            ntv_token_contract:    "nativo_token.testnet".to_string(),
                };
        return result;
    }

    // Receive an NFT with the method nft_transfer_call 
    // This method is called from the NFT contract
    // When transfered succesful it is saved as a new requesting for auctioning
    pub fn nft_on_transfer(&mut self,sender_id: AccountId,previous_owner_id: AccountId,token_id: String,msg: String)  -> PromiseOrValue<bool>{
        env::log_str(&msg.to_string());
       
        let id:AuctionId = self.last_auction_id as u128;
        let contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        let msg_json: MsgInput = from_str(&msg).unwrap();
        let bid_start_id=0 as u128;
        //calculate amount to be payed 
        let amount_to_auctioner:u128 = u128::from(msg_json.auction_amount_requested)+(u128::from(msg_json.auction_amount_requested)*u128::from(self.contract_interest)/10000);
        env::log_str(&amount_to_auctioner.to_string());
        let media = msg_json.media.expect("the media is empty");

        //assert that the token and contract dont already exists in a old auction
        let new_auction = Auction{
            nft_contract:contract_id,
            nft_id:token_id,
            nft_owner:signer_id.clone() ,
            nft_media:Some(media),
            description:msg_json.description,
            auction_base_requested:msg_json.auction_amount_requested,
            auction_payback:msg_json.auction_amount_requested,
            status: AuctionStatus::Published,
            submission_time: (env::block_timestamp()/1000000),
            auction_time :None,
            auction_deadline:Some( (env::block_timestamp()/1000000) + (self.payment_period*1000)),
            bidder_id:None,
            
         };
        self.auctions_by_id.insert(&id, &new_auction);
       
        self.internal_add_auction_to_owner(&signer_id, &id);
        self.last_auction_id += 1;
        self.auctions_active += 1;

        env::log_str(
            &json!({
            "type": "new_auction".to_string(),
            "params":new_auction
            })
                .to_string(),
        );
        
     
        //If for some reason the contract failed it need to returns the NFT to the orig&inal owner (true)
        return PromiseOrValue::Value(false);
    }

    // Bid $NEAR Tokens to a Bid proposal
    #[payable]
    pub fn bid_for_nft(&mut self, auction_id: u128) -> Option<Auction> {
        //use a expect and explain that the auction wasnt found
        let mut auction:Auction = self.auctions_by_id.get(&auction_id.clone()).expect("the token doesn't have an active auction");   
        //prepare a new Bid structure with the bidder info 
        let new_bid =Bid{
            bidder_id:env::signer_account_id(),
            bid_amount:env::attached_deposit().into()};

        let signer_id =env::signer_account_id();
        let attached_deposit=env::attached_deposit();

        //Review that NFT is still available for auctioning
       assert_eq!(AuctionStatus::Published==auction.status || AuctionStatus::Bidded==auction.status ,true,"The NFT is not available for bidding");
       //check if the auction time has pased   
       if auction.auction_deadline.unwrap() <= (env::block_timestamp()/1000000){
                 // change the state to expired to dont allow more bids
                auction.status=AuctionStatus::Expired;

                self.auctions_by_id.insert(&auction_id, &auction);

                //panic by the end time
                assert_eq!(auction.auction_deadline.unwrap() >= (env::block_timestamp()/1000000),true,"The bid time has expired" );

            }
        //Review that  base amount is the required
        assert_eq!(attached_deposit>= u128::from(auction.auction_base_requested) ,true,"The amount payed is less than the base requested");

        //Review that current bid amount is more than the last one
        assert_eq!(attached_deposit> u128::from(auction.auction_payback),true,"The amount payed must be more than the payback");

        //Review that Bidder is not the same as NFT owner
        assert_ne!(signer_id.clone(),auction.nft_owner,"The owner cannot be the Bidder");

        //if exist a old bidder we must to refound the money to the old bidder
        if auction.bidder_id.is_some() {
            let old_bidder_id = auction.bidder_id.clone().unwrap();
            let old_bidder_balance = auction.auction_payback.clone();
            Promise::new(old_bidder_id).transfer(old_bidder_balance.into()); //before the owner recived the amount for treasury
            env::log_str("transfer to the old bidder done");
         }
        
         // Update the auction with the new bidder
        auction.status=AuctionStatus::Bidded;
        auction.bidder_id = Some(signer_id.clone());
        auction.auction_payback=attached_deposit.clone().into();
        auction.auction_time = Some(env::block_timestamp()/1000000);
        self.total_amount+=attached_deposit;  
        
         self.auctions_by_id.insert(&auction_id, &auction);
         
         self.internal_add_auction_to_bidder(&signer_id, &auction_id);
         self.internal_add_bid_to_auction(auction_id, &new_bid);

        env::log_str(
            &json!({
            "type": "bid_for_auction".to_string(),
            "params":auction
            })
                .to_string(),
        );
        return Some(auction);
    }

   
    //Canceled public offer for bidding by the nft owner
    #[payable]
    pub fn withdraw_nft_owner(&mut self, auction_id: u128){
        //use a expect and explain that the auction wasnt found
        let mut auction:Auction = self.auctions_by_id.get(&auction_id).expect("the token doesn't have an active auction");
        let signer_id =env::signer_account_id();
        let deposit = env::attached_deposit();

        assert!(auction.status!=AuctionStatus::Canceled,"The auction is canceled.");
        assert!(auction.status!=AuctionStatus::Claimed,"The auction was claimed.");
        //Review that claimer is the same as NFT owner
        if signer_id.clone() != auction.nft_owner.clone(){  
                   env::panic_str("You are not the owner of this NFT");
        }

        //1  owner cancel,the deadline is not over 
            if auction.auction_deadline.unwrap() > (env::block_timestamp()/1000000) {
                //if have a bid
                if auction.bidder_id.is_some(){
                     //Refound the bid
                    let old_bidder_id = auction.bidder_id.clone().unwrap();
                    let old_bidder_balance = auction.auction_payback.clone();
                    Promise::new(old_bidder_id).transfer(old_bidder_balance.into()); //before the owner recived the amount for treasury
                    self.internal_remove_auction_from_bidder(&auction.bidder_id.clone().unwrap(), &auction_id);
                    env::log_str("transfer to the old bidder done");
                } 
                   //just cancel the auction and transfer the NFT to the owner
                   auction.status=AuctionStatus::Canceled;
                   self.auctions_by_id.insert(&auction_id, &auction);
                   self.internal_remove_auction_from_owner(&signer_id.clone(), &auction_id);
                   self.auctions_active-=1;
                   //transfer the token

                   //log result 
                   env::log_str(
                       &json!({
                       "type": "withdraw_nft_owner".to_string(),
                       "params":auction
                       })
                           .to_string(),
                   );

                   // Inside a contract function on ContractA, a cross contract call is started
                   // From ContractA to ContractB
                   ext_contract_nft::nft_transfer(
                       signer_id.clone(),
                       auction.nft_id.to_string(),
                       "Withdraw of NFT from Nativo auctions".to_string(),
                       auction.nft_contract.clone(), // contract account id
                       deposit, // yocto NEAR to attach
                       Gas::from(5_000_000_000_000) // gas to attach
                       );


            }
       
        //2  owner cancel,the deadline is over and dont have a bid
         if auction.auction_deadline.unwrap() < (env::block_timestamp()/1000000) {
            //if have a bid
            if auction.bidder_id.is_some(){
                //if the deadline is over
                //panic to prevent the owner claim his token
                //to await the bidder can claim it
                panic!("Sorry,the auction is over,you must await the winner claim his prize");
             } 
               //just cancel the auction and transfer the NFT to the owner
               auction.status=AuctionStatus::Canceled;
               self.auctions_by_id.insert(&auction_id, &auction);
               self.internal_remove_auction_from_owner(&signer_id.clone(), &auction_id);
               self.auctions_active-=1;
               //transfer the token

               //log result 
               env::log_str(
                   &json!({
                   "type": "withdraw_nft_owner".to_string(),
                   "params":auction
                   })
                       .to_string(),
               );

               // Inside a contract function on ContractA, a cross contract call is started
               // From ContractA to ContractB
               ext_contract_nft::nft_transfer(
                   signer_id.clone(),
                   auction.nft_id.to_string(),
                   "Withdraw of NFT from Nativo auctions".to_string(),
                   auction.nft_contract, // contract account id
                   deposit, // yocto NEAR to attach
                   Gas::from(5_000_000_000_000) // gas to attach
                   );
        }
    }   
    

    //Canceled public offer for bid by the last bidder
    #[payable]
    pub fn withdraw_bid_bidder(&mut self, auction_id: u128){
        //use a expect and explain that the auction wasnt found
        let mut auction:Auction = self.auctions_by_id.get(&auction_id).expect("the token doesn't have an active auction");
        let signer_id =env::signer_account_id();
        let deposit = env::attached_deposit();
     // assert that the auctions wasnt canceled or claimed
        assert!(auction.status!=AuctionStatus::Canceled,"The auction was canceled.");
        assert!(auction.status!=AuctionStatus::Claimed,"The auction was claimed.");

         
        //Review that claimer is the same as NFT owner
       
        if signer_id != auction.bidder_id.clone().unwrap(){
            env::panic_str("You are not the last bidder ");
        }
        //if the auction is not over
        if auction.auction_deadline.unwrap() > (env::block_timestamp()/1000000) {
             //The bidder want to get back his money so we make a tranfers
                if auction.bidder_id.is_some() {
                

                    let old_bidder_id = auction.bidder_id.clone().unwrap();
                    let old_bidder_balance = auction.auction_payback.clone();
                    Promise::new(old_bidder_id).transfer(old_bidder_balance.into()); //before the owner recived the amount for treasury
                    self.internal_remove_auction_from_bidder(&auction.bidder_id.clone().unwrap(), &auction_id);
                    env::log_str("transfer to the old bidder done");
                }

                // we put assert that the status is biddded to allow more bids
                auction.status=AuctionStatus::Published;
                auction.bidder_id=None;
                auction.auction_payback=auction.auction_base_requested;
                //and we give one day more to be bidded
                auction.auction_deadline = Some(env::block_timestamp()/1000000+86400000);
                self.auctions_by_id.insert(&auction_id, &auction);
            
                env::log_str(
                    &json!({
                    "type": "withdraw_bid_bidder".to_string(),
                    "params":auction
                    })
                        .to_string(),
                );
        }            
       
        //if the auction is  over
        if auction.auction_deadline.unwrap() < (env::block_timestamp()/1000000) {
            //The bidder want to get back his money but he wins
               if auction.bidder_id.is_some() {
                env::panic_str("Sorry,you can't cancel the auction because has ended and you have win the auction,!please claim your prize¡.")
               }     
       }            
    
    }   
    
    //If time has passed and the auction has a bid
    //The bidder can claim the NFT and transfer to their wallet
    #[payable]
    pub fn claim_nft_winner(&mut self,auction_id:u128){
        //use a expect and explain that the auction wasnt found
        let mut auction:Auction = self.auctions_by_id.get(&auction_id).expect("the token doesn't have an active auction");
        let signer_id=env::signer_account_id();
        let time_stamp=env::block_timestamp()/1000000;
        let deposit = env::attached_deposit();
        let old_owner=auction.nft_owner.clone();
        let auction_payback=auction.auction_payback.clone();

        //assert that the bid time has passed
        assert_eq!(time_stamp>=auction.auction_deadline.unwrap(),true,"The payment auction time has not expired");
        

 
        //Review that claimer is the same as NFT auctioner
        if signer_id != auction.bidder_id.clone().unwrap(){
            env::panic_str("You can not claim this NFT");
        }

        auction.status=AuctionStatus::Claimed;
        auction.description=Some( format!("{}{:?}", "Nft claimend by ".to_string(), auction.bidder_id.clone() ) );
        
        self.auctions_by_id.insert(&auction_id, &auction);
        self.internal_remove_auction_from_owner(&auction.nft_owner, &auction_id);
        self.internal_remove_auction_from_bidder(&signer_id, &auction_id);
        self.auctions_active -=1;
        // env::log_str(
        //     &json!(&auction)
        //     .to_string(),
        // );

            //save the amount for the amount_sold
            let amount_sold :u128=auction.auction_payback.clone().into();
            self.auctions_amount_sold+=amount_sold;

            //save the ATH amount in an auction sold
            if self.auctions_current_ath<amount_sold {
                self.auctions_current_ath=amount_sold;
            }
           // self.auctions_amount_sold+= 
        // we pay the highest bid to the owner auction
        let contract_percent:u128 = self.contract_fee.into();
        let fee_percent=contract_percent/1000;
        let nativo_fee =amount_sold*fee_percent;
        let owner_payment =amount_sold-nativo_fee;
        Promise::new(self.treasury_account_id.clone()).transfer(nativo_fee); 

        Promise::new(old_owner.clone()).transfer(owner_payment); 


        //minting the nvt section
        if self.is_minting_ntv {

            let tokens_to_mint = amount_sold *self.ntv_multiply;
            // NTV for the buyer
            ext_nft::mint(
                signer_id.clone(),
                tokens_to_mint.to_string(),
                self.ntv_token_contract.to_string().try_into().unwrap(),
                0000000000000000000000001,
                10_000_000_000_000.into(),
            );
             // NTV for the owner
            ext_nft::mint(
                old_owner,
                tokens_to_mint.to_string(),
                self.ntv_token_contract.to_string().try_into().unwrap(),
                0000000000000000000000001,
                10_000_000_000_000.into(),
            );

        }else{
            env::log_str("the nvt token minting is disabled");      
          }

        env::log_str(
            &json!({
            "type": "claim_nft_winner".to_string(),
            "params":auction
            })
                .to_string(),
        );
        // Inside a contract function on ContractA, a cross contract call is started
        // From ContractA to ContractB
        ext_contract_nft::nft_transfer(
        signer_id,
        auction.nft_id.to_string(),
        "Withdraw of NFT from Nativo auctions".to_string(),
        auction.nft_contract, // contract account id
        deposit, // yocto NEAR to attach
        Gas::from(5_000_000_000_000) // gas to attach
        );
    }

 
}


// This are the tests
// PENDING
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_status("francis.near".to_string()));
    }
}