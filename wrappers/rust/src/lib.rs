include!("lib.uniffi.rs");

extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate libc;
extern crate failure;
extern crate num_traits;
#[macro_use]
extern crate num_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate indy_sys as ffi;

#[macro_use]
mod macros;

use thiserror::Error;

pub use futures::future;
use libc::c_char;

pub mod anoncreds;
pub mod blob_storage;
pub mod crypto;
pub mod did;
pub mod ledger;
pub mod logger;
pub mod payments;
pub mod pairwise;
pub mod pool;
pub mod wallet;
pub mod cache;
pub mod metrics;
mod utils;

use std::ffi::CString;
use std::fmt;
use std::ptr;
use std::ffi::CStr;

use failure::{Backtrace, Fail};

pub use crate::ffi::{
    RecordHandle,
    TailWriterHandle,
    BlobStorageReaderHandle,
    BlobStorageReaderCfgHandle,
    MetadataHandle,
    Timeout,
    TailsWriterHandle,
    IndyHandle,
    CommandHandle,
    WalletHandle,
    PoolHandle,
    SearchHandle,
    StorageHandle,
    INVALID_WALLET_HANDLE,
    INVALID_POOL_HANDLE,
    INVALID_COMMAND_HANDLE
};

/// Set libindy runtime configuration. Can be optionally called to change current params.
///
/// # Arguments
/// * `config` - {
///     "crypto_thread_pool_size": <int> - size of thread pool for the most expensive crypto operations. (4 by default)
/// }
pub fn set_runtime_config(config: &str) -> ErrorCode {
    let config = c_str!(config);

    ErrorCode::from(unsafe {
        ffi::indy_set_runtime_config(config.as_ptr())
    })
}

#[derive(Error, Debug, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
#[repr(i32)]
#[allow(dead_code)]
pub enum ErrorCode
{
    #[error("Success")]
    Success = 0,
    // Common errors

    // Caller passed invalid value as param 1 (null, invalid json and etc..)
    #[error("CommonInvalidParam1")]
    CommonInvalidParam1 = 100,
    // Caller passed invalid value as param 2 (null, invalid json and etc..)
    #[error("CommonInvalidParam2")]
    CommonInvalidParam2 = 101,
    // Caller passed invalid value as param 3 (null, invalid json and etc..)
    #[error("CommonInvalidParam3")]
    CommonInvalidParam3 = 102,
    // Caller passed invalid value as param 4 (null, invalid json and etc..)
    #[error("CommonInvalidParam4")]
    CommonInvalidParam4 = 103,

    // Caller passed invalid value as param 5 (null, invalid json and etc..)
    #[error("CommonInvalidParam5")]
    CommonInvalidParam5 = 104,
    // Caller passed invalid value as param 6 (null, invalid json and etc..)
    #[error("CommonInvalidParam6")]
    CommonInvalidParam6 = 105,
    // Caller passed invalid value as param 7 (null, invalid json and etc..)
    #[error("CommonInvalidParam7")]
    CommonInvalidParam7 = 106,
    // Caller passed invalid value as param 8 (null, invalid json and etc..)
    #[error("CommonInvalidParam8")]
    CommonInvalidParam8 = 107,
    // Caller passed invalid value as param 9 (null, invalid json and etc..)
    #[error("CommonInvalidParam9")]
    CommonInvalidParam9 = 108,

    // Caller passed invalid value as param 10 (null, invalid json and etc..)
    #[error("CommonInvalidParam10")]
    CommonInvalidParam10 = 109,
    // Caller passed invalid value as param 11 (null, invalid json and etc..)
    #[error("CommonInvalidParam11")]
    CommonInvalidParam11 = 110,
    // Caller passed invalid value as param 11 (null, invalid json and etc..)
    #[error("CommonInvalidParam12")]
    CommonInvalidParam12 = 111,
    // Invalid library state was detected in runtime. It signals library bug
    #[error("CommonInvalidState")]
    CommonInvalidState = 112,
    // Object (json, config, key, credential and etc...) passed by library caller has invalid structure
    #[error("CommonInvalidStructure")]
    CommonInvalidStructure = 113,

