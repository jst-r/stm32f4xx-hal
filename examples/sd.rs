#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};
use panic_semihosting as _;

use stm32f4xx_hal::{
    pac,
    prelude::*,
    sdio::{ClockFreq, Sdio},
};

#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(12.MHz())
        .require_pll48clk()
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    assert!(clocks.is_pll48clk_valid());

    let mut delay = core.SYST.delay(&clocks);

    let gpioc = device.GPIOC.split();
    let gpiod = device.GPIOD.split();

    let d0 = gpioc.pc8.into_alternate().internal_pull_up(true);
    let d1 = gpioc.pc9.into_alternate().internal_pull_up(true);
    let d2 = gpioc.pc10.into_alternate().internal_pull_up(true);
    let d3 = gpioc.pc11.into_alternate().internal_pull_up(true);
    let clk = gpioc.pc12.into_alternate().internal_pull_up(false);
    let cmd = gpiod.pd2.into_alternate().internal_pull_up(true);
    let mut sdio = Sdio::new(device.SDIO, (clk, cmd, d0, d1, d2, d3), &clocks);

    hprintln!("Waiting for card...").ok();

    // Wait for card to be ready
    loop {
        match sdio.init_card(ClockFreq::F24Mhz) {
            Ok(_) => break,
            Err(_err) => (),
        }

        delay.delay_ms(1000u32);
    }

    let nblocks = sdio.card().map(|c| c.block_count()).unwrap_or(0);
    hprintln!("Card detected: nbr of blocks: {:?}", nblocks).ok();

    // Read a block from the card and print the data
    let mut block = [0u8; 512];

    match sdio.read_block(0, &mut block) {
        Ok(()) => (),
        Err(err) => {
            hprintln!("Failed to read block: {:?}", err).ok();
        }
    }

    for b in block.iter() {
        hprint!("{:X} ", b).ok();
    }

    loop {
        continue;
    }
}
