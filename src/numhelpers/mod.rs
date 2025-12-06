pub trait NumDigits<Item> {
    type Item;
    fn num_digits(&self) -> Item;
}

impl NumDigits<u8> for u8 {
    type Item = u8;
    fn num_digits(&self) -> u8 {
        self.checked_ilog10().unwrap_or(0) as u8 + 1
    }
}

impl NumDigits<u16> for u16 {
    type Item = u16;
    fn num_digits(&self) -> u16 {
        self.checked_ilog10().unwrap_or(0) as u16 + 1
    }
}

impl NumDigits<u32> for u32 {
    type Item = u32;
    fn num_digits(&self) -> u32 {
        self.checked_ilog10().unwrap_or(0) as u32 + 1
    }
}

impl NumDigits<u64> for u64 {
    type Item = u64;
    fn num_digits(&self) -> u64 {
        self.checked_ilog10().unwrap_or(0) as u64 + 1
    }
}

impl NumDigits<u128> for u128 {
    type Item = u128;
    fn num_digits(&self) -> u128 {
        self.checked_ilog10().unwrap_or(0) as u128 + 1
    }
}

impl NumDigits<usize> for usize {
    type Item = usize;
    fn num_digits(&self) -> usize {
        self.checked_ilog10().unwrap_or(0) as usize + 1
    }
}
