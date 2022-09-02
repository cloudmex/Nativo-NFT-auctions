  
use crate::*;


#[near_bindgen]
impl NFTAuctions {

   /**/
    // set a new owner
   
    pub fn set_new_owner(&mut self,new_owner:AccountId) -> String {
        self.is_the_owner();
        self.owner_account_id=new_owner;
        self.owner_account_id.to_string()
    }
    // set a new treasury
     pub fn set_new_treasury(&mut self,new_treasury:AccountId) -> String {
        self.is_the_owner();
        self.treasury_account_id=new_treasury;
        self.treasury_account_id.to_string()
    }

     // set a new contract interest

     pub fn set_new_contract_interest(&mut self,new_contract_interest:u64) -> String {
         self.is_the_owner();
         self.contract_interest=new_contract_interest;
         self.contract_interest.to_string()
     }

      // set a new contract interest
       pub fn set_new_payment_period(&mut self,new_payment_period:u64) -> String {
          self.is_the_owner();
          self.payment_period=new_payment_period;
          self.payment_period.to_string()
      }
        // set a new contract interest
       pub fn set_new_contract_fee(&mut self,new_contract_fee:u64) -> String {
          self.is_the_owner();
          assert_eq!(new_contract_fee > 1 && new_contract_fee <1000,true,"the comision have to be between 1 and 1000" );
          
          self.contract_fee=new_contract_fee;
          self.contract_fee.to_string()
      }

       pub fn set_is_minting_ntv(&mut self,is_enable:bool) -> String {
          self.is_the_owner();
          self.is_minting_ntv=is_enable;
          self.is_minting_ntv.to_string()
      }
      pub fn get_auctions_stats(& self) -> Metrics {
         let metrics = Metrics {
             
              total_auctions: self.last_auction_id,
             
              total_amount_deposited: self.total_amount.into(),
             
              ntv_status:self.is_minting_ntv,
             
              total_auctions_active: self.auctions_active,
             
              total_auctions_amount_sold: self.auctions_amount_sold.into(),
             
              max_auctions_ath: self.auctions_current_ath.into(),
        };
        metrics
    }
     //method to test the remote upgrade
     pub fn market_accounts(&self) -> AccountActive {
         

        AccountActive {
              owner:self.owner_account_id.to_string(),
              treasury:self.treasury_account_id.to_string(),
              nvt:self.ntv_token_contract.to_string(),    
        }
     }
 


     //validate if the owner is the caller
     #[private]
    pub fn is_the_owner(&self)   {
        //validate that only the owner contract add new contract address
        assert_eq!(
            self.owner_account_id==env::predecessor_account_id(),
            true,
            "!you are not the contract owner address¡"
        );
    }

     
   
}

 
