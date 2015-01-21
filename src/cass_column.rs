extern crate cql_ffi;

pub use cql_ffi::CassColumnType;

pub struct CassColumn<'a> {
    column:&'a cql_ffi::CassValue
}
