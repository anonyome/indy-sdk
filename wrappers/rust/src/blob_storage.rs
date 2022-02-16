use futures::Future;

use crate::{ErrorCode, IndyError};

use std::ffi::CString;

use crate::ffi::blob_storage;
use crate::ffi::ResponseI32CB;

use crate::utils::callbacks::{ClosureHandler, ResultHandler};
use crate::{IndyHandle, CommandHandle};

// UNIFFI:  Added this convenience method, so the uniffi code can connect with a function without a Future.
pub fn u_open_reader(xtype: &str, config_json: &str) -> Result<i32, IndyError> {
   
    return open_reader(xtype, config_json).wait();
}

pub fn open_reader(xtype: &str, config_json: &str) -> Box<dyn Future<Item=IndyHandle, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_handle();

    let err = _open_reader(command_handle, xtype, config_json, cb);

    ResultHandler::handle(command_handle, err, receiver)
}

fn _open_reader(command_handle: CommandHandle, xtype: &str, config_json: &str, cb: Option<ResponseI32CB>) -> ErrorCode {
    let xtype = c_str!(xtype);
    let config_json = c_str!(config_json);

    ErrorCode::from(unsafe { blob_storage::indy_open_blob_storage_reader(command_handle, xtype.as_ptr(), config_json.as_ptr(), cb) })
}

// UNIFFI:  Added this convenience method, so the uniffi code can connect with a function without a Future.
pub fn u_open_writer(xtype: &str, config_json: &str) -> Result<i32, IndyError> {
    
    return open_writer(xtype, config_json).wait();
}

pub fn open_writer(xtype: &str, config_json: &str) -> Box<dyn Future<Item=CommandHandle, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_handle();

    let err = _open_writer(command_handle, xtype, config_json, cb);

    ResultHandler::handle(command_handle, err, receiver)
}

fn _open_writer(command_handle: CommandHandle, xtype: &str, config_json: &str, cb: Option<ResponseI32CB>) -> ErrorCode {
    let xtype = c_str!(xtype);
    let config_json = c_str!(config_json);

    ErrorCode::from(unsafe { blob_storage::indy_open_blob_storage_writer(command_handle, xtype.as_ptr(), config_json.as_ptr(), cb) })
}
