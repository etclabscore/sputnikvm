extern crate bigint;
extern crate sputnikvm;
extern crate sputnikvm_precompiled_modexp;
extern crate sputnikvm_precompiled_bn128;

use bigint::{Gas, U256, H160, Address};
use sputnikvm::{Precompiled, AccountPatch, Patch,
                ID_PRECOMPILED, ECREC_PRECOMPILED, SHA256_PRECOMPILED, RIP160_PRECOMPILED};
use sputnikvm_precompiled_modexp::MODEXP_PRECOMPILED;
use sputnikvm_precompiled_bn128::{BN128_ADD_PRECOMPILED, BN128_MUL_PRECOMPILED, BN128_PAIRING_PRECOMPILED};

pub static FRONTIER_PRECOMPILEDS: [(Address, Option<&'static [u8]>, &'static Precompiled); 4] = [
    (H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x01]),
     None,
     &ECREC_PRECOMPILED),
    (H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x02]),
     None,
     &SHA256_PRECOMPILED),
    (H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x03]),
     None,
     &RIP160_PRECOMPILED),
    (H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x04]),
     None,
     &ID_PRECOMPILED),
];

pub static BYZANTIUM_PRECOMPILEDS: [(Address, Option<&'static [u8]>, &'static Precompiled); 8] = [
    (H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x01]),
     None,
     &ECREC_PRECOMPILED),
    (H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x02]),
     None,
     &SHA256_PRECOMPILED),
    (H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x03]),
     None,
     &RIP160_PRECOMPILED),
    (H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x04]),
     None,
     &ID_PRECOMPILED),
    (H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x05]),
     None,
     &MODEXP_PRECOMPILED),
    (H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x06]),
     None,
     &BN128_ADD_PRECOMPILED),
    (H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x07]),
     None,
     &BN128_MUL_PRECOMPILED),
    (H160([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x08]),
     None,
     &BN128_PAIRING_PRECOMPILED),
];

pub struct FrontierAccountPatch;
impl AccountPatch for FrontierAccountPatch {
    fn initial_nonce() -> U256 { U256::zero() }
    fn initial_create_nonce() -> U256 { Self::initial_nonce() }
    fn empty_considered_exists() -> bool { true }
}

pub struct StateClearingAccountPatch;
impl AccountPatch for StateClearingAccountPatch {
    fn initial_nonce() -> U256 { U256::zero() }
    fn initial_create_nonce() -> U256 { Self::initial_nonce() + U256::from(1) }
    fn empty_considered_exists() -> bool { false }
}

/// Frontier patch.
pub struct FrontierPatch;
impl Patch for FrontierPatch {
    type Account = FrontierAccountPatch;

    fn code_deposit_limit(&self) -> Option<usize> { None }
    fn callstack_limit(&self) -> usize { 1024 }
    fn gas_extcode(&self) -> Gas { Gas::from(20usize) }
    fn gas_balance(&self) -> Gas { Gas::from(20usize) }
    fn gas_sload(&self) -> Gas { Gas::from(50usize) }
    fn gas_suicide(&self) -> Gas { Gas::from(0usize) }
    fn gas_suicide_new_account(&self) -> Gas { Gas::from(0usize) }
    fn gas_call(&self) -> Gas { Gas::from(40usize) }
    fn gas_expbyte(&self) -> Gas { Gas::from(10usize) }
    fn gas_transaction_create(&self) -> Gas { Gas::from(0usize) }
    fn force_code_deposit(&self) -> bool { true }
    fn has_delegate_call(&self) -> bool { false }
    fn has_static_call(&self) -> bool { false }
    fn has_revert(&self) -> bool { false }
    fn has_return_data(&self) -> bool { false }
    fn has_bitwise_shift(&self) -> bool { false }
    fn has_extcodehash(&self) -> bool { false }
    fn has_reduced_sstore_gas_metering(&self) -> bool { false }
    fn err_on_call_with_more_gas(&self) -> bool { true }
    fn call_create_l64_after_gas(&self) -> bool { false }
    fn memory_limit(&self) -> usize { usize::max_value() }
    fn precompileds(&self) -> &'static [(Address, Option<&'static [u8]>, &'static Precompiled)] {
        &FRONTIER_PRECOMPILEDS }
}

/// Homestead patch.
pub struct HomesteadPatch;
impl Patch for HomesteadPatch {
    type Account = FrontierAccountPatch;

