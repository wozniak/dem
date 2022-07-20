use crate::NetSvcMessage;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SSvcServerInfo {
    pub protocol: u16,
    pub server_count: u32,
    pub is_hltv: bool,
    pub is_dedicated: bool,
    pub client_crc: i32,
    pub string_table_crc: Option<u32>,
    pub max_classes: u16,
    pub map_crc: u32,
    pub player_count: u8,
    pub max_clients: u8,
    pub tick_interval: f32,
    pub platform: char,
    pub game_dir: ArrayString<260>,
    pub map_name: ArrayString<260>,
    pub sky_name: ArrayString<260>,
    pub host_name: ArrayString<260>,
}

impl SSvcServerInfo {
    pub(crate) fn bitread(
        reader: &mut (impl BitRead + BitReadExt),
        header: &Header,
    ) -> anyhow::Result<(NetSvcMessage, usize)> {
        let mut extrabits = 0;

        let protocol = reader.read::<u16>(16)?;
        let server_count = reader.read::<u32>(32)?;
        let is_hltv = reader.read_bit()?;
        let is_dedicated = reader.read_bit()?;
        let client_crc = reader.read::<i32>(32)?;

        let string_table_crc = if !header.is_oe() {
            extrabits += 32;
            Some(reader.read::<u32>(32)?)
        } else {
            None
        };

        let max_classes = reader.read::<u16>(16)?;
        let map_crc = reader.read::<u32>(32)?;
        let player_count = reader.read::<u8>(8)?;
        let max_clients = reader.read::<u8>(8)?;
        let tick_interval = reader.read_float()?;
        let platform = reader.read::<u8>(8)? as char;
        let game_dir = reader.read_nts_arraystring::<260>()?;
        let map_name = reader.read_nts_arraystring::<260>()?;
        let sky_name = reader.read_nts_arraystring::<260>()?;
        let host_name = reader.read_nts_arraystring::<260>()?;

        Ok((
            NetSvcMessage::SvcServerInfo(SSvcServerInfo {
                protocol,
                server_count,
                is_hltv,
                is_dedicated,
                client_crc,
                string_table_crc,
                max_classes,
                map_crc,
                player_count,
                max_clients,
                tick_interval,
                platform,
                game_dir,
                map_name,
                sky_name,
                host_name,
            }),
            16 + 32
                + 1
                + 1
                + 32
                + 16
                + 32
                + 8
                + 8
                + 32
                + 8
                + (game_dir.len() + map_name.len() + sky_name.len() + host_name.len() + 4) * 8
                + extrabits,
        ))
    }
}
