use crate::NetSvcMessage;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SSvcCreateStringTable {
    pub name: ArrayString<256>,
    pub max_entries: u16,
    pub num_entries: u16,
    pub user_data_size: Option<u16>,
    pub user_data_size_bits: Option<u8>,
    pub flags: StringTableFlags,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringTableFlags {
    pub data_compressed: bool,
    pub dictionary_maybe_enabled: bool,
}

impl SSvcCreateStringTable {
    pub(crate) fn bitread(
        reader: &mut (impl BitRead + BitReadExt),
        oe: bool,
    ) -> anyhow::Result<(NetSvcMessage, usize)> {
        let name = reader.read_nts_arraystring::<256>()?;
        let max_entries = reader.read::<u16>(16)?;
        let num_entries = reader.read::<u16>(max_entries.log2() + 1)?;
        let length = reader.read::<u32>(20)?;

        let mut extrabits = 0;
        let (user_data_size, user_data_size_bits) = if reader.read_bit()? {
            extrabits += 16;
            (Some(reader.read::<u16>(12)?), Some(reader.read::<u8>(4)?))
        } else {
            (None, None)
        };

        let flags = {
            let data_compressed = reader.read_bit()?;
            let dictionary_maybe_enabled = if !oe {
                extrabits += 1;
                reader.read_bit()?
            } else {
                false
            };
            StringTableFlags {
                data_compressed,
                dictionary_maybe_enabled,
            }
        };

        reader.skip(length)?;

        Ok((
            NetSvcMessage::SvcCreateStringTable(SSvcCreateStringTable {
                name,
                max_entries,
                num_entries,
                user_data_size,
                user_data_size_bits,
                flags,
            }),
            (name.len() + 1) * 8
                + 16
                + (max_entries.log2() as usize + 1)
                + 20
                + 1
                + 1
                + extrabits
                + length as usize,
        ))
    }
}
