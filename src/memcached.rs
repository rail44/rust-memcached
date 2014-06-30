#![crate_id = "memcached#0.1"]
#![crate_type = "lib"]

extern crate libc;

use std::ptr;
use std::mem::size_of;
use std::c_str::CString;
use libc::{
  c_char,
  c_uint,
  size_t,
  malloc,
  free,
  c_void
};
use native::{
  memcached_create,
  memcached_set,
  memcached_get,
  memcached_st,
  memcached_return_t,
  memcached_server_add,
};

pub mod native;

pub struct Client {
  cl: *mut memcached_st
}

pub struct Server<'a> {
  host: &'a str,
  port: uint,
}

impl<'a> Server<'a> {
  pub fn new<'a>(host: &'a str, port: uint) -> Server<'a> {
    Server { host: host, port: port }
  }
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
    unsafe {
      let value_length = malloc(size_of::<size_t>() as size_t) as *mut size_t;
      let error = malloc(size_of::<memcached_return_t>() as size_t) as *mut memcached_return_t;
      let flags = malloc(size_of::<c_uint>() as size_t) as *mut c_uint;
      let value = memcached_get(self.cl, key_c.clone().unwrap(), key_c.len() as u64, value_length, flags, error);
      free(value_length as *mut c_void);
      free(error as *mut c_void);
      free(flags as *mut c_void);
      CString::new(value as *const c_char, false)
    }
  }

  pub fn get_str(&self, key: &str) -> String {
    self.get(key).as_str().unwrap().into_string()
  }
}

#[test]
fn test() {
  let cl = Client::new();
  let server = Server::new("127.0.0.1", 11211);
  cl.add_server(&server);
  cl.set("hoge", "fuga", 10);
  assert!(cl.get_str("hoge").as_slice() == "fuga");
}
