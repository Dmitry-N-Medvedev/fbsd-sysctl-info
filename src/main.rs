#![allow(dead_code)]
#![allow(unused_imports)]

extern crate sysctl;

use sysctl::Sysctl;

fn get_ncpu() -> i32 {
    if let Ok(ctl) = sysctl::Ctl::new("hw.ncpu") {
        if let Ok(r) = ctl.value() {
            if let Some(ncpu) = r.as_int() {
                return *ncpu
            }
        }
    }

    0
}

#[cfg(target_os = "freebsd")]
fn main() {
    let ncpu = get_ncpu();

    println!("ncpu: {:?}", ncpu);

    for cpu_id in 0..ncpu {
        println!("CPU\t{}", cpu_id);
    }
}
