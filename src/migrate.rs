use crate::*;
use near_sdk::{Gas};

/// Gas for upgrading this contract on promise creation + deploying new contract.
pub const TGAS: u64 = 10_000_000_000_000;
pub const GAS_FOR_UPGRADE_SELF_DEPLOY: Gas = Gas(300_000_000_000_000);
pub const GAS_FOR_UPGRADE_REMOTE_DEPLOY: Gas = Gas(300_000_000_000_000);


#[near_bindgen]
impl NFTAuctions {

    #[cfg(target_arch = "wasm32")]
    pub fn upgrade(self) {
        use near_sys as sys;
        // assert!(env::predecessor_account_id() == self.minter_account_id);
        //input is code:<Vec<u8> on REGISTER 0
        //log!("bytes.length {}", code.unwrap().len());
        const GAS_FOR_UPGRADE: u64 = 20 * TGAS; //gas occupied by this fn
       // const BLOCKCHAIN_INTERFACE_NOT_SET_ERR: &str = "Blockchain interface not set.";
        //after upgrade we call *pub fn migrate()* on the NEW CODE
        let current_id = env::current_account_id();
        let migrate_method_name = "migrate".as_bytes().to_vec();
        let attached_gas = env::prepaid_gas() - env::used_gas() - Gas(GAS_FOR_UPGRADE);
        unsafe {
            // Load input (new contract code) into register 0
            sys::input(0);

            //prepare self-call promise
            let promise_id =
                sys::promise_batch_create(current_id.as_bytes().len() as _, current_id.as_bytes().as_ptr() as _);

            //1st action, deploy/upgrade code (takes code from register 0)
            sys::promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);

            // 2nd action, schedule a call to "migrate()".
            // Will execute on the **new code**
            sys::promise_batch_action_function_call(
                promise_id,
                migrate_method_name.len() as _,
                migrate_method_name.as_ptr() as _,
                0 as _,
                0 as _,
                0 as _,
                u64::from(attached_gas),
            );
        }
    }

/////////////////////METODO DE MIGRACIÖN
 
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
         let old_state: PrevNFTAuctions = env::state_read().expect("failed");
        
        env::log_str("old state readed");
        Self {

              owner_account_id:      old_state.owner_account_id,
              treasury_account_id:   old_state.treasury_account_id ,
              last_auction_id:       old_state.last_auction_id ,
              contract_interest:     old_state.contract_interest,
              auctions_by_id:        old_state.auctions_by_id,
              auctions_per_owner:    old_state.auctions_per_owner,
              auctions_per_bidder:   old_state.auctions_per_bidder,
              bids_by_auction_id:    old_state.bids_by_auction_id,
              total_amount:          old_state.total_amount,
              payment_period:        old_state.payment_period, // 1_000_000_000 * 60 * 60 * 24 * 7  ,
              contract_fee:          old_state.contract_fee,
              is_minting_ntv:        old_state.is_minting_ntv, //true,
              ntv_multiply:          old_state.ntv_multiply, //3,
              auctions_active:       old_state.auctions_active, //0,
              auctions_amount_sold:  old_state.auctions_amount_sold, //0,
              auctions_current_ath:  old_state.auctions_current_ath, //0,
              ntv_token_contract:    old_state.ntv_token_contract,

        }
    }


   
    #[private]
    #[init(ignore_state)]
    pub fn cleanup()  -> Self {
        let old_state: PrevNFTAuctions = env::state_read().expect("failed");

        env::log_str("clean up state");
        Self {
       
            owner_account_id:      env::signer_account_id(),
            treasury_account_id:   env::signer_account_id(),
            last_auction_id:       0,
            contract_interest:     old_state.contract_interest,
            auctions_by_id:        UnorderedMap::new(StorageKey::AuctionsById.try_to_vec().unwrap()),
            auctions_per_owner:    LookupMap::new(StorageKey::AuctionsPerOwner.try_to_vec().unwrap()),
            auctions_per_bidder:   LookupMap::new(StorageKey::AuctionsPerBidder.try_to_vec().unwrap()),
            bids_by_auction_id:    UnorderedMap::new(StorageKey::BidsById.try_to_vec().unwrap()),
            total_amount:          0,
            payment_period:        1_000_000_000 * 60 * 60 * 24 * 7,
            contract_fee:          old_state.contract_fee, 
            is_minting_ntv:        true,
            ntv_multiply:          3,
            auctions_active:       0,
            auctions_amount_sold:  0,
            auctions_current_ath:  0,
            ntv_token_contract:    "nativo_token.testnet".to_string(),
        }
    }

}