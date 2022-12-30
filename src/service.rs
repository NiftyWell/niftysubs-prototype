elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct Service<M: ManagedTypeApi> {
        // pub service_type: ServiceType,
        pub owner: ManagedAddress<M>,
        pub payment_token: EgldOrEsdtTokenIdentifier<M>,
        pub payment_nonce: u64,
        pub price: BigUint<M>,
        pub period_type: PeriodType,
        pub payment_period: u64,
        pub payments_total: BigUint<M>,
        pub custom_cut: bool,
        pub contract_cut_percentage: BigUint<M>,
        pub service_name: ManagedBuffer<M>,
        pub service_webpage: ManagedBuffer<M>,
        pub grace_period: u64
}

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct Subscription<M: ManagedTypeApi> {
    pub last_claim: u64,
    pub payment_token: EgldOrEsdtTokenIdentifier<M>,
    pub payment_nonce: u64,
    pub amount: BigUint<M>,
    pub prev_amount: BigUint<M>,
    pub unsubscribed: bool
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone)]
pub struct SubId<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub service_id: u64
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum PeriodType {
    None,
    Seconds,
    Epochs
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum Status {
    None,
    Active,
    Pending,
    Revoked
}

#[elrond_wasm::module]
pub trait ServiceModule:
    //crate::token_distribution::TokenDistributionModule
    crate::events::EventsModule
    + elrond_wasm_modules::pause::PauseModule
{
    #[endpoint(createService)]
    #[allow(clippy::too_many_arguments)]
    fn create_service(
        &self,
        payment_token: EgldOrEsdtTokenIdentifier,
        payment_nonce: OptionalValue<u64>,
        price: BigUint,
        period_type: PeriodType,
        payment_period: u64,
        opt_service_name: OptionalValue<ManagedBuffer>,
        opt_service_webpage: OptionalValue<ManagedBuffer>,
        opt_grace_period: OptionalValue<u64>
    ) -> u64 {
        self.require_not_paused();
        require!(
            period_type == PeriodType::Seconds || period_type == PeriodType::Epochs,
            "Invalid period type."
        );
        require!(
            payment_period > 0u64,
            "Payment period needs to be bigger than 0."
        );
        require!(
            price > BigUint::zero(),
            "Service price needs to be bigger than 0."
        );

        let mut service_name = ManagedBuffer::new();
        let mut service_webpage = ManagedBuffer::new();

        match opt_service_name{
            OptionalValue::Some(name) => {
                require!(
                    name.len() <= 64usize, 
                    "Service name needs to be under 64 bytes in length."
                );
                service_name = name;
            },
            OptionalValue::None => ()
        }

        match opt_service_webpage{
            OptionalValue::Some(page) => {
                require!(
                    page.len() <= 64usize, 
                    "Service webpage needs to be under 64 bytes in length."
                );
                service_webpage = page;
            },
            OptionalValue::None => ()
        }

        let service_id = self.last_valid_service_id().get()+1;
        self.last_valid_service_id().set(service_id);

        let accepted_payment_nonce = if payment_token.is_egld() {
            0
        } else {
            payment_nonce
                .into_option()
                .unwrap_or_default()
        };
        let mut grace_period = 0u64;
        match opt_grace_period{
            OptionalValue::Some(period) => {
                grace_period = period;
            },
            OptionalValue::None => ()
        }

        let service = Service {
            owner:  self.blockchain().get_caller(),
            payment_token: payment_token,
            payment_nonce: accepted_payment_nonce,
            price: price,
            period_type: period_type,
            payment_period: payment_period,
            payments_total: BigUint::zero(),
            custom_cut: false,
            contract_cut_percentage: self.contract_cut_percentage().get(),
            service_name: service_name,
            service_webpage: service_webpage,
            grace_period: grace_period,
        };

        self.service_by_id(service_id).set(&service);
        self.emit_create_service_event(service_id, service);
        self.services_by_address(&self.blockchain().get_caller()).insert(service_id);

        service_id
    }

    #[endpoint(updateService)]
    #[allow(clippy::too_many_arguments)]
    fn update_service(
        &self,
        service_id: u64,
        opt_payment_token: OptionalValue<EgldOrEsdtTokenIdentifier>,
        opt_payment_nonce: OptionalValue<u64>,
        opt_price: OptionalValue<BigUint>,
        opt_period_type: OptionalValue<PeriodType>,
        opt_payment_period: OptionalValue<u64>,
        opt_service_name: OptionalValue<ManagedBuffer>,
        opt_service_webpage: OptionalValue<ManagedBuffer>,
        opt_grace_period: OptionalValue<u64>
    ) -> u64 {
        self.require_not_paused();
        let old_service = self.try_get_service(service_id);
        require!(
            old_service.owner == self.blockchain().get_caller(),
            "You are not the owner of this service."
        );
        
        let mut new_service = old_service.clone();

        // Unpacking optional values
        match opt_payment_token {
            OptionalValue::Some(token) => {
                new_service.payment_token = token.clone();
                if token.is_egld() {
                    new_service.payment_nonce=0;
                } else {
                    match opt_payment_nonce {
                        OptionalValue::Some(nonce) => {
                            new_service.payment_nonce = nonce;
                        },
                        OptionalValue::None => panic!("A nonce is needed if a new ticker has been provided.")
                    }
                    
                };
            },
            OptionalValue::None => ()
        }

        match opt_price{
            OptionalValue::Some(price) => {
                require!(
                    price > BigUint::zero(),
                    "Service price needs to be bigger than 0."
                );
                new_service.price = price;
            },
            OptionalValue::None => ()
        }

        match opt_period_type{
            OptionalValue::Some(period) => {
                require!(
                    period == PeriodType::Seconds || period == PeriodType::Epochs,
                    "Invalid period type."
                );
                new_service.period_type = period;
            },
            OptionalValue::None => ()
        }

        match opt_payment_period{
            OptionalValue::Some(pay) => {
                require!(
                    pay > 0u64,
                    "Payment period needs to be bigger than 0."
                );
                new_service.payment_period = pay;
            },
            OptionalValue::None => ()
        }

        match opt_service_name{
            OptionalValue::Some(name) => {
                require!(
                    name.len() <= 64usize, 
                    "Service name needs to be under 65 in length."
                );
                new_service.service_name = name;
            },
            OptionalValue::None => ()
        }

        match opt_service_webpage{
            OptionalValue::Some(page) => {
                require!(
                    page.len() <= 64usize, 
                    "Service webpage needs to be under 65 in length."
                );
                new_service.service_webpage = page;
            },
            OptionalValue::None => ()
        }

        match opt_grace_period{
            OptionalValue::Some(period) => {
                new_service.grace_period = period;
            },
            OptionalValue::None => ()
        }
        self.service_by_id(service_id).set(new_service);
        service_id
    }

    #[only_owner]
    #[endpoint(setCustomCut)]
    #[allow(clippy::too_many_arguments)]
    fn set_custom_cut_service(
        &self,
        service_id: u64,
        cut_flag: bool,
        opt_custom_cut: OptionalValue<u64>
    ) -> bool {
        let mut service = self.try_get_service(service_id).clone();
        if cut_flag {
            match opt_custom_cut{
                OptionalValue::Some(cut) => {
                    require!(
                        cut <= PERCENTAGE_TOTAL,
                        "Invalid percentage value, should be betwen 0 and 10,000."
                    );
                    service.custom_cut = true;
                    service.contract_cut_percentage = BigUint::from(cut);
                    self.service_by_id(service_id).set(service);
                },
                OptionalValue::None => ()
            }
        }
        else{
            service.custom_cut = false;
            self.service_by_id(service_id).set(service);
        }
        cut_flag
    }

    #[endpoint(endService)]
    #[allow(clippy::too_many_arguments)]
    fn end_service(
        &self,
        service_id: u64
    ) -> u64 {
        self.require_not_paused();
        let service = self.try_get_service(service_id);
        let caller = self.blockchain().get_caller();
        require!(
            service.owner == caller,
            "Only the service owner can end the service."
        );
        // CLAIM CLAIMABLE TOKENS
        self.service_by_id(service_id).clear();
        service_id
    }

    #[payable("*")]
    #[endpoint(subscribe)]
    #[allow(clippy::too_many_arguments)]
    fn subscribe(
        &self,
        service_id: u64,
        #[payment_token] payment_token  : EgldOrEsdtTokenIdentifier,
        #[payment_nonce] payment_nonce  : u64,
        #[payment_amount] payment_amount: BigUint,
    ) -> u64 {
        self.require_not_paused();
        let service = self.try_get_service(service_id);
        require!(
            payment_token == service.payment_token,
            "Incorrect payment token."
        );
        require!(
            payment_nonce == service.payment_nonce,
            "Incorrect payment token nonce."
        );
        require!(
            payment_amount >= service.price,
            "Insufficient amount."
        );
        
        let caller = self.blockchain().get_caller();

        let sub_id = SubId{
            address: caller.clone(),
            service_id: service_id
        };

        let subscription_mapper = self.subscription_by_id(&sub_id);
        require!(subscription_mapper.is_empty(), "Subscription already exists");
        // Last claim is the current timestamp.
        let last_claim = self.blockchain().get_block_timestamp();
        let subscription = Subscription{
            last_claim: last_claim,
            payment_token: service.payment_token,
            payment_nonce: service.payment_nonce,
            amount: payment_amount,
            prev_amount: BigUint::zero(),
            unsubscribed: false,
        };
        self.subscription_by_id(&sub_id).set(subscription);
        self.subscribers(service_id).insert(caller.clone());
        self.subscriptions_by_address(&caller).insert(service_id);
        service_id
    }

    #[endpoint(unsubscribe)]
    #[allow(clippy::too_many_arguments)]
    fn unsubscribe(
        &self,
        service_id: u64,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let service_mapper = self.service_by_id(service_id);
        let subscription = self.try_get_subscription(caller.clone(), service_id);
        if service_mapper.is_empty() {
            // Unsubscribe and send all funds
            self.send().direct(&caller, &subscription.payment_token, subscription.payment_nonce, &subscription.amount);
            self.subscriptions_by_address(&caller).swap_remove(&service_id); // remove service from subscription list of user (used just as a view for dapps)
            self.subscription_by_id(&SubId{address:caller.clone(), service_id:service_id}).clear();
            self.subscribers(service_id).swap_remove(&caller);
            return Ok(());
        }
        // Service ID is valid.
        let service = service_mapper.get();
        

        // Require Status::Revoked
        require!(self.try_get_status(caller.clone(), service_id) == Status::Revoked,
            "Cannot unsubscribe during active subscription."    
        );

        let to_pay = self.get_to_pay(service.clone(), subscription.clone());
        let left = subscription.amount - to_pay.clone();
        // if there's something left after obligatory payments, send funds back to user
        if left > 0 {
            // the subscription will get deleted only after the claim. Address should also remain in subscribers mapper for this reason.
            self.subscription_by_id(&SubId{address:caller.clone(), service_id:service_id}).set(
                Subscription{
                    last_claim: subscription.last_claim, 
                    payment_token: service.payment_token.clone(),
                    payment_nonce: service.payment_nonce,
                    amount: to_pay.clone(), 
                    prev_amount: subscription.prev_amount, 
                    unsubscribed: true
                });
            self.send().direct(&caller, &service.payment_token, service.payment_nonce, &left);
            self.subscriptions_by_address(&caller).swap_remove(&service_id); // remove service from subscription list of user (used just as a view for dapps)
        }
        if to_pay <= 0 { // if it so happens that nothing was left to pay, fully unsubscribe user from service
            self.subscriptions_by_address(&caller).swap_remove(&service_id); // remove service from subscription list of user (used just as a view for dapps)
            self.subscription_by_id(&SubId{address:caller.clone(), service_id:service_id}).clear();
            self.subscribers(service_id).swap_remove(&caller);
        }
        Ok(())
    }

    fn get_passed_periods(&self, service: Service<Self::Api>, subscription: Subscription<Self::Api>) -> u64{
        let mut period_mult = 1u64; // To manage epochs & seconds
        if service.period_type == PeriodType::Epochs{
            period_mult = 86400u64;
        }
        let current_timestamp = self.blockchain().get_block_timestamp();
        if current_timestamp <= subscription.last_claim{ // We're still in an active sub period. 
            return 0u64; // This case is triggered only when owner claimed funds during the period.
        }
        // Case where owner didn't claim during active period.
        let passed_periods = (current_timestamp - subscription.last_claim)/(service.payment_period*period_mult)+1u64; 
        return passed_periods
    }

    #[view(getPassedPeriods)]
    fn view_get_passed_periods(&self, address: ManagedAddress, service_id: u64) -> u64{
        let subscription = self.try_get_subscription(address.clone(), service_id);
        let service = self.try_get_service(service_id);
        let mut period_mult = 1u64; // To manage epochs & seconds
        if service.period_type == PeriodType::Epochs{
            period_mult = 86400u64;
        }
        let current_timestamp = self.blockchain().get_block_timestamp();
        if current_timestamp <= subscription.last_claim{ // We're still in an active sub period. 
            return 0u64; // This case is triggered only when owner claimed funds during the period.
        }
        // Case where owner didn't claim during active period.
        let passed_periods = (current_timestamp - subscription.last_claim)/(service.payment_period*period_mult)+1u64; 
        return passed_periods
    }

    #[endpoint(claimFunds)]
    #[allow(clippy::too_many_arguments)]
    fn claim_funds(
        &self,
        service_id: u64,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let service = self.try_get_service(service_id);
        require!(
            service.owner == caller,
            "Only service owner can claim funds."
        );
        let mut period_mult = 1u64;
        if service.period_type == PeriodType::Epochs{
            period_mult = 86400u64;
        }
        let mut total_payments = BigUint::zero();
        for address in self.subscribers(service_id).iter()
        {
            let subscription = self.try_get_subscription(address.clone(), service_id);
            if subscription.unsubscribed {
                total_payments+=subscription.amount;
                self.subscribers(service_id).swap_remove(&caller);
                self.subscription_by_id(&SubId{address:address.clone(), service_id:service_id}).clear();
            }
            else{
                let mut to_pay = self.get_to_pay(service.clone(), subscription.clone());
                let payed_periods = (to_pay.clone()/service.price.clone()).to_u64().unwrap_or_default();
                if subscription.amount >= service.price{
                    if to_pay > BigUint::zero(){
                        let mut new_sub = Subscription{
                            last_claim: subscription.last_claim.clone(), 
                            payment_token: service.payment_token.clone(),
                            payment_nonce: service.payment_nonce,
                            amount: subscription.amount.clone(),
                            prev_amount: subscription.prev_amount.clone(),
                            unsubscribed: subscription.unsubscribed.clone()
                        };
                        if subscription.unsubscribed && subscription.amount.clone() - to_pay.clone() == BigUint::zero(){ // If user opted to unsubscribe, remove user subscription & address from subscribers list
                            self.subscription_by_id(&SubId{address:address.clone(), service_id:service_id}).clear();
                            self.subscribers(service_id).swap_remove(&address);
                        }
                        else{
                            new_sub.last_claim = subscription.last_claim+payed_periods*service.payment_period*period_mult; // Periods payed for.
                            new_sub.amount = subscription.amount-to_pay.clone();
                            new_sub.prev_amount = subscription.prev_amount.clone();
                            new_sub.unsubscribed = subscription.unsubscribed;
                        }
                        to_pay+=subscription.prev_amount.clone(); // always needs to be added since if no previous payments it'll just be + 0
                        new_sub.prev_amount = BigUint::zero(); // always needs to be 0 after a claimfunds.
                        self.subscription_by_id(&SubId{address:address.clone(), service_id:service_id}).set(new_sub.clone());
                        // Update total payments
                        let mut new_service = service.clone();
                        new_service.payments_total = new_service.payments_total + to_pay.clone();
                        self.service_by_id(service_id).set(new_service);
                        total_payments+=to_pay;
                    }
                }
            }
        }
        require!(
            total_payments > BigUint::zero(),
            "No payments to be claimed."
        );
        self.send().direct(&caller, &service.payment_token, service.payment_nonce, &total_payments);
        Ok(())
    }
    #[view(getToPay)]
    fn view_get_to_pay(&self, address: ManagedAddress, service_id: u64) -> BigUint{
        let subscription = self.try_get_subscription(address.clone(), service_id);
        let service = self.try_get_service(service_id);

        let passed_periods = self.get_passed_periods(service.clone(), subscription.clone());
        let to_pay = BigUint::from(passed_periods) * service.price.clone();
        if subscription.amount >= service.price{
            if to_pay > BigUint::zero(){
                let can_pay = subscription.amount.clone() - (subscription.amount.clone()%service.price.clone()); 
                if can_pay <= to_pay{ // Case where amount isn't a multiple of price
                    return can_pay.clone();
                }
                return to_pay; 
            }
        }
        return BigUint::zero();
    }

    fn get_to_pay(&self, service: Service<Self::Api>, subscription: Subscription<Self::Api>) -> BigUint {
        let passed_periods = self.get_passed_periods(service.clone(), subscription.clone());
        let to_pay = BigUint::from(passed_periods) * service.price.clone();
        if subscription.amount >= service.price{
            if to_pay > BigUint::zero(){
                let can_pay = subscription.amount.clone() - (subscription.amount.clone()%service.price.clone()); 
                if can_pay <= to_pay{ // Case where amount isn't a multiple of price
                    return can_pay.clone();
                }
                return to_pay; 
            }
        }
        return BigUint::zero();
    }

    #[payable("*")]
    #[endpoint(fundSubscription)]
    #[allow(clippy::too_many_arguments)]
    fn fund_subscription(
        &self,
        service_id: u64,
        #[payment_token] payment_token  : EgldOrEsdtTokenIdentifier,
        #[payment_nonce] payment_nonce  : u64,
        #[payment_amount] payment_amount: BigUint,
    ) -> u64 {
        self.require_not_paused();
        let service = self.try_get_service(service_id);
        require!(
            payment_token == service.payment_token,
            "Incorrect payment token."
        );
        require!(
            payment_nonce == service.payment_nonce,
            "Incorrect payment token nonce."
        );
        
        let caller = self.blockchain().get_caller();

        let sub_id = SubId{
            address: caller.clone(),
            service_id: service_id
        };

        let subscription_mapper = self.subscription_by_id(&sub_id);
        require!(!subscription_mapper.is_empty(), "Subscription doesn't exists");
        let subscription = subscription_mapper.get();

        let mut last_claim = subscription.last_claim;
        let mut amount = subscription.amount.clone() + payment_amount;
        let mut prev_amount = BigUint::zero();
        if self.try_get_status(caller, service_id) == Status::Revoked{ //less costly call to make
            // Add prev_amount for owner to be able to claim last what was payed before getting revoked.
            // last claim is set just like a new subscription.
            let to_pay = self.get_to_pay(service.clone(), subscription.clone());
            prev_amount = to_pay.clone();
            amount -=to_pay;
            last_claim = self.blockchain().get_block_timestamp(); // New subscription
        }
        self.subscription_by_id(&sub_id).update(|val| *val = Subscription{
            last_claim: last_claim,
            payment_token: service.payment_token,
            payment_nonce: service.payment_nonce,
            prev_amount: prev_amount, //shouldn't be zero, should check if revoked. if revoked move amount to be payed there.
            amount: amount,
            unsubscribed: subscription.unsubscribed
        });
        service_id
    }

    #[endpoint(retrieveFunds)]
    #[allow(clippy::too_many_arguments)]
    fn retrieve_funds(
        &self,
        service_id: u64,
        amount: BigUint
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let service = self.try_get_service(service_id);
        let subscription = self.try_get_subscription(caller.clone(), service_id);

        let to_pay = self.get_to_pay(service.clone(), subscription.clone());
        require!(
            amount <= subscription.amount && amount > BigUint::zero(),
            "You can't retrieve more funds than you have."
        );
        let left = subscription.amount.clone() - amount.clone();
        require!(
            left >= to_pay,
            "You can't retrieve this amount."
        );
    
        self.subscription_by_id(&SubId{address:caller.clone(), service_id:service_id}).set(
            Subscription{
                last_claim: subscription.last_claim, 
                payment_token: service.payment_token.clone(),
                payment_nonce: service.payment_nonce,
                amount: subscription.amount-amount.clone(), 
                prev_amount: subscription.prev_amount, 
                unsubscribed: false
            });
        self.send().direct(&caller, &service.payment_token, service.payment_nonce, &amount);
    
        Ok(())
    }

    #[view(getFullServiceData)]
    fn try_get_service(&self, service_id: u64) -> Service<Self::Api> {
        let service_mapper = self.service_by_id(service_id);
        require!(!service_mapper.is_empty(), "Service does not exist");
        service_mapper.get()
    }

    #[view(getFullSubscriptionData)]
    fn try_get_subscription(&self, address: ManagedAddress, service_id: u64) -> Subscription<Self::Api> {
        let sub_id = SubId{
            address: address.clone(),
            service_id: service_id
        };
        let subscription_mapper = self.subscription_by_id(&sub_id);
        require!(!subscription_mapper.is_empty(), "Subscription does not exist");
        subscription_mapper.get()
    }

    // Check Subscription status
    #[view(getStatus)]
    fn try_get_status(&self, address: ManagedAddress, service_id: u64) -> Status {
        let service = self.try_get_service(service_id);
        let subscription = self.try_get_subscription(address, service_id);
        let passed_periods = self.get_passed_periods(service.clone(), subscription.clone());
        let to_pay = BigUint::from(passed_periods)*service.price;
        if self.blockchain().get_block_timestamp() < subscription.last_claim{ // Previous claim so still in period
            return Status::Active;
        }
        if subscription.amount >= to_pay{
            return Status::Active;
        }
        Status::Revoked
    }

    // Service ID = Discount ID
    #[view(getServiceById)]
    #[storage_mapper("serviceById")]
    fn service_by_id(&self, service_id: u64) -> SingleValueMapper<Service<Self::Api>>;

    // Service ID = Discount ID
    #[view(getClaimableById)]
    #[storage_mapper("claimableById")]
    fn claimable_by_id(&self, service_id: u64) -> SingleValueMapper<BigUint>;

    #[view(getLastValidServiceId)]
    #[storage_mapper("lastValidServiceId")]
    fn last_valid_service_id(&self) -> SingleValueMapper<u64>;

    #[view(getServicesByAddress)]
    #[storage_mapper("servicesByAddress")]
    fn services_by_address(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[view(getContractCutPercentage)]
    #[storage_mapper("contractCutPercentage")]
    fn contract_cut_percentage(&self) -> SingleValueMapper<BigUint>;

    #[view(getSubscriptionById)]
    #[storage_mapper("subscriptionById")]
    fn subscription_by_id(&self, id: &SubId<Self::Api>) -> SingleValueMapper<Subscription<Self::Api>>;

    #[view(getSubscribers)]
    #[storage_mapper("subscribers")]
    fn subscribers(&self, service_id: u64) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getSubscriptionsByAddress)]
    #[storage_mapper("subscriptionsByAddress")]
    fn subscriptions_by_address(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

}
