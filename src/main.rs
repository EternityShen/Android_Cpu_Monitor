use std::thread::sleep;
use cpu_monitor::{Load, Thermal};
use std::time::Duration;
fn main() {
    let mut load = Load::new();
    let mut thermal = Thermal::new();
    loop {
        println!("Cpu负载{}", load.update());
        println!("Cpu温度{}", thermal.update());
        sleep(Duration::from_millis(100));
        sleep(Duration::from_millis(100));
    }
}
