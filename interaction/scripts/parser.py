#!/usr/bin/env python3.8
import sys
from erdpy.accounts import Account, Address
import json
import copy

def userToBech32(res):
    res = res.replace('"', '')
    addr = Address(res)
    addr = addr.bech32()
    space = " "
    print(f'{space}{addr}')


def serviceToText(res):
    #000000105354414b454e4654532d39373437343200000000000000058bd743360c5b4eae01e126aa0f0309793d626f552e06252b7a4cb399bf651499000000000000000000000000000005350000000000000535
    """
    51200754e9c01d5071bf5c3d4637bbb730b33b3beb5e8718ede87ca87e454c1d #Address
    00000004 #ticker length
    45474c44 #ticker
    0000000000000000#nonce
    00000008 #Price length
    0de0b6b3a7640000 #Price
    01 #Payment period type
    000000000000a8c0 #payment period 8bytes
    0000000000000002 #contract cut length
    01f4 #contract cut
    0000000c #service name length
    546573742073657276696365 #service name
    0000000f #service page length
    74657374736572766963652e636f6d #service page
    """
    res = res.replace('"', '')

    # variable sizes:
    address_length = 64
    ticker_length = 4*2
    nonce_size = 8*2
    price_length = 4*2
    payment_period = 8*2
    contract_cut_length = 8*2
    service_name_length = 4*2
    service_page_length = 4*2
    
    ticker_size = int('0x'+res[address_length:address_length+ticker_length], 16)*2
    up_to_size = address_length+ticker_length
    ticker_hex = res[up_to_size:up_to_size+ticker_size] # hex
    up_to_ticker = up_to_size+ticker_size

    nonce_hex = '0x'+res[up_to_ticker:up_to_ticker+nonce_size] # hex
    up_to_nonce = up_to_ticker+nonce_size

    price_size = int('0x'+res[up_to_nonce:up_to_nonce+price_length], 16)*2
    up_to_size = up_to_nonce+price_length
    price_hex = '0x'+res[up_to_size:up_to_size+price_size] # hex
    up_to_price = up_to_size+price_size
    period_type = int('0x'+res[up_to_price:up_to_price+2], 16)
    if period_type == 1:
        period_type = "Seconds"
    elif period_type == 2:
        period_type = "Epochs"
    up_to_type = up_to_price+2

    payment_period_val = str(int('0x'+res[up_to_type:up_to_type+payment_period], 16)) # hex
    up_to_payment_period = up_to_type+payment_period

    contract_cut_size = int('0x'+res[up_to_payment_period:up_to_payment_period+contract_cut_length], 16)*2
    contract_cut_hex = '0x'+res[up_to_payment_period+contract_cut_length:up_to_payment_period+contract_cut_length+contract_cut_size] # hex
    up_to_cut = up_to_payment_period+contract_cut_length+contract_cut_size

    service_name_size = int('0x'+res[up_to_cut:up_to_cut+service_name_length], 16)*2
    service_name_hex = res[up_to_cut+service_name_length:up_to_cut+service_name_length+service_name_size] # hex
    up_to_service_name = up_to_cut+service_name_length+service_name_size

    service_page_size = int('0x'+res[up_to_service_name:up_to_service_name+service_page_length], 16)*2
    service_page_hex = res[up_to_service_name+service_page_length:up_to_service_name+service_page_length+service_page_size] # hex
    ticker = bytearray.fromhex(ticker_hex).decode()
    nonce = str(int(nonce_hex, 16))
    price = str(int(price_hex, 16))

    payment_period = payment_period_val
    contract_cut = str(int(contract_cut_hex, 16))
    service_name = bytearray.fromhex(service_name_hex).decode()
    service_page = bytearray.fromhex(service_page_hex).decode()


    if len(nonce)%2!=0:
        nonce = '0'+nonce

    space = " "
    address = Address(res[:address_length])
    address = address.bech32()
    payment_token_print = ''
    if 'EGLD' in ticker:
        payment_token_print = f'{ticker}'
    else:
        payment_token_print = f'{ticker}-{nonce}'
    print(f"{space}----------")
    print(f'{space}Service owner: {address}')
    print(f'{space}Payment token: {payment_token_print}')
    print(f'{space}Service price: {float(price)*10**-18} token')
    print(f'{space}Service period type: {period_type}')
    print(f'{space}Service period: {payment_period}')
    print(f'{space}Service contract cut: {contract_cut}')
    print(f'{space}Service name: {service_name}')
    print(f'{space}Service page: {service_page}')
    print(f"{space}----------")
    print()

