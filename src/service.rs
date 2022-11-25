elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct Service<M: ManagedTypeApi> {
    pub service_type: ServiceType,
    pub owner: ManagedAddress<M>,
    pub payment_token: EgldOrEsdtTokenIdentifier<M>,
    pub payment_period: u64,
    pub payments_total: BigUint,
    pub discount_id: Option<u64>
    pub service_name: Option<ManagedBuffer>,
    pub service_name: Option<ManagedBuffer>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum ServiceType {
    None,
    Subscription
}

#[elrond_wasm::module]
