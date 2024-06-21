use crate::*;
use anyhow::Result;
use derive_new::new as New;
use std::path::Path;

#[derive(Debug, Clone, New, PartialEq, PartialOrd)]
pub enum FormatResult {
    No(String),
    Maybe(Option<f32>),
    Yes,
}

pub trait SplatFormat {
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