    fn code_deposit_limit(&self) -> Option<usize> { None }
    fn callstack_limit(&self) -> usize { 1024 }
    fn gas_extcode(&self) -> Gas { Gas::from(20usize) }
    fn gas_balance(&self) -> Gas { Gas::from(20usize) }
    fn gas_sload(&self) -> Gas { Gas::from(50usize) }
    fn gas_suicide(&self) -> Gas { Gas::from(0usize) }
    fn gas_suicide_new_account(&self) -> Gas { Gas::from(0usize) }
    fn gas_call(&self) -> Gas { Gas::from(40usize) }
    fn gas_expbyte(&self) -> Gas { Gas::from(10usize) }
    fn gas_transaction_create(&self) -> Gas { Gas::from(32000usize) }
    fn force_code_deposit(&self) -> bool { false }
    fn has_delegate_call(&self) -> bool { true }
    fn has_static_call(&self) -> bool { false }
    fn has_revert(&self) -> bool { false }
    fn has_return_data(&self) -> bool { false }
    fn has_bitwise_shift(&self) -> bool { false }
    fn has_extcodehash(&self) -> bool { false }
    fn has_reduced_sstore_gas_metering(&self) -> bool { false }
    fn err_on_call_with_more_gas(&self) -> bool { true }
    fn call_create_l64_after_gas(&self) -> bool { false }
    fn memory_limit(&self) -> usize { usize::max_value() }
    fn precompileds(&self) -> &'static [(Address, Option<&'static [u8]>, &'static Precompiled)] {
        &FRONTIER_PRECOMPILEDS }
}

/// Spurious Dragon patch.
pub struct SpuriousDragonPatch;
impl Patch for SpuriousDragonPatch {
    type Account = StateClearingAccountPatch;

    fn code_deposit_limit(&self) -> Option<usize> { Some(0x6000) }
    fn callstack_limit(&self) -> usize { 1024 }
    fn gas_extcode(&self) -> Gas { Gas::from(700usize) }
    fn gas_balance(&self) -> Gas { Gas::from(400usize) }
    fn gas_sload(&self) -> Gas { Gas::from(200usize) }
    fn gas_suicide(&self) -> Gas { Gas::from(5000usize) }
    fn gas_suicide_new_account(&self) -> Gas { Gas::from(25000usize) }
    fn gas_call(&self) -> Gas { Gas::from(700usize) }
    fn gas_expbyte(&self) -> Gas { Gas::from(50usize) }
    fn gas_transaction_create(&self) -> Gas { Gas::from(32000usize) }
    fn force_code_deposit(&self) -> bool { false }
    fn has_delegate_call(&self) -> bool { true }
    fn has_static_call(&self) -> bool { false }
    fn has_revert(&self) -> bool { false }
    fn has_return_data(&self) -> bool { false }
    fn has_bitwise_shift(&self) -> bool { false }
    fn has_extcodehash(&self) -> bool { false }
    fn has_reduced_sstore_gas_metering(&self) -> bool { false }
    fn err_on_call_with_more_gas(&self) -> bool { false }
    fn call_create_l64_after_gas(&self) -> bool { true }
    fn memory_limit(&self) -> usize { usize::max_value() }
    fn precompileds(&self) -> &'static [(Address, Option<&'static [u8]>, &'static Precompiled)] {
        &FRONTIER_PRECOMPILEDS }
}

/// Spurious Dragon patch.
pub struct ByzantiumPatch;
impl Patch for ByzantiumPatch {
    type Account = StateClearingAccountPatch;

    fn code_deposit_limit(&self) -> Option<usize> { Some(0x6000) }
    fn callstack_limit(&self) -> usize { 1024 }
    fn gas_extcode(&self) -> Gas { Gas::from(700usize) }
    fn gas_balance(&self) -> Gas { Gas::from(400usize) }
    fn gas_sload(&self) -> Gas { Gas::from(200usize) }
    fn gas_suicide(&self) -> Gas { Gas::from(5000usize) }
    fn gas_suicide_new_account(&self) -> Gas { Gas::from(25000usize) }
    fn gas_call(&self) -> Gas { Gas::from(700usize) }
    fn gas_expbyte(&self) -> Gas { Gas::from(50usize) }
    fn gas_transaction_create(&self) -> Gas { Gas::from(32000usize) }
    fn force_code_deposit(&self) -> bool { false }
    fn has_delegate_call(&self) -> bool { true }
    fn has_static_call(&self) -> bool { true }
    fn has_revert(&self) -> bool { true }
    fn has_return_data(&self) -> bool { true }
    fn has_bitwise_shift(&self) -> bool { false }
    fn has_extcodehash(&self) -> bool { false }
    fn has_reduced_sstore_gas_metering(&self) -> bool { false }
    fn err_on_call_with_more_gas(&self) -> bool { false }
    fn call_create_l64_after_gas(&self) -> bool { true }
    fn memory_limit(&self) -> usize { usize::max_value() }
    fn precompileds(&self) -> &'static [(Address, Option<&'static [u8]>, &'static Precompiled)] {
        &BYZANTIUM_PRECOMPILEDS }
}