    // IO Error
    #[error("CommonIOError")]
    CommonIOError = 114,
    // Caller passed invalid value as param 13 (null, invalid json and etc..)
    #[error("CommonInvalidParam13")]
    CommonInvalidParam13 = 115,
    // Caller passed invalid value as param 14 (null, invalid json and etc..)
    #[error("CommonInvalidParam14")]
    CommonInvalidParam14 = 116,
    // Caller passed invalid value as param 15 (null, invalid json and etc..)
    #[error("CommonInvalidParam15")]
    CommonInvalidParam15 = 117,
    // Caller passed invalid value as param 16 (null, invalid json and etc..)
    #[error("CommonInvalidParam16")]
    CommonInvalidParam16 = 118,

    // Caller passed invalid value as param 17 (null, invalid json and etc..)
    #[error("CommonInvalidParam17")]
    CommonInvalidParam17 = 119,
    // Caller passed invalid value as param 18 (null, invalid json and etc..)
    #[error("CommonInvalidParam18")]
    CommonInvalidParam18 = 120,
    // Caller passed invalid value as param 19 (null, invalid json and etc..)
    #[error("CommonInvalidParam19")]
    CommonInvalidParam19 = 121,
    // Caller passed invalid value as param 20 (null, invalid json and etc..)
    #[error("CommonInvalidParam20")]
    CommonInvalidParam20 = 122,
    // Caller passed invalid value as param 21 (null, invalid json and etc..)
    #[error("CommonInvalidParam21")]
    CommonInvalidParam21 = 123,

    // Caller passed invalid value as param 22 (null, invalid json and etc..)
    #[error("CommonInvalidParam22")]
    CommonInvalidParam22 = 124,
    // Caller passed invalid value as param 23 (null, invalid json and etc..)
    #[error("CommonInvalidParam23")]
    CommonInvalidParam23 = 125,
    // Caller passed invalid value as param 24 (null, invalid json and etc..)
    #[error("CommonInvalidParam24")]
    CommonInvalidParam24 = 126,
    // Caller passed invalid value as param 25 (null, invalid json and etc..)
    #[error("CommonInvalidParam25")]
    CommonInvalidParam25 = 127,
    // Caller passed invalid value as param 26 (null, invalid json and etc..)
    #[error("CommonInvalidParam26")]
    CommonInvalidParam26 = 128,

    // Caller passed invalid value as param 27 (null, invalid json and etc..)
    #[error("CommonInvalidParam27")]
    CommonInvalidParam27 = 129,
    // Wallet errors
    // Caller passed invalid wallet handle
    #[error("WalletInvalidHandle")]
    WalletInvalidHandle = 200,
    // Unknown type of wallet was passed on create_wallet
    #[error("WalletUnknownTypeError")]
    WalletUnknownTypeError = 201,
    // Attempt to register already existing wallet type
    #[error("WalletTypeAlreadyRegisteredError")]
    WalletTypeAlreadyRegisteredError = 202,
    // Attempt to create wallet with name used for another exists wallet
    #[error("WalletAlreadyExistsError")]
    WalletAlreadyExistsError = 203,

    // Requested entity id isn't present in wallet
    #[error("WalletNotFoundError")]
    WalletNotFoundError = 204,
    // Trying to use wallet with pool that has different name
    #[error("WalletIncompatiblePoolError")]
    WalletIncompatiblePoolError = 205,
    // Trying to open wallet that was opened already
    #[error("WalletAlreadyOpenedError")]
    WalletAlreadyOpenedError = 206,
    // Attempt to open encrypted wallet with invalid credentials
    #[error("WalletAccessFailed")]
    WalletAccessFailed = 207,
    // Input provided to wallet operations is considered not valid
    #[error("WalletInputError")]
    WalletInputError = 208,

