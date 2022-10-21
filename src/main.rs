use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

struct CpuInfo {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    io_wait: u64,
    irq: u64,
    soft_irq: u64,
}

fn main() {
    println!("{}", get_cpu_usage());
}

// 读取cpu相关信息
fn get_cpu_usage() -> u64 {
    let mut info1: CpuInfo = CpuInfo {
        user: 0,
        nice: 0,
        system: 0,
        idle: 0,
        io_wait: 0,
        irq: 0,
        soft_irq: 0,
    };
    let mut info2: CpuInfo = CpuInfo { ..info1 };

    get_cpu_info(&mut info1);
    thread::sleep(Duration::from_millis(50));
    get_cpu_info(&mut info2);

    let dalt_use = info2.user + info2.nice + info2.system + info2.irq + info2.soft_irq
        - info1.user
        - info1.nice
        - info1.system
        - info1.irq
        - info1.soft_irq;
    let dalt_total = dalt_use + info2.idle + info2.io_wait - info1.idle - info1.io_wait;

    if dalt_total == 0 {
        return dalt_total;
    }

    dalt_use * 100 / dalt_total
}

fn get_cpu_info(info: &mut CpuInfo) {
    let file = File::open("/proc/stat").unwrap();
    let mut buf_in = BufReader::new(file);
    let mut line = String::new();

    buf_in.read_line(&mut line).unwrap();

    let items: Vec<&str> = line.split_whitespace().collect();

    if items.len() < 8 {
        panic!("items len err, len{}", items.len())
    }

    info.user = items[1].parse().unwrap();
    info.nice = items[2].parse().unwrap();
    info.system = items[3].parse().unwrap();
    info.idle = items[4].parse().unwrap();
    info.io_wait = items[5].parse().unwrap();
    info.irq = items[6].parse().unwrap();
    info.soft_irq = items[7].parse().unwrap();
}
