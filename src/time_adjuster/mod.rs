use std::io::Error;
use std::process::Command;

pub fn set_time_by_offset(offset: i64) -> Result<(), Error> {
    if cfg!(target_arch = "mips") {

    } else if cfg!(target_os = "linux") {
        // date $(date +%m%d%H%M%Y.%S -d '1 hour ago')
        let offset_string = if offset >= 0 {
            format!("+ {} seconds", offset)
        } else {
            format!("- {} seconds", offset)
        };
        let output =
            Command::new("date").arg(format!("$(date +%m%d%H%M%Y.%S -d '{}')", offset_string));
    } else {
        panic!("Operative System or Architecture not supported.");
    }

    Ok(())
}
