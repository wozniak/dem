const MAX_PATH: usize = 260;

use crate::*;
use Game::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Game {
    HL2OldEngine,
    Portal3420,
    SourceUnpack,
    Source2013,
    L4D1005,
    L4D1040,
    L4D2000,
    Portal2,
    L4D2012,
    L4D2027,
    L4D2042,
    L4D2203,
    Unknown,
}

impl Game {
    pub fn is_l4d(&self) -> bool {
        [
            L4D1005, L4D1040, L4D2000, L4D2012, L4D2027, L4D2042, L4D2203,
        ]
        .contains(self)
    }

    pub fn is_orangebox(&self) -> bool {
        [HL2OldEngine, Portal3420, SourceUnpack, Source2013].contains(self)
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub stamp: ArrayString<8>,
    pub demo_protocol: i32,
    pub network_protocol: i32,
    pub server_name: ArrayString<MAX_PATH>,
    pub client_name: ArrayString<MAX_PATH>,
    pub map_name: ArrayString<MAX_PATH>,
    pub game_directory: ArrayString<MAX_PATH>,
    pub time: f32,
    pub ticks: i32,
    pub frames: i32,
    pub signon_len: i32,
    // info values
    pub game: Game,
    pub mssc: u8,
}

impl Header {
    pub(crate) fn from_br<R: Read, E: Endianness>(
        br: &mut BitReader<R, E>,
    ) -> anyhow::Result<Self> {
        let stamp = br.read_fixed_arraystr_notrim::<8>()?;
        let demo_protocol = br.read_signed::<i32>(32)?;
        let network_protocol = br.read_signed::<i32>(32)?;
        let server_name = br.read_fixed_arraystr::<MAX_PATH>()?;
        let client_name = br.read_fixed_arraystr::<MAX_PATH>()?;
        let map_name = br.read_fixed_arraystr::<MAX_PATH>()?;
        let game_directory = br.read_fixed_arraystr::<MAX_PATH>()?;
        let time = br.read_float()?;
        let ticks = br.read_signed::<i32>(32)?;
        let frames = br.read_signed::<i32>(32)?;
        let signon_len = br.read::<i32>(32)?;

        let game = match (demo_protocol, network_protocol) {
            (3, 7) => HL2OldEngine,
            (3, 14) => Portal3420,
            (3, 15) => SourceUnpack,
            (3, 24) => Source2013,
            (4, 37) => L4D1005,
            (4, 1040) => L4D1040,
            (4, 2000) => L4D2000,
            (4, 2001) => Portal2,
            (4, 2012) => L4D2012,
            (4, 2027) => L4D2027,
            (4, 2042) => L4D2042,
            (4, 2100) => L4D2203,
            _ => Unknown,
        };

        let mssc;

        if game.is_orangebox() {
            mssc = 1;
        } else if game == Portal2 || (game == Unknown && demo_protocol == 4) {
            mssc = 2;
        } else if game.is_l4d() {
            mssc = 4;
        } else {
            mssc = 1;
        }

        Ok(Header {
            stamp,
            demo_protocol,
            network_protocol,
            server_name,
            client_name,
            map_name,
            game_directory,
            time,
            ticks,
            frames,
            signon_len,
            game,
            mssc,
        })
    }

    pub fn is_valid(&self) -> bool {
        self.stamp.as_str() == "HL2DEMO\0"
    }

    pub fn is_oe(&self) -> bool {
        self.demo_protocol < 4
    }
}
