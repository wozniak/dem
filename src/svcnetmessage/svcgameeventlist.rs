use crate::NetSvcMessage;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SSvcGameEventList {
    pub events: u16,
    pub descriptors: Vec<GameEventDescription>,
}

impl SSvcGameEventList {
    pub(crate) fn bitread(
        reader: &mut (impl BitRead + BitReadExt),
    ) -> anyhow::Result<(NetSvcMessage, usize)> {
        let events = reader.read::<u16>(9)?;
        let length = reader.read::<u32>(20)?;

        let mut descriptors = Vec::with_capacity(length as usize);
        let mut descriptor_size = 0;
        for _ in 0..events {
            let (desc, size) = GameEventDescription::bitread(reader)?;
            descriptor_size += size;
            descriptors.push(desc);
        }

        Ok((
            NetSvcMessage::SvcGameEventList(SSvcGameEventList {
                events, descriptors,
            }),
            9 + 20 + descriptor_size,
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameEventDescription {
    event_id: u16,
    name: String,
    keys: Vec<(String, EventDescriptorType)>,
}

impl GameEventDescription {
    pub(crate) fn bitread(reader: &mut (impl BitRead + BitReadExt)) -> anyhow::Result<(GameEventDescription, usize)> {
        let event_id = reader.read::<u16>(9)?;
        let name = reader.read_nts()?;
        let namesize = name.len() + 1;
        let mut keys = Vec::new();
        let mut keysize = 0;
        loop {
            let kind = reader.read::<u8>(3)?;
            keysize += 3;
            if kind == 0 { break }
            let str = reader.read_nts()?;
            keysize += (str.len()+1)*8;
            keys.push((str, kind.into()));
        }
        Ok((GameEventDescription {
            event_id, name, keys,
        }, 9 + namesize * 8 + keysize))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventDescriptorType {
    String,
    Float,
    Int32,
    Int16,
    Int8,
    Bool,
    UInt64,
    Unknown(u8),
}

impl From<u8> for EventDescriptorType {
    fn from(n: u8) -> EventDescriptorType {
        use EventDescriptorType::*;
        match n {
            1 => String,
            2 => Float,
            3 => Int32,
            4 => Int16,
            5 => Int8,
            6 => Bool,
            7 => UInt64,
            _ => Unknown(n),
        }
    }
}
