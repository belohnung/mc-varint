#[cfg(not(no_std))]
type Result<T> = std::io::Result<T>;

#[cfg(no_std)]
type Result<T> = core::result::Result<T, !>;

fn zigzag_encode_32(src: i32) -> u32 {
    if src >= 0 { (src as u32) << 1 } else { (((-src) as u32) << 1) - 1 }
}

fn zigzag_decode_32(src: u32) -> i32 {
    if src & 1 != 0 { -((src >> 1) as i32) - 1 } else { (src >> 1) as i32 }
}

fn zigzag_encode_64(src: i64) -> u64 {
    if src >= 0 { (src as u64) << 1 } else { (((-src) as u64) << 1) - 1 }
}

fn zigzag_decode_64(src: u64) -> i64 {
    if src & 1 != 0 { -((src >> 1) as i64) - 1 } else { (src >> 1) as i64 }
}

pub trait VarIntRead {
    fn read_var_i64(&mut self) -> Result<i64>;

    fn read_var_i32(&mut self) -> Result<i32>;

    fn read_var_u64(&mut self) -> Result<u64> {
        self.read_var_i64().map(|i64v| zigzag_encode_64(i64v))
    }

    fn read_var_u32(&mut self) -> Result<u32> {
        self.read_var_i32().map(|i32v| zigzag_encode_32(i32v))
    }
}

pub trait VarIntWrite {
    fn write_var_i32(&mut self, value: i32) -> Result<usize>;

    fn write_var_i64(&mut self, value: i64) -> Result<usize>;

    fn write_var_u32(&mut self, value: u32) -> Result<usize> {
        self.write_var_i32(zigzag_decode_32(value))
    }

    fn write_var_u64(&mut self, value: u64) -> Result<usize> {
        self.write_var_i64(zigzag_decode_64(value))
    }
}

#[cfg(not(no_std))]
impl<R> VarIntRead for R
where R: std::io::Read {
    fn read_var_i64(&mut self) -> Result<i64> {
        let mut buf = [0];
        let mut ans = 0;
        for i in 0..8 {
            self.read_exact(&mut buf)?;
            ans |= ((buf[0] & 0b0111_1111) as i64) << 7 * i;
            if buf[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }

    fn read_var_i32(&mut self) -> Result<i32> {
        let mut buf = [0];
        let mut ans = 0;
        for i in 0..4 {
            self.read_exact(&mut buf)?;
            ans |= ((buf[0] & 0b0111_1111) as i32) << 7 * i;
            if buf[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }
}

#[cfg(not(no_std))]
impl<W> VarIntWrite for W
where W: std::io::Write {
    fn write_var_i32(&mut self, mut value: i32) -> Result<usize> {
        let mut buf = [0];
        let mut cnt = 0;
        loop {
            buf[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i32::max_value() >> 6);
            if value != 0 {
                buf[0] |= 0b1000_0000;
            }
            cnt += self.write(&mut buf)?;

            if value == 0 {
                break;
            }
        }
        Ok(cnt)
    }

    fn write_var_i64(&mut self, mut value: i64) -> Result<usize> {
        let mut buf = [0];
        let mut cnt = 0;
        loop {
            buf[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i64::max_value() >> 6);
            if value != 0 {
                buf[0] |= 0b1000_0000;
            }
            cnt += self.write(&mut buf)?;

            if value == 0 {
                break;
            }
        }
        Ok(cnt)
    }
}
