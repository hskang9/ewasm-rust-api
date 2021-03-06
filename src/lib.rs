/// ewasm_api is a library used to interface with Ethereum's EEI in Ewasm, a set of enhancements to
/// the Ethereum smart contract platform.
/// ewasm_api exposes both a set of unsafe "native" functions representing the actual EEI
/// functions, and a set of safe wrappers around them. It is recommended not to use the native
/// functions as they do not perform bounds-checking.
///
/// To use ewasm_api, simply include it as a dependency in your project.
///
/// # Examples
/// ```
/// extern crate ewasm_api;
///
/// use ewasm_api::{block_hash, finish_data};
///
/// #[no_mangle]
/// pub extern "C" fn main() {
///     let a: Hash = block_hash(1);
///     finish_data(&a.bytes);
/// }
/// ```
use std::vec::Vec;

/// The native host interface exposed to the ewasm contract. Do not use these functions unless, for
/// some reason, the safe wrapper is not flexible enough.
mod native {
    extern "C" {
        pub fn ethereum_useGas(amount: u64);
        pub fn ethereum_getGasLeft() -> u64;
        pub fn ethereum_getAddress(resultOffset: *const u32);
        pub fn ethereum_getBalance(addressOffset: *const u32, resultOffset: *const u32);
        pub fn ethereum_getBlockCoinbase(resultOffset: *const u32);
        pub fn ethereum_getBlockDifficulty(resultOffset: *const u32);
        pub fn ethereum_getBlockGasLimit() -> u64;
        pub fn ethereum_getBlockHash(number: u64, resultOffset: *const u32) -> u32;
        pub fn ethereum_getBlockNumber() -> u64;
        pub fn ethereum_getBlockTimestamp() -> u64;
        pub fn ethereum_getTxGasPrice(valueOffset: *const u32);
        pub fn ethereum_getTxOrigin(resultOffset: *const u32);
        pub fn ethereum_log(
            dataOffset: *const u32,
            length: u32,
            numberOfTopics: u32,
            topic1: *const u32,
            topic2: *const u32,
            topic3: *const u32,
            topic4: *const u32,
        );
        pub fn ethereum_call(
            gas: u64,
            addressOffset: *const u32,
            valueOffset: *const u32,
            dataOffset: *const u32,
            dataLength: u32,
        ) -> u32;
        pub fn ethereum_callCode(
            gas: u64,
            addressOffset: *const u32,
            valueOffset: *const u32,
            dataOffset: *const u32,
            dataLength: u32,
        ) -> u32;
        pub fn ethereum_callDelegate(
            gas: u64,
            addressOffset: *const u32,
            dataOffset: *const u32,
            dataLength: u32,
        ) -> u32;
        pub fn ethereum_callStatic(
            gas: u64,
            addressOffset: *const u32,
            dataOffset: *const u32,
            dataLength: u32,
        ) -> u32;
        pub fn ethereum_create(
            valueOffset: *const u32,
            dataOffset: *const u32,
            dataLength: u32,
            resultOffset: *const u32,
        ) -> u32;
        pub fn ethereum_returnDataCopy(resultOffset: *const u32, dataOffset: u32, length: u32);
        pub fn ethereum_getReturnDataSize() -> u32;
        pub fn ethereum_finish(dataOffset: *const u32, length: u32) -> !;
        pub fn ethereum_revert(dataOffset: *const u32, length: u32) -> !;
        pub fn ethereum_callDataCopy(resultOffset: *const u32, dataOffset: u32, length: u32);
        pub fn ethereum_getCallDataSize() -> u32;
        pub fn ethereum_getCaller(resultOffset: *const u32);
        pub fn ethereum_getCallValue(resultOffset: *const u32);
        pub fn ethereum_codeCopy(resultOffset: *const u32, codeOffset: u32, length: u32);
        pub fn ethereum_getCodeSize() -> u32;
        pub fn ethereum_externalCodeCopy(
            addressOffset: *const u32,
            resultOffset: *const u32,
            codeOffset: u32,
            length: u32,
        );
        pub fn ethereum_getExternalCodeSize(addressOfset: *const u32) -> u32;
        pub fn ethereum_storageLoad(keyOffset: *const u32, resultOffset: *const u32);
        pub fn ethereum_storageStore(keyOffset: *const u32, valueOffset: *const u32);
        pub fn ethereum_selfDestruct(addressOffset: *const u32) -> !;
    }
}

