type Result<T> = std::io::Result<T>;

fn zigzag_encode_32(src: i32) -> u32 {
    if src >= 0 {
        (src as u32) << 1
    } else {
        (((-src) as u32) << 1) - 1
    }
}

fn zigzag_decode_32(src: u32) -> i32 {
    if src & 1 != 0 {
        -((src >> 1) as i32) - 1
    } else {
        (src >> 1) as i32
    }
}

fn zigzag_encode_64(src: i64) -> u64 {
    if src >= 0 {
        (src as u64) << 1
    } else {
        (((-src) as u64) << 1) - 1
    }
}

fn zigzag_decode_64(src: u64) -> i64 {
    if src & 1 != 0 {
        -((src >> 1) as i64) - 1
    } else {
        (src >> 1) as i64
    }
}

pub trait VarIntRead {
    fn read_var_i64(&mut self) -> Result<i64> {
        self.read_var_u64().map(|x| zigzag_decode_64(x))
    }

    fn read_var_i32(&mut self) -> Result<i32> {
        self.read_var_u32().map(|x| zigzag_decode_32(x))
    }

    fn read_var_u64(&mut self) -> Result<u64>;

    fn read_var_u32(&mut self) -> Result<u32>;
}

pub trait VarIntWrite {
    fn write_var_i32(&mut self, value: i32) -> Result<usize> {
        self.write_var_u32(zigzag_encode_32(value))
    }

    fn write_var_i64(&mut self, value: i64) -> Result<usize> {
        self.write_var_u64(zigzag_encode_64(value))
    }

    fn write_var_u32(&mut self, value: u32) -> Result<usize>;

    fn write_var_u64(&mut self, value: u64) -> Result<usize>;
}

impl<R> VarIntRead for R
where
    R: std::io::Read,
{
    fn read_var_u64(&mut self) -> Result<u64> {
        let mut buf = [0];
        let mut ans = 0;
        for i in 0..9 {
            self.read_exact(&mut buf)?;

            ans |= (buf[0] as u64 & 0x7F) << 7 * i;

            if buf[0] & 0x80 == 0 {
                break;
            }
        }
        Ok(ans)
    }

    fn read_var_u32(&mut self) -> Result<u32> {
        let mut buf = [0];
        let mut ans = 0;
        for i in 0..5 {
            self.read_exact(&mut buf)?;

            ans |= (buf[0] as u32 & 0x7F) << 7 * i;

            if buf[0] & 0x80 == 0 {
                break;
            }
        }
        Ok(ans)
    }
}

impl<W> VarIntWrite for W
where
    W: std::io::Write,
{
    fn write_var_u32(&mut self, mut value: u32) -> Result<usize> {
        let mut buf = [0; (u32::BITS as usize + 6) / 7];
        let mut i = 0;

        loop {
            buf[i] = (value & 0b0111_1111) as u8;
            value >>= 7;
            if value != 0 {
                buf[i] |= 0b1000_0000;
            }
            i += 1;

            if value == 0 {
                break;
            }
        }

        self.write_all(&buf[..i])?;
        Ok(i)
    }

    fn write_var_u64(&mut self, mut value: u64) -> Result<usize> {
        let mut buf = [0; (u64::BITS as usize + 6) / 7];
        let mut i = 0;

        loop {
            buf[i] = (value & 0b0111_1111) as u8;
            value >>= 7;
            if value != 0 {
                buf[i] |= 0b1000_0000;
            }
            i += 1;

            if value == 0 {
                break;
            }
        }

        self.write_all(&buf[..i])?;
        Ok(i)
    }
}
