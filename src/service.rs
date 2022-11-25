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
        pub payments_total: BigUint,
        pub contract_cut_percentage: BigUint<M>,
        pub discount_id: Option<u64>,
        pub service_name: Option<ManagedBuffer>,
        pub service_webpage: Option<ManagedBuffer>,
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
    //+ crate::events::EventsModule
    //+ crate::common_util_functions::CommonUtilFunctions
    + elrond_wasm_modules::pause::PauseModule
{
    #[endpoint(createService)]
    #[allow(clippy::too_many_arguments)]
    fn create_service(
        &self,
        payment_token: EgldOrEsdtTokenIdentifier<M>,
        payment_nonce: u64,
        price: BigUint<M>,
        period_type: PeriodType,
        payment_period: u64,
        payments_total: BigUint,
        contract_cut_percentage: BigUint<M>,
        opt_discount_id: Option<u64>,
        opt_service_name: Option<ManagedBuffer>,
        opt_service_webpage: Option<ManagedBuffer>,
    ) -> u64 {
        self.require_not_paused();
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

    #[storage_mapper("servicesByAddress")]
    fn services_by_address(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;
}
