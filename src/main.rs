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

fn get_cpu_freq(cpu_id: i32) -> i32 {
    let freq_req = ["dev.cpu.", &cpu_id.to_string(), ".freq"].concat();
    if let Ok(ctl) = sysctl::Ctl::new(&freq_req){
       if let Ok(r) = ctl.value() {
           if let Some(freq) = r.as_int() {
               return *freq
           }
       }
    }

    0
}

fn get_cpu_temperature(cpu_id: i32) -> f32 {
    let temperature_req = ["dev.cpu.", &cpu_id.to_string(), ".temperature"].concat();

    if let Ok(ctl) = sysctl::Ctl::new(&temperature_req) {
        if let Ok(r) = ctl.value() {
            if let Some(temperature) = r.as_temperature() {
                return temperature.celsius()
            } 
        }
    }

    0.0
}

#[cfg(target_os = "freebsd")]
fn main() {
    let ncpu = get_ncpu();

    println!("ncpu: {:?}", ncpu);

    for cpu_id in 0..ncpu {
        let cpu_freq = get_cpu_freq(cpu_id);
        let cpu_temp = get_cpu_temperature(cpu_id);

        println!("CPU\t{}\t{:.2}Hrz\t{:.2}C", cpu_id, cpu_freq, cpu_temp);
    }
}
