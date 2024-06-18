// use half::f16;
// use nalgebra::{Vector3, Vector4, UnitQuaternion, Quaternion};
use crate::*;
use anyhow::Result;
use derive_new::new as New;
use std::{
    cmp::{self},
    path::Path,
};

#[derive(Debug, Clone, New)]
pub struct SplatDefinition {
    pub properties: Vec<Property>,
}

impl SplatDefinition {
    pub fn size(&self) -> usize {
        self.properties.iter().map(|p| p.size()).sum()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, New)]
pub struct Property {
    pub name: String,
    pub optional: bool,
    pub storage: Storage,
}

impl Property {
    pub fn size(&self) -> usize {
        match self.storage {
            Storage::Char => 1,
            Storage::Char2 => 2,
            Storage::Char3 => 3,
            Storage::Char4 => 4,
            Storage::Float => 4,
            Storage::Float2 => 8,
            Storage::Half2 => 4,
            Storage::Float3 => 12,
            Storage::Half3 => 6,
            Storage::Float4 => 16,
            Storage::Half4 => 8,
            Storage::HalfQuaternion => 8,
            Storage::Quaternion => 16,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, New)]
pub enum Storage {
    Char,
    Char2,
    Char3,
    Char4,
    Float,
    Float2,
    Half2,
    Float3,
    Half3,
    Float4,
    Half4,
    HalfQuaternion,
    Quaternion,
}

#[allow(dead_code)]
#[derive(Debug, Clone, New, PartialEq, PartialOrd)]
pub enum FormatResult {
    No(String),
    Maybe(Option<f32>),
    Yes,
}

pub trait SplatFormat {
    type BinarySplat;

    fn definition() -> SplatDefinition;

    fn is_format(path: &Path) -> FormatResult;
    fn load(path: &Path) -> Result<Vec<Self::BinarySplat>>;
    fn save(splats: &[Self::BinarySplat], path: &Path) -> Result<()>;
}

#[derive(Debug)]
pub enum SplatFormats {
    SplatB,
    SplatC,
}

pub fn guess_format(path: &Path) -> Option<SplatFormats> {
    let mut ordered_results = vec![
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
