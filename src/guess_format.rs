use crate::*;
use strum::EnumIter;

use std::{
    cmp::{self},
    path::Path,
};

#[derive(Debug, EnumIter)]
pub enum SplatFormats {
    SplatA,
    SplatB,
    SplatC,
}

impl SplatFormats {
    pub fn description(&self) -> &'static str {
        match self {
            SplatFormats::SplatA => "The original `.ply` based splat format as defined by - <https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/> - with optional normals and spherical harmonics. XXX bytes pers splat.",
            SplatFormats::SplatB => "`.splat` format as used by antimatter15's splat viewer: <https://github.com/antimatter15/splat>. 32 bytes per splat.",
            SplatFormats::SplatC => "`.splatc` half float format as defined by Sean Cier's MetalSplatter <https://github.com/scier/MetalSplatter> project. 26 bytes per splat.",
        }
    }
}

pub fn guess_format(path: &Path) -> Option<SplatFormats> {
    let mut ordered_results = vec![
        (SplatFormats::SplatA, SplatA::is_format(path)),
        (SplatFormats::SplatB, SplatB::is_format(path)),
        (SplatFormats::SplatC, SplatC::is_format(path)),
    ];

    ordered_results.sort_by(|a, b| match (&a.1, &b.1) {
        (FormatResult::Yes, FormatResult::Yes) => cmp::Ordering::Equal,
        (FormatResult::Yes, _) => cmp::Ordering::Less,
        (_, FormatResult::Yes) => cmp::Ordering::Greater,
        (FormatResult::Maybe(a), FormatResult::Maybe(b)) => a.partial_cmp(b).unwrap(),
        (FormatResult::Maybe(_), _) => cmp::Ordering::Less,
        (_, FormatResult::Maybe(_)) => cmp::Ordering::Greater,
        (FormatResult::No(a), FormatResult::No(b)) => a.cmp(b),
    });

    for (format, result) in ordered_results {
        match result {
            FormatResult::Yes => return Some(format),
            FormatResult::Maybe(_) => return Some(format),
            _ => (),
        }
    }
    None
}
