use crate::*;

#[derive(PartialEq, Clone, Debug)]
pub struct SUserCmd {
    cmd: i32,
    command_number: Option<i32>,
    tick_count: Option<i32>,
    view_angles: [Option<f32>; 3],
    forward_move: Option<f32>,
    side_move: Option<f32>,
    up_move: Option<f32>,
    buttons: Option<i32>,
    impulse: Option<i8>,
    weapon_select: Option<u16>,
    weapon_subtype: Option<u8>,
    mouse_dx: Option<i16>,
    mouse_dy: Option<i16>,
}

impl SUserCmd {
    pub(crate) fn from_br(br: &mut impl BitRead) -> anyhow::Result<Self> {
        let cmd = br.read_signed::<i32>(32)?;

        let padding = br.read::<u32>(32)? * 8;

        let command_number = br.read_option_signed::<i32>(32)?;
        let tick_count = br.read_option_signed::<i32>(32)?;

        let view_angles_x = br.read_option::<u32>(32)?.map(|n| f32::from_bits(n));
        let view_angles_y = br.read_option::<u32>(32)?.map(|n| f32::from_bits(n));
        let view_angles_z = br.read_option::<u32>(32)?.map(|n| f32::from_bits(n));

        let view_angles = [view_angles_x, view_angles_y, view_angles_z];

        let forward_move = br.read_option::<u32>(32)?.map(|n| f32::from_bits(n));
        let side_move = br.read_option::<u32>(32)?.map(|n| f32::from_bits(n));
        let up_move = br.read_option::<u32>(32)?.map(|n| f32::from_bits(n));

        let buttons = br.read_option::<i32>(32)?;
        let impulse = br.read_option::<i8>(8)?;

        let weapon_select = br.read_option::<u16>(11)?;
        let weapon_subtype = if weapon_select.is_some() {
            Some(br.read::<u8>(6)?)
        } else {
            None
        };

        let mouse_dx = br.read_option_signed::<i16>(16)?;
        let mouse_dy = br.read_option_signed::<i16>(16)?;

        br.byte_align();

        Ok(SUserCmd {
            cmd,
            command_number,
            tick_count,
            view_angles,
            forward_move,
            side_move,
            up_move,
            buttons,
            impulse,
            weapon_select,
            weapon_subtype,
            mouse_dx,
            mouse_dy,
        })
    }
}
