# MultiversX Subscriptions Smart Contract Documentation

This document provides a detailed overview of the Elrond Subscriptions Smart Contract written in Rust.

## Function Descriptions

### `fund_subscription`

This function is used to fund a subscription. It takes in the service_id to identify the subscription service, payment_token which is the token identifier of the payment, payment_nonce which is the nonce of the payment token, and payment_amount which is the amount paid for the subscription. The function then checks the service and subscription details and makes necessary updates if the subscription status is revoked. It returns the service_id.

### `set_claim_discount_token`

This function is used to set the claim discount token. It takes in payment_token as an argument and sets the claim discount token to the provided token. This function can only be called by the owner of the contract.

### `set_claim_discount_rate`

This function is used to set the claim discount rate. It takes in a rate as an argument and sets the claim discount rate to the provided rate. This function can only be called by the owner of the contract.

### `retrieve_funds`

This function is used to retrieve funds from a subscription. It takes in a service_id and amount as arguments. It then checks if the amount is less than or equal to the subscription amount and more than zero. It also checks if the remaining subscription amount is more than the amount to pay after the retrieval. If all the checks pass, it updates the subscription details and sends the amount to the caller.

### `claim_contract_rewards`

This function is used to claim the rewards of the contract. It iterates over the reward tokens and sends the corresponding reward amount to the caller. This function can only be called by the owner of the contract.

### `try_get_service`

This function is used to get the service details for a given service_id. It returns the service details if the service exists.

### `try_get_subscription`

This function is used to get the subscription details for a given address and service_id. It returns the subscription details if the subscription exists.

### `try_get_status`

This function is used to get the status of a subscription for a given address and service_id. It returns the status of the subscription.

## Storage Mapper Descriptions

### `service_by_id`

This mapper is used to store and retrieve the service details for a given service_id.

### `claimable_by_id`

This mapper is used to store and retrieve the claimable amount for a given service_id.

### `last_valid_service_id`

This mapper is used to store and retrieve the last valid service_id.

### `services_by_address`

This mapper is used to store and retrieve the services associated with a given address.

### `contract_cut_percentage`

This mapper is used to store and retrieve the contract cut percentage.

### `subscription_by_id`

This mapper is used to store and retrieve the subscription details for a given subscription id.

### `subscribers`

This mapper is used to store and retrieve the subscribers of a given service_id.

### `subscriptions_by_address`

This mapper is used to store and retrieve the subscriptions associated with a given address.

### `contract_rewards`

This mapper is used to store and retrieve the contract rewards for a given token.

### `reward_tokens`

This mapper is used to store and retrieve the reward tokens.

### `claim_discount_token`

This mapper is used to store and retrieve the claim discount token.

### `claim_discount_rate`

This mapper is used to store and retrieve the claim discount rate.

## Key Structures

### `Service`

The `Service` structure is used to store information about each service. This includes the payment token and nonce, price per period, and other relevant details.

### `Subscription`

The `Subscription` structure is used to store information about each subscription. This includes the last claim, payment token and nonce, amount, previous amount, and status (unsubscribed or not).

### `SubId`

The `SubId` structure is used to uniquely identify a subscription. It is composed of the subscriber's address and the service_id.

## Status Enum

The `Status` enum is used to denote the current status of a subscription. It can be either 'Active' or 'Revoked'.

## Error Handling

This contract uses the SCResult type for error handling. The `require!` macro is used to assert conditions, and if the condition is not met, an error is returned and the execution is halted.

For example, in the `fund_subscription` function, if the payment token does not match the service's payment token, the function will return an error and halt execution.

## Important Points

1. The contract owner can set the claim discount token and rate.
2. Subscribers can fund their subscription, and they can also retrieve funds from their subscription.
3. Subscribers cannot retrieve more funds than they have in their subscription.
4. If a subscription is revoked, the remaining amount is updated and the last claim is reset.
5. The contract owner can claim the contract's rewards.
6. The `try_get_subscription`, `try_get_service`, and `try_get_status` functions are used to fetch subscription, service, and status details, respectively. These are view functions and do not modify the state of the contract.
7. The contract maintains various mappers to store and retrieve data efficiently.

