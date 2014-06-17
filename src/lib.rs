#![crate_id = "memcached#0.1"]
#![crate_type = "lib"]
extern crate libc;

use std::ptr;
use native::{
  memcached_create,
  memcached_set,
  memcached_get,
  memcached_st,
  memcached_return_t,
  memcached_server_add,
};
use libc::{
  c_char,
  c_uint,
  size_t
};
use std::c_str::CString;

pub mod native;

pub struct Client {
  cl: *mut memcached_st
}

pub struct Server<'a> {
  host: &'a str,
  port: uint,
}

impl Client {
  pub fn new() -> Client {
    Client{ cl: unsafe { memcached_create(ptr::mut_null()) } }
  }

  pub fn add_server(&self, server: &Server) {
    unsafe { memcached_server_add(self.cl, server.host.to_c_str().unwrap(), server.port as u16) };
  }

  pub fn set(&self, key: &str, value: &str, expire: int) {
    let key_c = key.to_c_str();
    let value_c = value.to_c_str();
    unsafe {
      memcached_set(self.cl, key_c.clone().unwrap(), key_c.len() as u64, value_c.clone().unwrap(), value_c.len() as u64, expire as i64, 0);
    }
  }

  pub fn get(&self, key: &str) -> CString {
    let key_c = key.to_c_str();
    let value_length = 0 as *mut size_t;
    let error = 0 as *mut memcached_return_t;
    let flags = 0 as *mut c_uint;
    unsafe {
      let value = memcached_get(self.cl, key_c.clone().unwrap(), key_c.len() as u64, value_length, flags, error) as *c_char;
      CString::new(value, false)
    }
  }
}

#[test]
fn test() {
  let cache = Client::new();
  let server = &Server { host: "127.0.0.1", port: 11211 };
  cache.add_server(server);
  cache.set("hoge", "fuga", 10);
  assert!(cache.get("hoge").as_str().unwrap() == "fuga");
}
