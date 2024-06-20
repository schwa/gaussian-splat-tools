use crate::*;
use anyhow::Result;
use derive_new::new as New;
use std::path::Path;

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
    fn definition() -> SplatDefinition;

    fn is_format(path: &Path) -> FormatResult;
    fn load(path: &Path) -> Result<Vec<UberSplat>>;
    fn save(splats: &[UberSplat], path: &Path) -> Result<()>;
}

pub fn load_splats(path: &Path) -> Result<Vec<UberSplat>> {
    let format = guess_format(path).unwrap();
    match format {
        SplatFormats::SplatA => SplatA::load(path),
        SplatFormats::SplatB => SplatB::load(path),
        SplatFormats::SplatC => SplatC::load(path),
    }
}

pub fn save_splats(splats: Vec<UberSplat>, format: SplatFormats, path: &Path) -> Result<()> {
    match format {
        SplatFormats::SplatA => SplatA::save(&splats, path),
        SplatFormats::SplatB => SplatB::save(&splats, path),
        SplatFormats::SplatC => SplatC::save(&splats, path),
    }
}
