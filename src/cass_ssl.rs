extern crate cql_ffi;

pub struct CassSsl<'a> {
    pub ssl:&'a mut cql_ffi::CassSsl
}

