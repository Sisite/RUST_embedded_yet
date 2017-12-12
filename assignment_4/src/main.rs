#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use core::fmt::Write;
use cortex_m::asm;
use cortex_m_semihosting::hio;
use cortex_m::peripheral::DWT;

static WORDARR: [u32; 4] = [
    0x9fdd9158, 0x85715808, 0xac73323a, 0x0
];

static mut BYTEARR: [u8; 4] = [0; 4];
static mut SEED: u32 = 0x0e0657c1;
fn main() {
    unsafe {
        (*DWT.get()).enable_cycle_counter();
        (*DWT.get()).cyccnt.write(0);
    }
    //let mut bytearr: [u8; 132] = [0; 132];
    //let mut seed: u32 = 0x0e0657c1;
    let mut stdout = hio::hstdout().unwrap();
    decode(0);
    unsafe {
        for x in BYTEARR.iter() {
            write!(stdout, "{}", *x as char).unwrap();
        };
    }
    asm::bkpt();
}

fn decode(mut recdepth: usize) -> u32 {
    let m: u32;
    let r: u32;
    let y: u32;
    let x: u32;
    unsafe {
        x = !codgen();
    }
    if WORDARR[recdepth] == 0 {
        unsafe {
            BYTEARR[recdepth] = 0;
        }
        r = x;
    } else {
        recdepth += 1;
        y = decode(recdepth);
        m = (x.wrapping_sub(y)).wrapping_sub(WORDARR[recdepth - 1]);
        unsafe {
            BYTEARR[recdepth - 1] = ((m >> 13) & 0xFF) as u8;
        }
        let cod1;
        unsafe {
            cod1 = codgen();
        }
        r = x.wrapping_add(y)
            .wrapping_add(m)
            .wrapping_add(!cod1)
            .wrapping_add(1)
            .wrapping_add(5);
    }
    return r;
}

unsafe fn codgen() -> u32 {
    let n: u32 = SEED.count_zeros() as u32;
    let y: i32 = SEED as i32 >> 6;
    let x: u32 = SEED.rotate_left(30);

    SEED = x ^ y as u32 ^ n;
    let new_seed: u32 = SEED;
    return new_seed ^ 0x464b713e;
}

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler(){
    asm::bkpt();
}