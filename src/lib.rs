use std::fs::{File};
use std::io::{BufReader, Seek, Read};
use std::path::PathBuf;
use std::fs::read_to_string;

pub struct Load{
    stat_file: BufReader<File>,
    total: u64,
    idle: u64,
}

impl Load {
    pub fn new() -> Self {
        let stat_file = File::open("/proc/stat").unwrap();
        let stat_file = BufReader::new(stat_file);
        Self {
            stat_file,
            total: 0,
            idle: 0,
        }
    }

    pub fn update(&mut self) -> u32 {
        self.stat_file.seek(std::io::SeekFrom::Start(0)).unwrap();
        let mut buffer = [0u8; 1024];
        let bytes_read = self.stat_file.read(&mut buffer).unwrap();
        let stat_line = String::from_utf8_lossy(&buffer[..bytes_read]);
        let fields: Vec<&str> = stat_line.split_whitespace().collect();
        let prev_total = self.total;
        let prev_idle = self.idle;
        let new_total = fields[1..6].iter().map(|s| s.parse::<u64>().unwrap()).sum::<u64>();
        let new_idle = fields[4].parse::<u64>().unwrap();
        let delta_total = new_total.saturating_sub(prev_total);
        let delta_idle = new_idle.saturating_sub(prev_idle);
        let usage = if delta_total == 0 {
            0.0
        } else {
            ((delta_total - delta_idle) as f64) / (delta_total as f64) * 100.0
        };
        self.total = new_total;
        self.idle = new_idle;
        usage as u32
    }
}

pub struct Thermal {
    thermal_file: BufReader<File>,
}

impl Thermal {
    pub fn new() -> Self {
        let thermal_file = PathBuf::from("/sys/class/thermal");
        let alldir = thermal_file.read_dir().unwrap();
        for entry in alldir {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let path = path;
                if path.starts_with("thermal_zone") {
                    let type_path = path.join("type");
                    if let Ok(type_str) = read_to_string(&type_path) {
                        if type_str.contains("mtktscpu") 
                        || type_str.contains("soc_max")
                        || type_str.contains("cpu-1-") {
                            let temp_path = path.join("temp");
                            let temp_file = File::open(temp_path).unwrap();
                            let temp_file = BufReader::new(temp_file);
                            return Self {
                                thermal_file: temp_file,
                            };
                        }
                    }
                }
            }
        }
        panic!("没有找到对的温度文件");
    }

    pub fn update(&mut self) -> u32 {
        self.thermal_file.seek(std::io::SeekFrom::Start(0)).unwrap();
        let mut temp_line = String::new();
        self.thermal_file.read_to_string(&mut temp_line).unwrap();
        let temp = temp_line.trim().parse::<u32>().unwrap();
        temp / 1000
    }
}
