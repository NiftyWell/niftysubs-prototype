////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    niftysubs
    (
        claimContractRewards
        claimFunds
        createDiscount
        createService
        discountClaimFunds
        endService
        fundSubscription
        getClaimDiscountRate
        getClaimDiscountToken
        getClaimableById
        getContractCutPercentage
        getContractRewards
        getFullDiscountData
        getFullServiceData
        getFullSubscriptionData
        getLastValidServiceId
        getPassedPeriods
        getRewardTokens
        getServiceById
        getServicesByAddress
        getStatus
        getSubscribers
        getSubscriptionById
        getSubscriptionsByAddress
        getToPay
        isPaused
        pause
        retrieveFunds
        setClaimDiscountRate
        setClaimDiscountToken
        setCustomCut
        setCutPercentage
        subscribe
        unpause
        unsubscribe
        updateService
    )
}

elrond_wasm_node::wasm_empty_callback! {}
