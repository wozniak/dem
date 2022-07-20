use crate::*;

pub(crate) fn parse_concmd(br: &mut impl BitRead) -> anyhow::Result<String> {
    let size = br.read::<u32>(32)?;
    let str = br.read_nts()?;
    br.skip((size - str.len() as u32 - 1) * 8)?;
    Ok(str)
}
