// use half::f16;
// use nalgebra::{Vector3, Vector4, UnitQuaternion, Quaternion};
use crate::*;

use std::{
    cmp::{self},
    path::Path,
};

#[derive(Debug)]
pub enum SplatFormats {
    SplatA,
    SplatB,
    SplatC,
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
