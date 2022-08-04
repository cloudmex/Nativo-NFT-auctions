# Nativo NFT - P2P auctions

![Logo](https://v2.nativonft.app/static/media/nativologocrop.15afa4d2.png)

NFT auctions allows you to have access to liquidity without loosing the ownership of your NFT's
1. Secure your NFT in NFT auctions and request an amount of tokens
2. People auction you the amount of tokens you expect to receive
3. You have the option to payback the tokens + interest or to give NFT to the auctioner

### Initializing the contract
export CONTRACT_ID="dev-1656099423166-23523101345922"  Dev account
  // Sub account
export CONTRACT_ID="v1.nativo-auctions.testnet"     

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

### View who is the owner
near view $CONTRACT_ID get_contract_owmer
### View who is the treasury
near view $CONTRACT_ID get_contract_treasury
### View the contract interes
near view $CONTRACT_ID get_contract_interest
### View the contract payment period
near view $CONTRACT_ID get_payment_period
### View the contract contract_fee
near view $CONTRACT_ID get_contract_fee

### View is the ntv minting is enabled
near view $CONTRACT_ID is_ntv_enable_minting
### View the number of auctions actives
near view $CONTRACT_ID get_auctions_actives
### View the amount of auctions sold
near view $CONTRACT_ID get_auctions_amount_sold
### View the ATH amount of auctions sold
near view $CONTRACT_ID get_auctions_current_ath

### get all metrics
near view $CONTRACT_ID get_auctions_stats
### get all th account setted

near view $CONTRACT_ID market_accounts

 

 
### set  a new owner
near call $CONTRACT_ID set_new_owner '{"new_owner":"dokxo.testnet"}' --accountId v1.nativo-auctions.testnet

### set  a new treasury
near call $CONTRACT_ID set_new_treasury '{"new_treasury":"dokxo.testnet"}' --accountId dokxo.testnet
### set  a new contract interest
near call $CONTRACT_ID set_new_contract_interest '{"new_contract_interest":100}' --accountId dokxo.testnet
### set  a new payment period
near call $CONTRACT_ID set_new_payment_period '{"new_payment_period":86400}' --accountId v1.nativo-auctions.testnet

### set  a new contract fee
near call $CONTRACT_ID set_new_contract_fee '{"new_contract_fee":100}' --accountId dokxo.testnet

### set  a new is nvt minting
near call $CONTRACT_ID set_is_minting_ntv '{"is_enable":true}' --accountId dokxo.testnet




### Viewing all the auctions paginated published or Bidded
near view $CONTRACT_ID get_nfts_for_auction '{"from_index":"0","limit":50}'

### Viewing all the auctions paginated 
near view $CONTRACT_ID get_all_nfts_for_auction '{"from_index":"0","limit":50}'
### View last auction
near view $CONTRACT_ID get_last_auction

### View bids by auction  
near view $CONTRACT_ID get_nft_auction '{"auction_id":4}'

### auction NEARS in exchange of an NFT or APY
near call $CONTRACT_ID bid_for_nft '{"auction_id":17}' --accountId dokxo.testnet --deposit 2.1

 

### Cancel your auction and recover your NFT
near call $CONTRACT_ID withdraw_nft_owner '{"auction_id":8}' --accountId dokxo_test.testnet --depositYocto 1 --gas 100000000000000

### If the time to pay the auction has already expired, the lender can claim the token
near call $CONTRACT_ID withdraw_nft_auctioner ‘{“auction_id”:15}’ --accountId joehank.testnet --depositYocto 1 --gas 100000000000000


### if the time is out  only the bidder can claim his new NFT
near call $CONTRACT_ID claim_nft_winner '{"auction_id":6}' --accountId dokxo.testnet --depositYocto 1 --gas 100000000000000


### Ask for a auctioning - Mintbase
near call alst77.mintspace2.testnet nft_transfer_call '{"receiver_id": "dev-1648670267690-23487881027419","token_id":"0", "msg": "{\"description\": \"list a new nft for auctioning\", \"auction_amount_requested\": 100000000000000000000000000 }"}' --accountId alan_test.testnet --depositYocto 1 --gas 300000000000000


near call alst77.mintspace2.testnet nft_transfer '{"receiver_id": "alan_test.testnet","token_id":"0","msg":""}' --accountId $CONTRACT_ID --depositYocto 1 
near view alst77.mintspace2.testnet  nft_token '{"token_id":"0"}' 

### Ask for a auctioning - Paras Id
near call paras-token-v2.testnet nft_transfer_call '{"receiver_id": "dev-1647921766612-74437195022952","token_id": "299:9", "msg": "{\"description\": \"list my nft for auctioning\", \"auction_requested\": \"100000000000000000000000000\"}"}' --accountId alan_test.testnet --depositYocto 1  --gas 300000000000000

near view paras-token-v2.testnet nft_token '{"token_id":"299:9"}' 

### Ask for a auctioning - Nativo NFT
near call minterv2.nativo-minter.testnet nft_transfer_call '{"receiver_id": "v1.nativo-auctions.testnet","token_id":"60", "msg": "{\"description\": \"list a new nft for Auction\",\"media\": \"bafybeid3pwnszy3popscmhitsf4q2rmly7ivr5kz7wvkj7hyc3yxahaa7a\", \"auction_amount_requested\": \"100000000000000000000000\" }"}' --accountId dokxo_test.testnet --depositYocto 1 --gas 100000000000000



 
 




near call factory.shardnet.near create_staking_pool '{"staking_pool_id": "dokxo-stakewar", "owner_id": "dokxo.shardnet.near", "stake_public_key": "ed25519:Hxg9QgYsM9ZmpZd7gCNWtBiD9kAdpmZiTPjz6UceH31V", "reward_fee_fraction": {"numerator": 5, "denominator": 100}, "code_hash":"DD428g9eqLL8fWUxv8QSpVFzyHi1Qd16P8ephYCTmMSZ"}' --accountId="dokxo.shardnet.near" --amount=30 --gas=300000000000000

near call dokxo-stakewar.factory.shardnet.near update_reward_fee_fraction '{"reward_fee_fraction": {"numerator": 1, "denominator": 100}}' --accountId dokxo.shardnet.near --gas=300000000000000


near call dokxo-stakewar.factory.shardnet.near deposit_and_stake --amount 200 --accountId dokxo.shardnet.near --gas=300000000000000

near call dokxo-stakewar.factory.shardnet.near unstake '{"amount": "1000000000000000000000000"}' --accountId dokxo.shardnet.near --gas=300000000000000

near call dokxonewpool.factory.shardnet.near unstake_all --accountId dokxo.shardnet.near  --gas=300000000000000

near call dokxo-stakewar.factory.shardnet.near ping '{}' --accountId dokxo.shardnet.near --gas=300000000000000


near view dokxo-stakewar.factory.shardnet.near get_account_total_balance '{"account_id": "dokxo.shardnet.near"}'

near view dokxo-stakewar.factory.shardnet.near get_account_unstaked_balance '{"account_id": "dokxo.shardnet.near"}'


near view dokxo-stakewar.factory.shardnet.near get_accounts '{"from_index": 0, "limit": 10}' --accountId dokxo.shardnet.near

near call dokxo-stakewar.factory.shardnet.near pause_staking '{}' --accountId dokxo.shardnet.near

near call dokxo-stakewar.factory.shardnet.near resume_staking '{}' --accountId dokxo.shardnet.near


curl -s -d '{"jsonrpc": "2.0", "method": "validators", "id": "dontcare", "params": [null]}' -H 'Content-Type: application/json' 127.0.0.1:3030 | jq -c '.result.current_validators[] | select(.account_id | contains ("dokxonewpool"))'



#!/bin/sh
# Ping call to renew Proposal added to crontab

export NEAR_ENV=shardnet
export LOGS=/home/danielmora/logs
export POOLID=dokxonewpool
export ACCOUNTID=dokxo

echo "---" >> $LOGS/all.log
date >> $LOGS/all.log
near call $POOLID.factory.shardnet.near ping '{}' --accountId $ACCOUNTID.shardnet.near --gas=300000000000000 >> $LOGS/all.log
near proposals | grep $POOLID >> $LOGS/all.log
near validators current | grep $POOLID >> $LOGS/all.log
near validators next | grep $POOLID >> $LOGS/all.log




CONTRACT_ID=dokxo.shardnet.near

# Change numerator and denomitor to adjust the % for split.
NEAR_ENV=shardnet near call $CONTRACT_ID new '{"staking_pool_account_id": "dokxo-stakewar.factory.shardnet.near", "owner_id":"dokxo.shardnet.near", "reward_receivers": [["dokxo.shardnet.near", {"numerator": 3, "denominator":10}], ["dokxo.shardnet.near", {"numerator": 70, "denominator":100}]]}' --accountId $CONTRACT_ID


CONTRACT_ID=dokxo.shardnet.near

NEAR_ENV=shardnet near call $CONTRACT_ID withdraw '{}' --accountId $CONTRACT_ID --gas 200000000000000
