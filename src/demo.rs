use crate::*;

use bitstream_io::read::BitReader;
use bitstream_io::LE;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Demo {
    pub header: Header,
    pub frames: Vec<Frame>,
}

impl Demo {
    pub fn open<P: AsRef<Path>>(path: P) -> anyhow::Result<Demo> {
        let file = std::fs::File::open(path)?;
        let mut bitreader = BitReader::<_, LE>::new(file);

        let header = Header::from_br(&mut bitreader)?;
        if !header.is_valid() {
            return Err(anyhow::anyhow!("invalid file signature"));
        }

        let mut frames = Vec::new();

        loop {
            let frame = Frame::from_br(&mut bitreader, &header)?;
            if frame.data == FrameData::Stop {
                frames.push(frame);
                break;
            } else {
                frames.push(frame);
            }
        }

        Ok(Demo { header, frames })
    }

    pub fn tick_interval(&self) -> f32 {
        self.svc_tick_interval()
            .unwrap_or(self.header.time / self.header.ticks as f32)
    }

    fn svc_tick_interval(&self) -> Option<f32> {
        self.frames
            .iter()
            .filter_map(|f| f.as_signon())
            .map(|p| p.data.iter())
            .flatten()
            .filter_map(|p| {
                if let NetSvcMessage::SvcServerInfo(info) = p {
                    Some(info)
                } else {
                    None
                }
            })
            .next()
            .map(|info| info.tick_interval)
    }
}
