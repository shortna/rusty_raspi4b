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
        unsafe { &mut *(Self::BASE as *mut AUXRegisters) }
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
        register_volatile_or(u32_register_mut!(self.registers.enable), BITu32!(0));
    }

    /* UNSUPPORTED */
    pub fn enable_spi(&mut self) {
        register_volatile_or(u32_register_mut!(self.registers.enable), BITu32!(1));
    }

    /* UNSUPPORTED */
    pub fn enable_spi2(&mut self) {
        register_volatile_or(u32_register_mut!(self.registers.enable), BITu32!(2));
    }

    /* UNSUPPORTED */
    pub fn disable_mini_uart(&mut self) {
        register_volatile_and(u32_register_mut!(self.registers.enable), !BITu32!(0));
    }

    /* UNSUPPORTED */
    pub fn disable_spi(&mut self) {
        register_volatile_and(u32_register_mut!(self.registers.enable), !BITu32!(1));
    }

    /* UNSUPPORTED */
    pub fn disable_spi2(&mut self) {
        register_volatile_and(u32_register_mut!(self.registers.enable), !BITu32!(2));
    }

    pub fn irq_pending_mini_uart(&self) -> bool {
        let irq = unsafe { read_volatile(u32_register!(self.registers.irq)) };
        (irq & BITu32!(0)) > 0
    }

    /* UNSUPPORTED */
    pub fn irq_pending_spi(&self) -> bool {
        let irq = unsafe { read_volatile(u32_register!(self.registers.irq)) };
        (irq & BITu32!(1)) > 0
    }

    /* UNSUPPORTED */
    pub fn irq_pending_spi2(&self) -> bool {
        let irq = unsafe { read_volatile(u32_register!(self.registers.irq)) };
        (irq & BITu32!(2)) > 0
    }
}

pub mod peripherals {
    use crate::utils::bits::*;
    use core::ptr::{read_volatile, write_volatile};

    #[repr(C)]
    pub struct SPI {}

    #[repr(C)]
    pub struct MiniUart {
        io: u32,  /* 0x40 AUX_MU_IO_REG Mini UART I/O Data */
        ier: u32, /* 0x44 AUX_MU_IER_REG Mini UART Interrupt Enable */
        iir: u32, /* 0x48 AUX_MU_IIR_REG Mini UART Interrupt Identify */
        /* UNSUPPORTED in QEMU */
        lcr: u32, /* 0x4c AUX_MU_LCR_REG Mini UART Line Control */
        /* UNSUPPORTED in QEMU */
        mcr: u32, /* 0x50 AUX_MU_MCR_REG Mini UART Modem Control */
        lsr: u32, /* 0x54 AUX_MU_LSR_REG Mini UART Line Status */
        /* UNSUPPORTED in QEMU */
        msr: u32, /* 0x58 AUX_MU_MSR_REG Mini UART Modem Status */
        /* UNSUPPORTED in QEMU */
        scratch: u32, /* 0x5c AUX_MU_SCRATCH Mini UART Scratch */
        cntl: u32,    /* 0x60 AUX_MU_CNTL_REG Mini UART Extra Control */
        stat: u32,    /* 0x64 AUX_MU_STAT_REG Mini UART Extra Status */
        /* UNSUPPORTED in QEMU */
        baud: u32, /* 0x68 AUX_MU_BAUD_REG Mini UART Baudrate */
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub enum BaudRate {
        Baud476 = 476,
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
            unsafe {
                write_volatile(u32_register_mut!(self.io), byte);
            }
        }

        pub fn receive(&self) -> u32 {
            unsafe { read_volatile(u32_register!(self.io)) }
        }

        pub fn enable_receive_interrupt(&mut self) {
            register_volatile_or(u32_register_mut!(self.ier), BITu32!(1));
        }

        pub fn disable_receive_interrupt(&mut self) {
            register_volatile_and(u32_register_mut!(self.ier), !BITu32!(1));
        }

        pub fn enable_transmit_interrupt(&mut self) {
            register_volatile_or(u32_register_mut!(self.ier), BITu32!(0));
        }

