#![no_std]
#![no_main]
use cortex_m_rt::entry;
use panic_probe as _;
#[entry]
fn main() -> !{
    embassy_stm32::init(Default::default());

    loop {
        
    }
}
