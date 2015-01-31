extern crate cql_ffi;

use cql_ffi_safe::string::CassString;
use cql_ffi_safe::error::CassError;
use cql_ffi_safe::result::CassResult;
use cql_ffi_safe::bytes::CassBytes;
use cql_ffi_safe::uuid::CassUuid;
use cql_ffi_safe::inet::CassInet;
use cql_ffi_safe::decimal::CassDecimal;
use cql_ffi_safe::collection::CassCollection;
use cql_ffi_safe::value::CassBindable;

use cql_ffi::CassConsistency;
use cql_ffi::CassError::CASS_OK;

use std::str::FromStr;

#[derive(Debug)]
pub struct CassStatement<'a> {
    pub statement:&'a mut cql_ffi::CassStatement
}
impl<'a> CassStatement<'a> {
    pub fn new(query:&str, parameter_count: u64) -> CassStatement {unsafe{
        println!("param_count: {}",parameter_count);
        let string:CassString = FromStr::from_str(query).unwrap();
        println!("cassstring: {:?}",string);
        let statement = cql_ffi::cass_statement_new(string.string, parameter_count);
        CassStatement{statement:&mut*statement}
    }}

    pub fn add_key_index(&mut self, index: u64) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_add_key_index(self.statement, index) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn set_keyspace(&mut self, keyspace: &str) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_set_keyspace(self.statement, keyspace.as_ptr() as *const i8) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn set_consistency(&mut self, consistency: CassConsistency) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_set_consistency(self.statement, consistency) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn set_serial_consistency(&mut self, serial_consistency: CassConsistency) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_set_serial_consistency(self.statement, serial_consistency) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn set_paging_size(&mut self, page_size: i32) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_set_paging_size(self.statement, page_size) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn set_paging_state(&mut self, result: CassResult) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_set_paging_state(self.statement, result.result) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_all(&mut self, values:Vec<CassBindable>) -> Result<(),CassError> {
        let mut idx = 1;
        for value in values.iter() {
            match value {
                &CassBindable::BOOL(val) => try!(self.bind_bool(idx, val)),
                &CassBindable::I32(val) => try!(self.bind_i32(idx,val)),
                &CassBindable::I64(val) => try!(self.bind_i64(idx,val)),
                &CassBindable::F32(val) => try!(self.bind_f32(idx,val)),
                &CassBindable::F64(val) => try!(self.bind_f64(idx,val)),
                &CassBindable::BLOB(ref val) => try!(self.bind_bytes(idx,val)),
                &CassBindable::UUID(_) => return Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
                &CassBindable::STR(ref val) => try!(self.bind_string(idx,val.as_slice())),
            };
            idx += 1;
        }
        Ok(())
    }

    pub fn bind_null(&mut self, index: u64) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_null(self.statement, index) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_i32(&mut self, index: u64, value:i32) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_int32(self.statement, index, value) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_i64(&mut self, index: u64, value:i64) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_int64(self.statement, index, value) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_f32(&mut self, index: u64, value:f32) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_float(self.statement, index, value) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_f64(&mut self, index: u64, value:f64) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_double(self.statement, index, value) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_bool(&mut self, index: u64, value:bool) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_bool(self.statement, index, if value {1} else {0}) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_string(&mut self, index: u64, value:&str) -> Result<(),CassError> {unsafe{
        let string:CassString = FromStr::from_str(value).unwrap();
        match cql_ffi::cass_statement_bind_string(self.statement, index, string.string) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_bytes(&mut self, index: u64, value:&Vec<u8>) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_bytes(self.statement, index, CassBytes::new(value.as_slice()).bytes) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    //FIXME make this take a uuid::Uuid
    pub fn bind_uuid(&mut self, index: u64, uuid:CassUuid) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_uuid(self.statement, index, uuid.uuid) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    //FIXME make this take a TcpAddr
    pub fn bind_inet(&mut self, index: u64, inet:CassInet) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_inet(self.statement, index, inet.inet) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    //FIXME make this take a rust Decimal
    pub fn bind_decimal(&mut self, index: u64, decimal:CassDecimal) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_decimal(self.statement, index, decimal.decimal) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    //FIXME what the frack?
    //~ pub fn bind_custom(&mut self, index: u64, &[u8]) -> Result<(),CassError> {unsafe{
        //~ match cql_ffi::cass_statement_bind_decimal(self.statement, index, decimal.decimal) {
            //~ CASS_OK => Ok(()),
            //~ rc => Err(CassError{error:rc})
        //~ }
    //~ }}
    
    pub fn bind_collection(&mut self, index: u64, collection:CassCollection) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_collection(self.statement, index, collection.collection) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_i32_by_name(&mut self, name: &str, val:i32) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_int32_by_name(self.statement, name.as_ptr() as *const i8, val) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_i64_by_name(&mut self, name: &str, val:i64) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_int64_by_name(self.statement, name.as_ptr() as *const i8, val) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_f32_by_name(&mut self, name: &str, val:f32) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_float_by_name(self.statement, name.as_ptr() as *const i8, val) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_f64_by_name(&mut self, name: &str, val:f64) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_double_by_name(self.statement, name.as_ptr() as *const i8, val) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_bool_by_name(&mut self, name: &str, val:bool) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_bool_by_name(self.statement, name.as_ptr() as *const i8, if val {1} else {0}) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_string_by_name(&mut self, name: &str, value:&str) -> Result<(),CassError> {unsafe{
        let string:CassString = FromStr::from_str(value).unwrap();
        match cql_ffi::cass_statement_bind_string_by_name(self.statement, name.as_ptr() as *const i8, string.string) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn bind_bytes_by_name(&mut self, name: &str, value:&[u8]) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_bytes_by_name(self.statement, name.as_ptr() as *const i8, CassBytes::new(value).bytes) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    //FIXME this should take a uuid::Uuid
    pub fn bind_uuid_by_name(&mut self, name: &str, uuid:CassUuid) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_uuid_by_name(self.statement, name.as_ptr() as *const i8, uuid.uuid) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    //FIXME this should take a TcpAddr
    pub fn bind_inet_by_name(&mut self, name: &str, inet:CassInet) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_inet_by_name(self.statement, name.as_ptr() as *const i8, inet.inet) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    //FIXME this should take a native rust decimal type
    pub fn bind_decimal_by_name(&mut self, name: &str, decimal:CassDecimal) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_decimal_by_name(self.statement, name.as_ptr() as *const i8, decimal.decimal) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    //FIXME wtf
    //~ pub fn bind_custom_by_name(&mut self, name: &str, custom:&[u8]) -> Result<(),CassError> {unsafe{
        //~ match cql_ffi::cass_statement_bind_custom_by_name(self.statement, name.as_ptr() as *const i8, custom) {
            //~ CASS_OK => Ok(()),
            //~ rc => Err(CassError{error:rc})
        //~ }
    //~ }}

    //FIXME should be able to pass in normal rust collections
    pub fn bind_collection_by_name(&mut self, name: &str, collection:CassCollection) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_statement_bind_collection_by_name(self.statement, name.as_ptr() as *const i8, collection.collection) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}
 }

#[unsafe_destructor]
impl<'a> Drop for CassStatement<'a> {
    fn drop(&mut self) {unsafe{
        cql_ffi::cass_statement_free(self.statement)
    }}
}

