 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env,ext_contract,Gas, Balance, near_bindgen, AccountId, PromiseOrValue,PanicOnDefault,CryptoHash,serde_json::json};
use near_sdk::serde::{Deserialize, Serialize};

use near_sdk::json_types::{U128,Base64VecU8};
use near_sdk::serde_json::{from_str};
use near_sdk::{Promise, PromiseResult};
use uint::construct_uint;
use std::collections::HashMap;
use near_sdk::promise_result_as_success;


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
pub type TokenId = String;

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

     // change methods
    fn get_promise_result(&self,contract_id:AccountId,signer_id:AccountId,msg_json:MsgInput) -> String;
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId, //purchaser (person to transfer the NFT to)
        token_id: TokenId, //token ID to transfer
        approval_id: u64, //market contract's approval ID in order to transfer the token on behalf of the owner
        memo: String, //memo (to include some context)
        /*
            the price that the token was purchased for. This will be used in conjunction with the royalty percentages
            for the token in order to determine how much money should go to which account. 
        */
        balance: U128,
        //the maximum amount of accounts the market can payout at once (this is limited by GAS)
		max_len_payout: u32,
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
    fn nft_token(& self,token_id: TokenId);
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>) ;
    fn resolve_purchase(
        &mut self,
        buyer_id: AccountId,
        own: AccountId,

        price: U128,
    ) -> Promise;

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
            payment_period: 120, //604800, //this is for a week
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
        
       
        let contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        let msg_json: MsgInput = from_str(&msg).unwrap();
        let bid_start_id=0 as u128;
 
       //the auction contract set itsefl as approval id
        ext_nft::nft_approve(
            token_id.clone(),
            env::current_account_id(),
            None, 
            env::predecessor_account_id(),
            1, Gas(15_000_000_000_000) );
        //manage the token info as auction     
        let p= ext_nft::nft_token(
            token_id.clone(),
            env::predecessor_account_id(), //contract account we're calling
            0, //NEAR deposit we attach to the call
            Gas(100_000_000_000_000), //GAS we're attaching
        ) 
        .then(ext_contract_nft::get_promise_result(contract_id,signer_id,msg_json,
            env::current_account_id(), // el mismo contrato local
            0,                                             // yocto NEAR a ajuntar al callback
            Gas(15_000_000_000_000),                            // gas a ajuntar al callback
        ));
             

     
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
            bidded_at:NFTAuctions::to_sec_u64(env::block_timestamp()),
            bid_amount:env::attached_deposit().into()
        };

        let signer_id =env::signer_account_id();
        let attached_deposit=env::attached_deposit();

        //Review that NFT is still available for auctioning
       assert_eq!(AuctionStatus::Published==auction.status || AuctionStatus::Bidded==auction.status ,true,"The NFT is not available for bidding");
       //check if the auction time has pased   
       if auction.auction_deadline.unwrap() <= NFTAuctions::to_sec_u64(env::block_timestamp()){
                 // change the state to expired to dont allow more bids
                auction.status=AuctionStatus::Finished;

                self.auctions_by_id.insert(&auction_id, &auction);

                //panic by the end time
                assert_eq!(auction.auction_deadline.unwrap() >= NFTAuctions::to_sec_u64(env::block_timestamp()),true,"The bid time has expired" );

            }
        //Review that  base amount is the required
        assert_eq!(attached_deposit>= u128::from(auction.auction_base_requested) ,true,"The amount payed is less than the base requested");

        //Review that current bid amount is more than the last one
        assert_eq!(attached_deposit> u128::from(auction.auction_payback),true,"The amount payed must be more than the payback");

        //Review that Bidder is not the same as NFT owner
        assert_ne!(signer_id.clone(),auction.nft_owner,"The owner cannot be the Bidder");

        
        //if exist a old bidder we must to refound the money to the old bidder
        if auction.bidder_id.is_some() {
            //Review that Bidder is not the same as Told bidder
        assert_ne!(signer_id.clone(),auction.bidder_id.clone().unwrap(),"you cannot bid again ");

            let old_bidder_id = auction.bidder_id.clone().unwrap();
            let old_bidder_balance = auction.auction_payback.clone();
            Promise::new(old_bidder_id).transfer(old_bidder_balance.into()); //before the owner recived the amount for treasury
            env::log_str("transfer to the old bidder done");
         }
        
         // Update the auction with the new bidder
        auction.status=AuctionStatus::Bidded;
        auction.bidder_id = Some(signer_id.clone());
        auction.auction_payback=attached_deposit.clone().into();
        auction.auction_time = Some(NFTAuctions::to_sec_u64(env::block_timestamp()));
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
        let oldowner= auction.clone().nft_owner;

        assert!(auction.status!=AuctionStatus::Canceled,"The auction is canceled.");
        assert!(auction.status!=AuctionStatus::Claimed,"The auction was claimed.");
        //Review that claimer is the same as NFT owner
        if signer_id.clone() != auction.nft_owner.clone(){  
                   env::panic_str("You are not the owner of this NFT");
        }

        //1  owner cancel,the deadline is not over 
            if auction.auction_deadline.unwrap() > NFTAuctions::to_sec_u64(env::block_timestamp()) {
                //if have a bid
                if auction.bidder_id.is_some(){
                    //  //Refound the bid
                    // let old_bidder_id = auction.bidder_id.clone().unwrap();
                    // let old_bidder_balance = auction.auction_payback.clone();
                    // Promise::new(old_bidder_id).transfer(old_bidder_balance.into()); //before the owner recived the amount for treasury
                    // self.internal_remove_auction_from_bidder(&auction.bidder_id.clone().unwrap(), &auction_id);
                    // env::log_str("transfer to the old bidder done");

                    //
                    env::panic_str("You have a bid,you can not cancel the auction");
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
         if auction.auction_deadline.unwrap() < NFTAuctions::to_sec_u64(env::block_timestamp()){
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
    // #[payable]
    // #[private]
    // fn withdraw_bid_bidder(&mut self, auction_id: u128){
    //     //use a expect and explain that the auction wasnt found
    //     let mut auction:Auction = self.auctions_by_id.get(&auction_id).expect("the token doesn't have an active auction");
    //     let signer_id =env::signer_account_id();
    //     let deposit = env::attached_deposit();
    //  // assert that the auctions wasnt canceled or claimed
    //     assert!(auction.status!=AuctionStatus::Canceled,"The auction was canceled.");
    //     assert!(auction.status!=AuctionStatus::Claimed,"The auction was claimed.");

         
    //     //Review that claimer is the same as NFT owner
       
    //     if signer_id != auction.bidder_id.clone().unwrap(){
    //         env::panic_str("You are not the last bidder ");
    //     }
    //     //if the auction is not over
    //     if auction.auction_deadline.unwrap() > NFTAuctions::to_sec_u64(env::block_timestamp()) {
    //          //The bidder want to get back his money so we make a tranfers
    //             if auction.bidder_id.is_some() {
                

    //                 let old_bidder_id = auction.bidder_id.clone().unwrap();
    //                 let old_bidder_balance = auction.auction_payback.clone();
    //                 Promise::new(old_bidder_id).transfer(old_bidder_balance.into()); //before the owner recived the amount for treasury
    //                 self.internal_remove_auction_from_bidder(&auction.bidder_id.clone().unwrap(), &auction_id);
    //                 env::log_str("transfer to the old bidder done");
    //             }

    //             // we put assert that the status is biddded to allow more bids
    //             auction.status=AuctionStatus::Published;
    //             auction.bidder_id=None;
    //             auction.auction_payback=auction.auction_base_requested;
    //             //and we give one day more to be bidded
    //             auction.auction_deadline = Some(NFTAuctions::to_sec_u64(env::block_timestamp())+86400);
    //             self.auctions_by_id.insert(&auction_id, &auction);
            
    //             env::log_str(
    //                 &json!({
    //                 "type": "withdraw_bid_bidder".to_string(),
    //                 "params":auction
    //                 })
    //                     .to_string(),
    //             );
    //     }            
       
    //     //if the auction is  over
    //     if auction.auction_deadline.unwrap() < NFTAuctions::to_sec_u64(env::block_timestamp()) {
    //         //The bidder want to get back his money but he wins
    //            if auction.bidder_id.is_some() {
    //             env::panic_str("Sorry,you can't cancel the auction because has ended and you have win the auction,!please claim your prize¡.")
    //            }     
    //    }            
    
    // }   
    
    //If time has passed and the auction has a bid
    //The bidder can claim the NFT and transfer to their wallet
    #[payable]
    pub fn claim_nft_winner(&mut self,auction_id:u128){
        //use a expect and explain that the auction wasnt found
        let mut auction:Auction = self.auctions_by_id.get(&auction_id).expect("the token doesn't have an active auction");
        let signer_id=env::signer_account_id();
        let time_stamp=NFTAuctions::to_sec_u64(env::block_timestamp());
        
        let old_owner=auction.nft_owner.clone();
        let auction_payback=auction.auction_payback.clone();
       
        //assert that the bid time has passed
        assert_eq!(time_stamp>=auction.auction_deadline.unwrap(),true,"The payment auction time has not expired");
        

 
        //Review that claimer is the same as NFT auctioner
        if signer_id != auction.bidder_id.clone().unwrap(){
            env::panic_str("You can not claim this NFT");
        }

        auction.status=AuctionStatus::Claimed;
        
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
        let contract_percent:u64 = self.contract_fee;
        let fee_percent:f64= contract_percent as f64/1000 as f64;
        let nativo_fee =amount_sold as f64*fee_percent;
        let owner_payment =amount_sold-nativo_fee as u128;
        //we retrive the fee p
        Promise::new(self.treasury_account_id.clone()).transfer(nativo_fee as u128); 

     
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

//  // // Inside a contract function on ContractA, a cross contract call is started
//         // // From ContractA to ContractB
//         ext_contract_nft::nft_transfer(
//             signer_id,
//             auction.nft_id.to_string(),
//             "Withdraw of NFT from Nativo auctions".to_string(),
//             auction.nft_contract, // contract account id
//             deposit, // yocto NEAR to attach
//             Gas::from(5_000_000_000_000) // gas to attach
//             );


//add the old owner as approval
       //the auction contract set itsefl as approval id
       ext_nft::nft_approve(
        auction.clone().nft_id,
        auction.clone().nft_owner,
      None, 
      auction.clone().nft_contract,
        1, Gas(15_000_000_000_000) );

          //process the purchase (which will remove the sale, transfer and get the payout from the nft contract, and then distribute royalties) 
          self.process_claim(
            auction.clone().nft_contract,
            auction,
            U128(owner_payment),
            signer_id,
        );
       
    }

 

     // Método de procesamiento para promesa
     pub fn get_promise_result(&mut self ,contract_id:AccountId,signer_id:AccountId,msg_json:MsgInput) {
         
        assert_eq!(
            env::promise_results_count(),
            1,
            "This is a callbacl module"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                env::log_str( &"the external contract failed".to_string());
                 
             }
            PromiseResult::Successful(result) => {
                let value = std::str::from_utf8(&result).unwrap();
              //  env::log_str("regreso al market");
              //  env::log_str(value);
                let tg: JsonToken = near_sdk::serde_json::from_str(&value).unwrap();
                let id:AuctionId = self.last_auction_id as u128;

                let roy :Option<HashMap<AccountId, u32>>= tg.royalty;

                let new_auction = Auction{
                    auction_id:Some((self.last_auction_id as u64).into()),
                    nft_contract:contract_id,
                    nft_id:tg.token_id,
                    nft_owner:signer_id.clone(),
                    nft_media:Some(tg.metadata.media.expect("the media is empty")),
                    description:Some(tg.metadata.description.unwrap_or("the description was empty".to_string()) ),
                    auction_base_requested:msg_json.auction_amount_requested,
                    auction_payback:msg_json.auction_amount_requested,
                    status: AuctionStatus::Published,
                    submission_time: NFTAuctions::to_sec_u64(env::block_timestamp()) ,
                    auction_time :None,
                    auction_deadline:Some( NFTAuctions::to_sec_u64(env::block_timestamp()) + (self.payment_period)),
                    bidder_id:None,
                    approved_account_ids:Some( tg.approved_account_ids ),
                    royalty: if roy.is_some() { roy }else{ None }, 

                     
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


                
            }
        }
    }
   
      



    // 
     //private function used when a sale is purchased. 
    //this will remove the sale, transfer and get the payout from the nft contract, and then distribute royalties
    #[private]
    pub fn process_claim(
        &mut self,
        nft_contract_id: AccountId,
        auction: Auction,
        price: U128,
        buyer_id: AccountId,
    ) -> Promise {
      
      let token_id = auction.clone().nft_id;
      let mut new_a=  self.auctions_by_id.get(&(auction.clone().auction_id.unwrap() as u128)).unwrap();
        let own =  auction.clone().nft_owner;

  
      let approvals = auction.clone().approved_account_ids.unwrap();
      let approvalid = *approvals.get(&env::current_account_id()).unwrap();
      //new_a.approved_account_ids.unwrap().insert(auction.clone().nft_owner, approvalid+1);
      let mut hm_apr =new_a.approved_account_ids.unwrap();
      hm_apr.remove(&env::current_account_id());
      hm_apr.insert(auction.clone().nft_owner, approvalid+1) ;
      
      new_a.approved_account_ids=Some(hm_apr);
      
      self.auctions_by_id.insert(&(auction.clone().auction_id.unwrap() as u128) , &new_a.clone());


       // initiate a cross contract call to the nft contract. This will transfer the token to the buyer and return
        //a payout object used for the market to distribute funds to the appropriate accounts.
        ext_contract_nft::nft_transfer_payout(
            buyer_id.clone(), //purchaser (person to transfer the NFT to)
            token_id, //token ID to transfer
            approvalid+1, //market contract's approval ID in order to transfer the token on behalf of the owner
            "payout from market".to_string(), //memo (to include some context)
            /*
                the price that the token was purchased for. This will be used in conjunction with the royalty percentages
                for the token in order to determine how much money should go to which account. 
            */
            price,
			10, //the maximum amount of accounts the market can payout at once (this is limited by GAS)
            nft_contract_id, //contract to initiate the cross contract call to
            1, //yoctoNEAR to attach to the call
            Gas::from(5_000_000_000_000), //GAS to attach to the call
        )
        //after the transfer payout has been initiated, we resolve the promise by calling our own resolve_purchase function. 
        //resolve purchase will take the payout object returned from the nft_transfer_payout and actually pay the accounts
        .then(ext_nft::resolve_purchase(
            buyer_id, 
            own,//the buyer and price are passed in incase something goes wrong and we need to refund the buyer
            price,
            env::current_account_id(), //we are invoking this function on the current contract
            0, //don't attach any deposit
            Gas::from(5_000_000_000_000), //GAS attached to the call to payout royalties
        ))

      
    }


    #[private]
    pub fn resolve_purchase(
        &mut self,
        buyer_id: AccountId,
        own: AccountId,

        price: U128,
    ) -> U128 {
        // checking for payout information returned from the nft_transfer_payout method
        let payout_option = promise_result_as_success().and_then(|value| {
            //if we set the payout_option to None, that means something went wrong and we should refund the buyer
            near_sdk::serde_json::from_slice::<Payout>(&value)
                //converts the result to an optional value
                .ok()
                //returns None if the none. Otherwise executes the following logic
                .and_then(|payout_object| {
                    //we'll check if length of the payout object is > 10 or it's empty. In either case, we return None
                    if payout_object.payout.len() > 10 || payout_object.payout.is_empty() {
                      //  env::log_str("Cannot have more than 10 royalties");
                        None
                    
                    //if the payout object is the correct length, we move forward
                    } else {
                        //we'll keep track of how much the nft contract wants us to payout. Starting at the full price payed by the buyer
                        let mut remainder = price.0;
                        
                        //loop through the payout and subtract the values from the remainder. 
                        for &value in payout_object.payout.values() {
                            //checked sub checks for overflow or any errors and returns None if there are problems
                            remainder = remainder.checked_sub(value.0)?;
                        }
                        //Check to see if the NFT contract sent back a faulty payout that requires us to pay more or too little. 
                        //The remainder will be 0 if the payout summed to the total price. The remainder will be 1 if the royalties
                        //we something like 3333 + 3333 + 3333. 
                        if remainder == 0 || remainder == 1 {
                            //set the payout_option to be the payout because nothing went wrong
                            Some(payout_object.payout)
                        } else {
                            //if the remainder was anything but 1 or 0, we return None
                            None
                        }
                    }
                })
        });

        // if the payout option was some payout, we set this payout variable equal to that some payout
        let payout = if let Some(payout_option) = payout_option {
            payout_option
        //if the payout option was None, we refund the buyer for the price they payed and return
        } else {
            Promise::new(buyer_id).transfer(u128::from(price));
            // leave function and return the price that was refunded
            return price;
        };

        // NEAR payouts
        for (receiver_id, amount) in payout {
            if receiver_id.eq(&env::current_account_id()){
                env::log_str(&format!("rece: {} amount: {}",own.clone(),amount.0.clone()));
                Promise::new(own.clone()).transfer(amount.0);
            }
            else{
                env::log_str(&format!("rece: {} amount: {}",receiver_id.clone(),amount.0.clone()));
                Promise::new(receiver_id).transfer(amount.0);
            }
            
        }

        //return the price payout out
        price
    }

}