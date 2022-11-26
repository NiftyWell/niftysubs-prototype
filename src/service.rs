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

        let service_name = match opt_service_name {
            OptionalValue::Some(name) => name,
            OptionalValue::None => ManagedBuffer::new()
        };

        let service_webpage = match opt_service_webpage {
            OptionalValue::Some(webpage) => webpage,
            OptionalValue::None => ManagedBuffer::new()
        };

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

    #[view(getFullServiceData)]
    fn try_get_service(&self, service_id: u64) -> Service<Self::Api> {
        let service_mapper = self.service_by_id(service_id);
        require!(!service_mapper.is_empty(), "Service does not exist");
        service_mapper.get()
    }

    // Service ID = Discount ID
    #[storage_mapper("serviceById")]
    fn service_by_id(&self, service_id: u64) -> SingleValueMapper<Service<Self::Api>>;

    #[storage_mapper("lastValidServiceId")]
    fn last_valid_service_id(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("servicesByAddress")]
    fn services_by_address(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[storage_mapper("contractCutPercentage")]
    fn contract_cut_percentage(&self) -> SingleValueMapper<BigUint>;
}
