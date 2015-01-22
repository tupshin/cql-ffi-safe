extern crate cql_ffi;

pub use cql_ffi::CassColumnType;

pub struct CassColumn<'a> {
    pub column:&'a cql_ffi::CassValue
}
