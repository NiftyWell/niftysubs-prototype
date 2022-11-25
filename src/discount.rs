elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct Discount<M: ManagedTypeApi> {
        pub discount_type: DiscountType,
        pub owner: ManagedAddress<M>,
        pub payment_token: EgldOrEsdtTokenIdentifier<M>,
        pub payment_nonce: u64,
        pub percent: BigUint<M>,
        pub discount_id: Option<u64>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum DiscountType {
    None,
    Fixed,
    Dynamic
}

#[elrond_wasm::module]
pub trait DiscountModule:
    //crate::token_distribution::TokenDistributionModule
    //+ crate::events::EventsModule
    //+ crate::common_util_functions::CommonUtilFunctions
    + elrond_wasm_modules::pause::PauseModule
{
    #[endpoint(createDiscount)]
    #[allow(clippy::too_many_arguments)]
    fn create_discount(
        &self,
        discount_type: DiscountType,
        payment_token: EgldOrEsdtTokenIdentifier<M>,
        payment_nonce: u64,
        percent: BigUint<M>,
        discount_id: u64
    ) -> u64 {
        self.require_not_paused();

    }

    #[view(getFullDiscountData)]
    fn try_get_discount(&self, discount_id: u64) -> Discount<Self::Api> {
        let discount_mapper = self.discount_by_id(discount_id);
        require!(!discount_mapper.is_empty(), "discount does not exist");
        discount_mapper.get()
    }

    // Discount ID = discount ID
    #[storage_mapper("discountById")]
    fn discount_by_id(&self, discount_id: u64) -> SingleValueMapper<Discount<Self::Api>>;

    #[storage_mapper("discountsByAddress")]
    fn discounts_by_address(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;
}
