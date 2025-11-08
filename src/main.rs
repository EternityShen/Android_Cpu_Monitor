use std::thread::sleep;
use cpu_monitor::{Load, Thermal};
use std::time::Duration;
fn main() {
    let mut load = Load::new();
    let mut thermal = Thermal::new();
    loop {
        println!("Cpu负载{}", load.get_load());
        println!("Cpu温度{}", thermal.get_thermal());
        sleep(Duration::from_millis(100));
        sleep(Duration::from_millis(100));
    }
}
