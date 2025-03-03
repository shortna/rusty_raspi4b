use crate::utils::bits::*;
use core::ptr::read_volatile;

pub enum GPIOPin {
    PIN0,
    PIN1,
    PIN2,
    PIN3,
    PIN4,
    PIN5,
    PIN6,
    PIN7,
    PIN8,
    PIN9,
    PIN10,
    PIN11,
    PIN12,
    PIN13,
    PIN14,
    PIN15,
    PIN16,
    PIN17,
    PIN18,
    PIN19,
    PIN20,
    PIN21,
    PIN22,
    PIN23,
    PIN24,
    PIN25,
    PIN26,
    PIN27,
    PIN28,
    PIN29,
    PIN30,
    PIN31,
    PIN32,
    PIN33,
    PIN34,
    PIN35,
    PIN36,
    PIN37,
    PIN38,
    PIN39,
    PIN40,
    PIN41,
    PIN42,
    PIN43,
    PIN44,
    PIN45,
    PIN46,
    PIN47,
    PIN48,
    PIN49,
    PIN50,
    PIN51,
    PIN52,
    PIN53,
}

pub struct GPIO {
    registers: &'static mut GPIORegisters,
}

impl GPIO {
    pub const fn new() -> GPIO {
        let periph: GPIO = GPIO {
            registers: GPIORegisters::new(),
        };
        return periph;
    }

    pub fn pin_function_set(&mut self, pin: GPIOPin, function: GPIOFunction) {
        let pin_ = pin as u32;

        let reg = match pin_ {
            0..=9 => &mut self.registers.gpfsel0,
            10..=19 => &mut self.registers.gpfsel1,
            20..=29 => &mut self.registers.gpfsel2,
            30..=39 => &mut self.registers.gpfsel3,
            40..=49 => &mut self.registers.gpfsel4,
            50..=53 => &mut self.registers.gpfsel5,
            _ => unreachable!(),
        };

        const BITS_PER_PIN: u32 = 3;
        const BITS_PER_REGISTER: u32 = 30;
        let shift = (pin_ * BITS_PER_PIN) % BITS_PER_REGISTER;
        register_volatile_and(reg, !(0x7u32 << shift));
        register_volatile_or(reg, (function as u32) << shift);
    }

    pub fn pin_function_get(&self, pin: GPIOPin) -> GPIOFunction {
        let pin_ = pin as u32;

        let reg = match pin_ {
            0..=9 => &self.registers.gpfsel0,
            10..=19 => &self.registers.gpfsel1,
            20..=29 => &self.registers.gpfsel2,
            30..=39 => &self.registers.gpfsel3,
            40..=49 => &self.registers.gpfsel4,
            50..=53 => &self.registers.gpfsel5,
            _ => unreachable!(),
        };

        const BITS_PER_PIN: u32 = 3;
        const BITS_PER_REGISTER: u32 = 30;
        let pin_start_bit = (pin_ * BITS_PER_PIN) % BITS_PER_REGISTER;
        let mask = ((pin_start_bit << BITS_PER_PIN) - 1) & !(pin_start_bit - 1);
        let value = unsafe { read_volatile(reg) };
        return ((value & mask) >> pin_start_bit).try_into().unwrap();
    }

    pub fn pin_set(&mut self, pin: GPIOPin) {
        let pin_ = pin as u32;

        let reg = match pin_ {
            0..=31 => &mut self.registers.gpset0,
            32..=53 => &mut self.registers.gpset1,
            _ => unreachable!(),
        };

        const BITS_PER_REGISTER: u32 = 32;
        register_volatile_or(reg, 1u32 << (pin_ % BITS_PER_REGISTER));
    }

    pub fn pin_clear(&mut self, pin: GPIOPin) {
        let pin_ = pin as u32;

        let reg = match pin_ {
            0..=31 => &mut self.registers.gpclr0,
            32..=53 => &mut self.registers.gpclr1,
            _ => unreachable!(),
        };

        const BITS_PER_REGISTER: u32 = 32;
        register_volatile_or(reg, 1u32 << (pin_ % BITS_PER_REGISTER));
    }

    pub fn pin_level(&self, pin: GPIOPin) -> GPIOPinLevel {
        let pin_ = pin as u32;

        let reg = match pin_ {
            0..=31 => &self.registers.gplev0,
            32..=53 => &self.registers.gplev1,
            _ => unreachable!(),
        };

        const BITS_PER_REGISTER: u32 = 32;
        let value = unsafe { read_volatile(reg) } & (pin_ % BITS_PER_REGISTER);
        match value > 0 {
            true => return GPIOPinLevel::High,
            false => return GPIOPinLevel::Low,
        }
    }
}

