#![allow(non_camel_case_types)]

pub type int8_t = ::libc::c_char;
pub type int16_t = ::libc::c_short;
pub type int32_t = ::libc::c_int;
pub type int64_t = ::libc::c_long;
pub type uint8_t = ::libc::c_uchar;
pub type uint16_t = ::libc::c_ushort;
pub type uint32_t = ::libc::c_uint;
pub type uint64_t = ::libc::c_ulong;
pub type int_least8_t = ::libc::c_char;
pub type int_least16_t = ::libc::c_short;
pub type int_least32_t = ::libc::c_int;
pub type int_least64_t = ::libc::c_long;
pub type uint_least8_t = ::libc::c_uchar;
pub type uint_least16_t = ::libc::c_ushort;
pub type uint_least32_t = ::libc::c_uint;
pub type uint_least64_t = ::libc::c_ulong;
pub type int_fast8_t = ::libc::c_char;
pub type int_fast16_t = ::libc::c_long;
pub type int_fast32_t = ::libc::c_long;
pub type int_fast64_t = ::libc::c_long;
pub type uint_fast8_t = ::libc::c_uchar;
pub type uint_fast16_t = ::libc::c_ulong;
pub type uint_fast32_t = ::libc::c_ulong;
pub type uint_fast64_t = ::libc::c_ulong;
pub type intptr_t = ::libc::c_long;
pub type uintptr_t = ::libc::c_ulong;
pub type intmax_t = ::libc::c_long;
pub type uintmax_t = ::libc::c_ulong;
pub type __gwchar_t = ::libc::c_int;
pub struct imaxdiv_t {
    pub quot: ::libc::c_long,
    pub rem: ::libc::c_long,
}
pub type ptrdiff_t = ::libc::c_long;
pub type size_t = ::libc::c_ulong;
pub type wchar_t = ::libc::c_int;
pub type idtype_t = ::libc::c_uint;
pub static P_ALL: ::libc::c_uint = 0;
pub static P_PID: ::libc::c_uint = 1;
pub static P_PGID: ::libc::c_uint = 2;
pub type __u_char = ::libc::c_uchar;
pub type __u_short = ::libc::c_ushort;
pub type __u_int = ::libc::c_uint;
pub type __u_long = ::libc::c_ulong;
pub type __int8_t = ::libc::c_char;
pub type __uint8_t = ::libc::c_uchar;
pub type __int16_t = ::libc::c_short;
pub type __uint16_t = ::libc::c_ushort;
pub type __int32_t = ::libc::c_int;
pub type __uint32_t = ::libc::c_uint;
pub type __int64_t = ::libc::c_long;
pub type __uint64_t = ::libc::c_ulong;
pub type __quad_t = ::libc::c_long;
pub type __u_quad_t = ::libc::c_ulong;
pub type __dev_t = ::libc::c_ulong;
pub type __uid_t = ::libc::c_uint;
pub type __gid_t = ::libc::c_uint;
pub type __ino_t = ::libc::c_ulong;
pub type __ino64_t = ::libc::c_ulong;
pub type __mode_t = ::libc::c_uint;
pub type __nlink_t = ::libc::c_ulong;
pub type __off_t = ::libc::c_long;
pub type __off64_t = ::libc::c_long;
pub type __pid_t = ::libc::c_int;
pub struct __fsid_t {
    pub __val: [::libc::c_int, ..2u],
}
pub type __clock_t = ::libc::c_long;
pub type __rlim_t = ::libc::c_ulong;
pub type __rlim64_t = ::libc::c_ulong;
pub type __id_t = ::libc::c_uint;
pub type __time_t = ::libc::c_long;
pub type __useconds_t = ::libc::c_uint;
pub type __suseconds_t = ::libc::c_long;
pub type __daddr_t = ::libc::c_int;
pub type __key_t = ::libc::c_int;
pub type __clockid_t = ::libc::c_int;
pub type __timer_t = *mut ::libc::c_void;
pub type __blksize_t = ::libc::c_long;
pub type __blkcnt_t = ::libc::c_long;
pub type __blkcnt64_t = ::libc::c_long;
pub type __fsblkcnt_t = ::libc::c_ulong;
pub type __fsblkcnt64_t = ::libc::c_ulong;
pub type __fsfilcnt_t = ::libc::c_ulong;
pub type __fsfilcnt64_t = ::libc::c_ulong;
pub type __fsword_t = ::libc::c_long;
pub type __ssize_t = ::libc::c_long;
pub type __syscall_slong_t = ::libc::c_long;
pub type __syscall_ulong_t = ::libc::c_ulong;
pub type __loff_t = __off64_t;
pub type __qaddr_t = *mut __quad_t;
pub type __caddr_t = *mut ::libc::c_char;
pub type __intptr_t = ::libc::c_long;
pub type __socklen_t = ::libc::c_uint;
pub struct Union_wait {
    pub data: [u32, ..1u],
}
impl Union_wait {
    pub fn w_status(&mut self) -> *mut ::libc::c_int {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __wait_terminated(&mut self) -> *mut Struct_Unnamed1 {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __wait_stopped(&mut self) -> *mut Struct_Unnamed2 {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub struct Struct_Unnamed1 {
    pub __w_termsig: ::libc::c_uint,
    pub __w_coredump: ::libc::c_uint,
    pub __w_retcode: ::libc::c_uint,
    pub unnamed_field1: ::libc::c_uint,
}
pub struct Struct_Unnamed2 {
    pub __w_stopval: ::libc::c_uint,
    pub __w_stopsig: ::libc::c_uint,
    pub unnamed_field1: ::libc::c_uint,
}
pub struct __WAIT_STATUS {
    pub data: [u64, ..1u],
}
impl __WAIT_STATUS {
    pub fn __uptr(&mut self) -> *mut *mut Union_wait {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __iptr(&mut self) -> *mut *mut ::libc::c_int {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub struct div_t {
    pub quot: ::libc::c_int,
    pub rem: ::libc::c_int,
}
pub struct ldiv_t {
    pub quot: ::libc::c_long,
    pub rem: ::libc::c_long,
}
pub struct lldiv_t {
    pub quot: ::libc::c_longlong,
    pub rem: ::libc::c_longlong,
}
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type ssize_t = __ssize_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type time_t = __time_t;
pub type clockid_t = __clockid_t;
pub type timer_t = __timer_t;
pub type ulong = ::libc::c_ulong;
pub type ushort = ::libc::c_ushort;
pub type _uint = ::libc::c_uint;
pub type u_int8_t = ::libc::c_uchar;
pub type u_int16_t = ::libc::c_ushort;
pub type u_int32_t = ::libc::c_uint;
pub type u_int64_t = ::libc::c_ulong;
pub type register_t = ::libc::c_long;
pub type __sig_atomic_t = ::libc::c_int;
pub struct __sigset_t {
    pub __val: [::libc::c_ulong, ..16u],
}
pub type sigset_t = __sigset_t;
pub struct Struct_timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub struct Struct_timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = ::libc::c_long;
pub struct fd_set {
    pub __fds_bits: [__fd_mask, ..16u],
}
pub type fd_mask = __fd_mask;
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
pub type pthread_t = ::libc::c_ulong;
pub struct Union_pthread_attr_t {
    pub data: [u64, ..7u],
}
impl Union_pthread_attr_t {
    pub fn __size(&mut self) -> *mut [::libc::c_char, ..56u] {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __align(&mut self) -> *mut ::libc::c_long {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub type pthread_attr_t = Union_pthread_attr_t;
pub struct Struct___pthread_internal_list {
    pub __prev: *mut Struct___pthread_internal_list,
    pub __next: *mut Struct___pthread_internal_list,
}
pub type __pthread_list_t = Struct___pthread_internal_list;
pub struct Struct___pthread_mutex_s {
    pub __lock: ::libc::c_int,
    pub __count: ::libc::c_uint,
    pub __owner: ::libc::c_int,
    pub __nusers: ::libc::c_uint,
    pub __kind: ::libc::c_int,
    pub __spins: ::libc::c_short,
    pub __elision: ::libc::c_short,
    pub __list: __pthread_list_t,
}
pub struct pthread_mutex_t {
    pub data: [u64, ..5u],
}
impl pthread_mutex_t {
    pub fn __data(&mut self) -> *mut Struct___pthread_mutex_s {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __size(&mut self) -> *mut [::libc::c_char, ..40u] {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __align(&mut self) -> *mut ::libc::c_long {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub struct pthread_mutexattr_t {
    pub data: [u32, ..1u],
}
impl pthread_mutexattr_t {
    pub fn __size(&mut self) -> *mut [::libc::c_char, ..4u] {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __align(&mut self) -> *mut ::libc::c_int {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub struct Struct_Unnamed3 {
    pub __lock: ::libc::c_int,
    pub __futex: ::libc::c_uint,
    pub __total_seq: ::libc::c_ulonglong,
    pub __wakeup_seq: ::libc::c_ulonglong,
    pub __woken_seq: ::libc::c_ulonglong,
    pub __mutex: *mut ::libc::c_void,
    pub __nwaiters: ::libc::c_uint,
    pub __broadcast_seq: ::libc::c_uint,
}
pub struct pthread_cond_t {
    pub data: [u64, ..6u],
}
impl pthread_cond_t {
    pub fn __data(&mut self) -> *mut Struct_Unnamed3 {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __size(&mut self) -> *mut [::libc::c_char, ..48u] {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __align(&mut self) -> *mut ::libc::c_longlong {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub struct pthread_condattr_t {
    pub data: [u32, ..1u],
}
impl pthread_condattr_t {
    pub fn __size(&mut self) -> *mut [::libc::c_char, ..4u] {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __align(&mut self) -> *mut ::libc::c_int {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub type pthread_key_t = ::libc::c_uint;
pub type pthread_once_t = ::libc::c_int;
pub struct Struct_Unnamed4 {
    pub __lock: ::libc::c_int,
    pub __nr_readers: ::libc::c_uint,
    pub __readers_wakeup: ::libc::c_uint,
    pub __writer_wakeup: ::libc::c_uint,
    pub __nr_readers_queued: ::libc::c_uint,
    pub __nr_writers_queued: ::libc::c_uint,
    pub __writer: ::libc::c_int,
    pub __shared: ::libc::c_int,
    pub __pad1: ::libc::c_ulong,
    pub __pad2: ::libc::c_ulong,
    pub __flags: ::libc::c_uint,
}
pub struct pthread_rwlock_t {
    pub data: [u64, ..7u],
}
impl pthread_rwlock_t {
    pub fn __data(&mut self) -> *mut Struct_Unnamed4 {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __size(&mut self) -> *mut [::libc::c_char, ..56u] {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __align(&mut self) -> *mut ::libc::c_long {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub struct pthread_rwlockattr_t {
    pub data: [u64, ..1u],
}
impl pthread_rwlockattr_t {
    pub fn __size(&mut self) -> *mut [::libc::c_char, ..8u] {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __align(&mut self) -> *mut ::libc::c_long {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub type pthread_spinlock_t = ::libc::c_int;
pub struct pthread_barrier_t {
    pub data: [u64, ..4u],
}
impl pthread_barrier_t {
    pub fn __size(&mut self) -> *mut [::libc::c_char, ..32u] {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __align(&mut self) -> *mut ::libc::c_long {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub struct pthread_barrierattr_t {
    pub data: [u32, ..1u],
}
impl pthread_barrierattr_t {
    pub fn __size(&mut self) -> *mut [::libc::c_char, ..4u] {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __align(&mut self) -> *mut ::libc::c_int {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub struct Struct_random_data {
    pub fptr: *mut int32_t,
    pub rptr: *mut int32_t,
    pub state: *mut int32_t,
    pub rand_type: ::libc::c_int,
    pub rand_deg: ::libc::c_int,
    pub rand_sep: ::libc::c_int,
    pub end_ptr: *mut int32_t,
}
pub struct Struct_drand48_data {
    pub __x: [::libc::c_ushort, ..3u],
    pub __old_x: [::libc::c_ushort, ..3u],
    pub __c: ::libc::c_ushort,
    pub __init: ::libc::c_ushort,
    pub __a: ::libc::c_ulonglong,
}
pub type __compar_fn_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *::libc::c_void, arg2: *::libc::c_void)
                              -> ::libc::c_int>;
pub type memcached_socket_t = ::libc::c_int;
pub struct Struct_iovec {
    pub iov_base: *mut ::libc::c_void,
    pub iov_len: size_t,
}
pub type socklen_t = __socklen_t;
pub type Enum___socket_type = ::libc::c_uint;
pub static SOCK_STREAM: ::libc::c_uint = 1;
pub static SOCK_DGRAM: ::libc::c_uint = 2;
pub static SOCK_RAW: ::libc::c_uint = 3;
pub static SOCK_RDM: ::libc::c_uint = 4;
pub static SOCK_SEQPACKET: ::libc::c_uint = 5;
pub static SOCK_DCCP: ::libc::c_uint = 6;
pub static SOCK_PACKET: ::libc::c_uint = 10;
pub static SOCK_CLOEXEC: ::libc::c_uint = 524288;
pub static SOCK_NONBLOCK: ::libc::c_uint = 2048;
pub type sa_family_t = ::libc::c_ushort;
pub struct Struct_sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::libc::c_char, ..14u],
}
pub struct Struct_sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_align: ::libc::c_ulong,
    pub __ss_padding: [::libc::c_char, ..112u],
}
pub type Enum_Unnamed5 = ::libc::c_uint;
pub static MSG_OOB: ::libc::c_uint = 1;
pub static MSG_PEEK: ::libc::c_uint = 2;
pub static MSG_DONTROUTE: ::libc::c_uint = 4;
pub static MSG_CTRUNC: ::libc::c_uint = 8;
pub static MSG_PROXY: ::libc::c_uint = 16;
pub static MSG_TRUNC: ::libc::c_uint = 32;
pub static MSG_DONTWAIT: ::libc::c_uint = 64;
pub static MSG_EOR: ::libc::c_uint = 128;
pub static MSG_WAITALL: ::libc::c_uint = 256;
pub static MSG_FIN: ::libc::c_uint = 512;
pub static MSG_SYN: ::libc::c_uint = 1024;
pub static MSG_CONFIRM: ::libc::c_uint = 2048;
pub static MSG_RST: ::libc::c_uint = 4096;
pub static MSG_ERRQUEUE: ::libc::c_uint = 8192;
pub static MSG_NOSIGNAL: ::libc::c_uint = 16384;
pub static MSG_MORE: ::libc::c_uint = 32768;
pub static MSG_WAITFORONE: ::libc::c_uint = 65536;
pub static MSG_FASTOPEN: ::libc::c_uint = 536870912;
pub static MSG_CMSG_CLOEXEC: ::libc::c_uint = 1073741824;
pub struct Struct_msghdr {
    pub msg_name: *mut ::libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut Struct_iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut ::libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: ::libc::c_int,
}
pub struct Struct_cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: ::libc::c_int,
    pub cmsg_type: ::libc::c_int,
    pub __cmsg_data: *mut ::libc::c_uchar,
}
pub type Enum_Unnamed6 = ::libc::c_uint;
pub static SCM_RIGHTS: ::libc::c_uint = 1;
pub struct Struct_linger {
    pub l_onoff: ::libc::c_int,
    pub l_linger: ::libc::c_int,
}
pub struct Struct_osockaddr {
    pub sa_family: ::libc::c_ushort,
    pub sa_data: [::libc::c_uchar, ..14u],
}
pub type Enum_Unnamed7 = ::libc::c_uint;
pub static SHUT_RD: ::libc::c_uint = 0;
pub static SHUT_WR: ::libc::c_uint = 1;
pub static SHUT_RDWR: ::libc::c_uint = 2;
pub type in_addr_t = uint32_t;
pub struct Struct_in_addr {
    pub s_addr: in_addr_t,
}
pub struct Struct_ip_opts {
    pub ip_dst: Struct_in_addr,
    pub ip_opts: [::libc::c_char, ..40u],
}
pub struct Struct_ip_mreqn {
    pub imr_multiaddr: Struct_in_addr,
    pub imr_address: Struct_in_addr,
    pub imr_ifindex: ::libc::c_int,
}
pub struct Struct_in_pktinfo {
    pub ipi_ifindex: ::libc::c_int,
    pub ipi_spec_dst: Struct_in_addr,
    pub ipi_addr: Struct_in_addr,
}
pub type Enum_Unnamed8 = ::libc::c_uint;
pub static IPPROTO_IP: ::libc::c_uint = 0;
pub static IPPROTO_ICMP: ::libc::c_uint = 1;
pub static IPPROTO_IGMP: ::libc::c_uint = 2;
pub static IPPROTO_IPIP: ::libc::c_uint = 4;
pub static IPPROTO_TCP: ::libc::c_uint = 6;
pub static IPPROTO_EGP: ::libc::c_uint = 8;
pub static IPPROTO_PUP: ::libc::c_uint = 12;
pub static IPPROTO_UDP: ::libc::c_uint = 17;
pub static IPPROTO_IDP: ::libc::c_uint = 22;
pub static IPPROTO_TP: ::libc::c_uint = 29;
pub static IPPROTO_DCCP: ::libc::c_uint = 33;
pub static IPPROTO_IPV6: ::libc::c_uint = 41;
pub static IPPROTO_RSVP: ::libc::c_uint = 46;
pub static IPPROTO_GRE: ::libc::c_uint = 47;
pub static IPPROTO_ESP: ::libc::c_uint = 50;
pub static IPPROTO_AH: ::libc::c_uint = 51;
pub static IPPROTO_MTP: ::libc::c_uint = 92;
pub static IPPROTO_BEETPH: ::libc::c_uint = 94;
pub static IPPROTO_ENCAP: ::libc::c_uint = 98;
pub static IPPROTO_PIM: ::libc::c_uint = 103;
pub static IPPROTO_COMP: ::libc::c_uint = 108;
pub static IPPROTO_SCTP: ::libc::c_uint = 132;
pub static IPPROTO_UDPLITE: ::libc::c_uint = 136;
pub static IPPROTO_RAW: ::libc::c_uint = 255;
pub static IPPROTO_MAX: ::libc::c_uint = 256;
pub type Enum_Unnamed9 = ::libc::c_uint;
pub static IPPROTO_HOPOPTS: ::libc::c_uint = 0;
pub static IPPROTO_ROUTING: ::libc::c_uint = 43;
pub static IPPROTO_FRAGMENT: ::libc::c_uint = 44;
pub static IPPROTO_ICMPV6: ::libc::c_uint = 58;
pub static IPPROTO_NONE: ::libc::c_uint = 59;
pub static IPPROTO_DSTOPTS: ::libc::c_uint = 60;
pub static IPPROTO_MH: ::libc::c_uint = 135;
pub type in_port_t = uint16_t;
pub type Enum_Unnamed10 = ::libc::c_uint;
pub static IPPORT_ECHO: ::libc::c_uint = 7;
pub static IPPORT_DISCARD: ::libc::c_uint = 9;
pub static IPPORT_SYSTAT: ::libc::c_uint = 11;
pub static IPPORT_DAYTIME: ::libc::c_uint = 13;
pub static IPPORT_NETSTAT: ::libc::c_uint = 15;
pub static IPPORT_FTP: ::libc::c_uint = 21;
pub static IPPORT_TELNET: ::libc::c_uint = 23;
pub static IPPORT_SMTP: ::libc::c_uint = 25;
pub static IPPORT_TIMESERVER: ::libc::c_uint = 37;
pub static IPPORT_NAMESERVER: ::libc::c_uint = 42;
pub static IPPORT_WHOIS: ::libc::c_uint = 43;
pub static IPPORT_MTP: ::libc::c_uint = 57;
pub static IPPORT_TFTP: ::libc::c_uint = 69;
pub static IPPORT_RJE: ::libc::c_uint = 77;
pub static IPPORT_FINGER: ::libc::c_uint = 79;
pub static IPPORT_TTYLINK: ::libc::c_uint = 87;
pub static IPPORT_SUPDUP: ::libc::c_uint = 95;
pub static IPPORT_EXECSERVER: ::libc::c_uint = 512;
pub static IPPORT_LOGINSERVER: ::libc::c_uint = 513;
pub static IPPORT_CMDSERVER: ::libc::c_uint = 514;
pub static IPPORT_EFSSERVER: ::libc::c_uint = 520;
pub static IPPORT_BIFFUDP: ::libc::c_uint = 512;
pub static IPPORT_WHOSERVER: ::libc::c_uint = 513;
pub static IPPORT_ROUTESERVER: ::libc::c_uint = 520;
pub static IPPORT_RESERVED: ::libc::c_uint = 1024;
pub static IPPORT_USERRESERVED: ::libc::c_uint = 5000;
pub struct Struct_in6_addr {
    pub __in6_u: Union_Unnamed11,
}
pub struct Union_Unnamed11 {
    pub data: [u32, ..4u],
}
impl Union_Unnamed11 {
    pub fn __u6_addr8(&mut self) -> *mut [uint8_t, ..16u] {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __u6_addr16(&mut self) -> *mut [uint16_t, ..8u] {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn __u6_addr32(&mut self) -> *mut [uint32_t, ..4u] {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub struct Struct_sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: Struct_in_addr,
    pub sin_zero: [::libc::c_uchar, ..8u],
}
pub struct Struct_sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: Struct_in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub struct Struct_ip_mreq {
    pub imr_multiaddr: Struct_in_addr,
    pub imr_interface: Struct_in_addr,
}
pub struct Struct_ip_mreq_source {
    pub imr_multiaddr: Struct_in_addr,
    pub imr_interface: Struct_in_addr,
    pub imr_sourceaddr: Struct_in_addr,
}
pub struct Struct_ipv6_mreq {
    pub ipv6mr_multiaddr: Struct_in6_addr,
    pub ipv6mr_interface: ::libc::c_uint,
}
pub struct Struct_group_req {
    pub gr_interface: uint32_t,
    pub gr_group: Struct_sockaddr_storage,
}
pub struct Struct_group_source_req {
    pub gsr_interface: uint32_t,
    pub gsr_group: Struct_sockaddr_storage,
    pub gsr_source: Struct_sockaddr_storage,
}
pub struct Struct_ip_msfilter {
    pub imsf_multiaddr: Struct_in_addr,
    pub imsf_interface: Struct_in_addr,
    pub imsf_fmode: uint32_t,
    pub imsf_numsrc: uint32_t,
    pub imsf_slist: [Struct_in_addr, ..1u],
}
pub struct Struct_group_filter {
    pub gf_interface: uint32_t,
    pub gf_group: Struct_sockaddr_storage,
    pub gf_fmode: uint32_t,
    pub gf_numsrc: uint32_t,
    pub gf_slist: [Struct_sockaddr_storage, ..1u],
}
pub struct Struct_rpcent {
    pub r_name: *mut ::libc::c_char,
    pub r_aliases: *mut *mut ::libc::c_char,
    pub r_number: ::libc::c_int,
}
pub struct Struct_netent {
    pub n_name: *mut ::libc::c_char,
    pub n_aliases: *mut *mut ::libc::c_char,
    pub n_addrtype: ::libc::c_int,
    pub n_net: uint32_t,
}
pub struct Struct_hostent {
    pub h_name: *mut ::libc::c_char,
    pub h_aliases: *mut *mut ::libc::c_char,
    pub h_addrtype: ::libc::c_int,
    pub h_length: ::libc::c_int,
    pub h_addr_list: *mut *mut ::libc::c_char,
}
pub struct Struct_servent {
    pub s_name: *mut ::libc::c_char,
    pub s_aliases: *mut *mut ::libc::c_char,
    pub s_port: ::libc::c_int,
    pub s_proto: *mut ::libc::c_char,
}
pub struct Struct_protoent {
    pub p_name: *mut ::libc::c_char,
    pub p_aliases: *mut *mut ::libc::c_char,
    pub p_proto: ::libc::c_int,
}
pub struct Struct_addrinfo {
    pub ai_flags: ::libc::c_int,
    pub ai_family: ::libc::c_int,
    pub ai_socktype: ::libc::c_int,
    pub ai_protocol: ::libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut Struct_sockaddr,
    pub ai_canonname: *mut ::libc::c_char,
    pub ai_next: *mut Struct_addrinfo,
}
pub struct Struct_sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::libc::c_char, ..108u],
}
pub struct Struct___locale_struct {
    pub __locales: [*mut Struct___locale_data, ..13u],
    pub __ctype_b: *::libc::c_ushort,
    pub __ctype_tolower: *::libc::c_int,
    pub __ctype_toupper: *::libc::c_int,
    pub __names: [*::libc::c_char, ..13u],
}
pub enum Struct___locale_data { }
pub type __locale_t = *mut Struct___locale_struct;
pub type locale_t = __locale_t;
pub type tcp_seq = u_int32_t;
pub struct Struct_tcphdr;
pub struct Union_Unnamed12 {
    pub data: [u32, ..5u],
}
impl Union_Unnamed12 { }
pub struct Struct_Unnamed13 {
    pub th_sport: u_int16_t,
    pub th_dport: u_int16_t,
    pub th_seq: tcp_seq,
    pub th_ack: tcp_seq,
    pub th_x2: u_int8_t,
    pub th_off: u_int8_t,
    pub th_flags: u_int8_t,
    pub th_win: u_int16_t,
    pub th_sum: u_int16_t,
    pub th_urp: u_int16_t,
}
pub struct Struct_Unnamed14 {
    pub source: u_int16_t,
    pub dest: u_int16_t,
    pub seq: u_int32_t,
    pub ack_seq: u_int32_t,
    pub res1: u_int16_t,
    pub doff: u_int16_t,
    pub fin: u_int16_t,
    pub syn: u_int16_t,
    pub rst: u_int16_t,
    pub psh: u_int16_t,
    pub ack: u_int16_t,
    pub urg: u_int16_t,
    pub res2: u_int16_t,
    pub window: u_int16_t,
    pub check: u_int16_t,
    pub urg_ptr: u_int16_t,
}
pub type Enum_Unnamed15 = ::libc::c_uint;
pub static TCP_ESTABLISHED: ::libc::c_uint = 1;
pub static TCP_SYN_SENT: ::libc::c_uint = 2;
pub static TCP_SYN_RECV: ::libc::c_uint = 3;
pub static TCP_FIN_WAIT1: ::libc::c_uint = 4;
pub static TCP_FIN_WAIT2: ::libc::c_uint = 5;
pub static TCP_TIME_WAIT: ::libc::c_uint = 6;
pub static TCP_CLOSE: ::libc::c_uint = 7;
pub static TCP_CLOSE_WAIT: ::libc::c_uint = 8;
pub static TCP_LAST_ACK: ::libc::c_uint = 9;
pub static TCP_LISTEN: ::libc::c_uint = 10;
pub static TCP_CLOSING: ::libc::c_uint = 11;
pub type Enum_tcp_ca_state = ::libc::c_uint;
pub static TCP_CA_Open: ::libc::c_uint = 0;
pub static TCP_CA_Disorder: ::libc::c_uint = 1;
pub static TCP_CA_CWR: ::libc::c_uint = 2;
pub static TCP_CA_Recovery: ::libc::c_uint = 3;
pub static TCP_CA_Loss: ::libc::c_uint = 4;
pub struct Struct_tcp_info {
    pub tcpi_state: u_int8_t,
    pub tcpi_ca_state: u_int8_t,
    pub tcpi_retransmits: u_int8_t,
    pub tcpi_probes: u_int8_t,
    pub tcpi_backoff: u_int8_t,
    pub tcpi_options: u_int8_t,
    pub tcpi_snd_wscale: u_int8_t,
    pub tcpi_rcv_wscale: u_int8_t,
    pub tcpi_rto: u_int32_t,
    pub tcpi_ato: u_int32_t,
    pub tcpi_snd_mss: u_int32_t,
    pub tcpi_rcv_mss: u_int32_t,
    pub tcpi_unacked: u_int32_t,
    pub tcpi_sacked: u_int32_t,
    pub tcpi_lost: u_int32_t,
    pub tcpi_retrans: u_int32_t,
    pub tcpi_fackets: u_int32_t,
    pub tcpi_last_data_sent: u_int32_t,
    pub tcpi_last_ack_sent: u_int32_t,
    pub tcpi_last_data_recv: u_int32_t,
    pub tcpi_last_ack_recv: u_int32_t,
    pub tcpi_pmtu: u_int32_t,
    pub tcpi_rcv_ssthresh: u_int32_t,
    pub tcpi_rtt: u_int32_t,
    pub tcpi_rttvar: u_int32_t,
    pub tcpi_snd_ssthresh: u_int32_t,
    pub tcpi_snd_cwnd: u_int32_t,
    pub tcpi_advmss: u_int32_t,
    pub tcpi_reordering: u_int32_t,
    pub tcpi_rcv_rtt: u_int32_t,
    pub tcpi_rcv_space: u_int32_t,
    pub tcpi_total_retrans: u_int32_t,
}
pub struct Struct_tcp_md5sig {
    pub tcpm_addr: Struct_sockaddr_storage,
    pub __tcpm_pad1: u_int16_t,
    pub tcpm_keylen: u_int16_t,
    pub __tcpm_pad2: u_int32_t,
    pub tcpm_key: [u_int8_t, ..80u],
}
pub struct Struct_tcp_repair_opt {
    pub opt_code: u_int32_t,
    pub opt_val: u_int32_t,
}
pub type Enum_Unnamed16 = ::libc::c_uint;
pub static TCP_NO_QUEUE: ::libc::c_uint = 0;
pub static TCP_RECV_QUEUE: ::libc::c_uint = 1;
pub static TCP_SEND_QUEUE: ::libc::c_uint = 2;
pub static TCP_QUEUES_NR: ::libc::c_uint = 3;
pub struct Struct_tcp_cookie_transactions {
    pub tcpct_flags: u_int16_t,
    pub __tcpct_pad1: u_int8_t,
    pub tcpct_cookie_desired: u_int8_t,
    pub tcpct_s_data_desired: u_int16_t,
    pub tcpct_used: u_int16_t,
    pub tcpct_value: [u_int8_t, ..536u],
}
pub type Enum_memcached_behavior_t = ::libc::c_uint;
pub static MEMCACHED_BEHAVIOR_NO_BLOCK: ::libc::c_uint = 0;
pub static MEMCACHED_BEHAVIOR_TCP_NODELAY: ::libc::c_uint = 1;
pub static MEMCACHED_BEHAVIOR_HASH: ::libc::c_uint = 2;
pub static MEMCACHED_BEHAVIOR_KETAMA: ::libc::c_uint = 3;
pub static MEMCACHED_BEHAVIOR_SOCKET_SEND_SIZE: ::libc::c_uint = 4;
pub static MEMCACHED_BEHAVIOR_SOCKET_RECV_SIZE: ::libc::c_uint = 5;
pub static MEMCACHED_BEHAVIOR_CACHE_LOOKUPS: ::libc::c_uint = 6;
pub static MEMCACHED_BEHAVIOR_SUPPORT_CAS: ::libc::c_uint = 7;
pub static MEMCACHED_BEHAVIOR_POLL_TIMEOUT: ::libc::c_uint = 8;
pub static MEMCACHED_BEHAVIOR_DISTRIBUTION: ::libc::c_uint = 9;
pub static MEMCACHED_BEHAVIOR_BUFFER_REQUESTS: ::libc::c_uint = 10;
pub static MEMCACHED_BEHAVIOR_USER_DATA: ::libc::c_uint = 11;
pub static MEMCACHED_BEHAVIOR_SORT_HOSTS: ::libc::c_uint = 12;
pub static MEMCACHED_BEHAVIOR_VERIFY_KEY: ::libc::c_uint = 13;
pub static MEMCACHED_BEHAVIOR_CONNECT_TIMEOUT: ::libc::c_uint = 14;
pub static MEMCACHED_BEHAVIOR_RETRY_TIMEOUT: ::libc::c_uint = 15;
pub static MEMCACHED_BEHAVIOR_KETAMA_WEIGHTED: ::libc::c_uint = 16;
pub static MEMCACHED_BEHAVIOR_KETAMA_HASH: ::libc::c_uint = 17;
pub static MEMCACHED_BEHAVIOR_BINARY_PROTOCOL: ::libc::c_uint = 18;
pub static MEMCACHED_BEHAVIOR_SND_TIMEOUT: ::libc::c_uint = 19;
pub static MEMCACHED_BEHAVIOR_RCV_TIMEOUT: ::libc::c_uint = 20;
pub static MEMCACHED_BEHAVIOR_SERVER_FAILURE_LIMIT: ::libc::c_uint = 21;
pub static MEMCACHED_BEHAVIOR_IO_MSG_WATERMARK: ::libc::c_uint = 22;
pub static MEMCACHED_BEHAVIOR_IO_BYTES_WATERMARK: ::libc::c_uint = 23;
pub static MEMCACHED_BEHAVIOR_IO_KEY_PREFETCH: ::libc::c_uint = 24;
pub static MEMCACHED_BEHAVIOR_HASH_WITH_PREFIX_KEY: ::libc::c_uint = 25;
pub static MEMCACHED_BEHAVIOR_NOREPLY: ::libc::c_uint = 26;
pub static MEMCACHED_BEHAVIOR_USE_UDP: ::libc::c_uint = 27;
pub static MEMCACHED_BEHAVIOR_AUTO_EJECT_HOSTS: ::libc::c_uint = 28;
pub static MEMCACHED_BEHAVIOR_NUMBER_OF_REPLICAS: ::libc::c_uint = 29;
pub static MEMCACHED_BEHAVIOR_RANDOMIZE_REPLICA_READ: ::libc::c_uint = 30;
pub static MEMCACHED_BEHAVIOR_CORK: ::libc::c_uint = 31;
pub static MEMCACHED_BEHAVIOR_TCP_KEEPALIVE: ::libc::c_uint = 32;
pub static MEMCACHED_BEHAVIOR_TCP_KEEPIDLE: ::libc::c_uint = 33;
pub static MEMCACHED_BEHAVIOR_LOAD_FROM_FILE: ::libc::c_uint = 34;
pub static MEMCACHED_BEHAVIOR_REMOVE_FAILED_SERVERS: ::libc::c_uint = 35;
pub static MEMCACHED_BEHAVIOR_DEAD_TIMEOUT: ::libc::c_uint = 36;
pub static MEMCACHED_BEHAVIOR_MAX: ::libc::c_uint = 37;
pub type memcached_behavior_t = Enum_memcached_behavior_t;
pub type Enum_memcached_callback_t = ::libc::c_uint;
pub static MEMCACHED_CALLBACK_PREFIX_KEY: ::libc::c_uint = 0;
pub static MEMCACHED_CALLBACK_USER_DATA: ::libc::c_uint = 1;
pub static MEMCACHED_CALLBACK_CLEANUP_FUNCTION: ::libc::c_uint = 2;
pub static MEMCACHED_CALLBACK_CLONE_FUNCTION: ::libc::c_uint = 3;
pub static MEMCACHED_CALLBACK_GET_FAILURE: ::libc::c_uint = 7;
pub static MEMCACHED_CALLBACK_DELETE_TRIGGER: ::libc::c_uint = 8;
pub static MEMCACHED_CALLBACK_MAX: ::libc::c_uint = 9;
pub static MEMCACHED_CALLBACK_NAMESPACE: ::libc::c_uint = 0;
pub type memcached_callback_t = Enum_memcached_callback_t;
pub type Enum_memcached_connection_t = ::libc::c_uint;
pub static MEMCACHED_CONNECTION_TCP: ::libc::c_uint = 0;
pub static MEMCACHED_CONNECTION_UDP: ::libc::c_uint = 1;
pub static MEMCACHED_CONNECTION_UNIX_SOCKET: ::libc::c_uint = 2;
pub type memcached_connection_t = Enum_memcached_connection_t;
pub type Enum_memcached_hash_t = ::libc::c_uint;
pub static MEMCACHED_HASH_DEFAULT: ::libc::c_uint = 0;
pub static MEMCACHED_HASH_MD5: ::libc::c_uint = 1;
pub static MEMCACHED_HASH_CRC: ::libc::c_uint = 2;
pub static MEMCACHED_HASH_FNV1_64: ::libc::c_uint = 3;
pub static MEMCACHED_HASH_FNV1A_64: ::libc::c_uint = 4;
pub static MEMCACHED_HASH_FNV1_32: ::libc::c_uint = 5;
pub static MEMCACHED_HASH_FNV1A_32: ::libc::c_uint = 6;
pub static MEMCACHED_HASH_HSIEH: ::libc::c_uint = 7;
pub static MEMCACHED_HASH_MURMUR: ::libc::c_uint = 8;
pub static MEMCACHED_HASH_JENKINS: ::libc::c_uint = 9;
pub static MEMCACHED_HASH_CUSTOM: ::libc::c_uint = 10;
pub static MEMCACHED_HASH_MAX: ::libc::c_uint = 11;
pub type memcached_hash_t = Enum_memcached_hash_t;
pub type Enum_memcached_return_t = ::libc::c_uint;
pub static MEMCACHED_SUCCESS: ::libc::c_uint = 0;
pub static MEMCACHED_FAILURE: ::libc::c_uint = 1;
pub static MEMCACHED_HOST_LOOKUP_FAILURE: ::libc::c_uint = 2;
pub static MEMCACHED_CONNECTION_FAILURE: ::libc::c_uint = 3;
pub static MEMCACHED_CONNECTION_BIND_FAILURE: ::libc::c_uint = 4;
pub static MEMCACHED_WRITE_FAILURE: ::libc::c_uint = 5;
pub static MEMCACHED_READ_FAILURE: ::libc::c_uint = 6;
pub static MEMCACHED_UNKNOWN_READ_FAILURE: ::libc::c_uint = 7;
pub static MEMCACHED_PROTOCOL_ERROR: ::libc::c_uint = 8;
pub static MEMCACHED_CLIENT_ERROR: ::libc::c_uint = 9;
pub static MEMCACHED_SERVER_ERROR: ::libc::c_uint = 10;
pub static MEMCACHED_ERROR: ::libc::c_uint = 11;
pub static MEMCACHED_DATA_EXISTS: ::libc::c_uint = 12;
pub static MEMCACHED_DATA_DOES_NOT_EXIST: ::libc::c_uint = 13;
pub static MEMCACHED_NOTSTORED: ::libc::c_uint = 14;
pub static MEMCACHED_STORED: ::libc::c_uint = 15;
pub static MEMCACHED_NOTFOUND: ::libc::c_uint = 16;
pub static MEMCACHED_MEMORY_ALLOCATION_FAILURE: ::libc::c_uint = 17;
pub static MEMCACHED_PARTIAL_READ: ::libc::c_uint = 18;
pub static MEMCACHED_SOME_ERRORS: ::libc::c_uint = 19;
pub static MEMCACHED_NO_SERVERS: ::libc::c_uint = 20;
pub static MEMCACHED_END: ::libc::c_uint = 21;
pub static MEMCACHED_DELETED: ::libc::c_uint = 22;
pub static MEMCACHED_VALUE: ::libc::c_uint = 23;
pub static MEMCACHED_STAT: ::libc::c_uint = 24;
pub static MEMCACHED_ITEM: ::libc::c_uint = 25;
pub static MEMCACHED_ERRNO: ::libc::c_uint = 26;
pub static MEMCACHED_FAIL_UNIX_SOCKET: ::libc::c_uint = 27;
pub static MEMCACHED_NOT_SUPPORTED: ::libc::c_uint = 28;
pub static MEMCACHED_NO_KEY_PROVIDED: ::libc::c_uint = 29;
pub static MEMCACHED_FETCH_NOTFINISHED: ::libc::c_uint = 30;
pub static MEMCACHED_TIMEOUT: ::libc::c_uint = 31;
pub static MEMCACHED_BUFFERED: ::libc::c_uint = 32;
pub static MEMCACHED_BAD_KEY_PROVIDED: ::libc::c_uint = 33;
pub static MEMCACHED_INVALID_HOST_PROTOCOL: ::libc::c_uint = 34;
pub static MEMCACHED_SERVER_MARKED_DEAD: ::libc::c_uint = 35;
pub static MEMCACHED_UNKNOWN_STAT_KEY: ::libc::c_uint = 36;
pub static MEMCACHED_E2BIG: ::libc::c_uint = 37;
pub static MEMCACHED_INVALID_ARGUMENTS: ::libc::c_uint = 38;
pub static MEMCACHED_KEY_TOO_BIG: ::libc::c_uint = 39;
pub static MEMCACHED_AUTH_PROBLEM: ::libc::c_uint = 40;
pub static MEMCACHED_AUTH_FAILURE: ::libc::c_uint = 41;
pub static MEMCACHED_AUTH_CONTINUE: ::libc::c_uint = 42;
pub static MEMCACHED_PARSE_ERROR: ::libc::c_uint = 43;
pub static MEMCACHED_PARSE_USER_ERROR: ::libc::c_uint = 44;
pub static MEMCACHED_DEPRECATED: ::libc::c_uint = 45;
pub static MEMCACHED_IN_PROGRESS: ::libc::c_uint = 46;
pub static MEMCACHED_SERVER_TEMPORARILY_DISABLED: ::libc::c_uint = 47;
pub static MEMCACHED_SERVER_MEMORY_ALLOCATION_FAILURE: ::libc::c_uint = 48;
pub static MEMCACHED_MAXIMUM_RETURN: ::libc::c_uint = 49;
pub static MEMCACHED_CONNECTION_SOCKET_CREATE_FAILURE: ::libc::c_uint = 11;
pub type memcached_return_t = Enum_memcached_return_t;
pub type Enum_memcached_server_distribution_t = ::libc::c_uint;
pub static MEMCACHED_DISTRIBUTION_MODULA: ::libc::c_uint = 0;
pub static MEMCACHED_DISTRIBUTION_CONSISTENT: ::libc::c_uint = 1;
pub static MEMCACHED_DISTRIBUTION_CONSISTENT_KETAMA: ::libc::c_uint = 2;
pub static MEMCACHED_DISTRIBUTION_RANDOM: ::libc::c_uint = 3;
pub static MEMCACHED_DISTRIBUTION_CONSISTENT_KETAMA_SPY: ::libc::c_uint = 4;
pub static MEMCACHED_DISTRIBUTION_CONSISTENT_WEIGHTED: ::libc::c_uint = 5;
pub static MEMCACHED_DISTRIBUTION_VIRTUAL_BUCKET: ::libc::c_uint = 6;
pub static MEMCACHED_DISTRIBUTION_CONSISTENT_MAX: ::libc::c_uint = 7;
pub type memcached_server_distribution_t =
    Enum_memcached_server_distribution_t;
pub type memcached_st = Struct_memcached_st;
pub type memcached_stat_st = Struct_memcached_stat_st;
pub type memcached_analysis_st = Struct_memcached_analysis_st;
pub type memcached_result_st = Struct_memcached_result_st;
pub enum Struct_memcached_array_st { }
pub type memcached_array_st = Struct_memcached_array_st;
pub enum Struct_memcached_error_t { }
pub type memcached_error_t = Struct_memcached_error_t;
pub type memcached_server_st = Struct_memcached_server_st;
pub type memcached_server_instance_st = *Struct_memcached_server_st;
pub type memcached_server_list_st = *mut Struct_memcached_server_st;
pub type memcached_callback_st = Struct_memcached_callback_st;
pub type memcached_string_st = Struct_memcached_string_st;
pub type memcached_string_t = Struct_memcached_string_t;
pub enum Struct_memcached_continuum_item_st { }
pub type memcached_continuum_item_st = Struct_memcached_continuum_item_st;
pub type memcached_execute_fn =
    ::std::option::Option<extern "C" fn
                              (arg1: *memcached_st,
                               arg2: *mut memcached_result_st,
                               arg3: *mut ::libc::c_void)
                              -> memcached_return_t>;
pub type memcached_server_fn =
    ::std::option::Option<extern "C" fn
                              (arg1: *memcached_st,
                               arg2: memcached_server_instance_st,
                               arg3: *mut ::libc::c_void)
                              -> memcached_return_t>;
pub type memcached_stat_fn =
    ::std::option::Option<extern "C" fn
                              (arg1: memcached_server_instance_st,
                               arg2: *::libc::c_char, arg3: size_t,
                               arg4: *::libc::c_char, arg5: size_t,
                               arg6: *mut ::libc::c_void)
                              -> memcached_return_t>;
pub type memcached_free_fn =
    ::std::option::Option<extern "C" fn
                              (arg1: *memcached_st, arg2: *mut ::libc::c_void,
                               arg3: *mut ::libc::c_void)>;
pub type memcached_malloc_fn =
    ::std::option::Option<extern "C" fn
                              (arg1: *memcached_st, arg2: size_t,
                               arg3: *mut ::libc::c_void)
                              -> *mut ::libc::c_void>;
pub type memcached_realloc_fn =
    ::std::option::Option<extern "C" fn
                              (arg1: *memcached_st, arg2: *mut ::libc::c_void,
                               arg3: size_t, arg4: *mut ::libc::c_void)
                              -> *mut ::libc::c_void>;
pub type memcached_calloc_fn =
    ::std::option::Option<extern "C" fn
                              (arg1: *memcached_st, arg2: size_t,
                               arg3: size_t, arg4: *mut ::libc::c_void)
                              -> *mut ::libc::c_void>;
pub type memcached_clone_fn =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut memcached_st, arg2: *memcached_st)
                              -> memcached_return_t>;
pub type memcached_cleanup_fn =
    ::std::option::Option<extern "C" fn(arg1: *memcached_st)
                              -> memcached_return_t>;
pub type memcached_trigger_key_fn =
    ::std::option::Option<extern "C" fn
                              (arg1: *memcached_st, arg2: *::libc::c_char,
                               arg3: size_t, arg4: *mut memcached_result_st)
                              -> memcached_return_t>;
pub type memcached_trigger_delete_key_fn =
    ::std::option::Option<extern "C" fn
                              (arg1: *memcached_st, arg2: *::libc::c_char,
                               arg3: size_t) -> memcached_return_t>;
pub type memcached_dump_fn =
    ::std::option::Option<extern "C" fn
                              (arg1: *memcached_st, arg2: *::libc::c_char,
                               arg3: size_t, arg4: *mut ::libc::c_void)
                              -> memcached_return_t>;
pub type hashkit_return_t = ::libc::c_uint;
pub static HASHKIT_SUCCESS: ::libc::c_uint = 0;
pub static HASHKIT_FAILURE: ::libc::c_uint = 1;
pub static HASHKIT_MEMORY_ALLOCATION_FAILURE: ::libc::c_uint = 2;
pub static HASHKIT_INVALID_HASH: ::libc::c_uint = 3;
pub static HASHKIT_INVALID_ARGUMENT: ::libc::c_uint = 4;
pub static HASHKIT_MAXIMUM_RETURN: ::libc::c_uint = 5;
pub type hashkit_hash_algorithm_t = ::libc::c_uint;
pub static HASHKIT_HASH_DEFAULT: ::libc::c_uint = 0;
pub static HASHKIT_HASH_MD5: ::libc::c_uint = 1;
pub static HASHKIT_HASH_CRC: ::libc::c_uint = 2;
pub static HASHKIT_HASH_FNV1_64: ::libc::c_uint = 3;
pub static HASHKIT_HASH_FNV1A_64: ::libc::c_uint = 4;
pub static HASHKIT_HASH_FNV1_32: ::libc::c_uint = 5;
pub static HASHKIT_HASH_FNV1A_32: ::libc::c_uint = 6;
pub static HASHKIT_HASH_HSIEH: ::libc::c_uint = 7;
pub static HASHKIT_HASH_MURMUR: ::libc::c_uint = 8;
pub static HASHKIT_HASH_JENKINS: ::libc::c_uint = 9;
pub static HASHKIT_HASH_CUSTOM: ::libc::c_uint = 10;
pub static HASHKIT_HASH_MAX: ::libc::c_uint = 11;
pub type hashkit_distribution_t = ::libc::c_uint;
pub static HASHKIT_DISTRIBUTION_MODULA: ::libc::c_uint = 0;
pub static HASHKIT_DISTRIBUTION_RANDOM: ::libc::c_uint = 1;
pub static HASHKIT_DISTRIBUTION_KETAMA: ::libc::c_uint = 2;
pub static HASHKIT_DISTRIBUTION_MAX: ::libc::c_uint = 3;
pub type hashkit_st = Struct_hashkit_st;
pub enum Struct_hashkit_string_st { }
pub type hashkit_string_st = Struct_hashkit_string_st;
pub type hashkit_hash_fn =
    ::std::option::Option<extern "C" fn
                              (arg1: *::libc::c_char, arg2: size_t,
                               arg3: *mut ::libc::c_void) -> uint32_t>;
pub struct Struct_hashkit_st {
    pub base_hash: Struct_hashkit_function_st,
    pub distribution_hash: Struct_hashkit_function_st,
    pub flags: Struct_Unnamed17,
    pub options: Struct_Unnamed18,
    pub _key: *mut ::libc::c_void,
}
pub struct Struct_hashkit_function_st {
    pub function: hashkit_hash_fn,
    pub context: *mut ::libc::c_void,
}
pub struct Struct_Unnamed17 {
    pub is_base_same_distributed: ::libc::c_int,
}
pub struct Struct_Unnamed18 {
    pub is_allocated: ::libc::c_int,
}
pub struct Struct_memcached_callback_st {
    pub callback: *mut memcached_execute_fn,
    pub context: *mut ::libc::c_void,
    pub number_of_callback: uint32_t,
}
pub struct Struct_memcached_string_st {
    pub end: *mut ::libc::c_char,
    pub string: *mut ::libc::c_char,
    pub current_size: size_t,
    pub root: *mut Struct_memcached_st,
    pub options: Struct_Unnamed19,
}
pub struct Struct_Unnamed19 {
    pub is_allocated: ::libc::c_int,
    pub is_initialized: ::libc::c_int,
}
pub struct Struct_memcached_result_st {
    pub item_flags: uint32_t,
    pub item_expiration: time_t,
    pub key_length: size_t,
    pub item_cas: uint64_t,
    pub root: *mut Struct_memcached_st,
    pub value: memcached_string_st,
    pub numeric_value: uint64_t,
    pub count: uint64_t,
    pub item_key: [::libc::c_char, ..251u],
    pub options: Struct_Unnamed20,
}
pub struct Struct_Unnamed20 {
    pub is_allocated: ::libc::c_int,
    pub is_initialized: ::libc::c_int,
}
pub struct Struct_memcached_allocator_t {
    pub calloc: memcached_calloc_fn,
    pub free: memcached_free_fn,
    pub malloc: memcached_malloc_fn,
    pub realloc: memcached_realloc_fn,
    pub context: *mut ::libc::c_void,
}
pub struct Struct_memcached_sasl_st {
    pub callbacks: *mut ::libc::c_void,
    pub is_allocated: ::libc::c_int,
}
pub struct Struct_memcached_st {
    pub state: Struct_Unnamed21,
    pub flags: Struct_Unnamed22,
    pub distribution: memcached_server_distribution_t,
    pub hashkit: hashkit_st,
    pub server_info: Struct_Unnamed23,
    pub number_of_hosts: uint32_t,
    pub servers: *mut memcached_server_st,
    pub last_disconnected_server: *mut memcached_server_st,
    pub snd_timeout: int32_t,
    pub rcv_timeout: int32_t,
    pub server_failure_limit: uint32_t,
    pub io_msg_watermark: uint32_t,
    pub io_bytes_watermark: uint32_t,
    pub io_key_prefetch: uint32_t,
    pub tcp_keepidle: uint32_t,
    pub poll_timeout: int32_t,
    pub connect_timeout: int32_t,
    pub retry_timeout: int32_t,
    pub dead_timeout: int32_t,
    pub send_size: ::libc::c_int,
    pub recv_size: ::libc::c_int,
    pub user_data: *mut ::libc::c_void,
    pub query_id: uint64_t,
    pub number_of_replicas: uint32_t,
    pub result: memcached_result_st,
    pub ketama: Struct_Unnamed24,
    pub virtual_bucket: *mut Struct_memcached_virtual_bucket_t,
    pub allocators: Struct_memcached_allocator_t,
    pub on_clone: memcached_clone_fn,
    pub on_cleanup: memcached_cleanup_fn,
    pub get_key_failure: memcached_trigger_key_fn,
    pub delete_trigger: memcached_trigger_delete_key_fn,
    pub callbacks: *mut memcached_callback_st,
    pub sasl: Struct_memcached_sasl_st,
    pub error_messages: *mut Struct_memcached_error_t,
    pub _namespace: *mut Struct_memcached_array_st,
    pub configure: Struct_Unnamed25,
    pub options: Struct_Unnamed26,
}
pub struct Struct_Unnamed21 {
    pub is_purging: ::libc::c_int,
    pub is_processing_input: ::libc::c_int,
    pub is_time_for_rebuild: ::libc::c_int,
}
pub struct Struct_Unnamed22 {
    pub auto_eject_hosts: ::libc::c_int,
    pub binary_protocol: ::libc::c_int,
    pub buffer_requests: ::libc::c_int,
    pub hash_with_namespace: ::libc::c_int,
    pub no_block: ::libc::c_int,
    pub reply: ::libc::c_int,
    pub randomize_replica_read: ::libc::c_int,
    pub support_cas: ::libc::c_int,
    pub tcp_nodelay: ::libc::c_int,
    pub use_sort_hosts: ::libc::c_int,
    pub use_udp: ::libc::c_int,
    pub verify_key: ::libc::c_int,
    pub tcp_keepalive: ::libc::c_int,
    pub is_aes: ::libc::c_int,
}
pub struct Struct_Unnamed23 {
    pub version: ::libc::c_uint,
}
pub struct Struct_Unnamed24 {
    pub weighted: ::libc::c_int,
    pub continuum_count: uint32_t,
    pub continuum_points_counter: uint32_t,
    pub next_distribution_rebuild: time_t,
    pub continuum: *mut memcached_continuum_item_st,
}
pub enum Struct_memcached_virtual_bucket_t { }
pub struct Struct_Unnamed25 {
    pub initial_pool_size: uint32_t,
    pub max_pool_size: uint32_t,
    pub version: int32_t,
    pub filename: *mut Struct_memcached_array_st,
}
pub struct Struct_Unnamed26 {
    pub is_allocated: ::libc::c_int,
}
pub type Enum_memcached_server_state_t = ::libc::c_uint;
pub static MEMCACHED_SERVER_STATE_NEW: ::libc::c_uint = 0;
pub static MEMCACHED_SERVER_STATE_ADDRINFO: ::libc::c_uint = 1;
pub static MEMCACHED_SERVER_STATE_IN_PROGRESS: ::libc::c_uint = 2;
pub static MEMCACHED_SERVER_STATE_CONNECTED: ::libc::c_uint = 3;
pub static MEMCACHED_SERVER_STATE_IN_TIMEOUT: ::libc::c_uint = 4;
pub struct Struct_memcached_server_st {
    pub options: Struct_Unnamed27,
    pub number_of_hosts: uint32_t,
    pub cursor_active: uint32_t,
    pub port: in_port_t,
    pub fd: memcached_socket_t,
    pub io_bytes_sent: uint32_t,
    pub server_failure_counter: uint32_t,
    pub server_failure_counter_query_id: uint64_t,
    pub weight: uint32_t,
    pub version: uint32_t,
    pub state: Enum_memcached_server_state_t,
    pub io_wait_count: Struct_Unnamed28,
    pub major_version: uint8_t,
    pub micro_version: uint8_t,
    pub minor_version: uint8_t,
    pub _type: memcached_connection_t,
    pub read_ptr: *mut ::libc::c_char,
    pub read_buffer_length: size_t,
    pub read_data_length: size_t,
    pub write_buffer_offset: size_t,
    pub address_info: *mut Struct_addrinfo,
    pub address_info_next: *mut Struct_addrinfo,
    pub next_retry: time_t,
    pub root: *mut Struct_memcached_st,
    pub limit_maxbytes: uint64_t,
    pub error_messages: *mut Struct_memcached_error_t,
    pub read_buffer: [::libc::c_char, ..8196u],
    pub write_buffer: [::libc::c_char, ..8196u],
    pub hostname: [::libc::c_char, ..1025u],
}
pub struct Struct_Unnamed27 {
    pub is_allocated: ::libc::c_int,
    pub is_initialized: ::libc::c_int,
    pub is_shutting_down: ::libc::c_int,
    pub is_dead: ::libc::c_int,
}
pub struct Struct_Unnamed28 {
    pub read: uint32_t,
    pub write: uint32_t,
    pub timeouts: uint32_t,
}
pub struct Struct_memcached_stat_st {
    pub connection_structures: ::libc::c_ulong,
    pub curr_connections: ::libc::c_ulong,
    pub curr_items: ::libc::c_ulong,
    pub pid: pid_t,
    pub pointer_size: ::libc::c_ulong,
    pub rusage_system_microseconds: ::libc::c_ulong,
    pub rusage_system_seconds: ::libc::c_ulong,
    pub rusage_user_microseconds: ::libc::c_ulong,
    pub rusage_user_seconds: ::libc::c_ulong,
    pub threads: ::libc::c_ulong,
    pub time: ::libc::c_ulong,
    pub total_connections: ::libc::c_ulong,
    pub total_items: ::libc::c_ulong,
    pub uptime: ::libc::c_ulong,
    pub bytes: ::libc::c_ulonglong,
    pub bytes_read: ::libc::c_ulonglong,
    pub bytes_written: ::libc::c_ulonglong,
    pub cmd_get: ::libc::c_ulonglong,
    pub cmd_set: ::libc::c_ulonglong,
    pub evictions: ::libc::c_ulonglong,
    pub get_hits: ::libc::c_ulonglong,
    pub get_misses: ::libc::c_ulonglong,
    pub limit_maxbytes: ::libc::c_ulonglong,
    pub version: [::libc::c_char, ..24u],
    pub __future: *mut ::libc::c_void,
    pub root: *mut memcached_st,
}
pub struct Struct_memcached_string_t {
    pub c_str: *::libc::c_char,
    pub size: size_t,
}
pub struct Struct_memcached_analysis_st {
    pub root: *mut memcached_st,
    pub average_item_size: uint32_t,
    pub longest_uptime: uint32_t,
    pub least_free_server: uint32_t,
    pub most_consumed_server: uint32_t,
    pub oldest_server: uint32_t,
    pub pool_hit_ratio: ::libc::c_double,
    pub most_used_bytes: uint64_t,
    pub least_remaining_bytes: uint64_t,
}
pub type memcached_return = memcached_return_t;
pub type memcached_server_distribution = memcached_server_distribution_t;
pub type memcached_behavior = memcached_behavior_t;
pub type memcached_callback = memcached_callback_t;
pub type memcached_hash = memcached_hash_t;
pub type memcached_connection = memcached_connection_t;
pub type memcached_clone_func = memcached_clone_fn;
pub type memcached_cleanup_func = memcached_cleanup_fn;
pub type memcached_execute_function = memcached_execute_fn;
pub type memcached_server_function = memcached_server_fn;
pub type memcached_trigger_key = memcached_trigger_key_fn;
pub type memcached_trigger_delete_key = memcached_trigger_delete_key_fn;
pub type memcached_dump_func = memcached_dump_fn;
#[link(name = "memcached")]
extern "C" {
    pub static in6addr_any: Struct_in6_addr;
    pub static in6addr_loopback: Struct_in6_addr;
    pub fn imaxabs(__n: intmax_t) -> intmax_t;
    pub fn imaxdiv(__numer: intmax_t, __denom: intmax_t) -> imaxdiv_t;
    pub fn strtoimax(__nptr: *::libc::c_char,
                     __endptr: *mut *mut ::libc::c_char,
                     __base: ::libc::c_int) -> intmax_t;
    pub fn strtoumax(__nptr: *::libc::c_char,
                     __endptr: *mut *mut ::libc::c_char,
                     __base: ::libc::c_int) -> uintmax_t;
    pub fn wcstoimax(__nptr: *__gwchar_t, __endptr: *mut *mut __gwchar_t,
                     __base: ::libc::c_int) -> intmax_t;
    pub fn wcstoumax(__nptr: *__gwchar_t, __endptr: *mut *mut __gwchar_t,
                     __base: ::libc::c_int) -> uintmax_t;
    pub fn __ctype_get_mb_cur_max() -> size_t;
    pub fn atof(__nptr: *::libc::c_char) -> ::libc::c_double;
    pub fn atoi(__nptr: *::libc::c_char) -> ::libc::c_int;
    pub fn atol(__nptr: *::libc::c_char) -> ::libc::c_long;
    pub fn atoll(__nptr: *::libc::c_char) -> ::libc::c_longlong;
    pub fn strtod(__nptr: *::libc::c_char, __endptr: *mut *mut ::libc::c_char)
     -> ::libc::c_double;
    pub fn strtof(__nptr: *::libc::c_char, __endptr: *mut *mut ::libc::c_char)
     -> ::libc::c_float;
    pub fn strtold(__nptr: *::libc::c_char,
                   __endptr: *mut *mut ::libc::c_char) -> ::libc::c_double;
    pub fn strtol(__nptr: *::libc::c_char, __endptr: *mut *mut ::libc::c_char,
                  __base: ::libc::c_int) -> ::libc::c_long;
    pub fn strtoul(__nptr: *::libc::c_char,
                   __endptr: *mut *mut ::libc::c_char, __base: ::libc::c_int)
     -> ::libc::c_ulong;
    pub fn strtoq(__nptr: *::libc::c_char, __endptr: *mut *mut ::libc::c_char,
                  __base: ::libc::c_int) -> ::libc::c_longlong;
    pub fn strtouq(__nptr: *::libc::c_char,
                   __endptr: *mut *mut ::libc::c_char, __base: ::libc::c_int)
     -> ::libc::c_ulonglong;
    pub fn strtoll(__nptr: *::libc::c_char,
                   __endptr: *mut *mut ::libc::c_char, __base: ::libc::c_int)
     -> ::libc::c_longlong;
    pub fn strtoull(__nptr: *::libc::c_char,
                    __endptr: *mut *mut ::libc::c_char, __base: ::libc::c_int)
     -> ::libc::c_ulonglong;
    pub fn l64a(__n: ::libc::c_long) -> *mut ::libc::c_char;
    pub fn a64l(__s: *::libc::c_char) -> ::libc::c_long;
    pub fn select(__nfds: ::libc::c_int, __readfds: *mut fd_set,
                  __writefds: *mut fd_set, __exceptfds: *mut fd_set,
                  __timeout: *mut Struct_timeval) -> ::libc::c_int;
    pub fn pselect(__nfds: ::libc::c_int, __readfds: *mut fd_set,
                   __writefds: *mut fd_set, __exceptfds: *mut fd_set,
                   __timeout: *Struct_timespec, __sigmask: *__sigset_t) ->
     ::libc::c_int;
    pub fn gnu_dev_major(__dev: ::libc::c_ulonglong) -> ::libc::c_uint;
    pub fn gnu_dev_minor(__dev: ::libc::c_ulonglong) -> ::libc::c_uint;
    pub fn gnu_dev_makedev(__major: ::libc::c_uint, __minor: ::libc::c_uint)
     -> ::libc::c_ulonglong;
    pub fn random() -> ::libc::c_long;
    pub fn srandom(__seed: ::libc::c_uint);
    pub fn initstate(__seed: ::libc::c_uint, __statebuf: *mut ::libc::c_char,
                     __statelen: size_t) -> *mut ::libc::c_char;
    pub fn setstate(__statebuf: *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn random_r(__buf: *mut Struct_random_data, __result: *mut int32_t) ->
     ::libc::c_int;
    pub fn srandom_r(__seed: ::libc::c_uint, __buf: *mut Struct_random_data)
     -> ::libc::c_int;
    pub fn initstate_r(__seed: ::libc::c_uint,
                       __statebuf: *mut ::libc::c_char, __statelen: size_t,
                       __buf: *mut Struct_random_data) -> ::libc::c_int;
    pub fn setstate_r(__statebuf: *mut ::libc::c_char,
                      __buf: *mut Struct_random_data) -> ::libc::c_int;
    pub fn rand() -> ::libc::c_int;
    pub fn srand(__seed: ::libc::c_uint);
    pub fn rand_r(__seed: *mut ::libc::c_uint) -> ::libc::c_int;
    pub fn drand48() -> ::libc::c_double;
    pub fn erand48(__xsubi: [::libc::c_ushort, ..3u]) -> ::libc::c_double;
    pub fn lrand48() -> ::libc::c_long;
    pub fn nrand48(__xsubi: [::libc::c_ushort, ..3u]) -> ::libc::c_long;
    pub fn mrand48() -> ::libc::c_long;
    pub fn jrand48(__xsubi: [::libc::c_ushort, ..3u]) -> ::libc::c_long;
    pub fn srand48(__seedval: ::libc::c_long);
    pub fn seed48(__seed16v: [::libc::c_ushort, ..3u]) ->
     *mut ::libc::c_ushort;
    pub fn lcong48(__param: [::libc::c_ushort, ..7u]);
    pub fn drand48_r(__buffer: *mut Struct_drand48_data,
                     __result: *mut ::libc::c_double) -> ::libc::c_int;
    pub fn erand48_r(__xsubi: [::libc::c_ushort, ..3u],
                     __buffer: *mut Struct_drand48_data,
                     __result: *mut ::libc::c_double) -> ::libc::c_int;
    pub fn lrand48_r(__buffer: *mut Struct_drand48_data,
                     __result: *mut ::libc::c_long) -> ::libc::c_int;
    pub fn nrand48_r(__xsubi: [::libc::c_ushort, ..3u],
                     __buffer: *mut Struct_drand48_data,
                     __result: *mut ::libc::c_long) -> ::libc::c_int;
    pub fn mrand48_r(__buffer: *mut Struct_drand48_data,
                     __result: *mut ::libc::c_long) -> ::libc::c_int;
    pub fn jrand48_r(__xsubi: [::libc::c_ushort, ..3u],
                     __buffer: *mut Struct_drand48_data,
                     __result: *mut ::libc::c_long) -> ::libc::c_int;
    pub fn srand48_r(__seedval: ::libc::c_long,
                     __buffer: *mut Struct_drand48_data) -> ::libc::c_int;
    pub fn seed48_r(__seed16v: [::libc::c_ushort, ..3u],
                    __buffer: *mut Struct_drand48_data) -> ::libc::c_int;
    pub fn lcong48_r(__param: [::libc::c_ushort, ..7u],
                     __buffer: *mut Struct_drand48_data) -> ::libc::c_int;
    pub fn malloc(__size: size_t) -> *mut ::libc::c_void;
    pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::libc::c_void;
    pub fn realloc(__ptr: *mut ::libc::c_void, __size: size_t) ->
     *mut ::libc::c_void;
    pub fn free(__ptr: *mut ::libc::c_void);
    pub fn cfree(__ptr: *mut ::libc::c_void);
    pub fn alloca(__size: size_t) -> *mut ::libc::c_void;
    pub fn valloc(__size: size_t) -> *mut ::libc::c_void;
    pub fn posix_memalign(__memptr: *mut *mut ::libc::c_void,
                          __alignment: size_t, __size: size_t) ->
     ::libc::c_int;
    pub fn abort();
    pub fn atexit(__func: ::std::option::Option<extern "C" fn()>) ->
     ::libc::c_int;
    pub fn on_exit(__func:
                       ::std::option::Option<extern "C" fn
                                                 (arg1: ::libc::c_int,
                                                  arg2: *mut ::libc::c_void)>,
                   __arg: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn exit(__status: ::libc::c_int);
    pub fn _Exit(__status: ::libc::c_int);
    pub fn getenv(__name: *::libc::c_char) -> *mut ::libc::c_char;
    pub fn putenv(__string: *mut ::libc::c_char) -> ::libc::c_int;
    pub fn setenv(__name: *::libc::c_char, __value: *::libc::c_char,
                  __replace: ::libc::c_int) -> ::libc::c_int;
    pub fn unsetenv(__name: *::libc::c_char) -> ::libc::c_int;
    pub fn clearenv() -> ::libc::c_int;
    pub fn mktemp(__template: *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn mkstemp(__template: *mut ::libc::c_char) -> ::libc::c_int;
    pub fn mkstemps(__template: *mut ::libc::c_char,
                    __suffixlen: ::libc::c_int) -> ::libc::c_int;
    pub fn mkdtemp(__template: *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn system(__command: *::libc::c_char) -> ::libc::c_int;
    pub fn realpath(__name: *::libc::c_char, __resolved: *mut ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn bsearch(__key: *::libc::c_void, __base: *::libc::c_void,
                   __nmemb: size_t, __size: size_t, __compar: __compar_fn_t)
     -> *mut ::libc::c_void;
    pub fn qsort(__base: *mut ::libc::c_void, __nmemb: size_t, __size: size_t,
                 __compar: __compar_fn_t);
    pub fn abs(__x: ::libc::c_int) -> ::libc::c_int;
    pub fn labs(__x: ::libc::c_long) -> ::libc::c_long;
    pub fn llabs(__x: ::libc::c_longlong) -> ::libc::c_longlong;
    pub fn div(__numer: ::libc::c_int, __denom: ::libc::c_int) -> div_t;
    pub fn ldiv(__numer: ::libc::c_long, __denom: ::libc::c_long) -> ldiv_t;
    pub fn lldiv(__numer: ::libc::c_longlong, __denom: ::libc::c_longlong) ->
     lldiv_t;
    pub fn ecvt(__value: ::libc::c_double, __ndigit: ::libc::c_int,
                __decpt: *mut ::libc::c_int, __sign: *mut ::libc::c_int) ->
     *mut ::libc::c_char;
    pub fn fcvt(__value: ::libc::c_double, __ndigit: ::libc::c_int,
                __decpt: *mut ::libc::c_int, __sign: *mut ::libc::c_int) ->
     *mut ::libc::c_char;
    pub fn gcvt(__value: ::libc::c_double, __ndigit: ::libc::c_int,
                __buf: *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn qecvt(__value: ::libc::c_double, __ndigit: ::libc::c_int,
                 __decpt: *mut ::libc::c_int, __sign: *mut ::libc::c_int) ->
     *mut ::libc::c_char;
    pub fn qfcvt(__value: ::libc::c_double, __ndigit: ::libc::c_int,
                 __decpt: *mut ::libc::c_int, __sign: *mut ::libc::c_int) ->
     *mut ::libc::c_char;
    pub fn qgcvt(__value: ::libc::c_double, __ndigit: ::libc::c_int,
                 __buf: *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn ecvt_r(__value: ::libc::c_double, __ndigit: ::libc::c_int,
                  __decpt: *mut ::libc::c_int, __sign: *mut ::libc::c_int,
                  __buf: *mut ::libc::c_char, __len: size_t) -> ::libc::c_int;
    pub fn fcvt_r(__value: ::libc::c_double, __ndigit: ::libc::c_int,
                  __decpt: *mut ::libc::c_int, __sign: *mut ::libc::c_int,
                  __buf: *mut ::libc::c_char, __len: size_t) -> ::libc::c_int;
    pub fn qecvt_r(__value: ::libc::c_double, __ndigit: ::libc::c_int,
                   __decpt: *mut ::libc::c_int, __sign: *mut ::libc::c_int,
                   __buf: *mut ::libc::c_char, __len: size_t) ->
     ::libc::c_int;
    pub fn qfcvt_r(__value: ::libc::c_double, __ndigit: ::libc::c_int,
                   __decpt: *mut ::libc::c_int, __sign: *mut ::libc::c_int,
                   __buf: *mut ::libc::c_char, __len: size_t) ->
     ::libc::c_int;
    pub fn mblen(__s: *::libc::c_char, __n: size_t) -> ::libc::c_int;
    pub fn mbtowc(__pwc: *mut wchar_t, __s: *::libc::c_char, __n: size_t) ->
     ::libc::c_int;
    pub fn wctomb(__s: *mut ::libc::c_char, __wchar: wchar_t) ->
     ::libc::c_int;
    pub fn mbstowcs(__pwcs: *mut wchar_t, __s: *::libc::c_char, __n: size_t)
     -> size_t;
    pub fn wcstombs(__s: *mut ::libc::c_char, __pwcs: *wchar_t, __n: size_t)
     -> size_t;
    pub fn rpmatch(__response: *::libc::c_char) -> ::libc::c_int;
    pub fn getsubopt(__optionp: *mut *mut ::libc::c_char,
                     __tokens: **mut ::libc::c_char,
                     __valuep: *mut *mut ::libc::c_char) -> ::libc::c_int;
    pub fn getloadavg(__loadavg: *mut ::libc::c_double,
                      __nelem: ::libc::c_int) -> ::libc::c_int;
    pub fn readv(__fd: ::libc::c_int, __iovec: *Struct_iovec,
                 __count: ::libc::c_int) -> ssize_t;
    pub fn writev(__fd: ::libc::c_int, __iovec: *Struct_iovec,
                  __count: ::libc::c_int) -> ssize_t;
    pub fn preadv(__fd: ::libc::c_int, __iovec: *Struct_iovec,
                  __count: ::libc::c_int, __offset: __off_t) -> ssize_t;
    pub fn pwritev(__fd: ::libc::c_int, __iovec: *Struct_iovec,
                   __count: ::libc::c_int, __offset: __off_t) -> ssize_t;
    pub fn __cmsg_nxthdr(__mhdr: *mut Struct_msghdr,
                         __cmsg: *mut Struct_cmsghdr) -> *mut Struct_cmsghdr;
    pub fn socket(__domain: ::libc::c_int, __type: ::libc::c_int,
                  __protocol: ::libc::c_int) -> ::libc::c_int;
    pub fn socketpair(__domain: ::libc::c_int, __type: ::libc::c_int,
                      __protocol: ::libc::c_int, __fds: [::libc::c_int, ..2u])
     -> ::libc::c_int;
    pub fn bind(__fd: ::libc::c_int, __addr: *Struct_sockaddr,
                __len: socklen_t) -> ::libc::c_int;
    pub fn getsockname(__fd: ::libc::c_int, __addr: *mut Struct_sockaddr,
                       __len: *mut socklen_t) -> ::libc::c_int;
    pub fn connect(__fd: ::libc::c_int, __addr: *Struct_sockaddr,
                   __len: socklen_t) -> ::libc::c_int;
    pub fn getpeername(__fd: ::libc::c_int, __addr: *mut Struct_sockaddr,
                       __len: *mut socklen_t) -> ::libc::c_int;
    pub fn send(__fd: ::libc::c_int, __buf: *::libc::c_void, __n: size_t,
                __flags: ::libc::c_int) -> ssize_t;
    pub fn recv(__fd: ::libc::c_int, __buf: *mut ::libc::c_void, __n: size_t,
                __flags: ::libc::c_int) -> ssize_t;
    pub fn sendto(__fd: ::libc::c_int, __buf: *::libc::c_void, __n: size_t,
                  __flags: ::libc::c_int, __addr: *Struct_sockaddr,
                  __addr_len: socklen_t) -> ssize_t;
    pub fn recvfrom(__fd: ::libc::c_int, __buf: *mut ::libc::c_void,
                    __n: size_t, __flags: ::libc::c_int,
                    __addr: *mut Struct_sockaddr, __addr_len: *mut socklen_t)
     -> ssize_t;
    pub fn sendmsg(__fd: ::libc::c_int, __message: *Struct_msghdr,
                   __flags: ::libc::c_int) -> ssize_t;
    pub fn recvmsg(__fd: ::libc::c_int, __message: *mut Struct_msghdr,
                   __flags: ::libc::c_int) -> ssize_t;
    pub fn getsockopt(__fd: ::libc::c_int, __level: ::libc::c_int,
                      __optname: ::libc::c_int, __optval: *mut ::libc::c_void,
                      __optlen: *mut socklen_t) -> ::libc::c_int;
    pub fn setsockopt(__fd: ::libc::c_int, __level: ::libc::c_int,
                      __optname: ::libc::c_int, __optval: *::libc::c_void,
                      __optlen: socklen_t) -> ::libc::c_int;
    pub fn listen(__fd: ::libc::c_int, __n: ::libc::c_int) -> ::libc::c_int;
    pub fn accept(__fd: ::libc::c_int, __addr: *mut Struct_sockaddr,
                  __addr_len: *mut socklen_t) -> ::libc::c_int;
    pub fn shutdown(__fd: ::libc::c_int, __how: ::libc::c_int) ->
     ::libc::c_int;
    pub fn sockatmark(__fd: ::libc::c_int) -> ::libc::c_int;
    pub fn isfdtype(__fd: ::libc::c_int, __fdtype: ::libc::c_int) ->
     ::libc::c_int;
    pub fn ntohl(__netlong: uint32_t) -> uint32_t;
    pub fn ntohs(__netshort: uint16_t) -> uint16_t;
    pub fn htonl(__hostlong: uint32_t) -> uint32_t;
    pub fn htons(__hostshort: uint16_t) -> uint16_t;
    pub fn bindresvport(__sockfd: ::libc::c_int,
                        __sock_in: *mut Struct_sockaddr_in) -> ::libc::c_int;
    pub fn bindresvport6(__sockfd: ::libc::c_int,
                         __sock_in: *mut Struct_sockaddr_in6) ->
     ::libc::c_int;
    pub fn inet_addr(__cp: *::libc::c_char) -> in_addr_t;
    pub fn inet_lnaof(__in: Struct_in_addr) -> in_addr_t;
    pub fn inet_makeaddr(__net: in_addr_t, __host: in_addr_t) ->
     Struct_in_addr;
    pub fn inet_netof(__in: Struct_in_addr) -> in_addr_t;
    pub fn inet_network(__cp: *::libc::c_char) -> in_addr_t;
    pub fn inet_ntoa(__in: Struct_in_addr) -> *mut ::libc::c_char;
    pub fn inet_pton(__af: ::libc::c_int, __cp: *::libc::c_char,
                     __buf: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn inet_ntop(__af: ::libc::c_int, __cp: *::libc::c_void,
                     __buf: *mut ::libc::c_char, __len: socklen_t) ->
     *::libc::c_char;
    pub fn inet_aton(__cp: *::libc::c_char, __inp: *mut Struct_in_addr) ->
     ::libc::c_int;
    pub fn inet_neta(__net: in_addr_t, __buf: *mut ::libc::c_char,
                     __len: size_t) -> *mut ::libc::c_char;
    pub fn inet_net_ntop(__af: ::libc::c_int, __cp: *::libc::c_void,
                         __bits: ::libc::c_int, __buf: *mut ::libc::c_char,
                         __len: size_t) -> *mut ::libc::c_char;
    pub fn inet_net_pton(__af: ::libc::c_int, __cp: *::libc::c_char,
                         __buf: *mut ::libc::c_void, __len: size_t) ->
     ::libc::c_int;
    pub fn inet_nsap_addr(__cp: *::libc::c_char, __buf: *mut ::libc::c_uchar,
                          __len: ::libc::c_int) -> ::libc::c_uint;
    pub fn inet_nsap_ntoa(__len: ::libc::c_int, __cp: *::libc::c_uchar,
                          __buf: *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn setrpcent(__stayopen: ::libc::c_int);
    pub fn endrpcent();
    pub fn getrpcbyname(__name: *::libc::c_char) -> *mut Struct_rpcent;
    pub fn getrpcbynumber(__number: ::libc::c_int) -> *mut Struct_rpcent;
    pub fn getrpcent() -> *mut Struct_rpcent;
    pub fn getrpcbyname_r(__name: *::libc::c_char,
                          __result_buf: *mut Struct_rpcent,
                          __buffer: *mut ::libc::c_char, __buflen: size_t,
                          __result: *mut *mut Struct_rpcent) -> ::libc::c_int;
    pub fn getrpcbynumber_r(__number: ::libc::c_int,
                            __result_buf: *mut Struct_rpcent,
                            __buffer: *mut ::libc::c_char, __buflen: size_t,
                            __result: *mut *mut Struct_rpcent) ->
     ::libc::c_int;
    pub fn getrpcent_r(__result_buf: *mut Struct_rpcent,
                       __buffer: *mut ::libc::c_char, __buflen: size_t,
                       __result: *mut *mut Struct_rpcent) -> ::libc::c_int;
    pub fn __h_errno_location() -> *mut ::libc::c_int;
    pub fn herror(__str: *::libc::c_char);
    pub fn hstrerror(__err_num: ::libc::c_int) -> *::libc::c_char;
    pub fn sethostent(__stay_open: ::libc::c_int);
    pub fn endhostent();
    pub fn gethostent() -> *mut Struct_hostent;
    pub fn gethostbyaddr(__addr: *::libc::c_void, __len: __socklen_t,
                         __type: ::libc::c_int) -> *mut Struct_hostent;
    pub fn gethostbyname(__name: *::libc::c_char) -> *mut Struct_hostent;
    pub fn gethostbyname2(__name: *::libc::c_char, __af: ::libc::c_int) ->
     *mut Struct_hostent;
    pub fn gethostent_r(__result_buf: *mut Struct_hostent,
                        __buf: *mut ::libc::c_char, __buflen: size_t,
                        __result: *mut *mut Struct_hostent,
                        __h_errnop: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn gethostbyaddr_r(__addr: *::libc::c_void, __len: __socklen_t,
                           __type: ::libc::c_int,
                           __result_buf: *mut Struct_hostent,
                           __buf: *mut ::libc::c_char, __buflen: size_t,
                           __result: *mut *mut Struct_hostent,
                           __h_errnop: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn gethostbyname_r(__name: *::libc::c_char,
                           __result_buf: *mut Struct_hostent,
                           __buf: *mut ::libc::c_char, __buflen: size_t,
                           __result: *mut *mut Struct_hostent,
                           __h_errnop: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn gethostbyname2_r(__name: *::libc::c_char, __af: ::libc::c_int,
                            __result_buf: *mut Struct_hostent,
                            __buf: *mut ::libc::c_char, __buflen: size_t,
                            __result: *mut *mut Struct_hostent,
                            __h_errnop: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn setnetent(__stay_open: ::libc::c_int);
    pub fn endnetent();
    pub fn getnetent() -> *mut Struct_netent;
    pub fn getnetbyaddr(__net: uint32_t, __type: ::libc::c_int) ->
     *mut Struct_netent;
    pub fn getnetbyname(__name: *::libc::c_char) -> *mut Struct_netent;
    pub fn getnetent_r(__result_buf: *mut Struct_netent,
                       __buf: *mut ::libc::c_char, __buflen: size_t,
                       __result: *mut *mut Struct_netent,
                       __h_errnop: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn getnetbyaddr_r(__net: uint32_t, __type: ::libc::c_int,
                          __result_buf: *mut Struct_netent,
                          __buf: *mut ::libc::c_char, __buflen: size_t,
                          __result: *mut *mut Struct_netent,
                          __h_errnop: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn getnetbyname_r(__name: *::libc::c_char,
                          __result_buf: *mut Struct_netent,
                          __buf: *mut ::libc::c_char, __buflen: size_t,
                          __result: *mut *mut Struct_netent,
                          __h_errnop: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn setservent(__stay_open: ::libc::c_int);
    pub fn endservent();
    pub fn getservent() -> *mut Struct_servent;
    pub fn getservbyname(__name: *::libc::c_char, __proto: *::libc::c_char) ->
     *mut Struct_servent;
    pub fn getservbyport(__port: ::libc::c_int, __proto: *::libc::c_char) ->
     *mut Struct_servent;
    pub fn getservent_r(__result_buf: *mut Struct_servent,
                        __buf: *mut ::libc::c_char, __buflen: size_t,
                        __result: *mut *mut Struct_servent) -> ::libc::c_int;
    pub fn getservbyname_r(__name: *::libc::c_char, __proto: *::libc::c_char,
                           __result_buf: *mut Struct_servent,
                           __buf: *mut ::libc::c_char, __buflen: size_t,
                           __result: *mut *mut Struct_servent) ->
     ::libc::c_int;
    pub fn getservbyport_r(__port: ::libc::c_int, __proto: *::libc::c_char,
                           __result_buf: *mut Struct_servent,
                           __buf: *mut ::libc::c_char, __buflen: size_t,
                           __result: *mut *mut Struct_servent) ->
     ::libc::c_int;
    pub fn setprotoent(__stay_open: ::libc::c_int);
    pub fn endprotoent();
    pub fn getprotoent() -> *mut Struct_protoent;
    pub fn getprotobyname(__name: *::libc::c_char) -> *mut Struct_protoent;
    pub fn getprotobynumber(__proto: ::libc::c_int) -> *mut Struct_protoent;
    pub fn getprotoent_r(__result_buf: *mut Struct_protoent,
                         __buf: *mut ::libc::c_char, __buflen: size_t,
                         __result: *mut *mut Struct_protoent) ->
     ::libc::c_int;
    pub fn getprotobyname_r(__name: *::libc::c_char,
                            __result_buf: *mut Struct_protoent,
                            __buf: *mut ::libc::c_char, __buflen: size_t,
                            __result: *mut *mut Struct_protoent) ->
     ::libc::c_int;
    pub fn getprotobynumber_r(__proto: ::libc::c_int,
                              __result_buf: *mut Struct_protoent,
                              __buf: *mut ::libc::c_char, __buflen: size_t,
                              __result: *mut *mut Struct_protoent) ->
     ::libc::c_int;
    pub fn setnetgrent(__netgroup: *::libc::c_char) -> ::libc::c_int;
    pub fn endnetgrent();
    pub fn getnetgrent(__hostp: *mut *mut ::libc::c_char,
                       __userp: *mut *mut ::libc::c_char,
                       __domainp: *mut *mut ::libc::c_char) -> ::libc::c_int;
    pub fn innetgr(__netgroup: *::libc::c_char, __host: *::libc::c_char,
                   __user: *::libc::c_char, __domain: *::libc::c_char) ->
     ::libc::c_int;
    pub fn getnetgrent_r(__hostp: *mut *mut ::libc::c_char,
                         __userp: *mut *mut ::libc::c_char,
                         __domainp: *mut *mut ::libc::c_char,
                         __buffer: *mut ::libc::c_char, __buflen: size_t) ->
     ::libc::c_int;
    pub fn rcmd(__ahost: *mut *mut ::libc::c_char, __rport: ::libc::c_ushort,
                __locuser: *::libc::c_char, __remuser: *::libc::c_char,
                __cmd: *::libc::c_char, __fd2p: *mut ::libc::c_int) ->
     ::libc::c_int;
    pub fn rcmd_af(__ahost: *mut *mut ::libc::c_char,
                   __rport: ::libc::c_ushort, __locuser: *::libc::c_char,
                   __remuser: *::libc::c_char, __cmd: *::libc::c_char,
                   __fd2p: *mut ::libc::c_int, __af: sa_family_t) ->
     ::libc::c_int;
    pub fn rexec(__ahost: *mut *mut ::libc::c_char, __rport: ::libc::c_int,
                 __name: *::libc::c_char, __pass: *::libc::c_char,
                 __cmd: *::libc::c_char, __fd2p: *mut ::libc::c_int) ->
     ::libc::c_int;
    pub fn rexec_af(__ahost: *mut *mut ::libc::c_char, __rport: ::libc::c_int,
                    __name: *::libc::c_char, __pass: *::libc::c_char,
                    __cmd: *::libc::c_char, __fd2p: *mut ::libc::c_int,
                    __af: sa_family_t) -> ::libc::c_int;
    pub fn ruserok(__rhost: *::libc::c_char, __suser: ::libc::c_int,
                   __remuser: *::libc::c_char, __locuser: *::libc::c_char) ->
     ::libc::c_int;
    pub fn ruserok_af(__rhost: *::libc::c_char, __suser: ::libc::c_int,
                      __remuser: *::libc::c_char, __locuser: *::libc::c_char,
                      __af: sa_family_t) -> ::libc::c_int;
    pub fn iruserok(__raddr: uint32_t, __suser: ::libc::c_int,
                    __remuser: *::libc::c_char, __locuser: *::libc::c_char) ->
     ::libc::c_int;
    pub fn iruserok_af(__raddr: *::libc::c_void, __suser: ::libc::c_int,
                       __remuser: *::libc::c_char, __locuser: *::libc::c_char,
                       __af: sa_family_t) -> ::libc::c_int;
    pub fn rresvport(__alport: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn rresvport_af(__alport: *mut ::libc::c_int, __af: sa_family_t) ->
     ::libc::c_int;
    pub fn getaddrinfo(__name: *::libc::c_char, __service: *::libc::c_char,
                       __req: *Struct_addrinfo,
                       __pai: *mut *mut Struct_addrinfo) -> ::libc::c_int;
    pub fn freeaddrinfo(__ai: *mut Struct_addrinfo);
    pub fn gai_strerror(__ecode: ::libc::c_int) -> *::libc::c_char;
    pub fn getnameinfo(__sa: *Struct_sockaddr, __salen: socklen_t,
                       __host: *mut ::libc::c_char, __hostlen: socklen_t,
                       __serv: *mut ::libc::c_char, __servlen: socklen_t,
                       __flags: ::libc::c_int) -> ::libc::c_int;
    pub fn memcpy(__dest: *mut ::libc::c_void, __src: *::libc::c_void,
                  __n: size_t) -> *mut ::libc::c_void;
    pub fn memmove(__dest: *mut ::libc::c_void, __src: *::libc::c_void,
                   __n: size_t) -> *mut ::libc::c_void;
    pub fn memccpy(__dest: *mut ::libc::c_void, __src: *::libc::c_void,
                   __c: ::libc::c_int, __n: size_t) -> *mut ::libc::c_void;
    pub fn memset(__s: *mut ::libc::c_void, __c: ::libc::c_int, __n: size_t)
     -> *mut ::libc::c_void;
    pub fn memcmp(__s1: *::libc::c_void, __s2: *::libc::c_void, __n: size_t)
     -> ::libc::c_int;
    pub fn memchr(__s: *::libc::c_void, __c: ::libc::c_int, __n: size_t) ->
     *mut ::libc::c_void;
    pub fn strcpy(__dest: *mut ::libc::c_char, __src: *::libc::c_char) ->
     *mut ::libc::c_char;
    pub fn strncpy(__dest: *mut ::libc::c_char, __src: *::libc::c_char,
                   __n: size_t) -> *mut ::libc::c_char;
    pub fn strcat(__dest: *mut ::libc::c_char, __src: *::libc::c_char) ->
     *mut ::libc::c_char;
    pub fn strncat(__dest: *mut ::libc::c_char, __src: *::libc::c_char,
                   __n: size_t) -> *mut ::libc::c_char;
    pub fn strcmp(__s1: *::libc::c_char, __s2: *::libc::c_char) ->
     ::libc::c_int;
    pub fn strncmp(__s1: *::libc::c_char, __s2: *::libc::c_char, __n: size_t)
     -> ::libc::c_int;
    pub fn strcoll(__s1: *::libc::c_char, __s2: *::libc::c_char) ->
     ::libc::c_int;
    pub fn strxfrm(__dest: *mut ::libc::c_char, __src: *::libc::c_char,
                   __n: size_t) -> size_t;
    pub fn strcoll_l(__s1: *::libc::c_char, __s2: *::libc::c_char,
                     __l: __locale_t) -> ::libc::c_int;
    pub fn strxfrm_l(__dest: *mut ::libc::c_char, __src: *::libc::c_char,
                     __n: size_t, __l: __locale_t) -> size_t;
    pub fn strdup(__s: *::libc::c_char) -> *mut ::libc::c_char;
    pub fn strndup(__string: *::libc::c_char, __n: size_t) ->
     *mut ::libc::c_char;
    pub fn strchr(__s: *::libc::c_char, __c: ::libc::c_int) ->
     *mut ::libc::c_char;
    pub fn strrchr(__s: *::libc::c_char, __c: ::libc::c_int) ->
     *mut ::libc::c_char;
    pub fn strcspn(__s: *::libc::c_char, __reject: *::libc::c_char) -> size_t;
    pub fn strspn(__s: *::libc::c_char, __accept: *::libc::c_char) -> size_t;
    pub fn strpbrk(__s: *::libc::c_char, __accept: *::libc::c_char) ->
     *mut ::libc::c_char;
    pub fn strstr(__haystack: *::libc::c_char, __needle: *::libc::c_char) ->
     *mut ::libc::c_char;
    pub fn strtok(__s: *mut ::libc::c_char, __delim: *::libc::c_char) ->
     *mut ::libc::c_char;
    pub fn __strtok_r(__s: *mut ::libc::c_char, __delim: *::libc::c_char,
                      __save_ptr: *mut *mut ::libc::c_char) ->
     *mut ::libc::c_char;
    pub fn strtok_r(__s: *mut ::libc::c_char, __delim: *::libc::c_char,
                    __save_ptr: *mut *mut ::libc::c_char) ->
     *mut ::libc::c_char;
    pub fn strlen(__s: *::libc::c_char) -> size_t;
    pub fn strnlen(__string: *::libc::c_char, __maxlen: size_t) -> size_t;
    pub fn strerror(__errnum: ::libc::c_int) -> *mut ::libc::c_char;
    pub fn strerror_r(__errnum: ::libc::c_int, __buf: *mut ::libc::c_char,
                      __buflen: size_t) -> ::libc::c_int;
    pub fn strerror_l(__errnum: ::libc::c_int, __l: __locale_t) ->
     *mut ::libc::c_char;
    pub fn __bzero(__s: *mut ::libc::c_void, __n: size_t);
    pub fn bcopy(__src: *::libc::c_void, __dest: *mut ::libc::c_void,
                 __n: size_t);
    pub fn bzero(__s: *mut ::libc::c_void, __n: size_t);
    pub fn bcmp(__s1: *::libc::c_void, __s2: *::libc::c_void, __n: size_t) ->
     ::libc::c_int;
    pub fn index(__s: *::libc::c_char, __c: ::libc::c_int) ->
     *mut ::libc::c_char;
    pub fn rindex(__s: *::libc::c_char, __c: ::libc::c_int) ->
     *mut ::libc::c_char;
    pub fn ffs(__i: ::libc::c_int) -> ::libc::c_int;
    pub fn strcasecmp(__s1: *::libc::c_char, __s2: *::libc::c_char) ->
     ::libc::c_int;
    pub fn strncasecmp(__s1: *::libc::c_char, __s2: *::libc::c_char,
                       __n: size_t) -> ::libc::c_int;
    pub fn strsep(__stringp: *mut *mut ::libc::c_char,
                  __delim: *::libc::c_char) -> *mut ::libc::c_char;
    pub fn strsignal(__sig: ::libc::c_int) -> *mut ::libc::c_char;
    pub fn __stpcpy(__dest: *mut ::libc::c_char, __src: *::libc::c_char) ->
     *mut ::libc::c_char;
    pub fn stpcpy(__dest: *mut ::libc::c_char, __src: *::libc::c_char) ->
     *mut ::libc::c_char;
    pub fn __stpncpy(__dest: *mut ::libc::c_char, __src: *::libc::c_char,
                     __n: size_t) -> *mut ::libc::c_char;
    pub fn stpncpy(__dest: *mut ::libc::c_char, __src: *::libc::c_char,
                   __n: size_t) -> *mut ::libc::c_char;
    pub fn libhashkit_has_algorithm(arg1: hashkit_hash_algorithm_t) ->
     ::libc::c_int;
    pub fn libhashkit_one_at_a_time(key: *::libc::c_char, key_length: size_t)
     -> uint32_t;
    pub fn libhashkit_fnv1_64(key: *::libc::c_char, key_length: size_t) ->
     uint32_t;
    pub fn libhashkit_fnv1a_64(key: *::libc::c_char, key_length: size_t) ->
     uint32_t;
    pub fn libhashkit_fnv1_32(key: *::libc::c_char, key_length: size_t) ->
     uint32_t;
    pub fn libhashkit_fnv1a_32(key: *::libc::c_char, key_length: size_t) ->
     uint32_t;
    pub fn libhashkit_crc32(key: *::libc::c_char, key_length: size_t) ->
     uint32_t;
    pub fn libhashkit_hsieh(key: *::libc::c_char, key_length: size_t) ->
     uint32_t;
    pub fn libhashkit_murmur(key: *::libc::c_char, key_length: size_t) ->
     uint32_t;
    pub fn libhashkit_jenkins(key: *::libc::c_char, key_length: size_t) ->
     uint32_t;
    pub fn libhashkit_md5(key: *::libc::c_char, key_length: size_t) ->
     uint32_t;
    pub fn hashkit_one_at_a_time(key: *::libc::c_char, key_length: size_t,
                                 context: *mut ::libc::c_void) -> uint32_t;
    pub fn hashkit_fnv1_64(key: *::libc::c_char, key_length: size_t,
                           context: *mut ::libc::c_void) -> uint32_t;
    pub fn hashkit_fnv1a_64(key: *::libc::c_char, key_length: size_t,
                            context: *mut ::libc::c_void) -> uint32_t;
    pub fn hashkit_fnv1_32(key: *::libc::c_char, key_length: size_t,
                           context: *mut ::libc::c_void) -> uint32_t;
    pub fn hashkit_fnv1a_32(key: *::libc::c_char, key_length: size_t,
                            context: *mut ::libc::c_void) -> uint32_t;
    pub fn hashkit_crc32(key: *::libc::c_char, key_length: size_t,
                         context: *mut ::libc::c_void) -> uint32_t;
    pub fn hashkit_hsieh(key: *::libc::c_char, key_length: size_t,
                         context: *mut ::libc::c_void) -> uint32_t;
    pub fn hashkit_murmur(key: *::libc::c_char, key_length: size_t,
                          context: *mut ::libc::c_void) -> uint32_t;
    pub fn hashkit_jenkins(key: *::libc::c_char, key_length: size_t,
                           context: *mut ::libc::c_void) -> uint32_t;
    pub fn hashkit_md5(key: *::libc::c_char, key_length: size_t,
                       context: *mut ::libc::c_void) -> uint32_t;
    pub fn libhashkit_md5_signature(key: *::libc::c_uchar, length: size_t,
                                    result: *mut ::libc::c_uchar);
    pub fn hashkit_digest(_self: *hashkit_st, key: *::libc::c_char,
                          key_length: size_t) -> uint32_t;
    pub fn libhashkit_digest(key: *::libc::c_char, key_length: size_t,
                             hash_algorithm: hashkit_hash_algorithm_t) ->
     uint32_t;
    pub fn hashkit_set_function(hash: *mut hashkit_st,
                                hash_algorithm: hashkit_hash_algorithm_t) ->
     hashkit_return_t;
    pub fn hashkit_set_custom_function(hash: *mut hashkit_st,
                                       function: hashkit_hash_fn,
                                       context: *mut ::libc::c_void) ->
     hashkit_return_t;
    pub fn hashkit_get_function(hash: *hashkit_st) ->
     hashkit_hash_algorithm_t;
    pub fn hashkit_set_distribution_function(hash: *mut hashkit_st,
                                             hash_algorithm:
                                                 hashkit_hash_algorithm_t) ->
     hashkit_return_t;
    pub fn hashkit_set_custom_distribution_function(_self: *mut hashkit_st,
                                                    function: hashkit_hash_fn,
                                                    context:
                                                        *mut ::libc::c_void)
     -> hashkit_return_t;
    pub fn hashkit_get_distribution_function(_self: *hashkit_st) ->
     hashkit_hash_algorithm_t;
    pub fn libhashkit_string_hash(_type: hashkit_hash_algorithm_t) ->
     *::libc::c_char;
    pub fn hashkit_strerror(ptr: *mut hashkit_st, rc: hashkit_return_t) ->
     *::libc::c_char;
    pub fn hashkit_string_free(ptr: *mut hashkit_string_st);
    pub fn hashkit_string_length(_self: *hashkit_string_st) -> size_t;
    pub fn hashkit_string_c_str(_self: *hashkit_string_st) -> *::libc::c_char;
    pub fn hashkit_create(hash: *mut hashkit_st) -> *mut hashkit_st;
    pub fn hashkit_clone(destination: *mut hashkit_st, ptr: *hashkit_st) ->
     *mut hashkit_st;
    pub fn hashkit_compare(first: *hashkit_st, second: *hashkit_st) ->
     ::libc::c_int;
    pub fn hashkit_free(hash: *mut hashkit_st);
    pub fn hashkit_encrypt(arg1: *mut hashkit_st, source: *::libc::c_char,
                           source_length: size_t) -> *mut hashkit_string_st;
    pub fn hashkit_decrypt(arg1: *mut hashkit_st, source: *::libc::c_char,
                           source_length: size_t) -> *mut hashkit_string_st;
    pub fn hashkit_key(arg1: *mut hashkit_st, key: *::libc::c_char,
                       key_length: size_t) -> ::libc::c_int;
    pub fn memcached_last_error_message(arg1: *mut memcached_st) ->
     *::libc::c_char;
    pub fn memcached_error_print(arg1: *memcached_st);
    pub fn memcached_last_error(arg1: *mut memcached_st) ->
     memcached_return_t;
    pub fn memcached_last_error_errno(arg1: *mut memcached_st) ->
     ::libc::c_int;
    pub fn memcached_server_error(ptr: memcached_server_instance_st) ->
     *::libc::c_char;
    pub fn memcached_server_error_return(ptr: memcached_server_instance_st) ->
     memcached_return_t;
    pub fn memcached_stat_free(arg1: *memcached_st,
                               arg2: *mut memcached_stat_st);
    pub fn memcached_stat(ptr: *mut memcached_st, args: *mut ::libc::c_char,
                          error: *mut memcached_return_t) ->
     *mut memcached_stat_st;
    pub fn memcached_stat_servername(memc_stat: *mut memcached_stat_st,
                                     args: *mut ::libc::c_char,
                                     hostname: *::libc::c_char,
                                     port: in_port_t) -> memcached_return_t;
    pub fn memcached_stat_get_value(ptr: *memcached_st,
                                    memc_stat: *mut memcached_stat_st,
                                    key: *::libc::c_char,
                                    error: *mut memcached_return_t) ->
     *mut ::libc::c_char;
    pub fn memcached_stat_get_keys(ptr: *mut memcached_st,
                                   memc_stat: *mut memcached_stat_st,
                                   error: *mut memcached_return_t) ->
     *mut *mut ::libc::c_char;
    pub fn memcached_stat_execute(memc: *mut memcached_st,
                                  args: *::libc::c_char,
                                  func: memcached_stat_fn,
                                  context: *mut ::libc::c_void) ->
     memcached_return_t;
    pub fn memcached_set_memory_allocators(ptr: *mut memcached_st,
                                           mem_malloc: memcached_malloc_fn,
                                           mem_free: memcached_free_fn,
                                           mem_realloc: memcached_realloc_fn,
                                           mem_calloc: memcached_calloc_fn,
                                           context: *mut ::libc::c_void) ->
     memcached_return_t;
    pub fn memcached_get_memory_allocators(ptr: *memcached_st,
                                           mem_malloc:
                                               *mut memcached_malloc_fn,
                                           mem_free: *mut memcached_free_fn,
                                           mem_realloc:
                                               *mut memcached_realloc_fn,
                                           mem_calloc:
                                               *mut memcached_calloc_fn);
    pub fn memcached_get_memory_allocators_context(ptr: *memcached_st) ->
     *mut ::libc::c_void;
    pub fn memcached_analyze(memc: *mut memcached_st,
                             memc_stat: *mut memcached_stat_st,
                             error: *mut memcached_return_t) ->
     *mut memcached_analysis_st;
    pub fn memcached_analyze_free(arg1: *mut memcached_analysis_st);
    pub fn memcached_increment(ptr: *mut memcached_st, key: *::libc::c_char,
                               key_length: size_t, offset: uint32_t,
                               value: *mut uint64_t) -> memcached_return_t;
    pub fn memcached_decrement(ptr: *mut memcached_st, key: *::libc::c_char,
                               key_length: size_t, offset: uint32_t,
                               value: *mut uint64_t) -> memcached_return_t;
    pub fn memcached_increment_by_key(ptr: *mut memcached_st,
                                      group_key: *::libc::c_char,
                                      group_key_length: size_t,
                                      key: *::libc::c_char,
                                      key_length: size_t, offset: uint64_t,
                                      value: *mut uint64_t) ->
     memcached_return_t;
    pub fn memcached_decrement_by_key(ptr: *mut memcached_st,
                                      group_key: *::libc::c_char,
                                      group_key_length: size_t,
                                      key: *::libc::c_char,
                                      key_length: size_t, offset: uint64_t,
                                      value: *mut uint64_t) ->
     memcached_return_t;
    pub fn memcached_increment_with_initial(ptr: *mut memcached_st,
                                            key: *::libc::c_char,
                                            key_length: size_t,
                                            offset: uint64_t,
                                            initial: uint64_t,
                                            expiration: time_t,
                                            value: *mut uint64_t) ->
     memcached_return_t;
    pub fn memcached_decrement_with_initial(ptr: *mut memcached_st,
                                            key: *::libc::c_char,
                                            key_length: size_t,
                                            offset: uint64_t,
                                            initial: uint64_t,
                                            expiration: time_t,
                                            value: *mut uint64_t) ->
     memcached_return_t;
    pub fn memcached_increment_with_initial_by_key(ptr: *mut memcached_st,
                                                   group_key: *::libc::c_char,
                                                   group_key_length: size_t,
                                                   key: *::libc::c_char,
                                                   key_length: size_t,
                                                   offset: uint64_t,
                                                   initial: uint64_t,
                                                   expiration: time_t,
                                                   value: *mut uint64_t) ->
     memcached_return_t;
    pub fn memcached_decrement_with_initial_by_key(ptr: *mut memcached_st,
                                                   group_key: *::libc::c_char,
                                                   group_key_length: size_t,
                                                   key: *::libc::c_char,
                                                   key_length: size_t,
                                                   offset: uint64_t,
                                                   initial: uint64_t,
                                                   expiration: time_t,
                                                   value: *mut uint64_t) ->
     memcached_return_t;
    pub fn memcached_behavior_set(ptr: *mut memcached_st,
                                  flag: memcached_behavior_t, data: uint64_t)
     -> memcached_return_t;
    pub fn memcached_behavior_get(ptr: *mut memcached_st,
                                  flag: memcached_behavior_t) -> uint64_t;
    pub fn memcached_behavior_set_distribution(ptr: *mut memcached_st,
                                               _type:
                                                   memcached_server_distribution_t)
     -> memcached_return_t;
    pub fn memcached_behavior_get_distribution(ptr: *mut memcached_st) ->
     memcached_server_distribution_t;
    pub fn memcached_behavior_set_key_hash(ptr: *mut memcached_st,
                                           _type: memcached_hash_t) ->
     memcached_return_t;
    pub fn memcached_behavior_get_key_hash(ptr: *mut memcached_st) ->
     memcached_hash_t;
    pub fn memcached_behavior_set_distribution_hash(ptr: *mut memcached_st,
                                                    _type: memcached_hash_t)
     -> memcached_return_t;
    pub fn memcached_behavior_get_distribution_hash(ptr: *mut memcached_st) ->
     memcached_hash_t;
    pub fn libmemcached_string_behavior(flag: memcached_behavior_t) ->
     *::libc::c_char;
    pub fn libmemcached_string_distribution(flag:
                                                memcached_server_distribution_t)
     -> *::libc::c_char;
    pub fn memcached_bucket_set(_self: *mut memcached_st, host_map: *uint32_t,
                                forward_map: *uint32_t, buckets: uint32_t,
                                replicas: uint32_t) -> memcached_return_t;
    pub fn memcached_callback_set(ptr: *mut memcached_st,
                                  flag: memcached_callback_t,
                                  data: *::libc::c_void) ->
     memcached_return_t;
    pub fn memcached_callback_get(ptr: *mut memcached_st,
                                  flag: memcached_callback_t,
                                  error: *mut memcached_return_t) ->
     *mut ::libc::c_void;
    pub fn memcached_delete(ptr: *mut memcached_st, key: *::libc::c_char,
                            key_length: size_t, expiration: time_t) ->
     memcached_return_t;
    pub fn memcached_delete_by_key(ptr: *mut memcached_st,
                                   group_key: *::libc::c_char,
                                   group_key_length: size_t,
                                   key: *::libc::c_char, key_length: size_t,
                                   expiration: time_t) -> memcached_return_t;
    pub fn memcached_dump(ptr: *mut memcached_st,
                          function: *mut memcached_dump_fn,
                          context: *mut ::libc::c_void,
                          number_of_callbacks: uint32_t) ->
     memcached_return_t;
    pub fn memcached_set_encoding_key(arg1: *mut memcached_st,
                                      str: *::libc::c_char, length: size_t) ->
     memcached_return_t;
    pub fn memcached_exist(memc: *mut memcached_st, key: *::libc::c_char,
                           key_length: size_t) -> memcached_return_t;
    pub fn memcached_exist_by_key(memc: *mut memcached_st,
                                  group_key: *::libc::c_char,
                                  group_key_length: size_t,
                                  key: *::libc::c_char, key_length: size_t) ->
     memcached_return_t;
    pub fn memcached_fetch_execute(ptr: *mut memcached_st,
                                   callback: *mut memcached_execute_fn,
                                   context: *mut ::libc::c_void,
                                   number_of_callbacks: uint32_t) ->
     memcached_return_t;
    pub fn memcached_flush(ptr: *mut memcached_st, expiration: time_t) ->
     memcached_return_t;
    pub fn memcached_flush_buffers(mem: *mut memcached_st) ->
     memcached_return_t;
    pub fn memcached_get(ptr: *mut memcached_st, key: *::libc::c_char,
                         key_length: size_t, value_length: *mut size_t,
                         flags: *mut uint32_t, error: *mut memcached_return_t)
     -> *mut ::libc::c_char;
    pub fn memcached_mget(ptr: *mut memcached_st, keys: **::libc::c_char,
                          key_length: *size_t, number_of_keys: size_t) ->
     memcached_return_t;
    pub fn memcached_get_by_key(ptr: *mut memcached_st,
                                group_key: *::libc::c_char,
                                group_key_length: size_t,
                                key: *::libc::c_char, key_length: size_t,
                                value_length: *mut size_t,
                                flags: *mut uint32_t,
                                error: *mut memcached_return_t) ->
     *mut ::libc::c_char;
    pub fn memcached_mget_by_key(ptr: *mut memcached_st,
                                 group_key: *::libc::c_char,
                                 group_key_length: size_t,
                                 keys: **::libc::c_char, key_length: *size_t,
                                 number_of_keys: size_t) ->
     memcached_return_t;
    pub fn memcached_fetch(ptr: *mut memcached_st, key: *mut ::libc::c_char,
                           key_length: *mut size_t, value_length: *mut size_t,
                           flags: *mut uint32_t,
                           error: *mut memcached_return_t) ->
     *mut ::libc::c_char;
    pub fn memcached_fetch_result(ptr: *mut memcached_st,
                                  result: *mut memcached_result_st,
                                  error: *mut memcached_return_t) ->
     *mut memcached_result_st;
    pub fn memcached_mget_execute(ptr: *mut memcached_st,
                                  keys: **::libc::c_char, key_length: *size_t,
                                  number_of_keys: size_t,
                                  callback: *mut memcached_execute_fn,
                                  context: *mut ::libc::c_void,
                                  number_of_callbacks: uint32_t) ->
     memcached_return_t;
    pub fn memcached_mget_execute_by_key(ptr: *mut memcached_st,
                                         group_key: *::libc::c_char,
                                         group_key_length: size_t,
                                         keys: **::libc::c_char,
                                         key_length: *size_t,
                                         number_of_keys: size_t,
                                         callback: *mut memcached_execute_fn,
                                         context: *mut ::libc::c_void,
                                         number_of_callbacks: uint32_t) ->
     memcached_return_t;
    pub fn memcached_generate_hash_value(key: *::libc::c_char,
                                         key_length: size_t,
                                         hash_algorithm: memcached_hash_t) ->
     uint32_t;
    pub fn memcached_get_hashkit(ptr: *memcached_st) -> *hashkit_st;
    pub fn memcached_set_hashkit(ptr: *mut memcached_st,
                                 hashk: *mut hashkit_st) ->
     memcached_return_t;
    pub fn memcached_generate_hash(ptr: *memcached_st, key: *::libc::c_char,
                                   key_length: size_t) -> uint32_t;
    pub fn memcached_autoeject(ptr: *mut memcached_st);
    pub fn libmemcached_string_hash(_type: memcached_hash_t) ->
     *::libc::c_char;
    pub fn libmemcached_check_configuration(option_string: *::libc::c_char,
                                            length: size_t,
                                            error_buffer: *mut ::libc::c_char,
                                            error_buffer_size: size_t) ->
     memcached_return_t;
    pub fn memcached_servers_parse(server_strings: *::libc::c_char) ->
     memcached_server_list_st;
    pub fn memcached_quit(ptr: *mut memcached_st);
    pub fn memcached_result_free(result: *mut memcached_result_st);
    pub fn memcached_result_reset(ptr: *mut memcached_result_st);
    pub fn memcached_result_create(ptr: *memcached_st,
                                   result: *mut memcached_result_st) ->
     *mut memcached_result_st;
    pub fn memcached_result_key_value(_self: *memcached_result_st) ->
     *::libc::c_char;
    pub fn memcached_result_key_length(_self: *memcached_result_st) -> size_t;
    pub fn memcached_result_value(_self: *memcached_result_st) ->
     *::libc::c_char;
    pub fn memcached_result_length(_self: *memcached_result_st) -> size_t;
    pub fn memcached_result_flags(_self: *memcached_result_st) -> uint32_t;
    pub fn memcached_result_cas(_self: *memcached_result_st) -> uint64_t;
    pub fn memcached_result_set_value(ptr: *mut memcached_result_st,
                                      value: *::libc::c_char, length: size_t)
     -> memcached_return_t;
    pub fn memcached_result_set_flags(_self: *mut memcached_result_st,
                                      flags: uint32_t);
    pub fn memcached_result_set_expiration(_self: *mut memcached_result_st,
                                           expiration: time_t);
    pub fn memcached_server_cursor(ptr: *memcached_st,
                                   callback: *memcached_server_fn,
                                   context: *mut ::libc::c_void,
                                   number_of_callbacks: uint32_t) ->
     memcached_return_t;
    pub fn memcached_server_by_key(ptr: *mut memcached_st,
                                   key: *::libc::c_char, key_length: size_t,
                                   error: *mut memcached_return_t) ->
     memcached_server_instance_st;
    pub fn memcached_server_error_reset(ptr: *mut memcached_server_st);
    pub fn memcached_server_free(ptr: *mut memcached_server_st);
    pub fn memcached_server_get_last_disconnect(ptr: *memcached_st) ->
     memcached_server_instance_st;
    pub fn memcached_server_add_udp(ptr: *mut memcached_st,
                                    hostname: *::libc::c_char,
                                    port: in_port_t) -> memcached_return_t;
    pub fn memcached_server_add_unix_socket(ptr: *mut memcached_st,
                                            filename: *::libc::c_char) ->
     memcached_return_t;
    pub fn memcached_server_add(ptr: *mut memcached_st,
                                hostname: *::libc::c_char, port: in_port_t) ->
     memcached_return_t;
    pub fn memcached_server_add_udp_with_weight(ptr: *mut memcached_st,
                                                hostname: *::libc::c_char,
                                                port: in_port_t,
                                                weight: uint32_t) ->
     memcached_return_t;
    pub fn memcached_server_add_unix_socket_with_weight(ptr:
                                                            *mut memcached_st,
                                                        filename:
                                                            *::libc::c_char,
                                                        weight: uint32_t) ->
     memcached_return_t;
    pub fn memcached_server_add_with_weight(ptr: *mut memcached_st,
                                            hostname: *::libc::c_char,
                                            port: in_port_t, weight: uint32_t)
     -> memcached_return_t;
    pub fn memcached_server_response_count(_self:
                                               memcached_server_instance_st)
     -> uint32_t;
    pub fn memcached_server_name(_self: memcached_server_instance_st) ->
     *::libc::c_char;
    pub fn memcached_server_port(_self: memcached_server_instance_st) ->
     in_port_t;
    pub fn memcached_server_type(ptr: memcached_server_instance_st) ->
     *::libc::c_char;
    pub fn memcached_server_list_free(ptr: memcached_server_list_st);
    pub fn memcached_server_push(ptr: *mut memcached_st,
                                 list: memcached_server_list_st) ->
     memcached_return_t;
    pub fn memcached_server_list_append(ptr: memcached_server_list_st,
                                        hostname: *::libc::c_char,
                                        port: in_port_t,
                                        error: *mut memcached_return_t) ->
     memcached_server_list_st;
    pub fn memcached_server_list_append_with_weight(ptr:
                                                        memcached_server_list_st,
                                                    hostname: *::libc::c_char,
                                                    port: in_port_t,
                                                    weight: uint32_t,
                                                    error:
                                                        *mut memcached_return_t)
     -> memcached_server_list_st;
    pub fn memcached_server_list_count(ptr: memcached_server_list_st) ->
     uint32_t;
    pub fn memcached_set(ptr: *mut memcached_st, key: *::libc::c_char,
                         key_length: size_t, value: *::libc::c_char,
                         value_length: size_t, expiration: time_t,
                         flags: uint32_t) -> memcached_return_t;
    pub fn memcached_add(ptr: *mut memcached_st, key: *::libc::c_char,
                         key_length: size_t, value: *::libc::c_char,
                         value_length: size_t, expiration: time_t,
                         flags: uint32_t) -> memcached_return_t;
    pub fn memcached_replace(ptr: *mut memcached_st, key: *::libc::c_char,
                             key_length: size_t, value: *::libc::c_char,
                             value_length: size_t, expiration: time_t,
                             flags: uint32_t) -> memcached_return_t;
    pub fn memcached_append(ptr: *mut memcached_st, key: *::libc::c_char,
                            key_length: size_t, value: *::libc::c_char,
                            value_length: size_t, expiration: time_t,
                            flags: uint32_t) -> memcached_return_t;
    pub fn memcached_prepend(ptr: *mut memcached_st, key: *::libc::c_char,
                             key_length: size_t, value: *::libc::c_char,
                             value_length: size_t, expiration: time_t,
                             flags: uint32_t) -> memcached_return_t;
    pub fn memcached_cas(ptr: *mut memcached_st, key: *::libc::c_char,
                         key_length: size_t, value: *::libc::c_char,
                         value_length: size_t, expiration: time_t,
                         flags: uint32_t, cas: uint64_t) ->
     memcached_return_t;
    pub fn memcached_set_by_key(ptr: *mut memcached_st,
                                group_key: *::libc::c_char,
                                group_key_length: size_t,
                                key: *::libc::c_char, key_length: size_t,
                                value: *::libc::c_char, value_length: size_t,
                                expiration: time_t, flags: uint32_t) ->
     memcached_return_t;
    pub fn memcached_add_by_key(ptr: *mut memcached_st,
                                group_key: *::libc::c_char,
                                group_key_length: size_t,
                                key: *::libc::c_char, key_length: size_t,
                                value: *::libc::c_char, value_length: size_t,
                                expiration: time_t, flags: uint32_t) ->
     memcached_return_t;
    pub fn memcached_replace_by_key(ptr: *mut memcached_st,
                                    group_key: *::libc::c_char,
                                    group_key_length: size_t,
                                    key: *::libc::c_char, key_length: size_t,
                                    value: *::libc::c_char,
                                    value_length: size_t, expiration: time_t,
                                    flags: uint32_t) -> memcached_return_t;
    pub fn memcached_prepend_by_key(ptr: *mut memcached_st,
                                    group_key: *::libc::c_char,
                                    group_key_length: size_t,
                                    key: *::libc::c_char, key_length: size_t,
                                    value: *::libc::c_char,
                                    value_length: size_t, expiration: time_t,
                                    flags: uint32_t) -> memcached_return_t;
    pub fn memcached_append_by_key(ptr: *mut memcached_st,
                                   group_key: *::libc::c_char,
                                   group_key_length: size_t,
                                   key: *::libc::c_char, key_length: size_t,
                                   value: *::libc::c_char,
                                   value_length: size_t, expiration: time_t,
                                   flags: uint32_t) -> memcached_return_t;
    pub fn memcached_cas_by_key(ptr: *mut memcached_st,
                                group_key: *::libc::c_char,
                                group_key_length: size_t,
                                key: *::libc::c_char, key_length: size_t,
                                value: *::libc::c_char, value_length: size_t,
                                expiration: time_t, flags: uint32_t,
                                cas: uint64_t) -> memcached_return_t;
    pub fn memcached_strerror(ptr: *mut memcached_st, rc: memcached_return_t)
     -> *::libc::c_char;
    pub fn memcached_touch(ptr: *mut memcached_st, key: *::libc::c_char,
                           key_length: size_t, expiration: time_t) ->
     memcached_return_t;
    pub fn memcached_touch_by_key(ptr: *mut memcached_st,
                                  group_key: *::libc::c_char,
                                  group_key_length: size_t,
                                  key: *::libc::c_char, key_length: size_t,
                                  expiration: time_t) -> memcached_return_t;
    pub fn memcached_verbosity(ptr: *mut memcached_st, verbosity: uint32_t) ->
     memcached_return_t;
    pub fn memcached_version(ptr: *mut memcached_st) -> memcached_return_t;
    pub fn memcached_lib_version() -> *::libc::c_char;
    pub fn memcached_set_sasl_callbacks(ptr: *mut memcached_st,
                                        callbacks: *::libc::c_void);
    pub fn memcached_set_sasl_auth_data(ptr: *mut memcached_st,
                                        username: *::libc::c_char,
                                        password: *::libc::c_char) ->
     memcached_return_t;
    pub fn memcached_destroy_sasl_auth_data(ptr: *mut memcached_st) ->
     memcached_return_t;
    pub fn memcached_get_sasl_callbacks(ptr: *mut memcached_st) ->
     *mut ::libc::c_void;
    pub fn memcached_servers_reset(ptr: *mut memcached_st);
    pub fn memcached_create(ptr: *mut memcached_st) -> *mut memcached_st;
    pub fn memcached(string: *::libc::c_char, string_length: size_t) ->
     *mut memcached_st;
    pub fn memcached_free(ptr: *mut memcached_st);
    pub fn memcached_reset(ptr: *mut memcached_st) -> memcached_return_t;
    pub fn memcached_reset_last_disconnected_server(ptr: *mut memcached_st);
    pub fn memcached_clone(clone: *mut memcached_st, ptr: *memcached_st) ->
     *mut memcached_st;
    pub fn memcached_get_user_data(ptr: *memcached_st) -> *mut ::libc::c_void;
    pub fn memcached_set_user_data(ptr: *mut memcached_st,
                                   data: *mut ::libc::c_void) ->
     *mut ::libc::c_void;
    pub fn memcached_push(destination: *mut memcached_st,
                          source: *memcached_st) -> memcached_return_t;
    pub fn memcached_server_instance_by_position(ptr: *memcached_st,
                                                 server_key: uint32_t) ->
     memcached_server_instance_st;
    pub fn memcached_server_count(arg1: *memcached_st) -> uint32_t;
    pub fn memcached_query_id(arg1: *memcached_st) -> uint64_t;
}