    // Decoding of wallet data during input/output failed
    #[error("WalletDecodingError")]
    WalletDecodingError = 209,
    // Storage error occurred during wallet operation
    #[error("WalletStorageError")]
    WalletStorageError = 210,
    // Error during encryption-related operations
    #[error("WalletEncryptionError")]
    WalletEncryptionError = 211,
    // Requested wallet item not found
    #[error("WalletItemNotFound")]
    WalletItemNotFound = 212,
    // Returned if wallet's add_record operation is used with record name that already exists
    #[error("WalletItemAlreadyExists")]
    WalletItemAlreadyExists = 213,

    // Returned if provided wallet query is invalid
    #[error("WalletQueryError")]
    WalletQueryError = 214,
    // Ledger errors
    // Trying to open pool ledger that wasn't created before
    #[error("PoolLedgerNotCreatedError")]
    PoolLedgerNotCreatedError = 300,
    // Caller passed invalid pool ledger handle
//    #[error("PoolLedgerInvalidPoolHandle")]
//    PoolLedgerInvalidPoolHandle = 301,
    #[error("PoolLedgerInvalidi32")]
    PoolLedgerInvalidi32 = 301,
    // Pool ledger terminated
    #[error("PoolLedgerTerminated")]
    PoolLedgerTerminated = 302,
    // No concensus during ledger operation
    #[error("LedgerNoConsensusError")]
    LedgerNoConsensusError = 303,

    // Attempt to parse invalid transaction response
    #[error("LedgerInvalidTransaction")]
    LedgerInvalidTransaction = 304,
    // Attempt to send transaction without the necessary privileges
    #[error("LedgerSecurityError")]
    LedgerSecurityError = 305,
    // Attempt to create pool ledger config with name used for another existing pool
    #[error("PoolLedgerConfigAlreadyExistsError")]
    PoolLedgerConfigAlreadyExistsError = 306,
    // Timeout for action
    #[error("PoolLedgerTimeout")]
    PoolLedgerTimeout = 307,
    // Attempt to open Pool for witch Genesis Transactions are not compatible with set Protocol version.
    // Call pool.indy_set_protocol_version to set correct Protocol version.
    #[error("PoolIncompatibleProtocolVersion")]
    PoolIncompatibleProtocolVersion = 308,

    // Item not found on ledger.
    #[error("LedgerNotFound")]
    LedgerNotFound = 309,

    // Revocation registry is full and creation of new registry is necessary
    #[error("AnoncredsRevocationRegistryFullError")]
    AnoncredsRevocationRegistryFullError = 400,
    #[error("AnoncredsInvalidUserRevocId")]
    AnoncredsInvalidUserRevocId = 401,
    // Attempt to generate master secret with duplicated name
    #[error("AnoncredsMasterSecretDuplicateNameError")]
    AnoncredsMasterSecretDuplicateNameError = 404,
    #[error("AnoncredsProofRejected")]
    AnoncredsProofRejected = 405,
    #[error("AnoncredsCredentialRevoked")]
    AnoncredsCredentialRevoked = 406,

    // Attempt to create credential definition with duplicated did schema pair
    #[error("AnoncredsCredDefAlreadyExistsError")]
    AnoncredsCredDefAlreadyExistsError = 407,
    // Signus errors
    // Unknown format of DID entity keys
    #[error("UnknownCryptoTypeError")]
    UnknownCryptoTypeError = 500,
    // Attempt to create duplicate did
    #[error("DidAlreadyExistsError")]
    DidAlreadyExistsError = 600,
    // Unknown payment method was given
    #[error("UnknownPaymentMethod")]
    UnknownPaymentMethod = 700,
    //No method were scraped from inputs/outputs or more than one were scraped
    #[error("IncompatiblePaymentError")]
    IncompatiblePaymentError = 701,

    // Insufficient funds on inputs
    #[error("PaymentInsufficientFundsError")]
    PaymentInsufficientFundsError = 702,

    // No such source on a ledger
    #[error("PaymentSourceDoesNotExistError")]
    PaymentSourceDoesNotExistError = 703,

    // Operation is not supported for payment method
    #[error("PaymentOperationNotSupportedError")]
    PaymentOperationNotSupportedError = 704,

    // Extra funds on inputs
    #[error("PaymentExtraFundsError")]
    PaymentExtraFundsError = 705,

