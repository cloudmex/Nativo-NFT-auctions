use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env,ext_contract, Balance, near_bindgen, AccountId, PromiseOrValue,PanicOnDefault,CryptoHash};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{U128};
use near_sdk::serde_json::{from_str};
use near_sdk::Promise;
use uint::construct_uint;

//use std::cmp::min;

//use crate::internal::*;
pub use crate::metadata::*;
pub use crate::migrate::*;

mod enumeration;
mod metadata;
mod migrate;

mod internal;

 
pub type EpochHeight = u64;

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
}


#[near_bindgen]
impl NFTAuctions {
    //Initialize the contract
    #![allow(dead_code, unused_variables,irrefutable_let_patterns)]
    
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
            payment_period:1_000_000_000 * 60 * 60 * 24 * 7,
            contract_fee, //200=2%
        };
        return result;
    }

    // Receive an NFT with the method nft_transfer_call 
    // This method is called from the NFT contract
    // When transfered succesful it is saved as a new requesting for auctioning
    pub fn nft_on_transfer(&mut self,sender_id: AccountId,previous_owner_id: AccountId,token_id: String,msg: String)  -> PromiseOrValue<bool>{
        env::log_str(&msg.to_string());
        /*if msg.is_empty() || msg=="" {
            env::log_str("ERR_INVALID_MESSAGE");
            None
        };*/
        //assert!(msg.is_empty() || msg=="" ,"ERR_INVALID_MESSAGE");
        let id:AuctionId = self.last_auction_id as u128;
        let contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        let msg_json: MsgInput = from_str(&msg).unwrap();
        let bid_start_id=0 as u128;
        //calculate amount to be payed 
        let amount_to_auctioner:u128 = u128::from(msg_json.auction_amount_requested)+(u128::from(msg_json.auction_amount_requested)*u128::from(self.contract_interest)/10000);
        env::log_str(&amount_to_auctioner.to_string());

        let new_auction = Auction{
            nft_contract:contract_id,
            nft_id:token_id,
            nft_owner:signer_id.clone() ,
            description:msg_json.description,
            auction_base_requested:msg_json.auction_amount_requested,
            auction_payback:msg_json.auction_amount_requested,
            status: AuctionStatus::Published,
            submission_time: env::block_timestamp(),
            auction_time:None,
            auction_deadline:None,
            bidder_id:None,
            
         };
        self.auctions_by_id.insert(&id, &new_auction);
       
        self.internal_add_auction_to_owner(&signer_id, &id);
        self.last_auction_id += 1;
        /*env::log_str(
            &json!(new_auction)
            .to_string(),
        );*/
        
        //If for some reason the contract failed it need to returns the NFT to the original owner (true)
        return PromiseOrValue::Value(false);
    }

    // Bid $NEAR Tokens to a Bid proposal
    #[payable]
    pub fn bid_for_nft(&mut self, auction_id: u128) -> Option<Auction> {
        //use a expect and explain that the auction wasnt found
        let mut auction:Auction = self.auctions_by_id.get(&auction_id.clone()).expect("the token doesn't have an active auction");    
        let new_bid =Bid{
            bidder_id:env::signer_account_id(),
            bid_amount:env::attached_deposit()};

        let signer_id =env::signer_account_id();
        let attached_deposit=env::attached_deposit();

        //Review that NFT is still available for auctioning
        assert_eq!(AuctionStatus::Published==auction.status || AuctionStatus::Bidded==auction.status ,true,"The NFT is not available for bidding");
        //Review that amount is the required
        assert_eq!(attached_deposit>=auction.auction_base_requested,true,"The amount payed is less than the base requested");

        assert_eq!(attached_deposit>auction.auction_payback,true,"The amount payed must be more than the payback");

        //Review that Bidder is not the same as NFT owner
        assert_ne!(signer_id.clone(),auction.nft_owner,"The owner cannot be the Bidder");

        //if exist a old bidder we must to refound the money bidder
        if auction.bidder_id.is_some() {
            let old_bidder_id = auction.bidder_id.clone().unwrap();
            let old_bidder_balance = auction.auction_payback.clone();
            Promise::new(old_bidder_id).transfer(old_bidder_balance); //before the owner recived the amount for treasury
            env::log_str("transfer to the old owner done");
         }
        
         // Update the auction with the new bidder
        auction.status=AuctionStatus::Bidded;
        auction.bidder_id = Some(signer_id.clone());
        auction.auction_payback=attached_deposit;
        auction.auction_time = Some(env::block_timestamp());
        //auction.auction_deadline = Some(env::block_timestamp()+60);
        auction.auction_deadline = Some(env::block_timestamp()+self.payment_period);

         

             
          
        
         self.auctions_by_id.insert(&auction_id, &auction);
         
         self.internal_add_auction_to_bidder(&signer_id, &auction_id);
         self.internal_add_bid_to_auction(auction_id, &new_bid);
        return Some(auction);
    }

    // #[payable]
    // pub fn pay_auction(&mut self, auction_id: u128) -> Option<auction> {
    //     let mut auction:auction = self.auctions_by_id.get(&auction_id).unwrap();
    //     let signer_id =env::signer_account_id();
    //     let attached_deposit=env::attached_deposit();
    //     let time_stamp=env::block_timestamp();


    //     //Review that NFT is still available for auctioning
    //     assert_eq!(auctionStatus::Bidded,auction.status,"The NFT is not auctioned");
    //     //Review that amount is the required
    //     //Here is pending of calculate the % of interest
    //     assert_eq!(attached_deposit,auction.auction_payback,"The amount payed is not equal as the requested");
    //     //Review that auctioner is not the same as NFT owner
    //     assert_eq!(signer_id,auction.nft_owner,"The payer should be the owner");
    //     //Review that auctioner is not the same as NFT owner
    //     env::log_str(&time_stamp.to_string());
    //     env::log_str(&auction.auction_deadline.unwrap().to_string());
    //     assert_eq!(time_stamp<=auction.auction_deadline.unwrap(),true,"The payment auction time has expired");

    //     //Here is pending of calculate the % of interest 
    //     Promise::new(auction.bidder_id.clone().unwrap()).transfer(u128::from(attached_deposit));
    //     // Inside a contract function on ContractA, a cross contract call is started
    //     // From ContractA to ContractB
    //     ext_contract_nft::nft_transfer(
    //     signer_id,
    //     auction.nft_id.clone().to_string(),
    //     "Withdraw of NFT from Nativo auctions".to_string(),
    //     auction.nft_contract.clone(), // contract account id
    //     1, // yocto NEAR to attach
    //     Gas::from(5_000_000_000_000) // gas to attach
    //     );

    //     auction.status=auctionStatus::Payed;
    //     self.auctions_by_id.insert(&auction_id, &auction);

    //     return Some(auction);
    // }

    // //Canceled public offer for auctioning
    // #[payable]
    // pub fn withdraw_nft_owner(&mut self, auction_id: u128){
    //     let mut auction:auction = self.auctions_by_id.get(&auction_id).unwrap();
    //     let signer_id =env::signer_account_id();
    //     let deposit = env::attached_deposit();

    //     //assert!(env::block_timestamp()<=auction.auction_time.unwrap()+self.payment_period&&auction.status==auctionStatus::auctioned,"The NFT is still pending of get auction payed");

    //     assert!(auction.status!=auctionStatus::Canceled,"The auction is canceled.");

    //     //Review that claimer is the same as NFT owner
    //     //assert_ne!(signer_id,auction.nft_owner,"You are not the owner of this NFT");

    //     if signer_id != auction.nft_owner.clone(){
    //         env::panic_str("You are not the owner of this NFT");
    //     }

    //     auction.status=auctionStatus::Canceled;
    //     self.auctions_by_id.insert(&auction_id, &auction);
    //     self.internal_remove_auction_from_owner(&signer_id, &auction_id);
    //     // env::log_str(
    //     //     &json!(&auction)
    //     //     .to_string(),
    //     // );

    //     // Inside a contract function on ContractA, a cross contract call is started
    //     // From ContractA to ContractB
    //     ext_contract_nft::nft_transfer(
    //     signer_id,
    //     auction.nft_id.to_string(),
    //     "Withdraw of NFT from Nativo auctions".to_string(),
    //     auction.nft_contract, // contract account id
    //     deposit, // yocto NEAR to attach
    //     Gas::from(5_000_000_000_000) // gas to attach
    //     );
    //     /*
    //     // When the cross contract call from A to B finishes the my_callback method is triggered.
    //     // Since my_callback is a callback, it will have access to the returned data from B
    //     .then(ext_self::my_callback(
    //     &env::current_account_id(), // this contract’s account id
    //     0, // yocto NEAR to attach to the callback
    //     5_000_000_000_000 // gas to attach to the callback
    //     ))*/
    // }   
    
    // //If time has passed and the NFT owner didn't pay
    // //The auctioner can claim the NFT and transfer to their wallet
    // #[payable]
    // pub fn withdraw_nft_auctioner(&mut self,auction_id:u128){
    //     let mut auction:auction = self.auctions_by_id.get(&auction_id).unwrap();
    //     let signer_id=env::signer_account_id();
    //     let time_stamp=env::block_timestamp();
    //     let deposit = env::attached_deposit();

    //     assert_eq!(time_stamp>=auction.auction_deadline.unwrap(),true,"The payment auction time has not expired");
        

    //     //assert!(auction.status!=auctionStatus::auctioned,"The NFT is under a auctioning process.");

    //     //Review that claimer is the same as NFT auctioner
    //     if signer_id != auction.bidder_id.clone().unwrap(){
    //         env::panic_str("You are not the auctioner of this NFT");
    //     }

    //     auction.status=auctionStatus::Expired;
    //     self.auctions_by_id.insert(&auction_id, &auction);
    //     self.internal_remove_auction_from_owner(&signer_id, &auction_id);
    //     self.internal_remove_auction_from_bidder(&signer_id, &auction_id);
    //     // env::log_str(
    //     //     &json!(&auction)
    //     //     .to_string(),
    //     // );

    //     // Inside a contract function on ContractA, a cross contract call is started
    //     // From ContractA to ContractB
    //     ext_contract_nft::nft_transfer(
    //     signer_id,
    //     auction.nft_id.to_string(),
    //     "Withdraw of NFT from Nativo auctions".to_string(),
    //     auction.nft_contract, // contract account id
    //     deposit, // yocto NEAR to attach
    //     Gas::from(5_000_000_000_000) // gas to attach
    //     );
    // }


    /**/

     //method to test the remote upgrade
     pub fn remote_done(&self) -> String {
        "Holaa remote now2 ".to_string()
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