#![feature(unsafe_destructor,core,collections,std_misc,libc)]

extern crate cql_ffi;
extern crate uuid;
extern crate libc;

///A Rust create that provides (theoretically) safe bindings to the unsafe cql-ffi crate,
///which in turns wrap the DataStax C++ Cassandra driver.
///This the most stable and full feature option to use Cassandra from Rust today.
pub use cql_ffi_safe::bytes::*;
pub use cql_ffi_safe::types::*;
pub use cql_ffi_safe::string::*;
pub use cql_ffi_safe::inet::*;
pub use cql_ffi_safe::decimal::*;
pub use cql_ffi_safe::uuid::*;
pub use cql_ffi_safe::cluster::*;
pub use cql_ffi_safe::session::*;
pub use cql_ffi_safe::statement::*;
pub use cql_ffi_safe::batch::*;
pub use cql_ffi_safe::future::*;
pub use cql_ffi_safe::prepared::*;
pub use cql_ffi_safe::result::*;
pub use cql_ffi_safe::iterator::*;
pub use cql_ffi_safe::row::*;
pub use cql_ffi_safe::value::*;
pub use cql_ffi_safe::collection::*;
pub use cql_ffi_safe::ssl::*;
pub use cql_ffi_safe::schema::*;
pub use cql_ffi_safe::log::*;
pub use cql_ffi_safe::consistency::*;
pub use cql_ffi_safe::column::*;
pub use cql_ffi_safe::error::*;

mod cql_ffi_safe {
    pub mod bytes;
    pub mod types;
    pub mod string;
    pub mod inet;
    pub mod decimal;
    pub mod uuid;
    pub mod cluster;        
    pub mod session;
    pub mod statement;
    pub mod batch;
    pub mod future;
    pub mod prepared;
    pub mod result;
    pub mod iterator;
    pub mod row;
    pub mod value;
    pub mod collection; 
    pub mod ssl;
    pub mod schema;
    pub mod log;
    pub mod error;
    pub mod consistency;
    pub mod column;
}
