elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::service::{Service};

#[allow(clippy::too_many_arguments)]
#[elrond_wasm::module]
pub trait EventsModule {
    fn emit_create_service_event(self, service_id: u64, service: Auction<Self::Api>){
        self.create_service_event(
            &service.owner,
            service.payment_token.token_identifier,
            service.payment_token.token_nonce,
            service.payment_period,
            service.payments_total,
            service.contract_cut_percentage,
            &service.discount_id.unwrap_or_else(u64::zero), // means there's no discount.
            &service.service_name.unwrap_or_else(u64::zero), // means there's no service name.
            &service.service_webpage.unwrap_or_else(u64::zero) // means there's no website
        );
    }
}