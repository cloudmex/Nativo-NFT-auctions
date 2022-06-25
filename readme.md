# Nativo NFT - P2P auctions

![Logo](https://v2.nativonft.app/static/media/nativologocrop.15afa4d2.png)

NFT auctions allows you to have access to liquidity without loosing the ownership of your NFT's
1. Secure your NFT in NFT auctions and request an amount of tokens
2. People auction you the amount of tokens you expect to receive
3. You have the option to payback the tokens + interest or to give NFT to the auctioner

### Initializing the contract
export CONTRACT_ID="dev-1656099423166-23523101345922"  Dev account
export CONTRACT_ID="v1.nativo-auctions.testnet"        Sub account

near call $CONTRACT_ID new '{"owner_account_id": "v1.nativo-auctions.testnet","treasury_account_id": "v1.nativo-auctions.testnet","contract_interest": 800,"contract_fee": 200  }' --accountId dokxo.testnet 

### to make a contract deploy run:
./build.sh

### to make a contract migration run:
./migrate.sh
### Upgrade command by owner
near deploy --wasmFile res/nft_aucions.wasm --initFunction "migrate"  --initArgs "{}"  --accountId $CONTRACT


### create a subaccount
near create-account v1.nativo-auctions.testnet --masterAccount nativo-auctions.testnet
### delete a subaccount
near delete v1.nativo-auctions.testnet nativo-auctions.testnet

### Viewing all the auctions paginated published or Bidded
near view $CONTRACT_ID get_nfts_for_auction '{"from_index":"0","limit":50}'

### Viewing all the auctions paginated 
near view $CONTRACT_ID get_all_nfts_for_auction '{"from_index":"0","limit":50}'
### View last auction
near view $CONTRACT_ID get_last_auction

### View bis by auction  
near view $CONTRACT_ID get_bid_auction '{"auction_id":0}'

### auction NEARS in exchange of an NFT or APY
near call $CONTRACT_ID bid_for_nft '{"auction_id":2}' --accountId dokxo.testnet --deposit 0.1

### Pay a auction you received + interes rate (8%)
near call $CONTRACT_ID pay_auction '{"auction_id":1}' --accountId joehank.testnet --deposit 100

### Cancel your auction and recover your NFT
near call $CONTRACT_ID withdraw_nft_owner '{"auction_id":2}' --accountId dokxo_test.testnet --depositYocto 1 --gas 100000000000000

### If the time to pay the auction has already expired, the lender can claim the token
near call $CONTRACT_ID withdraw_nft_auctioner ‘{“auction_id”:15}’ --accountId joehank.testnet --depositYocto 1 --gas 100000000000000


near call $CONTRACT_ID claim_nft_winner '{"auction_id":1}' --accountId dokxo_test.testnet --depositYocto 1 --gas 100000000000000


### Ask for a auctioning - Mintbase
near call alst77.mintspace2.testnet nft_transfer_call '{"receiver_id": "dev-1648670267690-23487881027419","token_id":"0", "msg": "{\"description\": \"list a new nft for auctioning\", \"auction_amount_requested\": 100000000000000000000000000 }"}' --accountId alan_test.testnet --depositYocto 1 --gas 300000000000000


near call alst77.mintspace2.testnet nft_transfer '{"receiver_id": "alan_test.testnet","token_id":"0","msg":""}' --accountId $CONTRACT_ID --depositYocto 1 
near view alst77.mintspace2.testnet  nft_token '{"token_id":"0"}' 

### Ask for a auctioning - Paras Id
near call paras-token-v2.testnet nft_transfer_call '{"receiver_id": "dev-1647921766612-74437195022952","token_id": "299:9", "msg": "{\"description\": \"list my nft for auctioning\", \"auction_requested\": \"100000000000000000000000000\"}"}' --accountId alan_test.testnet --depositYocto 1  --gas 300000000000000

near view paras-token-v2.testnet nft_token '{"token_id":"299:9"}' 

### Ask for a auctioning - Nativo NFT
near call minterv2.nativo-minter.testnet nft_transfer_call '{"receiver_id": "v1.nativo-auctions.testnet","token_id":"48", "msg": "{\"description\": \"list a new nft for Auction\", \"auction_amount_requested\": \"100000000000000000000000\" }"}' --accountId dokxo_test.testnet --depositYocto 1 --gas 100000000000000



### method for test the upgrade

near view $CONTRACT_ID remote_done




near view minterv2.nativo-minter.testnet nft_token '{"token_id":"48"}' 
