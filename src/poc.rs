// extremely dumbed down version.
// used to have a much better one but had to purge it due to circumstances beyond my control (iykyk)
use std::time::{SystemTime, UNIX_EPOCH};

// the consts
const M1: u32 = 2147483563;
const A1: u32 = 40014;
const Q1: u32 = 53668;
const R1: u32 = 12211;

const M2: u32 = 2147483399;
const A2: u32 = 40692;
const Q2: u32 = 52774;
const R2: u32 = 3791;

struct Lcg {
    s1: u32,
    s2: u32,
}

impl Lcg {
    fn new(seed1: u32, seed2: u32) -> Self {
        Self { s1: seed1, s2: seed2 }
    }

    fn next(&mut self) -> f64 {
        self.s1 = A1.wrapping_mul(self.s1 % Q1).wrapping_sub((self.s1 / Q1) * R1) % M1;
        self.s2 = A2.wrapping_mul(self.s2 % Q2).wrapping_sub((self.s2 / Q2) * R2) % M2;
        let z = self.s1.wrapping_sub(self.s2);
        let result = if z < 1 { z.wrapping_add(M1 - 1) } else { z };
        result as f64 * 4.656613e-10
    }
}

fn gen(ip: &str, sec: u64, usec: u64, lcg_val: f64) -> String {
    let data = format!("{:.15}{}{}{:08.8}", ip, sec, usec, lcg_val * 10.0);
    let hash = md5::compute(data);
    format!("{:x}", hash)
}

fn brute(ip: &str, time: SystemTime, prevsess: &str) {
    let since_epoch = time.duration_since(UNIX_EPOCH).unwrap();
    let sec = since_epoch.as_secs();
    let usec = since_epoch.subsec_micros() as u64;

    for seed1 in 0..u32::MAX {
        for seed2 in 0..u32::MAX {
            let mut lcg = Lcg::new(seed1, seed2);
            let lcg_val = lcg.next();
            let sess_id = gen(ip, sec, usec, lcg_val);

            if sess_id == prevsess {
                println!("fished: Seeds: s1={}, s2={}", seed1, seed2);
                return;
            }
        }
    }
    println!("cant find shit");
}

fn main() {
    let ip = "1.1.1.1";
    let time = SystemTime::now();
    let prevsess = "5d41402abc4b2a76b9719d911017c592"; // sample hash.

    brute(ip, time, prevsess);
}
