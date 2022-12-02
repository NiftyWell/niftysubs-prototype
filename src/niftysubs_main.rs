#![no_std]

elrond_wasm::imports!();

use crate::service::PERCENTAGE_TOTAL;
pub mod service;
pub mod events;
pub mod discount;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::contract]
pub trait NiftySubs:
    service::ServiceModule
    + discount::DiscountModule
    + events::EventsModule
    + elrond_wasm_modules::pause::PauseModule
{
    #[init]
    fn init(&self){
    }

    #[only_owner]
    #[endpoint(setCutPercentage)]
    fn set_contract_cut_percentage(&self, new_cut_percentage: u64) {
        self.try_set_contract_cut_percentage(new_cut_percentage);
    }

    fn try_set_contract_cut_percentage(&self, new_cut_percentage: u64){
        require!(
            new_cut_percentage > 0 && new_cut_percentage < PERCENTAGE_TOTAL,
            "Invalid percentage value, should be betwen 0 and 10,000."
        );
        self.contract_cut_percentage()
            .set(BigUint::from(new_cut_percentage));
    }
}
