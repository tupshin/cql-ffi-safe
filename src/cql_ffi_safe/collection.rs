extern crate cql_ffi;

pub use cql_ffi::CassCollectionType;
use cql_ffi_safe::error::CassError;
use cql_ffi_safe::bytes::CassBytes;
use cql_ffi_safe::uuid::CassUuid;
use cql_ffi_safe::inet::CassInet;
use cql_ffi_safe::decimal::CassDecimal;
use cql_ffi_safe::value::CassBindable;

pub struct CassCollection(pub *mut cql_ffi::CassCollection);

impl CassCollection {
    pub fn new(_type: CassCollectionType, item_count: u64) -> Option<CassCollection> {unsafe{
        let collection = cql_ffi::cass_collection_new(_type, item_count);
        match collection.is_null() {
            true => None,
            false => Some(CassCollection(collection))
        }
    }}

    pub fn append(&mut self, value: CassBindable) -> Result<(),CassError> {
        match value {
            CassBindable::I32(val) => self.append_int32(val),
            CassBindable::I64(val) => self.append_int64(val),
            CassBindable::F32(val) => self.append_float(val),
            CassBindable::F64(val) => self.append_double(val),
            CassBindable::STR(val) => self.append_string(val.as_slice()),
            val => panic!("unsupported type: {:?}",val)
        }
    }

    pub fn append_int32(&mut self, value: i32) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_int32(self.0, value)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }

    pub fn append_int64(&mut self, value: i64) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_int64(self.0, value)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }

    pub fn append_float(&mut self, value: f32) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_float(self.0, value)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }    

    pub fn append_double(&mut self, value: f64) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_double(self.0, value)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }    
    
    pub fn cass_collection_append_bool(&mut self, value: bool) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_bool(self.0, if value==true {1} else {0})};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }    

    pub fn append_string(&mut self, value: &str) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_string(self.0,cql_ffi::cass_string_init2(value.as_ptr() as *const i8, value.len() as u64))};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }

    pub fn append_bytes(&mut self, value: &CassBytes) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_bytes(self.0, *value.0)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }

    pub fn append_uuid(&mut self, value: CassUuid) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_uuid(self.0, *value.0)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }

    pub fn append_inet(self, value: CassInet) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_inet(self.0, *value.0)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }

    pub fn append_decimal(&mut self, value: CassDecimal) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_decimal(self.0, *value.0)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }
}

impl Drop for CassCollection {
    fn drop(&mut self) {
        unsafe{cql_ffi::cass_collection_free(self.0)}
    }
}
