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
        pub contract_cut_percentage: BigUint<M>,
        pub service_name: ManagedBuffer<M>,
        pub service_webpage: ManagedBuffer<M>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum PeriodType {
    None,
    Seconds,
    Epochs
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

        let service = Service {
            owner:  self.blockchain().get_caller(),
            payment_token: payment_token,
            payment_nonce: accepted_payment_nonce,
            price: price,
            period_type: period_type,
            payment_period: payment_period,
            payments_total: BigUint::zero(),
            contract_cut_percentage: self.contract_cut_percentage().get(),
            service_name: service_name,
            service_webpage: service_webpage,
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
        self.service_by_id(service_id).set(new_service);
        service_id
    }

    #[view(getFullServiceData)]
    fn try_get_service(&self, service_id: u64) -> Service<Self::Api> {
        let service_mapper = self.service_by_id(service_id);
        require!(!service_mapper.is_empty(), "Service does not exist");
        service_mapper.get()
    }

    // Service ID = Discount ID
    #[view(getServiceById)]
    #[storage_mapper("serviceById")]
    fn service_by_id(&self, service_id: u64) -> SingleValueMapper<Service<Self::Api>>;

    #[view(getLastValidServiceId)]
    #[storage_mapper("lastValidServiceId")]
    fn last_valid_service_id(&self) -> SingleValueMapper<u64>;

    #[view(getServicesByAddress)]
    #[storage_mapper("servicesByAddress")]
    fn services_by_address(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[view(getContractCutPercentage)]
    #[storage_mapper("contractCutPercentage")]
    fn contract_cut_percentage(&self) -> SingleValueMapper<BigUint>;
}