    // The transaction is not allowed to a requester
    #[error("The transaction is not allowed to a requester")]
    TransactionNotAllowed,
}


impl From<i32> for ErrorCode {
    fn from(i: i32) -> Self {
        let conversion = num_traits::FromPrimitive::from_i32(i);
        if conversion.is_some() {
            conversion.unwrap()
        } else {
            panic!("Unable to convert from {}, unknown error code", i)
        }
    }
}

impl Into<i32> for ErrorCode {
    fn into(self) -> i32 {
        num_traits::ToPrimitive::to_i32(&self).unwrap()
    }
}

// IndyError2
// Purpose:
//  Converts an IndyError into IndyError2 format, which serves to pack the 
//  IndyError fields into a JSON string.  This enables the result to be more easily 
//  passed back across the uniffi interface.
#[derive(thiserror::Error, Debug)]
pub enum IndyError2 {
    #[error("{{\"error_code\" : {a}, \"message\" : {b}, \"indy_backtrace\" : {c}}}")]
    ErrorResult { a: ErrorCode, b: String, c: String },
}


#[derive(Debug)]
pub struct IndyError {
    pub error_code: ErrorCode,
    pub message: String,
    pub indy_backtrace: Option<String>
}

impl Fail for IndyError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.error_code.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> { self.error_code.backtrace() }
}

impl fmt::Display for IndyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)?;
        Ok(())
    }
}

impl IndyError {
    pub(crate) fn new(error_code: ErrorCode) -> Self {
        let mut error_json_p: *const c_char = ptr::null();

        unsafe { ffi::indy_get_current_error(&mut error_json_p); }
        let error_json = opt_rust_str!(error_json_p);

        let error_json = match error_json {
            Some(error_json_) => error_json_,
            None => {
                return IndyError {
                    error_code: ErrorCode::CommonInvalidState,
                    message: String::from("Invalid ErrorMessage pointer"),
                    indy_backtrace: None,
                };
            }
        };

        match ::serde_json::from_str::<ErrorDetails>(&error_json) {
            Ok(error) => IndyError {
                error_code,
                message: error.message,
                indy_backtrace: error.backtrace,
            },
            Err(err) => IndyError {
                error_code: ErrorCode::CommonInvalidState,
                message: err.to_string(),
                indy_backtrace: None,
            }
        }
    }
}

#[derive(Deserialize)]
pub struct ErrorDetails {
    message: String,
    backtrace: Option<String>
}

pub struct StringString {
    i0: String,
    i1: String
}

pub struct StringOptString {
    i0: String,
    i1: Option<String>
}

// i0 = message
// i1 = ver_key
pub struct StringVecU8 {
    i0: String,
    i1: Vec<u8>
}

//----- IndyError Conversion Functions -----
fn indy_error_to_indy_error2(error: IndyError) -> IndyError2 {

    // Convert the optional string into a string, handling the None case.
    let c1: String = match error.indy_backtrace {
        Some(x) => x,
        None => "".to_string(),
    };

    // Create the IndyError2 and return it.
    IndyError2::ErrorResult{
        a: error.error_code,
        b: error.message,
        c: c1,
    }
}

//----------------------------------
// Function Groups included below:
// 1) blob_storage
// 2) Wallet
// 3) Pool
// 4) Did
// 5) Cache
// 6) Crypto

//----- blob_storage Functions -----
// UNIFFI:  Added this convenience method, so the uniffi code can connect with a function without a Future.
pub fn open_reader(xtype: &str, config_json: &str) -> Result<i32, IndyError2> {
    
    let r = blob_storage::u_open_reader(xtype, config_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };

    return r;
}

// UNIFFI:  Added this convenience method, so the uniffi code can connect with a function without a Future.
pub fn open_writer(xtype: &str, config_json: &str) -> Result<i32, IndyError2> {
    
    let r = blob_storage::u_open_writer(xtype, config_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };

    return r;
}

