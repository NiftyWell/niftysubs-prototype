PEM="../wallets/walletKey.pem"
BLACKLIST="nifty-mint/interaction/blacklist.txt"
#PEM="../../../wallet/nifty-wallet.pem"
#CONTRACT_ADDRESS=erd1qqqqqqqqqqqqqpgqj55u3dxk84wyjw97h479dmvh7wx3hzmkzjvsggc0p2
CONTRACT_ADDRESS=erd1qqqqqqqqqqqqqpgq5v3d6r87gk0yr054ltqtfj4lufkydd9vfswsa79yq7
ADDRESS=$(erdpy data load --key=address)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction)

PROXY=https://devnet-gateway.elrond.com
API=https://devnet-api.elrond.com
CHAIN_ID=D

#PROXY=https://gateway.elrond.com
#API=https://api.elrond.com
#CHAIN_ID=1

# Token RWDTKN-0f72de in HEX 525744544b4e2d306637326465
# Token STAKENFTS-974742
# Token NFTBIT-6b732e
# NIFTY REX: NIFTYREX-d8c812: 4e494654595245582d643863383132
# NIFTY TALES: NFTTALES-636b0e: 4e465454414c45532d363336623065

SFT_COLL="0x5049454345532d396161623566"
#PIECES-9aab5f: 5049454345532d396161623566
###################
# DEPLOY CONTRACT
deploy(){
    erdpy --verbose contract deploy --project=niftysubs --pem=${PEM} --gas-limit=600000000 --proxy=${PROXY} --chain=${CHAIN_ID} --outfile="niftysubs.json" --recall-nonce --send
}

###################
# BUILD CONTRACT
build(){
    cd ~/dev/projects/niftyrex/contracts/services/niftysubs ; erdpy contract build ; cd ..
}

###################
# UPGRADE CONTRACT
upgrade(){
    erdpy --verbose contract upgrade ${CONTRACT_ADDRESS} --recall-nonce --bytecode=niftysubs/output/niftysubs.wasm --pem=${PEM} --gas-limit=600000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send
}

