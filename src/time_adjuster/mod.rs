use std::io::Error;
use std::process::Command;
use time::Duration;

pub fn set_time_by_offset(offset: Duration) -> Result<(), Error> {
    if cfg!(target_os = "linux") {
        let new = time::now().to_timespec() + offset;
        let timeval = libc::timeval {
            tv_sec: new.sec,
            tv_usec: (new.nsec as f64 * 0.001) as i32,
        };
        unsafe { libc::settimeofday(&timeval, std::ptr::null()) };
    } else {
        panic!("Operative System or Architecture not supported.");
    }

    Ok(())
}
