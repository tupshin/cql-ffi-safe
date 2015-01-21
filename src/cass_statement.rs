extern crate cql_ffi;

pub struct CassStatement<'a> {
    pub statement:&'a mut cql_ffi::CassStatement
}

