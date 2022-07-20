use crate::NetSvcMessage;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SSvcClassInfo {
    num_classes: u16,
    create_on_client: bool,
    server_classes: Option<Vec<ServerClass>>,
}

impl SSvcClassInfo {
    pub(crate) fn bitread(
        reader: &mut (impl BitRead + BitReadExt),
    ) -> anyhow::Result<(NetSvcMessage, usize)> {
        let mut table_bits = 0;
        let num_classes = reader.read::<u16>(16)?;
        let create_on_client = reader.read_bit()?;
        let server_classes = if !create_on_client {
            let mut vec = Vec::with_capacity(num_classes as usize);
            for _ in 0..num_classes {
                let id = reader.read::<u16>(16)?;
                let name = reader.read_nts()?;
                let data_table = reader.read_nts()?;
                table_bits += 16 + (name.len() + 1)*8 + (data_table.len() + 1)*8;
                vec.push(ServerClass {
                    id, name, data_table
                })
            }
            Some(vec)
        } else {
            None
        };

        Ok((
            NetSvcMessage::SvcClassInfo(SSvcClassInfo {
                num_classes, create_on_client, server_classes,
            }),
            16 + 1 + table_bits,
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServerClass {
    id: u16,
    name: String,
    data_table: String,
}
