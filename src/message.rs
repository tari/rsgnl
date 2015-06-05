extern crate libc;
extern crate rsnl;

use libc::{c_int};
use rsnl::message::{NetlinkMessage, nl_msg, nlmsghdr};
use rsnl::message::expose::{nl_msg_ptr, nlmsghdr_ptr};

#[repr(C)]
struct genlmsghdr;

#[link(name="nl-genl-3")]
extern "C" {
	// Exposed socket functions
    fn genlmsg_valid_hdr(hdr: *const nlmsghdr, hdrlen: c_int) -> i32;
    fn genlmsg_hdr(hdr: *const nlmsghdr) -> *const genlmsghdr;
}



// leaky!
pub struct GenlHeader {
    ptr: *const genlmsghdr,
}

pub fn valid_hdr(msg: &NetlinkMessage, hdrlen: i32) -> i32 {
    let hdrptr = match nlmsghdr_ptr(msg) {
        Some(hdr) => hdr,
        None => return 1
    };

    unsafe { genlmsg_valid_hdr(hdrptr, hdrlen) }
}

pub fn hdr(msg: &NetlinkMessage) -> Option<GenlHeader> {

    let hdrptr = match nlmsghdr_ptr(msg) {
        Some(hdr) => hdr,
        None => return None
    };

    Some(
        GenlHeader {
            ptr: unsafe { genlmsg_hdr(hdrptr) }
        }
    )
}