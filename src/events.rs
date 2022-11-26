elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::service::{Service};

#[allow(clippy::too_many_arguments)]
#[elrond_wasm::module]
pub trait EventsModule {
    fn emit_create_service_event(self, service_id: u64, service: Service<Self::Api>){
        self.create_service_event(
            &service.owner,
            service_id,
            service.payment_token,
            service.payment_nonce,
            &service.price,
            service.payment_period,
            &service.payments_total,
            service.contract_cut_percentage,
            &service.service_name, // means there's no service name.
            &service.service_webpage // means there's no website
        );
    }

    #[event("create_service_event")]
    fn create_service_event(
        &self,
        #[indexed] owner: &ManagedAddress,
        #[indexed] service_id: u64,
        #[indexed] payment_token: EgldOrEsdtTokenIdentifier,
        #[indexed] payment_nonce: u64,
        #[indexed] price: &BigUint,
        #[indexed] payment_period: u64,
        #[indexed] payments_total: &BigUint,
        contract_cut_percentage: BigUint,
        #[indexed] service_name: &ManagedBuffer, // means there's no service name.
        #[indexed] service_webpage: &ManagedBuffer // means there's no website
    );
}