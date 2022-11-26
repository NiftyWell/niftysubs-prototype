elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct Discount<M: ManagedTypeApi> {
        pub discount_type: DiscountType,
        pub owner: ManagedAddress<M>,
        pub payment_token: EgldOrEsdtTokenIdentifier<M>,
        pub payment_nonce: u64,
        pub percent: BigUint<M>,
        pub required_amount: BigUint<M>,
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
    crate::events::EventsModule
    + crate::service::ServiceModule
    + elrond_wasm_modules::pause::PauseModule
{
    #[endpoint(createDiscount)]
    #[allow(clippy::too_many_arguments)]
    fn create_discount(
        &self,
        discount_type: DiscountType,
        payment_token: EgldOrEsdtTokenIdentifier,
        payment_nonce: OptionalValue<u64>,
        percent: BigUint,
        amount: BigUint,
        service_id: u64
    ) -> u64 {
        self.require_not_paused();
        require!(
            discount_type == DiscountType::Fixed || discount_type == DiscountType::Dynamic,
            "Invalid discount type."
        );
        require!(
            amount > 0u64,
            "Coupon token amount needs to be bigger than 0."
        );

        let service = self.try_get_service(service_id);
        require!(
            service.owner == self.blockchain().get_caller(),
            "Only service owner can manage service discounts."
        );
        let discount_id = service_id;

        let accepted_payment_nonce = if payment_token.is_egld() {
            0
        } else {
            payment_nonce
                .into_option()
                .unwrap_or_default()
        };

        let discount = Discount {
            discount_type: discount_type,
            owner: self.blockchain().get_caller(),
            payment_token: payment_token,
            payment_nonce: accepted_payment_nonce,
            percent: percent,
            required_amount: amount,
        };

        self.discount_by_id(discount_id).set(&discount);
        // self.emit_create_discount_event(discount_id, discount);
        self.discounts_by_address(&self.blockchain().get_caller()).insert(discount_id);

        discount_id
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
