use crate::NetSvcMessage;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SNetSignOnState {
    signon_state: SignOnState,
    spawn_count: i32,
    num_server_players: Option<u32>,
    players_network_ids: Option<Vec<u8>>,
    mapname: Option<String>,
}

impl SNetSignOnState {
    pub(crate) fn bitread(
        reader: &mut (impl BitRead + BitReadExt),
        oe: bool,
    ) -> anyhow::Result<(NetSvcMessage, usize)> {
        let signon_state = SignOnState::try_from(reader.read::<u8>(8)?)?;
        let spawn_count = reader.read_signed::<i32>(32)?;

        let mut num_server_players = None;
        let mut players_network_ids = None;
        let mut mapname = None;
        let mut nebits = 0;

        if !oe {
            num_server_players = Some(reader.read::<u32>(32)?);

            let idlen = reader.read::<u32>(32)? as usize;
            players_network_ids = Some(reader.read_to_vec(idlen)?);

            let strlen = reader.read::<u32>(32)? as usize;
            mapname = Some(String::from_utf8(reader.read_to_vec(strlen)?)?);

            nebits = strlen * 8 + 32 + idlen * 8;
        }

        Ok((
            NetSvcMessage::NetSignonState(SNetSignOnState {
                signon_state,
                spawn_count,
                num_server_players,
                players_network_ids,
                mapname,
            }),
            8 + 32 + nebits,
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SignOnState {
    None,
    Challenge,
    Connected,
    New,
    PreSpawn,
    Spawn,
    Full,
    ChangeLevel,
}

impl TryFrom<u8> for SignOnState {
    type Error = anyhow::Error;

    fn try_from(i: u8) -> anyhow::Result<Self> {
        use SignOnState::*;
        let v = match i {
            0 => None,
            1 => Challenge,
            2 => Connected,
            3 => New,
            4 => PreSpawn,
            5 => Spawn,
            6 => Full,
            7 => ChangeLevel,
            _ => return Err(anyhow::anyhow!("no such signonstate {}", i)),
        };
        Ok(v)
    }
}
