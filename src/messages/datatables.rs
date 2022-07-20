use crate::*;

#[derive(PartialEq, Clone, Debug)]
pub struct SDataTables {}

impl SDataTables {
    pub(crate) fn from_br(br: &mut impl BitRead) -> anyhow::Result<Self> {
        let size = br.read::<u32>(32)?;
        br.skip(size * 8)?;
        Ok(SDataTables {})
    }
}
