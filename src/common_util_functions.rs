elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait CommonUtilFunctions: elrond_wasm_modules::pause::PauseModule {
    fn require_not_paused(&self){
        require!(self.not_paused(), "Marketplace is paused");
    }
}