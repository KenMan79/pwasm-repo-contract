// Contract doesn't use standard library
#![no_std]

extern crate pwasm_ethereum;
extern crate pwasm_abi;
extern crate pwasm_repo_contract;

use pwasm_abi::eth::EndpointInterface;

/// The main function receives a pointer for the call descriptor.
#[no_mangle]
pub fn call() {
	let mut endpoint = pwasm_repo_contract::Endpoint::new(pwasm_repo_contract::RepoContractInstance::new());
	// Read http://solidity.readthedocs.io/en/develop/abi-spec.html#formal-specification-of-the-encoding for details
	pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}

#[no_mangle]
pub fn deploy() {
	let mut endpoint = pwasm_repo_contract::Endpoint::new(pwasm_repo_contract::RepoContractInstance::new());
	endpoint.dispatch_ctor(&pwasm_ethereum::input());
}
