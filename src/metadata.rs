use crate::*;

//use std::mem::size_of;

pub type AuctionId = u128;
pub type Bidst = Bid;


/// Status of a auction.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum AuctionStatus {
    //First status when no body had auctioned for this NFT
    Published,
    //auction has been started
    Running,
    //auction has been paused by the owner
    Suspended,
    //auction has been bidded by someone but  is not finished yet
    Bidded,
    //auction has been finished
    Finished,
    /// Expired after period of time. auctioner  can claim the NFT.
    Expired,
    /// If NFT owner payed back for the auction
    Payed,
    // If no body auctioned for this NFT. This status gets after owners claim back its NFT.
    Canceled,
    //if the auction its ended and has a bid 
    Claimed,
    New,
    NotFound,
}

/// Proposal for auctioning that are sent to this DAO.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct Auction {
    /// Original nft owner.
    pub nft_owner: AccountId,
    /// Original nft contract.
    pub nft_contract: AccountId,
    /// NFT id in origin contract.
    pub nft_id: String,
    /// NFT media in origin contract.
    pub nft_media: Option<String>,
    /// Description of this auction.
    pub description: Option<String>,
    /// auction amount requested
    pub auction_base_requested: SalePriceInYoctoNear,
    /// auction amount that have to be payback to the nft owner
    pub auction_payback: SalePriceInYoctoNear,
    /// Current status of the auction
    pub status: AuctionStatus,
    /// Submission time
    pub submission_time: EpochHeight,
    /// When somebody auctioned.
    pub auction_time: Option<EpochHeight>,
    /// When will the bidding end and the bidder can withdraw the NFT
    /// Also is the deadline when NFT owner can payback
    pub auction_deadline: Option<EpochHeight>,
    pub bidder_id: Option<AccountId>,

 }

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    /// Id of the auction.
    pub bidder_id: AccountId,

    pub bid_amount: SalePriceInYoctoNear,
}



#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Metrics {
    //Index for auctions
    pub total_auctions: u64,
    //how much auctions are running
    pub total_auctions_active: u128,
    /// Total token amount deposited.
    pub total_amount_deposited: SalePriceInYoctoNear,
    // a flag to start/stop the ntv minting
    pub ntv_status:bool,
   
    //how much money has made by auctions
    pub total_auctions_amount_sold: SalePriceInYoctoNear,
    //how much ATH has made by auctions
    pub max_auctions_ath: SalePriceInYoctoNear,
}
/// This is format of output via JSON for the auction.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AuctionOutput {
    /// Id of the auction.
    pub id: AuctionId,
    #[serde(flatten)]
    pub auction: Auction,
}
/// This is format of output via JSON for the auction message.
#[derive( Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MsgInput {
    pub description: Option<String>,
    pub auction_amount_requested: SalePriceInYoctoNear,
    pub media: Option<String>,
}