#[repr(C)]
struct GPIORegisters {
    gpfsel0: u32,                 /* 0x00 GPFSEL0 GPIO Function Select 0 */
    gpfsel1: u32,                 /* 0x04 GPFSEL1 GPIO Function Select 1 */
    gpfsel2: u32,                 /* 0x08 GPFSEL2 GPIO Function Select 2 */
    gpfsel3: u32,                 /* 0x0c GPFSEL3 GPIO Function Select 3 */
    gpfsel4: u32,                 /* 0x10 GPFSEL4 GPIO Function Select 4 */
    gpfsel5: u32,                 /* 0x14 GPFSEL5 GPIO Function Select 5 */
    padding0: [u8; 0x4],          /* 0x18 padding */
    gpset0: u32,                  /* 0x1c GPSET0 GPIO Pin Output Set 0 */
    gpset1: u32,                  /* 0x20 GPSET1 GPIO Pin Output Set 1 */
    padding1: [u8; 0x4],          /* 0x24 padding */
    gpclr0: u32,                  /* 0x28 GPCLR0 GPIO Pin Output Clear 0 */
    gpclr1: u32,                  /* 0x2c GPCLR1 GPIO Pin Output Clear 1 */
    padding2: [u8; 0x4],          /* 0x30 padding */
    gplev0: u32,                  /* 0x34 GPLEV0 GPIO Pin Level 0 */
    gplev1: u32,                  /* 0x38 GPLEV1 GPIO Pin Level 1 */
    padding3: [u8; 0x4],          /* 0x3c padding */
    gpeds0: u32,                  /* 0x40 GPEDS0 GPIO Pin Event Detect Status 0 */
    gpeds1: u32,                  /* 0x44 GPEDS1 GPIO Pin Event Detect Status 1 */
    padding4: [u8; 0x4],          /* 0x48 padding */
    gpren0: u32,                  /* 0x4c GPREN0 GPIO Pin Rising Edge Detect Enable 0 */
    gpren1: u32,                  /* 0x50 GPREN1 GPIO Pin Rising Edge Detect Enable 1 */
    padding5: [u8; 0x4],          /* 0x54 padding */
    gpfen0: u32,                  /* 0x58 GPFEN0 GPIO Pin Falling Edge Detect Enable 0 */
    gpfen1: u32,                  /* 0x5c GPFEN1 GPIO Pin Falling Edge Detect Enable 1 */
    padding6: [u8; 0x4],          /* 0x60 padding */
    gphen0: u32,                  /* 0x64 GPHEN0 GPIO Pin High Detect Enable 0 */
    gphen1: u32,                  /* 0x68 GPHEN1 GPIO Pin High Detect Enable 1 */
    padding7: [u8; 0x4],          /* 0x6c padding */
    gplen0: u32,                  /* 0x70 GPLEN0 GPIO Pin Low Detect Enable 0 */
    gplen1: u32,                  /* 0x74 GPLEN1 GPIO Pin Low Detect Enable 1 */
    padding8: [u8; 0x4],          /* 0x78 padding */
    gparen0: u32,                 /* 0x7c GPAREN0 GPIO Pin Async. Rising Edge Detect 0 */
    gparen1: u32,                 /* 0x80 GPAREN1 GPIO Pin Async. Rising Edge Detect 1 */
    padding9: [u8; 0x4],          /* 0x84 padding */
    gpafen0: u32,                 /* 0x88 GPAFEN0 GPIO Pin Async. Falling Edge Detect 0 */
    gpafen1: u32,                 /* 0x8c GPAFEN1 GPIO Pin Async. Falling Edge Detect 1 */
    padding10: [u8; 0x54],        /* 0x90 padding */
    gpio_pup_pdn_cntrl_reg0: u32, /* 0xe4 GPIO_PUP_PDN_CNTRL_REG0 GPIO Pull-up / Pull-down Register 0 */
    gpio_pup_pdn_cntrl_reg1: u32, /* 0xe8 GPIO_PUP_PDN_CNTRL_REG1 GPIO Pull-up / Pull-down Register 1 */
    gpio_pup_pdn_cntrl_reg2: u32, /* 0xec GPIO_PUP_PDN_CNTRL_REG2 GPIO Pull-up / Pull-down Register 2 */
    gpio_pup_pdn_cntrl_reg3: u32, /* 0xf0 GPIO_PUP_PDN_CNTRL_REG3 GPIO Pull-up / Pull-down Register 3 */
}

pub enum GPIOPinLevel {
    High,
    Low,
}

pub enum GPIOFunction {
    INPUT = 0,
    OUTPUT = 1,
    ALT0 = 4,
    ALT1 = 5,
    ALT2 = 6,
    ALT3 = 7,
    ALT4 = 3,
    ALT5 = 2,
}

impl TryFrom<u32> for GPIOFunction {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => return Ok(GPIOFunction::INPUT),
            1 => return Ok(GPIOFunction::OUTPUT),
            4 => return Ok(GPIOFunction::ALT0),
            5 => return Ok(GPIOFunction::ALT1),
            6 => return Ok(GPIOFunction::ALT2),
            7 => return Ok(GPIOFunction::ALT3),
            3 => return Ok(GPIOFunction::ALT4),
            2 => return Ok(GPIOFunction::ALT5),
            _ => Err(u32::MAX),
        }
    }
}

impl GPIORegisters {
    const BASE: usize = 0xfe200000;

    pub const fn new() -> &'static mut GPIORegisters {
        unsafe {
            return &mut *(Self::BASE as *mut GPIORegisters);
        }
    }
}
