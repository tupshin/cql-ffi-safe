extern crate cql_ffi;

use std::old_io::net::ip::{IpAddr,Ipv4Addr,Ipv6Addr};

#[allow(missing_copy_implementations)]
pub struct CassInet(pub *const cql_ffi::CassInet);

pub fn init(addr:IpAddr) -> CassInet {unsafe{
    match addr {
        Ipv4Addr(oct1,oct2,oct3,oct4) => {
            let mut v:Vec<u8> = Vec::with_capacity(4);
            v.push(oct1);v.push(oct2);v.push(oct3);v.push(oct4);
            CassInet(&cql_ffi::cass_inet_init_v4(v.as_ptr()))
        },
        Ipv6Addr(seg1,seg2,seg3,seg4,seg5,seg6,seg7,seg8) => {
            let mut v:Vec<u16> = Vec::with_capacity(8);
            v.push(seg1);v.push(seg2);v.push(seg3);v.push(seg4);
            v.push(seg5);v.push(seg6);v.push(seg7);v.push(seg8);
            panic!("ip46 not yet supported")
            //CassInet{inet:cql_ffi::cass_inet_init_v6(v.as_ptr())}
        }
    }
}}
