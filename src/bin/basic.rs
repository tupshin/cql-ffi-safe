#![allow(unstable)]

extern crate cql_ffi_safe;

use cql_ffi_safe::*;

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
    statement.bind_string(0, key).unwrap();
    let mut future = session.execute(statement);
    future.wait();
    future.error_code().unwrap();
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
}

fn main() {
    match CassCluster::new().set_contact_points("127.0.0.1") {
        Ok(cluster) => {
            let mut session = CassSession::new();
            let input = Basic{bln:true, flt:0.001f32, dbl:0.0002f64, i32:1, i64:2 };
            session.connect(cluster).wait();
            session.execute(CassStatement::new(CREATE_KEYSPACE_CMD, 0));
            //FIXME if this is moved up above then weird crap happens, something is stomping on memory
            let create_table_cmd = "CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));";
            match execute_query(&mut session, create_table_cmd) {
                Err(err) => panic!("err={:?}",err),
                Ok(_) => {}
            }
            insert_into_basic(&mut session,"test", input).unwrap();
            let output = select_from_basic(&mut session,"test").unwrap();
            session.close().wait();

            println!("{:?}\n{:?}",input,output);
            assert!(input == output);
        },
        Err(err) => panic!(err)
    }
}