        pub fn disable_transmit_interrupt(&mut self) {
            register_volatile_and(u32_register_mut!(self.ier), !BITu32!(0));
        }

        pub fn clear_receive_fifo(&mut self) {
            register_volatile_or(u32_register_mut!(self.iir), BITu32!(1));
        }

        pub fn clear_transmit_fifo(&mut self) {
            register_volatile_or(u32_register_mut!(self.iir), BITu32!(2));
        }

        pub fn interrupt_id(&self) -> u32 {
            let value = unsafe { read_volatile(u32_register!(self.iir)) };
            (value & (BITu32!(1) | BITu32!(2))) >> 1u32
        }

        pub fn interrupt_pending(&self) -> bool {
            unsafe { !((read_volatile(u32_register!(self.iir)) & BITu32!(0)) > 0) }
        }

        pub fn set_8bit_mode(&mut self) {
            register_volatile_or(u32_register_mut!(self.lcr), BITu32!(0));
        }

        pub fn set_7bit_mode(&mut self) {
            register_volatile_and(u32_register_mut!(self.lcr), !BITu32!(0));
        }

        pub fn receive_overrun_clear(&mut self) {
            unsafe {
                let _ = read_volatile(u32_register!(self.lsr));
            }
        }

        pub fn transmitter_enable(&mut self) {
            register_volatile_or(u32_register_mut!(self.cntl), BITu32!(1));
        }

        pub fn receiver_enable(&mut self) {
            register_volatile_or(u32_register_mut!(self.cntl), BITu32!(0));
        }

        pub fn transmitter_disable(&mut self) {
            register_volatile_and(u32_register_mut!(self.cntl), !BITu32!(1));
        }

        pub fn receiver_disable(&mut self) {
            register_volatile_and(u32_register_mut!(self.cntl), !BITu32!(0));
        }

        pub fn receiver_symbol_avaliable(&self) -> bool {
            unsafe { (read_volatile(u32_register!(self.stat)) & BITu32!(0)) > 0 }
        }

        pub fn transmitter_space_avaliable(&self) -> bool {
            unsafe { (read_volatile(u32_register!(self.stat)) & BITu32!(1)) > 0 }
        }

        pub fn receiver_idle(&self) -> bool {
            unsafe { (read_volatile(u32_register!(self.stat)) & BITu32!(2)) > 0 }
        }

        pub fn tranmitter_idle(&self) -> bool {
            unsafe { (read_volatile(u32_register!(self.stat)) & BITu32!(3)) > 0 }
        }

        pub fn receive_overrun(&self) -> bool {
            unsafe { (read_volatile(u32_register!(self.stat)) & BITu32!(4)) > 0 }
        }

        pub fn transmit_fifo_empty(&self) -> bool {
            unsafe { (read_volatile(u32_register!(self.stat)) & BITu32!(8)) > 0 }
        }

        pub fn transmitter_done(&self) -> bool {
            unsafe { (read_volatile(u32_register!(self.stat)) & BITu32!(9)) > 0 }
        }

        pub fn receive_fifo_level(&self) -> u32 {
            unsafe { (read_volatile(u32_register!(self.stat)) >> 16u32) & 0x7 }
        }

        pub fn transmit_fifo_level(&self) -> u32 {
            unsafe { (read_volatile(u32_register!(self.stat)) >> 24u32) & 0x7 }
        }

        // well, that's a fun one
        // https://github.com/qemu/qemu/blob/d9a4282c4b690e45d25c2b933f318bb41eeb271d/hw/char/bcm2835_aux.c#L147
        pub fn set_baudrate(&mut self, baudrate: BaudRate) {
            const UART_CLOCK: u32 = 250_000_000;
            let baudrate_reg: u32 = (UART_CLOCK / (8 * baudrate as u32)) - 1;
            unsafe {
                write_volatile(u32_register_mut!(self.baud), baudrate_reg);
            }
        }

        pub fn get_baudrate(&self) -> u32 {
            const UART_CLOCK: u32 = 250_000_000;
            unsafe { UART_CLOCK / (8 * (read_volatile(u32_register!(self.baud)) + 1)) }
        }
    }
}
