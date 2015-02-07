#![feature(core)]
extern crate cql_ffi_safe;

use cql_ffi_safe::CassStatement;
use cql_ffi_safe::CassSession;
use cql_ffi_safe::CassCluster;
use cql_ffi_safe::FromCol;
use cql_ffi_safe::CassError;
use cql_ffi_safe::CassString;

static SELECT_QUERY_CMD:&'static str = "SELECT keyspace_name,durable_writes FROM system.schema_keyspaces;";
static CONTACT_POINTS:&'static str = "127.0.0.1,127.0.0.2,127.0.0.3";

fn main() {
    match CassCluster::new().set_contact_points(CONTACT_POINTS) {
        Err(err) => println!("{:?}",err),
        Ok(cluster) => {
            let mut session = CassSession::new();
            session.connect(cluster).wait();
            let mut future = session.execute(&CassStatement::new(SELECT_QUERY_CMD,0));
            future.wait();
            for row in future.get_result().unwrap().iter() {
                let value:Result<bool,CassError> = FromCol::from_col(row.get_column(1));
                let value:Result<CassString,CassError> = row.get_column(0).get_string();
                match value {
                    Ok(ks) => {
                        println!("ks: {:?}",ks);
                        let mut close_future = session.close();
                        close_future.wait();
                    },
                    Err(err) => println!("Error: {:?}", err)
                }
            }
        }
    }
}
