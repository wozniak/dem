use crate::NetSvcMessage;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SNetFile {
    transfer_id: i32,
    file_name: ArrayString<260>,
    requested: bool,
}

impl SNetFile {
    pub(crate) fn bitread(
        reader: &mut (impl BitRead + BitReadExt),
    ) -> anyhow::Result<(NetSvcMessage, usize)> {
        let transfer_id = reader.read_signed::<i32>(32)?;
        let file_name = reader.read_nts_arraystring::<260>()?;
        let requested = reader.read_bit()?;
        Ok((
            NetSvcMessage::NetFile(SNetFile {
                transfer_id,
                file_name,
                requested,
            }),
            32 + (file_name.len() + 1) * 8 + 1,
        ))
    }
}
