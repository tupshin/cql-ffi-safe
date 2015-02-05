#![feature(core)]
extern crate cql_ffi_safe;

use cql_ffi_safe::CassSession;
use cql_ffi_safe::CassError;
use cql_ffi_safe::CassStatement;
use cql_ffi_safe::CassFuture;
use cql_ffi_safe::CassCluster;

use std::num::ToPrimitive;

static NUM_CONCURRENT_REQUESTS:usize = 1000;

static INSERT_QUERY_CMD:&'static str = "INSERT INTO examples.async (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
static CREATE_KEYSPACE_CMD:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };";
static CREATE_TABLE_CMD:&'static str = "CREATE TABLE IF NOT EXISTS examples.async (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));";

fn execute_query(session: &mut CassSession, query: &str) -> Result<(),CassError> {
    let statement = CassStatement::new(query.clone(),0);
    let ref mut future = session.execute(statement);
    future.wait();
    match future.error_code() {
        Ok(_) => {Ok(())},
        err => panic!("{:?}",err)
    }
}

fn insert_into_async(session: &mut CassSession, key:&str) -> Result<(),CassError> {
    use cql_ffi_safe::CassBindable::*;
    let mut futures = Vec::<CassFuture>::new();
    for i in (0..NUM_CONCURRENT_REQUESTS) {
        let mut statement = CassStatement::new(INSERT_QUERY_CMD, 6);
        let mut key = key.to_string();
        key.push_str(&*i.to_string());
        try!(statement.bind_all(vec!(
            STR(key.to_string()),
            BOOL(if i % 2 == 0 {true} else {false}),
            F32(i.to_f32().unwrap() / 2.0f32),
            F64(i.to_f64().unwrap() / 200.0),
            I32(i.to_i32().unwrap() * 10),
           // I64( i.to_i64().unwrap() * 100)
        )));
        let future = session.execute(statement);
        futures.push(future);
    }
    for mut future in futures.iter_mut() {
        future.wait();
        try!( future.error_code());
    }
    Ok(())
}

pub fn main() {
    match CassCluster::new().set_contact_points("127.0.0.1,127.0.0.2,127.0.0.3") {
        Ok(cluster) => {
            let ref mut session = CassSession::new();
            session.connect(cluster).wait();
            session.execute(CassStatement::new(CREATE_KEYSPACE_CMD, 0));
            session.execute(CassStatement::new(CREATE_TABLE_CMD, 0));
            match execute_query(session, "USE examples") {
                Ok(result) => println!("{:?}",result),
                Err(err) => panic!("{:?}",err)
            }
            match insert_into_async(session, "test") {
                Ok(result) => println!("{:?}",result),
                Err(err) => panic!("{:?}",err.desc)
            }
            session.close().wait();
        },
        Err(err) => panic!("err: {:?}", err),
    }
}
