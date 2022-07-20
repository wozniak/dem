use crate::NetSvcMessage;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct S {
}

impl S {
    pub(crate) fn bitread(
        reader: &mut (impl BitRead + BitReadExt),
    ) -> anyhow::Result<(NetSvcMessage, usize)> {

        Ok((
            NetSvcMessage::(S {
            }),
            size,
        ))
    }
}
