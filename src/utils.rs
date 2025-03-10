pub mod bits {
    use core::ptr::{read_volatile, write_volatile};

    /// read value from dst, perform bitwise or on result with provided value and write it back
    pub fn register_volatile_or<T: core::ops::BitOr<Output = T>>(dst: *mut T, value: T) {
        unsafe {
            let v = read_volatile(dst as *const T);
            write_volatile(dst, v | value);
        }
    }

    /// read value from dst, perform bitwise `and` on result with provided value and write it back
    pub fn register_volatile_and<T: core::ops::BitAnd<Output = T>>(dst: *mut T, value: T) {
        unsafe {
            let v = read_volatile(dst as *const T);
            write_volatile(dst, v & value);
        }
    }

    macro_rules! BITu32 {
        ($b: expr) => {
            (1u32 << $b)
        };
    }
    pub(crate) use BITu32;
}

pub mod bariers {
    use core::arch::asm;
    pub fn memory_write_barier() {
        unsafe {
            asm!("dmb st");
        }
    }

    pub fn memory_read_barier() {
        unsafe {
            asm!("dmb ld");
        }
    }
}
