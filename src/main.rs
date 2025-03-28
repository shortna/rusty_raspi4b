#![no_std]
#![no_main]
#![allow(dead_code)]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

mod aux;
mod gpio;
mod utils;
use crate::aux::AUX_PERIPHERALS;
use crate::aux::peripherals::*;

use crate::gpio::*;
use crate::utils::bariers::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let _ = info;
    unsafe {
        asm!("ldr x0, =0xdeadbeef", options(nostack));
    }
    loop {}
}

global_asm!(include_str!("./init.S"));

fn init_mini_uart() {
    let aux = &raw mut AUX_PERIPHERALS;
    let uart = unsafe { &mut *(*aux).take_mini_uart() };
    let gpio_ = &raw mut GPIO;
    let gpio = unsafe { &mut *(*gpio_).take_gpio() };

    memory_write_barier();
    gpio.pin_function_set(GPIOPin::PIN14, GPIOFunction::ALT5);
    gpio.pin_function_set(GPIOPin::PIN15, GPIOFunction::ALT5);

    memory_write_barier();
    uart.receiver_disable();
    uart.transmitter_disable();

    while !uart.receiver_idle() {}
    while !uart.tranmitter_idle() {}

    uart.disable_transmit_interrupt();
    uart.disable_receive_interrupt();
    uart.receive_overrun_clear();

    uart.clear_transmit_fifo();
    uart.clear_receive_fifo();

    // unsupported in qemu
    // uart.set_baudrate(BaudRate::Baud476);
    uart.set_8bit_mode();

    uart.transmitter_enable();
    uart.receiver_enable();

    unsafe {
        (*gpio_).return_gpio(gpio);
        (*aux).return_mini_uart(uart);
    }
}

#[inline]
fn putchar(uart: &mut MiniUart, ch: u8) {
    uart.transmit(ch as u32);
}

fn print(uart: &mut MiniUart, str: &[u8]) {
    for ch in str {
        putchar(uart, *ch);
    }
    while !uart.tranmitter_idle() {}
}

#[unsafe(no_mangle)]
fn main() {
    memory_write_barier();
    let aux = &raw mut AUX_PERIPHERALS;
    let registers = unsafe { &mut *(*aux).take_aux_registers() };
    registers.enable_mini_uart();
    unsafe {
        (*aux).return_aux_registers(registers);
    }

    let aux = &raw mut AUX_PERIPHERALS;
    let mini_uart = unsafe { &mut *(*aux).take_mini_uart() };

    let str = b"Hello, World!";
    for i in 0..13 {
        print(mini_uart, &str[0..i]);
        print(mini_uart, b"\n");
    }

    loop {
        while !mini_uart.receiver_symbol_avaliable() {}
        let byte = mini_uart.receive();
        putchar(mini_uart, byte as u8);
    }
}
