use crate::*;

//use std::mem::size_of;

pub type AuctionId = u128;
pub type Bidst = Bid;

pub type TimestampSec = u32;
pub type Timestamp = u64;


pub const WEEK_BLOCK_TIMESTAMP_IN_SECS: u64 =604800;
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
     
    pub auction_amount_requested: SalePriceInYoctoNear,
   
}


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    //token ID
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    pub metadata: TokenMetadata,
    //creator of the token
    pub creator_id: AccountId,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
    //keep track of the royalty percentages for the token in a hash map
    pub royalty: HashMap<AccountId, u32>,
}
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}