##########
# PAUSE MODULE
pause(){
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="pause" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

unpause(){
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="unpause" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

isPaused(){
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="isPaused"
}


##########
# GETTERS

getCutPercentage()
{
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getContractCutPercentage"
}

getServiceById(){
    local id=0x04
    res=$(erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getServiceById" --arguments $id)
    ./niftysubs/interaction/scripts/parser.py "--parseService" $res $id
}

getLastValidServiceId(){
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getLastValidServiceId"
}

getServicesByAddress(){
    local service_address=erd12ysqw48fcqw4qudlts75vdamkuctxwemad0gwx8dap72slj9fswsadgmd5
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getServicesByAddress" --arguments $service_address
}

getContractCutPercentage(){
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getContractCutPercentage"
}

getStatus(){    
    local address=erd12ysqw48fcqw4qudlts75vdamkuctxwemad0gwx8dap72slj9fswsadgmd5
    local service_id=6
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getStatus" \
    --arguments \
    $address \
    $service_id
}
getPassedPeriods(){
    local address=erd12ysqw48fcqw4qudlts75vdamkuctxwemad0gwx8dap72slj9fswsadgmd5
    local service_id=6
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getPassedPeriods" \
    --arguments \
    $address \
    $service_id
}
getToPay(){    
    local address=erd12ysqw48fcqw4qudlts75vdamkuctxwemad0gwx8dap72slj9fswsadgmd5
    local service_id=6
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getToPay" \
    --arguments \
    $address \
    $service_id
}

getSubscriptionsByAddress(){
    local address=erd12ysqw48fcqw4qudlts75vdamkuctxwemad0gwx8dap72slj9fswsadgmd5
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getSubscriptionsByAddress" \
    --arguments \
    $address 
}

getSubscriptionById(){
    local service_id=3
    local address=erd12ysqw48fcqw4qudlts75vdamkuctxwemad0gwx8dap72slj9fswsadgmd5
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getFullSubscriptionData" \
    --arguments \
    $address \
    $service_id
}

getSftStackOwner(){
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=600000000 --function="sftStack" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

getSftQuantities()
{
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="sftQuantities" --arguments 4
}
getSFTS(){
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getSFTS" --arguments 1
}

getParsedSFTS()
{
    local nonce=11
    res=$(erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getSFTS" --arguments $nonce)
    ./nifty-mint/scripts/parser.py "--parseSFT" $res
}

getMintPrice()
{
    res=$(erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getMintPrice")
    ./nifty-mint/scripts/parser.py "--parsePrice" $res 
}

getAvailable()
{
    res=$(erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getAvailable")
    ./nifty-mint/scripts/parser.py "--parseAvailable" $res 
}

getAvailable2(){
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getAvailable"
}
getMaxMintQuantity(){
     erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="maxMintQuantity" 

    
}
getMaxTotMintQuantity(){
     erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="maxTotMintQuantity" 

    
}
getMintLimit(){
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getMintLimit" 
}

getAllowType(){
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getAllowType" 

}

getMinters()
{
     erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getMinters" 
}

getBlacklist()
{
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getBlacklist"
}


getAddressMints()
{
    local address='erd130t5xdsvtd82uq0py64q7qcf0y7kym649crz22m6fjeen0m9zjvs0md87j'
     erdpy --verbose contract query ${CONTRACT_ADDRESS} --proxy=${PROXY} --function="getAddressMints" --arguments=$address 
}

##########
# SETTERS
Freeze(){
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="setStatus" \
    --arguments 0x00 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

Unfreeze(){
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="setStatus" \
    --arguments 0x01 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}


# MINTLIMIT SETTER
# None == 00
# MaxPerTx == 01
# MaxPerWallet == 02
# MaxBoth == 03
setCutPercentage(){
    local cut=500
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="setCutPercentage" \
    --arguments ${cut} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

createService(){
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="createService" \
    --arguments \
        0x45474c44 \
        0x \
        1000000000000000000 \
        1 \
        43200 \
        0x546573742073657276696365 \
        0x74657374736572766963652e636f6d \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

createServiceUSDC(){
    local TOKEN_TICKER=0x555344432d373964396134
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="createService" \
    --arguments \
        ${TOKEN_TICKER} \
        0x00 \
        10000000000000000000 \
        2 \
        1 \
        0x555344432053657276696365 \
        0x75736463736572766963652e636f6d \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

createServiceUSDC600(){
    local TOKEN_TICKER=0x555344432d373964396134
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="createService" \
    --arguments \
        ${TOKEN_TICKER} \
        0x00 \
        10000000000000000000 \
        1 \
        600 \
        0x555344432053657276696365 \
        0x75736463736572766963652e636f6d \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

createServiceUSDC300(){
    local TOKEN_TICKER=0x555344432d373964396134
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="createService" \
    --arguments \
        ${TOKEN_TICKER} \
        0x00 \
        5000000000000000000 \
        1 \
        300 \
        0x555344432035204d494e \
        0x55534443356d696e2e636f6d \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}
createServiceUSDC3min(){
    local TOKEN_TICKER=0x555344432d373964396134
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="createService" \
    --arguments \
        ${TOKEN_TICKER} \
        0x00 \
        5000000000000000000 \
        1 \
        180 \
        0x336d696e55534443 \
        0x336d696e555344432e636f6d \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}
createServiceUSDC1min(){
    local TOKEN_TICKER=0x555344432d373964396134
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="createService" \
    --arguments \
        ${TOKEN_TICKER} \
        0x00 \
        2000000000000000000 \
        1 \
        60 \
        0x316d696e55534443 \
        0x316d696e555344432e636f6d \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

claimFunds(){
    local service_id=6
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="claimFunds" \
    --arguments \
        $service_id \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

fundSubscription(){
    local ticker=0x555344432d373964396134 #USDC-79d9a4
    local service_id=6
    local quantity=2000000000000000000 #2
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=15000000 --function="ESDTTransfer" \
    --arguments \
        ${ticker}\
        ${quantity} \
        0x66756e64537562736372697074696f6e \
        ${service_id}\
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

retrieveFunds(){
    local service_id=6
    local quantity=0000000000000000000 #2
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=15000000 --function="retrieveFunds" \
    --arguments \
        ${service_id}\
        ${quantity} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

createServiceSFT(){
    local TOKEN_TICKER=0x5049454345532d396161623566
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="createService" \
    --arguments \
        ${TOKEN_TICKER} \
        0x01 \
        1 \
        2 \
        1 \
        0x5346542053657276696365 \
        0x736674736572766963652e636f6d \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

updateService(){
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="updateService" \
    --arguments \
        4 \
        0x45474c44 \
        0x \
        1000000000000000000 \
        2 \
        30 \
        0x74686973697361666169726c796c6f6e676e616d6574686174696d676f696e67746f757365746f7665726966797468656c656e6774686c696d697474 \
        0x74686973697361666169726c796c6f6e676e616d6574686174696d676f696e67746f757365746f7665726966797468656c656e6774686c696d6974742e636f6d \
        0 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setCustomCut(){
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="setCustomCut" \
    --arguments \
        4 \
        0x01 \
        0x00 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}   
}

# ALLOWTYPE SETTER
# None == 00
# Blacklist == 01
# Whitelist == 02
# LimitBoth == 03
setAllowType(){
    local limit_type=0x01
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="setAllowType" \
    --arguments ${limit_type} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

addBlacklist(){
    new_blacklist=()
    while IFS= read -r line || [[ "$line" ]]; do
        new_blacklist+=("$line")
    done < ${BLACKLIST}
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=600000000 --function="addBlacklist" \
    --arguments erd130t5xdsvtd82uq0py64q7qcf0y7kym649crz22m6fjeen0m9zjvs0md87j\
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
    #--arguments ${new_blacklist[*]} \
}


clearBlacklist(){
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=60000000 --function="clearBlacklist" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
    
}

clearMinters(){
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=60000000 --function="clearMinters" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

clearAddressMints(){
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=60000000 --function="clearAddressMints" \
    --arguments erd130t5xdsvtd82uq0py64q7qcf0y7kym649crz22m6fjeen0m9zjvs0md87j \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setMaxMintQuantity(){
    local max_quantity=5
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="setMaxMintQuantity" \
    --arguments ${max_quantity}\
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}


setMaxTotMintQuantity(){
    local max_quantity=5
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="setMaxTotMintQuantity" \
    --arguments ${max_quantity}\
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}


registerMintToken(){
    erdpy --verbose contract call ${CONTRACxT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="registerMintToken" \
    --arguments ${TOKEN_TICKER} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}
setMintPrice(){
    local price=1000000000000000000
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="setMintPrice" \
    --arguments ${price} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

claimEsdt(){
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=10000000 --function="claimEsdt" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}


loadSFTS(){
    local NFT_COLLECTION=${SFT_COLL}
    erdpy contract call erd130t5xdsvtd82uq0py64q7qcf0y7kym649crz22m6fjeen0m9zjvs0md87j --recall-nonce --pem=${PEM} \
    --gas-limit=15000000 --function="MultiESDTNFTTransfer" \
    --arguments \
        ${CONTRACT_ADDRESS} \
        5 \
        ${NFT_COLLECTION} \
        7 \
        400 \
        ${NFT_COLLECTION} \
        8 \
        400 \
        ${NFT_COLLECTION} \
        9 \
        400 \
        ${NFT_COLLECTION} \
        10 \
        400 \
        ${NFT_COLLECTION} \
        11 \
        400 \
        0x6c6f616453465453 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

ONE_loadSFTS(){
    local NFT_COLLECTION=${SFT_COLL}
    erdpy contract call erd130t5xdsvtd82uq0py64q7qcf0y7kym649crz22m6fjeen0m9zjvs0md87j --recall-nonce --pem=${PEM} \
    --gas-limit=15000000 --function="MultiESDTNFTTransfer" \
    --arguments \
        ${CONTRACT_ADDRESS} \
        1 \
        ${NFT_COLLECTION} \
        1 \
        100 \
        0x6c6f616453465453 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

resetSFTS(){
    local NFT_COLLECTION=${SFT_COLL}
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=15000000 --function="restSFTS" \
    --arguments \
        1 \
        2 \
        3 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

unloadSFTS(){
    local NFT_COLLECTION=${SFT_COLL}
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=15000000 --function="unloadSFTS" \
    --arguments \
        ${NFT_COLLECTION} \
        1 \
        6 \
        ${NFT_COLLECTION} \
        2 \
        4 \
        ${NFT_COLLECTION} \
        3 \
        5 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

ONE_unloadSFTS(){
    local NFT_COLLECTION=${SFT_COLL}
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=15000000 --function="unloadSFTS" \
    --arguments \
        ${NFT_COLLECTION} \
        1 \
        49 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

shuffleStack(){ 
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=600000000 --function="shuffleStack" \
    --arguments \
        7 \
        11 \
        200 \
        200 \
        200 \
        200 \
        200 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

shuffleStackStep(){ 
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=600000000 --function="shuffleStackStep" \
    --arguments \
        7 \
        11 \
        200 \
        200 \
        200 \
        200 \
        200 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}


clearStack(){ 
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=600000000 --function="clearStack" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

clearStackSteps(){ 
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=600000000 --function="clearStackSteps" \
    --arguments \
        1000 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

Subscribe(){
    #Subscribe to usdc
    local ticker=0x555344432d373964396134 #USDC-79d9a4
    local service_id=6
    local quantity=2000000000000000000 #2 usdc
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=15000000 --function="ESDTTransfer" \
    --arguments \
        ${ticker}\
        ${quantity} \
        0x737562736372696265 \
        ${service_id}\
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

Unsubscribe(){
    #Subscribe to usdc
    local service_id=2
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=15000000 --function="unsubscribe" \
    --arguments \
        ${service_id}\
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

#######
# MINT

Mint(){
    # mint 3 SFTs
    local quantity=1000000000000000000
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=15000000 --function="ESDTTransfer" \
    --arguments \
        ${TOKEN_TICKER}\
        ${quantity} \
        0x6d696e74 \
        1 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

MintWrongQuant(){
    # mint 20 SFTs
    local quantity=16000000000000000000
    erdpy contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${PEM} \
    --gas-limit=50000000 --function="ESDTTransfer" \
    --arguments \
        ${TOKEN_TICKER}\
        ${quantity} \
        0x6d696e74 \
        16 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}