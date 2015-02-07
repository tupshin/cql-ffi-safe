extern crate cql_ffi;

use cql_ffi_safe::error::CassError;
use cql_ffi_safe::uuid::CassUuid;
use cql_ffi_safe::value::CassValueType;
use cql_ffi_safe::string::CassString;
use cql_ffi_safe::bytes::CassBytes;
use cql_ffi_safe::decimal::CassDecimal;
use cql_ffi_safe::iterator::CassIterator;
use cql_ffi_safe::inet::CassInet;
use cql_ffi_safe::value::CassBindable;
use cql_ffi_safe::value::CassReturnable;

use std::mem;

pub use cql_ffi::CassColumnType;

#[allow(missing_copy_implementations)]
pub struct CassColumn(pub *const cql_ffi::CassValue);

pub trait FromCol {
    fn from_col(s: CassColumn) -> Result<Self, <Self as FromCol>::Err>;
}

impl FromCol for String {
    fn from_col(column: CassColumn) -> Result<Self,CassError> {
        Ok(try!(column.get_string()).to_string())
    }
}

impl FromCol for bool {
    fn from_col(column: CassColumn) -> Result<Self,CassError> {
        column.get_bool()
    }
}

impl FromCol for i64 {
    fn from_col(column: CassColumn) -> Result<Self,CassError> {
        column.get_int64()
    }
}

impl FromCol for i32 {
    fn from_col(column: CassColumn) -> Result<Self,CassError> {
        column.get_int32()
    }
}

impl FromCol for f32 {
    fn from_col(column: CassColumn) -> Result<Self,CassError> {
        column.get_float()
    }
}

impl FromCol for f64 {
    fn from_col(column: CassColumn) -> Result<Self,CassError> {
        column.get_double()
    }
}

impl FromCol for CassUuid {
    fn from_col(column: CassColumn) -> Result<Self,CassError> {
        column.get_uuid()
    }
}
impl FromCol for CassInet {
    fn from_col(column: CassColumn) -> Result<Self,CassError> {
        column.get_inet()
    }
}

impl FromCol for Vec<u8> {
    fn from_col(column: CassColumn) -> Result<Self,CassError> {
        Ok(try!(column.get_bytes()).as_bytes())
    }
}
impl CassColumn {

    pub fn get(&self) -> Result<CassReturnable, CassError> {
        use cql_ffi::CassValueType::*;
        match self.get_type() {
            UNKNOWN => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            CUSTOM => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            VARCHAR|ASCII|TEXT => Ok(CassReturnable::STR(try!(self.get_string()).to_string())),
            INT => Ok(CassReturnable::I32(try!(self.get_int32()))),
            BIGINT => Ok(CassReturnable::I64(try!(self.get_int64()))),
            BLOB => Ok(CassReturnable::BLOB(try!(self.get_bytes()).as_bytes())),
            BOOLEAN => Ok(CassReturnable::BOOL(try!(self.get_bool()))),
            COUNTER => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            DECIMAL => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            VARINT => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            TIMESTAMP => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            TIMEUUID => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            LIST => Ok(CassReturnable::LIST(self.collection_iter())),
            MAP => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            SET => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            INET => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            DOUBLE => Ok(CassReturnable::I64(try!(self.get_int64()))),
            FLOAT => Ok(CassReturnable::F32(try!(self.get_float()))),
            UUID => Ok(CassReturnable::UUID(try!(self.get_uuid()))),
        }
    }

    pub fn get_type(&self) -> CassValueType {unsafe{
        cql_ffi::cass_value_type(self.0)
    }}

    pub fn map_iter(&self) -> CassIterator {unsafe{
        CassIterator(cql_ffi::cass_iterator_from_map(self.0))
    }}

    pub fn collection_iter(&self) -> CassIterator {unsafe{
        CassIterator(cql_ffi::cass_iterator_from_collection(self.0))
    }}

    pub fn get_int32(&self) -> Result<i32, CassError> {unsafe{
       assert!(self.get_type() == CassValueType::INT);
        let ref mut output = 0i32;
        match cql_ffi::cass_value_get_int32(self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError(err))
        }
    }}

    pub fn get_int64(&self) -> Result<i64, CassError> {unsafe{
       assert!(self.get_type() == CassValueType::BIGINT);
        let ref mut output = 0i64;
        match cql_ffi::cass_value_get_int64(self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError(err))
        }
    }}

    pub fn get_float(&self) -> Result<f32, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::FLOAT);
        let ref mut output = 0f32;
        match cql_ffi::cass_value_get_float(self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError(err))
        }
    }}

    pub fn get_double(&self) -> Result<f64, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::DOUBLE);
        let ref mut output = 0f64;
        match cql_ffi::cass_value_get_double(self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError(err))
        }
    }}

    pub fn get_bool(&self) -> Result<bool, CassError> {unsafe{
        match self.get_type() {
            CassValueType::BOOLEAN => {
                let ref mut b_bln = 0u32;
                match cql_ffi::cass_value_get_bool(self.0,b_bln) {
                    cql_ffi::CassError::CASS_OK => Ok(true),
                    err => Err(CassError(err))
                }
            },
            other =>  panic!("can't get bool from a {:?}", other)
        }
    }}

    //FIXME this should emit a uuid::Uuid instead of a CassUuid
    pub fn get_uuid(&self) -> Result<CassUuid, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_uuid(self.0,output) {
            cql_ffi::CassError::CASS_OK => {
                Ok(CassUuid(output))
            },
            err => Err(CassError(err))
        }
    }}

    //FIXME this should emit a TcpAddr instead of a CassInet
    pub fn get_inet(&self) -> Result<CassInet, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_inet(self.0, output) {
             cql_ffi::CassError::CASS_OK => Ok(CassInet(output)),
            err => Err(CassError(err))
        }
    }}

    pub fn get_string(&self) -> Result<CassString, CassError> {unsafe{
        use cql_ffi::CassValueType::*;
        match self.get_type() {
            VARCHAR|ASCII|TEXT => {
                let mut output:cql_ffi::CassString =  mem::zeroed();
                match cql_ffi::cass_value_get_string(self.0,&mut output) {
                    cql_ffi::CassError::CASS_OK => Ok(CassString(output)),
                    err => Err(CassError(err))
                }
            },
            //FIXME replace this with returning a proper Result<_,CassError>
            other => panic!("can't get string from a {:?}", other)
        }
    }}

    pub fn get_bytes(&self) -> Result<CassBytes, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_bytes(self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassBytes(output)),
            err => Err(CassError(err))
        }
    }}

    pub fn get_decimal(&self) -> Result<CassDecimal, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_decimal(self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassDecimal(output)),
            err => Err(CassError(err))
        }
    }}
}
