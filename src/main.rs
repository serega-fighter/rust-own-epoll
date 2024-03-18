use std::io;

#[cfg(target_family = "unix")]
#[link(name = "c")]
extern "C" {
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

/*
A calling convention defines how function calls are performed and will, amongst other
things, specify:
- How arguments are passed into the function
- What registers the function is expected to store at the start and restore before returning
- How the function returns its result
- How the stack is set up (we’ll get back to this one later)
*/
#[cfg(target_family = "unix")]
fn syscall(message: String) -> io::Result<()> {
    let msg_ptr = message.as_ptr();
    let len = message.len();
    let res = unsafe { write(1, msg_ptr, len) };

    if res == -1 {
        return Err(io::Error::last_os_error());
    }
    return Ok(());
}

fn main() {
    syscall(String::from("Hello, world from syscall!")).unwrap();
}