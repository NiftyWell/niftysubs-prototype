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
    pub amount: BigUint<M>
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
        let subscription = Subscription{
            last_claim: self.blockchain().get_block_timestamp(),
            amount: payment_amount,
        };
        self.subscription_by_id(&sub_id).set(subscription);
        self.subscribers(service_id).insert(caller);
        service_id
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
        for address in self.subscribers(service_id).iter()
        {
            let subscription = self.try_get_subscription(address.clone(), service_id);
            let current_timestamp = self.blockchain().get_block_timestamp();
            let passed_periods = (current_timestamp - subscription.last_claim)/service.payment_period;
            let to_pay = BigUint::from(passed_periods) * service.price.clone();
            if to_pay > BigUint::zero(){
                self.subscription_by_id(&SubId{address:address.clone(), service_id:service_id}).set(Subscription{last_claim: subscription.last_claim+passed_periods*service.payment_period, amount: subscription.amount-to_pay.clone()});
                self.send().direct(&caller, &service.payment_token, service.payment_nonce, &to_pay);
            }
        }
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
        let current_timestamp = self.blockchain().get_block_timestamp();
        let passed_period = (current_timestamp - subscription.last_claim)/service.payment_period;
        if passed_period < 1 {
            return Status::Active;
        }
        let to_pay = BigUint::from(passed_period) * service.price;
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
}
