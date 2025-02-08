#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

extern crate alloc;
use panic_halt as _;

use core::alloc::Layout;

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln, debug};


#[cfg(not(debug_assertions))] 
use stm32f1xx_hal::{pac, prelude::*, serial::Config, serial::Serial};

#[cfg(not(debug_assertions))] 
use core::fmt::Write;


// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 10240; // in bytes

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    #[cfg(not(debug_assertions))]
    {
        let dp = pac::Peripherals::take().expect("cannot take peripherals");
        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        let mut delay = dp.TIM1.delay_ms(&clocks);
        let mut afio = dp.AFIO.constrain();
        let mut gpioa = dp.GPIOA.split();
        // USART1 on Pins A9 and A10
        let pin_tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
        let pin_rx = gpioa.pa10;
        // Create an interface struct for USART1 with 9600 Baud
        let mut serial = Serial::new(
        dp.USART1,
        (pin_tx, pin_rx),
        &mut afio.mapr,
        Config::default()
            .baudrate(9600.bps()),
        &clocks,
        );

        delay.delay_ms(10_u32);
        writeln!(serial, "The code runs in hardware\r\n").ok();

    }

    #[cfg(debug_assertions)]
    {    
        hprintln!("").ok();
        hprintln!("----The code runs in qemu----").ok();
        hprintln!("").ok();
    }
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}
