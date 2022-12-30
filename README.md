# Interaction

## On devnet

Deploy & interact with contract:

```
python3 ./interaction/playground.py --pem=./testnet/wallets/users/alice.pem --proxy=http://localhost:7950
```

Interact with existing contract:

```
python3 ./interaction/playground.py --pem=./testnet/wallets/users/alice.pem --proxy=http://localhost:7950 --contract=erd1...
```

## On testnet

Deploy & interact with contract:

```
python3 ./interaction/playground.py --pem=my.pem --proxy=https://testnet-gateway.elrond.com
```

Interact with existing contract:

```
python3 ./interaction/playground.py --pem=my.pem --proxy=https://testnet-gateway.elrond.com --contract=erd1...
```

## Functions
Simplicity:
Things should be kept as simple and easy to understand as possible.
Functions shouldn't get too intricate.

## For Subscribers
Unsubscribe:
To unsubscribe a subscription must have the revoked status.

## For Service Owners
Remove Service: if service ID is empty allow user to retreive all funds.

## Fees
Add fee for service creators when claiming funds.

## Discounts
Add discounts for service creators when they claim funds.
Add discounts for subscribers based on the discount mappers.


