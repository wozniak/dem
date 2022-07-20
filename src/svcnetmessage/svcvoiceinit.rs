use crate::NetSvcMessage;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SSvcVoiceInit {
    codec: ArrayString<256>,
    quality: u8,
}

impl SSvcVoiceInit {
    pub(crate) fn bitread(
        reader: &mut (impl BitRead + BitReadExt),
    ) -> anyhow::Result<(NetSvcMessage, usize)> {
        let codec = reader.read_nts_arraystring::<256>()?;
        let quality = reader.read::<u8>(8)?;

        let mut skipped = 0;
        if quality == 255 {
            skipped = 32;
            reader.skip(32)?;
        }

        Ok((
            NetSvcMessage::SvcVoiceInit(SSvcVoiceInit {
                codec, quality,
            }),
            (codec.len() + 1)*8 + 8 + skipped,
        ))
    }
}
