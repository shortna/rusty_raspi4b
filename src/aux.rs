/// BCM2835
use crate::aux::peripherals::MiniUart;
use crate::utils::bits::*;
use core::option::Option;
use core::ptr::read_volatile;

#[repr(C)]
struct AUXRegisters {
    irq: u32,    /* 0x00 AUX_IRQ Auxiliary Interrupt status */
    enable: u32, /* 0x04 AUX_ENABLES Auxiliary enables */
}

impl AUXRegisters {
    const BASE: usize = 0xfe215000;
    pub const fn new() -> &'static mut AUXRegisters {
        unsafe {
            &mut *(Self::BASE as *mut AUXRegisters)
        }
    }
}

pub struct AUXPeripherals {
    registers: &'static mut AUXRegisters,
    mini_uart: Option<*mut MiniUart>,
}

impl AUXPeripherals {
    pub const fn new() -> AUXPeripherals {
        let periph: AUXPeripherals = AUXPeripherals {
            registers: AUXRegisters::new(),
            mini_uart: Some(MiniUart::new()),
        };
        periph
    }

    pub fn take_mini_uart(&mut self) -> *mut MiniUart {
        let p = self.mini_uart.take();
        p.unwrap()
    }

    pub fn enable_mini_uart(&mut self) {
        let reg = &mut self.registers.enable as *mut u32;
        register_volatile_or(reg, BITu32!(0));
    }

    pub fn enable_spi(&mut self) {
        let reg = &mut self.registers.enable as *mut u32;
        register_volatile_or(reg, BITu32!(1));
    }

    pub fn enable_spi2(&mut self) {
        let reg = &mut self.registers.enable as *mut u32;
        register_volatile_or(reg, BITu32!(2));
    }

    pub fn disable_mini_uart(&mut self) {
        let reg = &mut self.registers.enable as *mut u32;
        register_volatile_and(reg, !BITu32!(0));
    }

    pub fn disable_spi(&mut self) {
        let reg = &mut self.registers.enable as *mut u32;
        register_volatile_and(reg, !BITu32!(1));
    }

    pub fn disable_spi2(&mut self) {
        let reg = &mut self.registers.enable as *mut u32;
        register_volatile_and(reg, !BITu32!(2));
    }

    pub fn irq_pending_mini_uart(&self) -> bool {
        let reg = &self.registers.irq as *const u32;
        let irq = unsafe { read_volatile(reg) };
        (irq & BITu32!(0)) > 0
    }

    pub fn irq_pending_spi(&self) -> bool {
        let reg = &self.registers.irq as *const u32;
        let irq = unsafe { read_volatile(reg) };
        (irq & BITu32!(1)) > 0
    }

    pub fn irq_pending_spi2(&self) -> bool {
        let reg = &self.registers.irq as *const u32;
        let irq = unsafe { read_volatile(reg) };
        (irq & BITu32!(2)) > 0
    }
}

pub mod peripherals {
    use crate::utils::bits::*;
    use core::ptr::{read_volatile, write_volatile};

    #[repr(C)]
    pub struct MiniUart {
        io: u32,      /* 0x40 AUX_MU_IO_REG Mini UART I/O Data */
        ier: u32,     /* 0x44 AUX_MU_IER_REG Mini UART Interrupt Enable */
        iir: u32,     /* 0x48 AUX_MU_IIR_REG Mini UART Interrupt Identify */
        lcr: u32,     /* 0x4c AUX_MU_LCR_REG Mini UART Line Control */
        mcr: u32,     /* 0x50 AUX_MU_MCR_REG Mini UART Modem Control */
        lsr: u32,     /* 0x54 AUX_MU_LSR_REG Mini UART Line Status */
        msr: u32,     /* 0x58 AUX_MU_MSR_REG Mini UART Modem Status */
        scratch: u32, /* 0x5c AUX_MU_SCRATCH Mini UART Scratch */
        cntl: u32,    /* 0x60 AUX_MU_CNTL_REG Mini UART Extra Control */
        stat: u32,    /* 0x64 AUX_MU_STAT_REG Mini UART Extra Status */
        baud: u32,    /* 0x68 AUX_MU_BAUD_REG Mini UART Baudrate */
    }

    pub enum BaudRate {
        Baud1200 = 1200,
        Baud2400 = 2400,
        Baud4800 = 4800,
        Baud9600 = 9600,
        Baud19200 = 19200,
        Baud38400 = 38400,
        Baud57600 = 57600,
        Baud115200 = 115200,
        Baud230400 = 230400,
        Baud460800 = 460800,
        Baud921600 = 921600,
    }

    impl MiniUart {
        const BASE: usize = 0xfe215040;
        pub const fn new() -> *mut Self {
            Self::BASE as *mut Self
        }

        pub fn transmit(&mut self, byte: u32) {
            let reg = &mut self.io as *mut u32;
            unsafe {
                write_volatile(reg, byte);
            }
        }

