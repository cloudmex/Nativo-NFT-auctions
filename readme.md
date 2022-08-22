# Nativo NFT - P2P auctions

![Logo](https://develop.testnet.nativonft.app/static/media/LogoBlanco.30fcfa22.png)

## NFT auctions allows you to have access to liquidity without loosing the ownership of your NFT's
### 1. Secure your NFT in NFT auctions and request an amount of tokens
### 2. People can bid for your NFT the amount of tokens starting from you base price.

## Prepare the ENV
### Last Dev
` export CONTRACT_ID="dev-1661196610363-39728493289974" `
### Sub account
` export CONTRACT_ID="v2.nativo-auctions.testnet"       `
### Initializing the contract
` near call $CONTRACT_ID new '{"owner_account_id": "dokxo.testnet","treasury_account_id": "dokxo.testnet","contract_interest": 800,"contract_fee": 200  }' --accountId dokxo.testnet       `



### to make a contract deploy run:
` ./build.sh  `

### to make a contract migration run:
` ./migrate.sh `


### View who is the owner account
` near view $CONTRACT_ID get_contract_owmer   `
### View who is the treasury account
` near view $CONTRACT_ID get_contract_treasury`
### View the contract interes
` near view $CONTRACT_ID get_contract_interest`
### View the contract payment period
` near view $CONTRACT_ID get_payment_period   `
### View the contract contract_fee
` near view $CONTRACT_ID get_contract_fee     `

### View is the ntv minting is enabled
`near view $CONTRACT_ID is_ntv_enable_minting   `
### View the number of auctions actives
`near view $CONTRACT_ID get_auctions_actives    `
### View the amount of auctions sold
`near view $CONTRACT_ID get_auctions_amount_sold`
### View the ATH amount of auctions sold
`near view $CONTRACT_ID get_auctions_current_ath`

### View all metrics
`near view $CONTRACT_ID get_auctions_stats      `
### Get all th account setted
`near view $CONTRACT_ID market_accounts         `

 
### Set  a new owner
`near call $CONTRACT_ID set_new_owner '{"new_owner":"dokxo.testnet"}' --accountId v1.nativo-auctions.testnet`
### Set  a new treasury
`near call $CONTRACT_ID set_new_treasury '{"new_treasury":"dokxo.testnet"}' --accountId dokxo.testnet`
### Set  a new contract interest
`near call $CONTRACT_ID set_new_contract_interest '{"new_contract_interest":100}' --accountId dokxo.testnet`
### Set  a new contract fee
`near call $CONTRACT_ID set_new_contract_fee '{"new_contract_fee":100}' --accountId dokxo.testnet `

### Set  a new payment period
`near call $CONTRACT_ID set_new_payment_period '{"new_payment_period":300}' --accountId dokxo.testnet `
### set  a new is nvt minting
`near call $CONTRACT_ID set_is_minting_ntv '{"is_enable":true}' --accountId dokxo.testnet`


### View all the auctions paginated published or Bidded
`near view $CONTRACT_ID get_nfts_for_auction '{"from_index":"0","limit":50}' `
### View all the auctions paginated 
`near view $CONTRACT_ID get_all_nfts_for_auction '{"from_index":"0","limit":50}'`
### View last auction
`near view $CONTRACT_ID get_last_auction`
### View bids by auction  
`near view $CONTRACT_ID get_nft_auction '{"auction_id":4}'`
### Bid NEARS in exchange of an NFT 
`near call $CONTRACT_ID bid_for_nft '{"auction_id":0}' --accountId darkdokxo.testnet --deposit 0.5`

### View the tokens paginated by Owner
`near view $CONTRACT_ID auctions_for_owner '{"account_id":"alexiaab.testnet","from_index":"0","limit":50}' `
### View the auctions supply  by Owner
`near view $CONTRACT_ID auction_supply_for_owner '{"account_id":"alexiaab.testnet","from_index":"0","limit":50}' `

### View the tokens paginated by Bidder
`near view $CONTRACT_ID auctions_for_bidder '{"account_id":"alexiaab.testnet","from_index":"0","limit":50}' `
### View the auctions supply  by Bidder
`near view $CONTRACT_ID auction_supply_for_bidder '{"account_id":"alexiaab.testnet","from_index":"0","limit":50}' `

### Cancel your auction and recover your NFT
`near call $CONTRACT_ID withdraw_nft_owner '{"auction_id":28}' --accountId dokxo.testnet --depositYocto 1 --gas 100000000000000`
### Cancel the last bid by bidder
`near call $CONTRACT_ID withdraw_nft_auctioner ‘{“auction_id”:15}’ --accountId joehank.testnet --depositYocto 1 --gas 100000000000000`

### Claim the NFT if you were the highest bidder
`near call $CONTRACT_ID claim_nft_winner '{"auction_id":0}' --accountId dokxo.testnet --depositYocto 1 --gas 100000000000000`


### Ask for a auctioning - Mintbase
`near call alst77.mintspace2.testnet nft_transfer_call '{"receiver_id": "dev-1648670267690-23487881027419","token_id":"0", "msg": "{\"auction_amount_requested\": 100000000000000000000000000 }"}' --accountId alan_test.testnet --depositYocto 1 --gas 300000000000000`
### get the NFT info from -Mintbase
`near view alst77.mintspace2.testnet  nft_token '{"token_id":"0"}' `

### Ask for a auctioning - Paras Id
`near call paras-token-v2.testnet nft_transfer_call '{"receiver_id": "dev-1647921766612-74437195022952","token_id": "299:9", "msg": "{\"auction_requested\": \"100000000000000000000000000\"}"}' --accountId alan_test.testnet --depositYocto 1  --gas 300000000000000`
### get the NFT info from Paras Id
`near view paras-token-v2.testnet nft_token '{"token_id":"299:9"}' `

### Ask for a auctioning - Nativo NFT
`near call minterv2.nativo-minter.testnet nft_transfer_call '{"receiver_id": "dev-1661196610363-39728493289974","token_id":"70", "msg": "{ \"auction_amount_requested\": \"100000000000000000000000\" }"}' --accountId dokxo.testnet --depositYocto 1 --gas 300000000000000`
### get the NFT info from Nativo NFT
`near view minterv2.nativo-minter.testnet nft_token '{"token_id":"70"}' `

### recover the token 
`near call minterv2.nativo-minter.testnet nft_transfer '{"receiver_id": "alexiaab.testnet", "token_id": "85", "memo": "Go Team :)"}' --accountId v1.nativo-auctions.testnet --depositYocto 1 `


### delete sub-account

` near delete v1.nativo-auctions.testnet nativo-auctions.testnet `

### create sub-account

` near create-account v2.nativo-auctions.testnet --masterAccount nativo-auctions.testnet `

 
 

