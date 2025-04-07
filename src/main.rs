#![allow(non_snake_case)]

extern crate libc;

#[macro_use]
extern crate num_derive;

mod shell;

mod keycode;

mod env;

mod parser;

use env::EnvManager;
use libc::{ioctl, open, setsid, O_RDWR, TIOCSCTTY};
use shell::Shell;

fn main() {
    // 1. setsid：成为新的 session leader
    unsafe { setsid() };

    // 2. 打开 /dev/tty（或你实现的等价路径）
    let tty_path = std::ffi::CString::new("/dev/ttyS0").expect("CString::new failed");
    let tty_fd = unsafe { open(tty_path.as_ptr(), O_RDWR) };

    // 3. ioctl(fd, TIOCSCTTY)：设置为控制终端
    unsafe { ioctl(tty_fd, TIOCSCTTY) };
    EnvManager::init();
    let mut shell = Shell::new();
    shell.exec();
    return;
}
