use crate::svcnetmessage::*;
use crate::*;

#[derive(PartialEq, Clone, Debug)]
pub enum NetSvcMessage {
    NetNop,
    NetDisconnect(String),
    NetFile(SNetFile),
    NetSplitScreenUser,
    NetTick(SNetTick),
    NetStringCmd,
    NetSetConVar,
    NetSignonState(SNetSignOnState),
    SvcServerInfo(SSvcServerInfo),
    SvcSendTable,
    SvcClassInfo(SSvcClassInfo),
    SvcSetPause,
    SvcCreateStringTable(SSvcCreateStringTable),
    SvcUpdateStringTable,
    SvcVoiceInit(SSvcVoiceInit),
    SvcVoiceData,
    SvcPrint,
    SvcSounds,
    SvcSetView,
    SvcFixAngle,
    SvcCrosshairAngle,
    SvcBspDecal,
    SvcSplitScreen,
    SvcUserMessage,
    SvcEntityMessage,
    SvcGameEvent,
    SvcPacketEntities,
    SvcTempEntities,
    SvcPrefetch,
    SvcMenu,
    SvcGameEventList(SSvcGameEventList),
    SvcGetCvarValue,
    SvcCmdKeyValues,
    SvcPaintmapData,
    SvcEncryptedData,
    SvcHltvReplay,
    SvcBroadcastCommand,
    NetPlayerAvatarData,
}

#[derive(PartialEq, Clone, Debug)]
#[repr(C)]
pub struct CmdInfo {
    pub flags: i32,
    pub view_origin: [f32; 3],
    pub view_angles: [f32; 3],
    pub local_view_angles: [f32; 3],
    pub view_origin_2: [f32; 3],
    pub view_angles_2: [f32; 3],
    pub local_view_angles_2: [f32; 3],
}

impl CmdInfo {
    pub(crate) fn from_br(br: &mut (impl BitRead + BitReadExt)) -> anyhow::Result<Self> {
        let bytes = br.read_to_bytes::<76>()?;
        unsafe { Ok(core::mem::transmute(bytes)) }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct SPacket {
    pub info: Vec<CmdInfo>,
    pub in_seq: i32,
    pub out_seq: i32,
    pub data: Vec<NetSvcMessage>,
}

impl SPacket {
    pub(crate) fn from_br(
        br: &mut (impl BitRead + BitReadExt),
        header: &Header,
    ) -> anyhow::Result<Self> {
        use NetSvcMessage::*;

        // one for each local player
        let mut info = Vec::with_capacity(header.mssc as usize);
        for _ in 0..header.mssc {
            info.push(CmdInfo::from_br(br)?);
        }
        let in_seq = br.read::<i32>(32)?;
        let out_seq = br.read::<i32>(32)?;

        let oe = header.is_oe();

        let mut data = Vec::new();
        let mut remaining = br.read::<u32>(32)? as usize * 8;
        loop {
            if remaining < 6 {
                br.skip(remaining as u32)?;
                break
            }
            let kind = br.read::<u8>(6)?;
            remaining -= 6;
            let (variant, size) = match kind {
                0 => (NetNop, 0),
                1 => {
                    let str = br.read_nts()?;
                    (NetDisconnect(br.read_nts()?), 8 * str.len() + 1)
                }
                2 => SNetFile::bitread(br)?,
                3 if oe => SNetTick::bitread(br)?,
                //              4 if oe => NetStringCmd,
                //              5 if oe => NetSetConVar,
                6 if oe => SNetSignOnState::bitread(br, oe)?,
                //              7 if oe => SvcPrint,
                //              3 => NetSplitScreenUser,
                4 if !oe => SNetTick::bitread(br)?,
                //              5 => NetStringCmd,
                //              6 => NetSetConVar,
                7 => SNetSignOnState::bitread(br, oe)?,
                8 => SSvcServerInfo::bitread(br, header)?,
                //              9 => SvcSendTable,
                10 => SSvcClassInfo::bitread(br)?,
                //              11 => SvcSetPause,
                12 => SSvcCreateStringTable::bitread(br, oe)?,
                //              13 => SvcUpdateStringTable,
                14 => SSvcVoiceInit::bitread(br)?,
                //              15 => SvcVoiceData,
                //              16 => SvcPrint,
                //              17 => SvcSounds,
                //              18 => SvcSetView,
                //              19 => SvcFixAngle,
                //              20 => SvcCrosshairAngle,
                //              21 => SvcBspDecal,
                //              22 => SvcSplitScreen,
                //              23 => SvcUserMessage,
                //              24 => SvcEntityMessage,
                //              25 => SvcGameEvent,
                //              26 => SvcPacketEntities,
                //              27 => SvcTempEntities,
                //              28 => SvcPrefetch,
                //              29 => SvcMenu,
                30 => SSvcGameEventList::bitread(br)?,
                //              32 => SvcGetCvarValue,
                //              33 => SvcPaintmapData,
                _ => return Err(anyhow::anyhow!("unknown net/svc msg type {}", kind)),
            };

            data.push(dbg!(variant));
            remaining -= size;
        }

        Ok(SPacket {
            info,
            in_seq,
            out_seq,
            data,
        })
    }
}
