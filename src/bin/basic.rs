#![allow(unstable)]

extern crate cql_ffi_safe;
extern crate cql_ffi;


use cql_ffi_safe::*;
use cql_ffi::cass_result_free;

#[derive(Show,Copy,PartialEq)]
struct Basic {
    bln:bool,
    flt:f32,
    dbl:f64,
    i32:i32,
    i64:i64,
}

static INSERT_QUERY_CMD:&'static str = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
static SELECT_QUERY_CMD:&'static str = "SELECT * FROM examples.basic WHERE key = ?";
static CREATE_KEYSPACE_CMD:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE_CMD:&'static str = "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));";

fn execute_query(session: &mut CassSession, query: &str) -> Result<(),CassError> {
    let statement = CassStatement::new(query,0);
    let future = session.execute(statement);
    future.wait();
    future.error_code()
}

fn insert_into_basic(session:&mut CassSession, key:&str, basic:Basic) -> Result<(),CassError> {
    let query=INSERT_QUERY_CMD;
    let mut statement = CassStatement::new(query, 6);
    //FIXME with a macro that automatically does this for an arbitrary struct
    try!(statement.bind_all(vec!(
        CassBindable::STR(key.to_string()),
        CassBindable::BOOL(basic.bln),
        CassBindable::F32(basic.flt),
        CassBindable::F64(basic.dbl),
        CassBindable::I32(basic.i32),
        CassBindable::I64(basic.i64)
    )));
    session.execute(statement).wait();
    Ok(())
}

fn select_from_basic(session:&mut CassSession, key:&str) -> Result<Basic,CassError> {
    let query = SELECT_QUERY_CMD;
    let mut statement = CassStatement::new(query, 1);
    try!(statement.bind_string(0, key));
    let mut future = session.execute(statement);
    future.wait();
    try!(future.error_code());
    let result = future.get_result();
    let mut iterator = result.iter();
    match (iterator.get_type(),iterator.has_next()) {
        (_,false) => panic!(),
        (CassIteratorType::RESULT,true) => return {
            let row = iterator.get_row();
            Ok(Basic{
                bln:try!(row.get_column(1).get_bool()),
                dbl:try!(row.get_column(2).get_double()),
                flt:try!(row.get_column(3).get_float()),
                i32:try!(row.get_column(4).get_int32()),
                i64:try!(row.get_column(5).get_int64())
            })
        },
        (iter_type,true) => {panic!("wasn't expecting iterator type: {:?}", iter_type)}
    }
    //FIXME it is temporarily necessary to free your own results, as the Drop trait in cass_result.rs results in memory corruption
    unsafe{cql_ffi::cass_result_free(result.result)};
}


fn main() {
    match CassCluster::new().set_contact_points("127.0.0.1,127.0.0.2,127.0.0.3") {
        Ok(cluster) => {
            let mut session = CassSession::new();
            let input = Basic{bln:true, flt:0.001f32, dbl:0.0002f64, i32:1, i64:2 };
            session.connect(cluster).wait();
            session.execute(CassStatement::new(CREATE_KEYSPACE_CMD, 0));
            match execute_query(&mut session, CREATE_TABLE_CMD) {
                Err(err) => panic!("err: {:?}", err),
                Ok(_) => {}
            }
            match insert_into_basic(&mut session,"test", input) {
                Ok(response) => {
                    println!("response {:?}",response);
                }
                Err(err) => {}//panic!(err)
            }
            match select_from_basic(&mut session,"test") {
                Ok(output) => {
                    println!("{:?}\n{:?}",input,output);
                    assert!(input == output);
                    }
                Err(err) => panic!(err)
            }
            session.close().wait();
        },
        Err(err) => panic!(err)
    }
}
