#![feature(int_log)]

mod demo;
mod header;
mod message;

pub mod messages;
pub use messages::NetSvcMessage;
pub mod svcnetmessage;
pub use demo::Demo;
pub use header::Header;
pub use message::{Frame, FrameData};

use std::str::FromStr;

// stdafx type thing
pub(crate) use arrayvec::ArrayString;
pub(crate) use bitstream_io::read::BitRead;
pub(crate) use bitstream_io::read::BitReader;
pub(crate) use bitstream_io::Endianness;
pub(crate) use std::io::Read;

pub(crate) trait BitReadExt {
    fn read_nts(&mut self) -> anyhow::Result<String>;
    fn read_nts_arraystring<const CAP: usize>(&mut self) -> anyhow::Result<ArrayString<CAP>>;
    fn read_fixed_arraystr<const CAP: usize>(&mut self) -> anyhow::Result<ArrayString<CAP>>;
    fn read_fixed_arraystr_notrim<const CAP: usize>(&mut self) -> anyhow::Result<ArrayString<CAP>>;
    fn read_float(&mut self) -> anyhow::Result<f32>;
    fn read_option<T: bitstream_io::Numeric>(&mut self, bits: u32) -> anyhow::Result<Option<T>>;
    fn read_option_signed<T: bitstream_io::SignedNumeric>(
        &mut self,
        bits: u32,
    ) -> anyhow::Result<Option<T>>;
}

impl<R> BitReadExt for R
where
    R: BitRead,
{
    fn read_nts(&mut self) -> anyhow::Result<String> {
        let mut str = String::new();
        loop {
            let byte = self.read::<u8>(8)?;
            if byte == 0 {
                return Ok(str);
            } else {
                str.push(byte as char);
            }
        }
    }

    fn read_nts_arraystring<const CAP: usize>(&mut self) -> anyhow::Result<ArrayString<CAP>> {
        let mut str = ArrayString::<CAP>::new();
        for _ in 0..CAP {
            let byte = self.read::<u8>(8)?;
            if byte == 0 {
                return Ok(str);
            } else {
                str.push(byte as char);
            }
        }
        Err(anyhow::anyhow!("string length exceeds cap"))
    }

    fn read_fixed_arraystr<const SIZE: usize>(&mut self) -> anyhow::Result<ArrayString<SIZE>> {
        let bytes = self.read_to_bytes::<SIZE>()?;
        let z = bytes.iter().position(|b| *b == 0).unwrap_or(SIZE);
        let str = std::str::from_utf8(&bytes[..z])?;
        Ok(ArrayString::from_str(str)?)
    }

    fn read_fixed_arraystr_notrim<const SIZE: usize>(
        &mut self,
    ) -> anyhow::Result<ArrayString<SIZE>> {
        let bytes = self.read_to_bytes::<SIZE>()?;
        Ok(ArrayString::from_byte_string(&bytes)?)
    }

    fn read_float(&mut self) -> anyhow::Result<f32> {
        Ok(unsafe { core::mem::transmute(self.read_to_bytes::<4>()?) })
    }
    fn read_option<T: bitstream_io::Numeric>(&mut self, bits: u32) -> anyhow::Result<Option<T>> {
        if self.read_bit()? {
            Ok(Some(self.read::<T>(bits)?))
        } else {
            Ok(None)
        }
    }

    fn read_option_signed<T: bitstream_io::SignedNumeric>(
        &mut self,
        bits: u32,
    ) -> anyhow::Result<Option<T>> {
        if self.read_bit()? {
            Ok(Some(self.read_signed::<T>(bits)?))
        } else {
            Ok(None)
        }
    }
}