        pub fn receive(&self) -> u32 {
            let reg = &self.io as *const u32;
            unsafe {
                read_volatile(reg)
            }
        }

        pub fn enable_receive_interrupt(&mut self) {
            let reg = &mut self.ier as *mut u32;
            register_volatile_or(reg, BITu32!(1));
        }

        pub fn disable_receive_interrupt(&mut self) {
            let reg = &mut self.ier as *mut u32;
            register_volatile_and(reg, !BITu32!(1));
        }

        pub fn enable_transmit_interrupt(&mut self) {
            let reg = &mut self.ier as *mut u32;
            register_volatile_or(reg, BITu32!(0));
        }

        pub fn disable_transmit_interrupt(&mut self) {
            let reg = &mut self.ier as *mut u32;
            register_volatile_and(reg, !BITu32!(0));
        }

        pub fn clear_receive_fifo(&mut self) {
            let reg = &mut self.iir as *mut u32;
            register_volatile_or(reg, BITu32!(1));
        }

        pub fn clear_transmit_fifo(&mut self) {
            let reg = &mut self.iir as *mut u32;
            register_volatile_or(reg, BITu32!(2));
        }

        pub fn interrupt_id(&self) -> u32 {
            let reg = &self.iir as *const u32;
            let value = unsafe { read_volatile(reg) };
            (value & (BITu32!(1) | BITu32!(2))) >> 1u32
        }

        pub fn interrupt_pending(&self) -> bool {
            unsafe {
                let reg = &self.iir as *const u32;
                !((read_volatile(reg) & BITu32!(0)) > 0)
            }
        }

        pub fn set_8bit_mode(&mut self) {
            let reg = &mut self.lcr as *mut u32;
            register_volatile_or(reg, BITu32!(0));
        }

        pub fn set_7bit_mode(&mut self) {
            let reg = &mut self.lcr as *mut u32;
            register_volatile_and(reg, !BITu32!(0));
        }

        pub fn receive_overrun_clear(&mut self) {
            let reg = &self.lsr as *const u32;
            unsafe {
                let _ = read_volatile(reg);
            }
        }

        pub fn transmitter_enable(&mut self) {
            let reg = &mut self.cntl as *mut u32;
            register_volatile_or(reg, BITu32!(1));
        }

        pub fn receiver_enable(&mut self) {
            let reg = &mut self.cntl as *mut u32;
            register_volatile_or(reg, BITu32!(0));
        }

        pub fn transmitter_disable(&mut self) {
            let reg = &mut self.cntl as *mut u32;
            register_volatile_and(reg, !BITu32!(1));
        }

        pub fn receiver_disable(&mut self) {
            let reg = &mut self.cntl as *mut u32;
            register_volatile_and(reg, !BITu32!(0));
        }

        pub fn receiver_symbol_avaliable(&self) -> bool {
            let reg = &self.stat as *const u32;
            unsafe {
                (read_volatile(reg) & BITu32!(0)) > 0
            }
        }

        pub fn transmitter_space_avaliable(&self) -> bool {
            let reg = &self.stat as *const u32;
            unsafe {
                (read_volatile(reg) & BITu32!(1)) > 0
            }
        }

        pub fn receiver_idle(&self) -> bool {
            let reg = &self.stat as *const u32;
            unsafe {
                (read_volatile(reg) & BITu32!(2)) > 0
            }
        }

        pub fn tranmitter_idle(&self) -> bool {
            let reg = &self.stat as *const u32;
            unsafe {
                (read_volatile(reg) & BITu32!(3)) > 0
            }
        }

        pub fn receive_overrun(&self) -> bool {
            let reg = &self.stat as *const u32;
            unsafe {
                (read_volatile(reg) & BITu32!(4)) > 0
            }
        }

        pub fn transmit_fifo_empty(&self) -> bool {
            let reg = &self.stat as *const u32;
            unsafe {
                (read_volatile(reg) & BITu32!(8)) > 0
            }
        }

        pub fn transmitter_done(&self) -> bool {
            let reg = &self.stat as *const u32;
            unsafe {
                (read_volatile(reg) & BITu32!(9)) > 0
            }
        }

        pub fn receive_fifo_level(&self) -> u32 {
            let reg = &self.stat as *const u32;
            unsafe {
                (read_volatile(reg) >> 16u32) & 0x7
            }
        }

        pub fn transmit_fifo_level(&self) -> u32 {
            let reg = &self.stat as *const u32;
            unsafe {
                (read_volatile(reg) >> 24u32) & 0x7
            }
        }

        pub fn set_baudrate(&mut self, baudrate: BaudRate) {
            let reg = &mut self.baud as *mut u32;
            const UART_CLOCK: u32 = 250_000_000;
            let divisor: u32 = UART_CLOCK / (8 * (baudrate as u32 + 1));
            unsafe {
                write_volatile(reg, divisor);
            }
        }
    }
}