def nftToText(res):
    #000000105354414b454e4654532d39373437343200000000000000058bd743360c5b4eae01e126aa0f0309793d626f552e06252b7a4cb399bf651499000000000000000000000000000005350000000000000535
    res = res.replace('"', '')
    ticker_length = 8
    ticker = 32
    nonce = 16
    pool_id = 16
    epoch = 16
    last_claim = 16

    ticker = int('0x'+res[0:ticker_length], 16)*2  #*2 because we have bytes in the result which are 2 by 2s (0d, 50, etc)
    ticker_hex = res[ticker_length:ticker_length+ticker]
    nonce_hex = '0x'+res[ticker_length+ticker:ticker_length+ticker+nonce]
    pool_id_hex = '0x'+res[ticker_length+ticker+nonce:ticker_length+ticker+nonce+pool_id]
    epoch_hex = '0x'+res[ticker_length+ticker+nonce+pool_id:ticker_length+ticker+nonce+pool_id+epoch]
    last_claim_hex = '0x'+res[ticker_length+ticker+nonce+pool_id+epoch:ticker_length+ticker+nonce+pool_id+epoch+last_claim]

    ticker = bytearray.fromhex(ticker_hex).decode()
    nonce = str(int(nonce_hex, 16))
    if len(nonce)%2!=0:
        nonce = '0'+nonce

    pool_id = int(pool_id_hex, 16)
    epoch = int(epoch_hex, 16)
    last_claim = int(last_claim_hex, 16)

    space = " "
    print()
    print(f"{space}----------")
    print(f'{space}NFT: {ticker}-{nonce}')
    print(f'{space}pool id: {pool_id}')
    print(f'{space}epoch of stake: {epoch}')
    print(f'{space}timestamp of last claim: {last_claim}')
    print(f"{space}----------")
    print()

def getNFTs(res):
    for nft in res:
        nftToText(nft["hex"])
def getUnstakedNFTs(res):
    for nft in res:
        unstakedToText(nft["hex"])
def getUsers(res):
    for user in res:
        userToBech32(user["hex"])

def sftToText(res):
    res = res.replace('"', '')
    ticker_length = 8
    nonce = 16
    quantity_length = 8

    ticker = int('0x'+res[0:ticker_length], 16)*2 #*2 because we have bytes in the result which are 2 by 2s (0d, 50, etc)
    
    ticker_hex = res[ticker_length:ticker_length+ticker]
    nonce_hex = '0x'+res[ticker_length+ticker:ticker_length+ticker+nonce]
    
    quantity = int('0x'+res[ticker_length+ticker+nonce:ticker_length+ticker+nonce+quantity_length], 16)*2 # because we need bytes in index positions aka 2 for 1
    quantity_hex = '0x'+res[ticker_length+ticker+nonce+quantity_length:ticker_length+ticker+nonce+quantity_length+quantity]
    ticker_text = bytearray.fromhex(ticker_hex).decode()
    nonce = int(nonce_hex, 16)
    quantity = int(quantity_hex, 16)

    space = " "
    print(f'{space}ticker: {ticker_text}')
    print(f'{space}nonce: {nonce}')
    print(f'{space}quantity: {quantity}')

    
def poolToText(res):
    res = res.replace('"', '')
    pool_id = 16
    ticker_length = 8
    ticker = 32
    first_nonce = 16
    second_nonce = 16
    reward_buffer = 8
    reward = 16
    unbonding_time = 16 

    pool_id_hex = '0x'+res[0:pool_id]
    ticker = int('0x'+res[pool_id:pool_id+ticker_length], 16)*2
    reward=int('0x'+res[pool_id+ticker_length+ticker+first_nonce+second_nonce:pool_id+ticker_length+ticker+first_nonce+second_nonce+reward_buffer], 16)*2
    ticker_hex = res[pool_id+ticker_length:pool_id+ticker_length+ticker]
    first_nonce_hex = '0x'+res[pool_id+ticker_length+ticker:pool_id+ticker_length+ticker+first_nonce]
    second_nonce_hex = '0x'+res[pool_id+ticker_length+ticker+first_nonce:pool_id+ticker_length+ticker+first_nonce+second_nonce]
    reward_hex = '0x'+res[pool_id+ticker_length+ticker+first_nonce+second_nonce+reward_buffer:pool_id+ticker_length+ticker+first_nonce+second_nonce+reward_buffer+reward]
    unbonding_time_hex = '0x'+res[pool_id+ticker_length+ticker+first_nonce+second_nonce+reward_buffer+reward: pool_id+ticker_length+ticker+first_nonce+second_nonce+reward_buffer+reward+unbonding_time]

    pool_id = int(pool_id_hex, 16)
    ticker = bytearray.fromhex(ticker_hex).decode()
    first_nonce = int(first_nonce_hex, 16)
    second_nonce = int(second_nonce_hex, 16)
    reward = int(reward_hex, 16)
    unbonding_time = int(unbonding_time_hex, 16)
    space = " "
    print(f'{space}pool_id: {pool_id}')
    print(f'{space}ticker: {ticker}')
    print(f'{space}nonces: [{first_nonce},{second_nonce}]')
    print(f'{space}rewards: {reward*10**(-18)}')
    print(f'{space}unbonding time: {unbonding_time}')


def main():
    if len(sys.argv) == 1 :
        print("Error : Missing argument ")
        return -1
    if sys.argv[1] == "--parsePoolStaked":
        if '"hex":' in sys.argv :
            res = sys.argv[sys.argv.index('"hex":')+1]
            res_hex = '0x'+str(res.replace('"', "").replace(",",""))
            staked=int(res_hex,16)
            print(" ----------")
            print(f" NFTs Staked: {staked}")
            print()
        else:
            print(" ----------")
            print(" Pool is empty")
            print()
    if sys.argv[1] == "--parseService":
        if '"hex":' in sys.argv :
            id = sys.argv[len(sys.argv)-1:][0]
            id = int(id, 16)
            print(f"[====[ Service #{id}]====]")
            hex_pos = sys.argv.index('"hex":')+1
            res = sys.argv[hex_pos]
            serviceToText(res)
            print("[========================]")



if __name__ == "__main__":
    main()