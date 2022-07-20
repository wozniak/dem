use crate::*;
use messages::*;

#[derive(Debug, Clone)]
pub struct Frame {
    pub tick: Option<i32>,
    pub slot: Option<u8>,
    pub data: FrameData,
}

#[derive(PartialEq, Clone, Debug)]
pub enum FrameData {
    SignOn(SPacket),
    Packet(SPacket),
    SyncTick,
    ConsoleCmd(String),
    UserCmd(SUserCmd),
    DataTables(SDataTables),
    Stop,
    CustomData(Vec<u8>),
    StringTables(SStringTables),
}

impl Frame {
    pub(crate) fn from_br(br: &mut impl BitRead, header: &Header) -> anyhow::Result<Self> {
        let msg = br.read_signed::<i8>(8)?;

        let tick = if msg == 7 {
            None
        } else {
            Some(br.read_signed::<i32>(32)?)
        };

        let slot = if msg == 7 || header.is_oe() {
            None
        } else {
            Some(br.read::<u8>(8)?)
        };

        let data = match msg {
            1 => FrameData::SignOn(SPacket::from_br(br, header)?),
            2 => FrameData::Packet(SPacket::from_br(br, header)?),
            3 => FrameData::SyncTick,
            4 => FrameData::ConsoleCmd(parse_concmd(br)?),
            5 => FrameData::UserCmd(SUserCmd::from_br(br)?),
            6 => FrameData::DataTables(SDataTables::from_br(br)?),
            7 => FrameData::Stop,
            8 if !header.is_oe() => FrameData::CustomData(read_custom_data(br)?),
            8 | 9 => FrameData::StringTables(SStringTables::from_br(br)?),
            _ => return Err(anyhow::anyhow!("invalid message type {}", msg)),
        };

        return Ok(Self { tick, slot, data });
    }

    pub fn as_signon(&self) -> Option<&SPacket> {
        if let FrameData::SignOn(packet) = &self.data {
            Some(packet)
        } else {
            None
        }
    }

    pub fn as_packet(&self) -> Option<&SPacket> {
        if let FrameData::Packet(packet) = &self.data {
            Some(packet)
        } else {
            None
        }
    }
}

fn read_custom_data(br: &mut impl BitRead) -> anyhow::Result<Vec<u8>> {
    br.skip(32)?;
    let size = br.read::<i32>(32)?;
    br.skip(dbg!(size) as u32 * 8)?;
    Ok(vec![])
    // Ok(br.read_to_vec(size as usize)?)
}