fn unsafe_alloc_buffer(len: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(len);
    unsafe {
        ret.set_len(len);
    }
    ret
}

/// A little-endian unsigned 128-bit integer.
#[derive(Default, Copy, Clone)]
pub struct Uint128 {
    pub bytes: [u8; 16],
}

/// A little-endian unsigned 256-bit integer.
#[derive(Default, Copy, Clone)]
pub struct Uint256 {
    pub bytes: [u8; 32],
}

/// An array of 160 bits.
#[derive(Default, Copy, Clone)]
pub struct Bytes20 {
    pub bytes: [u8; 20],
}

/// An array of 256 bits.
#[derive(Default, Copy, Clone)]
pub struct Bytes32 {
    pub bytes: [u8; 32],
}

type EtherValue = Uint128;
type Address = Bytes20;
type StorageKey = Bytes32;
type StorageValue = Bytes32;
type Topic = Bytes32;
type Hash = Bytes32;
type Difficulty = Uint256;

/// Enum representing an error code for EEI calls. Currently used by `codeCopy`, `callDataCopy`,
/// `externalCodeCopy`, and `returnDataCopy`.
pub enum Error {
    OutOfBoundsCopy,
}

/// Enum describing the result of a call. Used by `call`, `callCode`, `callDelegate`, and
/// `callStatic`.
pub enum CallResult {
    Successful,
    Failure,
    Revert,
}

/// Enum describing the result of `create`. On success, the data contained is the address of the
/// newly created contract.
pub enum CreateResult {
    Successful(Address),
    Failure,
    Revert,
}

/// Subtracts the given amount from the VM's gas counter. This is usually injected by the metering
/// contract at deployment time, and hence is unneeded in most cases.
pub fn consume_gas(amount: u64) {
    unsafe {
        native::ethereum_useGas(amount);
    }
}

/// Returns the gas left in the current call.
pub fn gas_left() -> u64 {
    unsafe { native::ethereum_getGasLeft() }
}

