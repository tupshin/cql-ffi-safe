#![allow(unstable)]

extern crate cql_ffi_safe;

use std::ffi::CString;
use std::slice;

use cql_ffi_safe::*;

use std::num::ToPrimitive;

static NUM_CONCURRENT_REQUESTS:usize = 1000;

static INSERT_QUERY_CMD:&'static str = "INSERT INTO examples.async (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
static CREATE_KEYSPACE_CMD:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };";
static CREATE_TABLE_CMD:&'static str = "CREATE TABLE IF NOT EXISTS examples.async (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));";

fn connect_session(session:&mut CassSession, mut cluster:CassCluster) -> Result<(),CassError> {unsafe{
    let future = session.connect(&mut cluster);
    future.wait();
    match future.error_code() {
        Ok(rc) => {Ok(())},
        _=> panic!("{:?}",future)
    }
    //cass_future_free(future);
}}

fn execute_query(session: &mut CassSession, query: &str) -> Result<(),CassError> {unsafe{
    let statement = CassStatement::new(query.clone(),0);
    let future = session.execute(statement);
    future.wait();
    match future.error_code() {
        Ok(rc) => {Ok(())},
        _=> panic!("{:?}",future)
    }
}}

fn insert_into_async(session: &mut CassSession, key:&str) -> Result<(),CassError> {unsafe{
    let mut futures = Vec::<CassFuture>::new();
    for i in (0..NUM_CONCURRENT_REQUESTS) {
        let mut statement = CassStatement::new(INSERT_QUERY_CMD, 6);
        let mut key = key.to_string();
        key.push_str(&*i.to_string());
        statement.bind_string(0, key.as_slice());
        statement.bind_bool( 1, if i % 2 == 0 {true} else {false});
        statement.bind_f32(2, i.to_f32().unwrap() / 2.0f32);
        statement.bind_f64(3, i.to_f64().unwrap() / 200.0);
        statement.bind_i32(4, i.to_i32().unwrap() * 10);
        statement.bind_i64(5, i.to_i64().unwrap() * 100);
        let future = session.execute(statement);
        futures.push(future);
    }
    for mut future in futures.iter_mut() {
        future.wait();
        match future.error_code() {
            Ok(rc) => {},
            _=> panic!("{:?}",future)
        }
    }
    Ok(())
}}

pub fn main() {unsafe{
    match CassCluster::new().set_contact_points("127.0.0.1,127.0.0.2,127.0.0.3") {
        Ok(cluster) => {
            let ref mut session = CassSession::new();
            session.connect(cluster).wait();
            session.execute(CassStatement::new(CREATE_KEYSPACE_CMD, 0));
            session.execute(CassStatement::new(CREATE_TABLE_CMD, 0));
            execute_query(session, "USE examples");
            insert_into_async(session, "test");
            session.close().wait();
        },
        Err(err) => panic!(err)
    }
}}