//----- Wallet Functions -----
pub fn register_wallet_storage(xtype: &str) -> Result<(), IndyError2> {

    let r = wallet::u_register_wallet_storage(xtype);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn create_wallet(config: &str, credentials: &str) -> Result<(), IndyError2> {

    let r = wallet::u_create_wallet(config, credentials);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn open_wallet(config: &str, credentials: &str) -> Result<i32, IndyError2> {

    let r = wallet::u_open_wallet(config, credentials);
    let r = match r {
        Ok(value) => serde::__private::Ok(value.0),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn export_wallet(wallet_handle: i32, export_config: &str) -> Result<(), IndyError2> {

    let r = wallet::u_export_wallet(WalletHandle(wallet_handle), export_config);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn import_wallet(config: &str, credentials: &str, import_config: &str) -> Result<(), IndyError2> {

    let r = wallet::u_import_wallet(config, credentials, import_config);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn delete_wallet(config: &str, credentials: &str) -> Result<(), IndyError2> {

    let r = wallet::u_delete_wallet(config, credentials);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn close_wallet(wallet_handle: i32) -> Result<(), IndyError2> {

    let r = wallet::u_close_wallet(WalletHandle(wallet_handle));
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn add_wallet_record(wallet_handle: i32, xtype: &str, id: &str, value: &str, tags_json: &str) -> Result<(), IndyError2> {

    let r = wallet::u_add_wallet_record(WalletHandle(wallet_handle), xtype, id, value, tags_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn update_wallet_record_value(wallet_handle: i32, xtype: &str, id: &str, value: &str) -> Result<(), IndyError2> {

    let r = wallet::u_update_wallet_record_value(WalletHandle(wallet_handle), xtype, id, value);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn update_wallet_record_tags(wallet_handle: i32, xtype: &str, id: &str, tags_json: &str) -> Result<(), IndyError2> {

    let r = wallet::u_update_wallet_record_tags(WalletHandle(wallet_handle), xtype, id, tags_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn add_wallet_record_tags(wallet_handle: i32, xtype: &str, id: &str, tags_json: &str) -> Result<(), IndyError2> {

    let r = wallet::u_add_wallet_record_tags(WalletHandle(wallet_handle), xtype, id, tags_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn delete_wallet_record_tags(wallet_handle: i32, xtype: &str, id: &str, tag_names_json: &str) -> Result<(), IndyError2> {

    let r = wallet::u_delete_wallet_record_tags(WalletHandle(wallet_handle), xtype, id, tag_names_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn delete_wallet_record(wallet_handle: i32, xtype: &str, id: &str) -> Result<(), IndyError2> {

    let r = wallet::u_delete_wallet_record(WalletHandle(wallet_handle), xtype, id);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn get_wallet_record(wallet_handle: i32, xtype: &str, id: &str, options_json: &str) -> Result<String, IndyError2> {

    let r = wallet::u_get_wallet_record(WalletHandle(wallet_handle), xtype, id, options_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn open_wallet_search(wallet_handle: i32, xtype: &str, query_json: &str, options_json: &str) -> Result<i32, IndyError2> {

    let r = wallet::u_open_wallet_search(WalletHandle(wallet_handle), xtype, query_json, options_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn fetch_wallet_search_next_records(wallet_handle: i32, wallet_search_handle: i32, count: u64) -> Result<String, IndyError2> {

    let r = wallet::u_fetch_wallet_search_next_records(WalletHandle(wallet_handle), wallet_search_handle as SearchHandle, 
        count as usize);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn close_wallet_search(wallet_search_handle: i32) -> Result<(), IndyError2> {

    let r = wallet::u_close_wallet_search(wallet_search_handle as SearchHandle);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn generate_wallet_key(config: &str) -> Result<String, IndyError2> {

    let r = wallet::u_generate_wallet_key(Some(config));
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

//-------------------------------
// Pool Functions
pub fn create_pool_ledger_config(pool_name: &str, pool_config: &str) -> Result<(), IndyError2> {

    let r = pool::u_create_pool_ledger_config(pool_name, Some(pool_config));
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn open_pool_ledger(pool_name: &str, config: &str) -> Result<i32, IndyError2> {

    let r = pool::u_open_pool_ledger(pool_name, Some(config));
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}    

pub fn refresh_pool_ledger(pool_handle: i32) -> Result<(), IndyError2> {

    let r = pool::u_refresh_pool_ledger(pool_handle as PoolHandle);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}    

pub fn list_pools() -> Result<String, IndyError2> {

    let r = pool::u_list_pools();
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn close_pool_ledger(pool_handle: i32) -> Result<(), IndyError2> {

    let r = pool::u_close_pool_ledger(pool_handle as PoolHandle);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn delete_pool_ledger(pool_name: &str) -> Result<(), IndyError2> {

    let r = pool::u_delete_pool_ledger(pool_name);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}    

pub fn set_protocol_version(protocol_version: u64) -> Result<(), IndyError2> {

    let r = pool::u_set_protocol_version(protocol_version as usize);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

//-------------------------------
// Did Functions 
pub fn create_and_store_my_did(wallet_handle: i32, did_json: &str) -> Result<StringString, IndyError2> {

    let r = did::u_create_and_store_my_did(WalletHandle(wallet_handle), did_json);
    let r = match r {
        Ok(value) => {
            let val = StringString {
                i0:String::from(value.0),
                i1:String::from(value.1)
            };
            serde::__private::Ok(val)
        },
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;

}

pub fn replace_keys_start(wallet_handle: i32, tgt_did: &str, dentity_json: &str) -> Result<String, IndyError2> {

    let r = did::u_replace_keys_start(WalletHandle(wallet_handle), tgt_did, dentity_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn replace_keys_apply(wallet_handle: i32, tgt_did: &str) -> Result<(), IndyError2> {

    let r = did::u_replace_keys_apply(WalletHandle(wallet_handle), tgt_did);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn store_their_did(wallet_handle: i32, identity_json: &str) -> Result<(), IndyError2> {

    let r = did::u_store_their_did(WalletHandle(wallet_handle), identity_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn key_for_did(pool_handle: i32, wallet_handle: i32, did: &str) -> Result<String, IndyError2> {

    let r = did::u_key_for_did(pool_handle as PoolHandle, WalletHandle(wallet_handle), did);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn key_for_local_did(wallet_handle: i32, did: &str) -> Result<String, IndyError2> {

    let r = did::u_key_for_local_did(WalletHandle(wallet_handle), did);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn set_endpoint_for_did(wallet_handle: i32, did: &str, address: &str, transport_key: &str) -> Result<(), IndyError2> {

    let r = did::u_set_endpoint_for_did(WalletHandle(wallet_handle), did, address, transport_key);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn get_endpoint_for_did(wallet_handle: i32, pool_handle: i32, did: &str) -> Result<StringOptString, IndyError2> {

    let r = did::u_get_endpoint_for_did(WalletHandle(wallet_handle), pool_handle as PoolHandle, did);
    let r = match r {
        Ok(value) => {
            let val = StringOptString {
                i0: String::from(value.0),
                i1: value.1
            };
            serde::__private::Ok(val)
        },
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn set_did_metadata(wallet_handle: i32, tgt_did: &str, metadata: &str) -> Result<(), IndyError2> {

    let r = did::u_set_did_metadata(WalletHandle(wallet_handle), tgt_did, metadata);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn get_did_metadata(wallet_handle: i32, tgt_did: &str) -> Result<String, IndyError2> {

    let r = did::u_get_did_metadata(WalletHandle(wallet_handle), tgt_did);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn get_my_did_with_metadata(wallet_handle: i32, my_did: &str) -> Result<String, IndyError2> {

    let r = did::u_get_my_did_with_metadata(WalletHandle(wallet_handle), my_did);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn list_my_dids_with_metadata(wallet_handle: i32) -> Result<String, IndyError2> {

    let r = did::u_list_my_dids_with_metadata(WalletHandle(wallet_handle));
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn abbreviate_verkey(tgt_did: &str, verkey: &str) -> Result<String, IndyError2> {

    let r = did::u_abbreviate_verkey(tgt_did, verkey);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn qualify_did(wallet_handle: i32, did: &str, method: &str) -> Result<String, IndyError2> {

    let r = did::u_qualify_did(WalletHandle(wallet_handle), did, method);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

//-------------------------------
// Cache Functions 
pub fn get_schema(pool_handle: i32, wallet_handle: i32, 
        submitter_did: &str, id: &str, options_json: &str) -> Result<String, IndyError2> {

    let r = cache::u_get_schema(pool_handle as PoolHandle, WalletHandle(wallet_handle), submitter_did, id, options_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
            
    return r;
} 

pub fn get_cred_def(pool_handle: i32, wallet_handle: i32, 
    submitter_did: &str, id: &str, options_json: &str) -> Result<String, IndyError2> {

    let r = cache::u_get_schema(pool_handle as PoolHandle, WalletHandle(wallet_handle), submitter_did, id, options_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
                
    return r;
}  

pub fn purge_schema_cache(wallet_handle: i32, options_json: &str) -> Result<(), IndyError2> {

    let r = cache::u_purge_schema_cache(WalletHandle(wallet_handle), options_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
                
    return r;
}  

pub fn purge_cred_def_cache(wallet_handle: i32, options_json: &str) -> Result<(), IndyError2> {

    let r = cache::u_purge_cred_def_cache(WalletHandle(wallet_handle), options_json);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
                
    return r;
}  

//-------------------------------
// Crypto Functions 
pub fn create_key(wallet_handle: i32, my_key_json: &str) -> Result<String, IndyError2> {

    let r = crypto::u_create_key(WalletHandle(wallet_handle), Some(my_key_json));
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn set_key_metadata(wallet_handle: i32, verkey: &str, metadata: &str) -> Result<(), IndyError2> {

    let r = crypto::u_set_key_metadata(WalletHandle(wallet_handle), verkey, metadata);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn get_key_metadata(wallet_handle: i32, verkey: &str) -> Result<String, IndyError2> {

    let r = crypto::u_get_key_metadata(WalletHandle(wallet_handle), verkey);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn sign(wallet_handle: i32, signer_vk: &str, message: &[u8]) -> Result<Vec<u8>, IndyError2> {

    let r = crypto::u_sign(WalletHandle(wallet_handle), signer_vk, message);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn verify(signer_vk: &str, message: &[u8], signature: &[u8]) -> Result<bool, IndyError2> {

    let r = crypto::u_verify(signer_vk, message, signature);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn auth_crypt(wallet_handle: i32, sender_vk: &str, recipient_vk: &str, message: Vec<u8>) -> Result<Vec<u8>, IndyError2>{

    let r = crypto::u_auth_crypt(WalletHandle(wallet_handle), sender_vk, recipient_vk, &message);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn auth_decrypt(wallet_handle: i32, recipient_vk: &str, encrypted_message: &[u8]) -> Result<StringVecU8, IndyError2> {

    let r = crypto::u_auth_decrypt(WalletHandle(wallet_handle), recipient_vk, encrypted_message);
    let r = match r {
        Ok(value) => {
            let val = StringVecU8 {
                i0:String::from(value.0),
                i1:Vec::from(value.1)
            };
            serde::__private::Ok(val)
        },
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn anon_crypt(recipient_vk: &str, message: Vec<u8>) -> Result<Vec<u8>, IndyError2>{

    let r = crypto::u_anon_crypt(recipient_vk, &message);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn anon_decrypt(wallet_handle: i32, recipient_vk: &str, encrypted_message: &[u8]) -> Result<Vec<u8>, IndyError2> {

    let r = crypto::u_anon_decrypt(WalletHandle(wallet_handle), recipient_vk, encrypted_message);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn pack_message(wallet_handle: i32, message: &[u8], receiver_keys: &str, sender: &str) -> Result<Vec<u8>, IndyError2> {

    let r = crypto::u_pack_message(WalletHandle(wallet_handle), message, receiver_keys, sender);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}

pub fn unpack_message(wallet_handle: i32, jwe: &[u8]) -> Result<Vec<u8>, IndyError2> {

    let r = crypto::u_unpack_message(WalletHandle(wallet_handle), jwe);
    let r = match r {
        Ok(value) => serde::__private::Ok(value),
        Err(error) => serde::__private::Err(indy_error_to_indy_error2(error)),
    };
    
    return r;
}