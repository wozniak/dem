use crate::NetSvcMessage;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SNetTick {
    tick: i32,
    host_frametime: f32,
    host_frametime_stddev: f32,
}

impl SNetTick {
    pub(crate) fn bitread(
        reader: &mut (impl BitRead + BitReadExt),
    ) -> anyhow::Result<(NetSvcMessage, usize)> {
        let tick = reader.read_signed::<i32>(32)?;
        let host_frametime = reader.read_signed::<i16>(16)? as f32 / 10e5f32;
        let host_frametime_stddev = reader.read_signed::<i16>(16)? as f32 / 10e5f32;

        Ok((
            NetSvcMessage::NetTick(SNetTick {
                tick,
                host_frametime,
                host_frametime_stddev,
            }),
            32 + 16 + 16,
        ))
    }
}