/// Returns the executing address.
pub fn current_address() -> Address {
    let mut ret = Address::default();

    unsafe {
        native::ethereum_getAddress(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Returns the balance of the address given.
pub fn external_balance(address: &Address) -> EtherValue {
    let mut ret = EtherValue::default();

    unsafe {
        native::ethereum_getBalance(
            address.bytes.as_ptr() as *const u32,
            ret.bytes.as_mut_ptr() as *const u32,
        );
    }

    ret
}

/// Returns the beneficiary address for the block this transaction is in (current block)
pub fn block_coinbase() -> Address {
    let mut ret = Address::default();

    unsafe {
        native::ethereum_getBlockCoinbase(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Returns the difficulty of the most recent block.
pub fn block_difficulty() -> Difficulty {
    let mut ret = Difficulty::default();

    unsafe {
        native::ethereum_getBlockDifficulty(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Returns the gas limit of the most recent block.
pub fn block_gas_limit() -> u64 {
    unsafe { native::ethereum_getBlockGasLimit() }
}

/// Returns the hash of the `number`th most recent block.
pub fn block_hash(number: u64) -> Hash {
    let mut ret = Hash::default();

    unsafe {
        native::ethereum_getBlockHash(number, ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Returns the number of the most recent block.
pub fn block_number() -> u64 {
    unsafe { native::ethereum_getBlockNumber() }
}

/// Returns the timestamp of the most recent block.
pub fn block_timestamp() -> u64 {
    unsafe { native::ethereum_getBlockTimestamp() }
}

/// Returns the gas price of the currently executing call.
pub fn tx_gas_price() -> EtherValue {
    let mut ret = EtherValue::default();

    unsafe {
        native::ethereum_getTxGasPrice(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Returns the address of the original transaction sender.
pub fn tx_origin() -> Address {
    let mut ret = Address::default();

    unsafe {
        native::ethereum_getTxOrigin(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Appends log data to the transaction receipt, with a variable number of topics.
fn log(
    data: &[u8],
    topic_count: usize,
    topic1: *const u8,
    topic2: *const u8,
    topic3: *const u8,
    topic4: *const u8,
) {
    unsafe {
        native::ethereum_log(
            data.as_ptr() as *const u32,
            data.len() as u32,
            topic_count as u32,
            topic1 as *const u32,
            topic2 as *const u32,
            topic3 as *const u32,
            topic4 as *const u32,
        );
    }
}

/// Appends log data without a topic.
pub fn log0(data: &[u8]) {
    log(
        data,
        0,
        0 as *const u8,
        0 as *const u8,
        0 as *const u8,
        0 as *const u8,
    )
}

/// Appends log data with one topic.
pub fn log1(data: &[u8], topic1: &Topic) {
    log(
        data,
        1,
        topic1.bytes.as_ptr() as *const u8,
        0 as *const u8,
        0 as *const u8,
        0 as *const u8,
    )
}

/// Appends log data with two topics.
pub fn log2(data: &[u8], topic1: &Topic, topic2: &Topic) {
    log(
        data,
        2,
        topic1.bytes.as_ptr() as *const u8,
        topic2.bytes.as_ptr() as *const u8,
        0 as *const u8,
        0 as *const u8,
    )
}

/// Appends log data with three topics.
pub fn log3(data: &[u8], topic1: &Topic, topic2: &Topic, topic3: &Topic) {
    log(
        data,
        3,
        topic1.bytes.as_ptr() as *const u8,
        topic2.bytes.as_ptr() as *const u8,
        topic3.bytes.as_ptr() as *const u8,
        0 as *const u8,
    )
}

/// Appends log data with four topics.
pub fn log4(data: &[u8], topic1: &Topic, topic2: &Topic, topic3: &Topic, topic4: &Topic) {
    log(
        data,
        4,
        topic1.bytes.as_ptr() as *const u8,
        topic2.bytes.as_ptr() as *const u8,
        topic3.bytes.as_ptr() as *const u8,
        topic4.bytes.as_ptr() as *const u8,
    )
}

/// Executes a standard call to the specified address with the given gas limit, ether value, and
/// data.
pub fn call_mutable(
    gas_limit: u64,
    address: &Address,
    value: &EtherValue,
    data: &[u8],
) -> CallResult {
    let ret = unsafe {
        native::ethereum_call(
            gas_limit,
            address.bytes.as_ptr() as *const u32,
            value.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => panic!(),
    }
}

/// Executes another account's code in the context of the caller.
pub fn call_code(gas_limit: u64, address: &Address, value: &EtherValue, data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_callCode(
            gas_limit,
            address.bytes.as_ptr() as *const u32,
            value.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => panic!(),
    }
}

/// Executes a call similar to `call_code`, but retaining the currently executing call's sender
/// and value.
pub fn call_delegate(gas_limit: u64, address: &Address, data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_callDelegate(
            gas_limit,
            address.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => panic!(),
    }
}

/// Executes a static call which cannot mutate the state.
pub fn call_static(gas_limit: u64, address: &Address, data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_callStatic(
            gas_limit,
            address.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => panic!(),
    }
}

/// Creates a contract with the the given code, sending the specified ether value to its address.
pub fn create(value: &EtherValue, data: &[u8]) -> CreateResult {
    let mut address = Address::default();

    let ret = unsafe {
        native::ethereum_create(
            value.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
            address.bytes.as_mut_ptr() as *const u32,
        )
    };

    match ret {
        0 => CreateResult::Successful(address),
        1 => CreateResult::Failure,
        2 => CreateResult::Revert,
        _ => panic!(),
    }
}

/// Executes callDataCopy, but does not check for overflow.
pub fn unsafe_calldata_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        native::ethereum_callDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }

    ret
}

/// Returns a vector containing all data passed with the currently executing call.
pub fn calldata_acquire() -> Vec<u8> {
    unsafe_calldata_copy(0, calldata_size())
}

/// Returns the segment of call data beginning at `from`, and continuing for `length` bytes.
pub fn calldata_copy(from: usize, length: usize) -> Result<Vec<u8>, Error> {
    let size = calldata_size();

    if (size < from) || ((size - from) < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        Ok(unsafe_calldata_copy(from, length))
    }
}

/// Returns the length of the call data supplied with the currently executing call.
pub fn calldata_size() -> usize {
    unsafe { native::ethereum_getCallDataSize() as usize }
}

/// Returns the sender of the currently executing call.
pub fn caller() -> Address {
    let mut ret = Address::default();

    unsafe {
        native::ethereum_getCaller(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Returns the value sent with the currently executing call.
pub fn callvalue() -> EtherValue {
    let mut ret = EtherValue::default();

    unsafe {
        native::ethereum_getCallValue(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Executes codeCopy, but does not check for overflow.
pub fn unsafe_code_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        native::ethereum_codeCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }

    ret
}

/// Returns the currently executing code.
pub fn code_acquire() -> Vec<u8> {
    unsafe_code_copy(0, code_size())
}

/// Copies the segment of running code beginning at `from` and continuing for `length` bytes.
pub fn code_copy(from: usize, length: usize) -> Result<Vec<u8>, Error> {
    let size = code_size();

    if (size < from) || ((size - from) < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        Ok(unsafe_code_copy(from, length))
    }
}

/// Returns the size of the currently executing code.
pub fn code_size() -> usize {
    unsafe { native::ethereum_getCodeSize() as usize }
}

/// Executes externalCodeCopy, but does not check for overflow.
pub fn unsafe_external_code_copy(address: &Address, from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        native::ethereum_externalCodeCopy(
            address.bytes.as_ptr() as *const u32,
            ret.as_mut_ptr() as *const u32,
            from as u32,
            length as u32,
        );
    }

    ret
}

/// Returns the code at the specified address.
pub fn external_code_acquire(address: &Address) -> Vec<u8> {
    unsafe_external_code_copy(address, 0, external_code_size(address))
}

/// Returns the segment of code at `address` beginning at `from` and continuing for `length` bytes.
pub fn external_code_copy(address: &Address, from: usize, length: usize) -> Result<Vec<u8>, Error> {
    let size = external_code_size(address);

    if (size < from) || ((size - from) < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        Ok(unsafe_external_code_copy(address, from, length))
    }
}

/// Returns the size of the code at the specified address.
pub fn external_code_size(address: &Address) -> usize {
    unsafe { native::ethereum_getExternalCodeSize(address.bytes.as_ptr() as *const u32) as usize }
}

/// Executes returnDataCopy, but does not check for overflow.
pub fn unsafe_returndata_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        native::ethereum_returnDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }

    ret
}

/// Returns the data in the VM's return buffer.
pub fn returndata_acquire() -> Vec<u8> {
    unsafe_returndata_copy(0, returndata_size())
}

/// Returns the segment of return buffer data beginning at `from` and continuing for `length` bytes.
pub fn returndata_copy(from: usize, length: usize) -> Result<Vec<u8>, Error> {
    let size = returndata_size();

    if (size < from) || ((size - from) < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        Ok(unsafe_returndata_copy(from, length))
    }
}

/// Returns the length of the data in the VM's return buffer.
pub fn returndata_size() -> usize {
    unsafe { native::ethereum_getReturnDataSize() as usize }
}

/// Halts execution and reverts all changes to the state.
pub fn revert() -> ! {
    unsafe {
        native::ethereum_revert(0 as *const u32, 0 as u32);
    }
}

/// Fills the return buffer with the given data and halts execution, reverting all state changes.
pub fn revert_data(data: &[u8]) -> ! {
    unsafe {
        native::ethereum_revert(data.as_ptr() as *const u32, data.len() as u32);
    }
}

/// Ends execution, signalling success.
pub fn finish() -> ! {
    unsafe {
        native::ethereum_finish(0 as *const u32, 0 as u32);
    }
}

/// Fills the return buffer with the given data and halts execution, signalling success.
pub fn finish_data(data: &[u8]) -> ! {
    unsafe {
        native::ethereum_finish(data.as_ptr() as *const u32, data.len() as u32);
    }
}

/// Accesses the storage data at the specified key.
pub fn storage_load(key: &StorageKey) -> StorageValue {
    let mut ret = StorageValue::default();

    unsafe {
        native::ethereum_storageLoad(
            key.bytes.as_ptr() as *const u32,
            ret.bytes.as_mut_ptr() as *const u32,
        );
    }

    ret
}

/// Sets the storage data at the specified key.
pub fn storage_store(key: &StorageKey, value: &StorageValue) {
    unsafe {
        native::ethereum_storageStore(
            key.bytes.as_ptr() as *const u32,
            value.bytes.as_ptr() as *const u32,
        );
    }
}

/// Self-destructs the running contract, sending all its ether to a specified beneficiary address.
pub fn selfdestruct(address: &Address) -> ! {
    unsafe {
        native::ethereum_selfDestruct(address.bytes.as_ptr() as *const u32);
    }
}
