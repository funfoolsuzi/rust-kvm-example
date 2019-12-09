extern crate libc;
#[macro_use]
extern crate vmm_sys_util;


use std::fs;
use std::os::unix::io::AsRawFd;

fn main() {
    let kvmf = fs::File::open("/dev/kvm").unwrap();
    let kvmfd = kvmf.as_raw_fd();
    ioctl_io_nr!(KVM_GET_API_VERSION, kvm_prac::KVMIO, 0x00);
    unsafe {
        let res = libc::ioctl(kvmfd, KVM_GET_API_VERSION(), 0);
        if res <0 {
            let errno = *libc::__errno_location();
            println!("errno: {}", errno);
        }
        println!("kvm api version: {}", res);
    }
}
