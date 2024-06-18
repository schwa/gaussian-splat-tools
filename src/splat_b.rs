use crate::splat_format::*;
use crate::support::*;
use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use derive_new::new as New;
use nalgebra::{Vector3, Vector4};
use std::path::Path;

#[repr(C)]
#[derive(Debug, Clone, New, Copy)]
pub struct SplatB {
    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub color: Vector4<u8>,
    pub rotation: Vector4<u8>, // Quaternion stored w, x, y, z.
}

unsafe impl Zeroable for SplatB {}
unsafe impl Pod for SplatB {}

#[cfg(test)]
#[test]
fn test_splat_b_size() {
    assert_eq!(std::mem::size_of::<SplatB>(), 32);
    assert_eq!(SplatB::definition().size(), 32);
}

impl SplatFormat for SplatB {
    type BinarySplat = SplatB;

    fn definition() -> SplatDefinition {
        SplatDefinition::new(vec![
            Property::new("position".to_string(), false, Storage::Float3),
            Property::new("scale".to_string(), false, Storage::Float3),
            Property::new("color".to_string(), false, Storage::Char4),
            Property::new("rotation".to_string(), false, Storage::Char4),
        ])
    }

    fn is_format(path: &Path) -> FormatResult {
        let size = std::fs::metadata(path).unwrap().len();
        if size % SplatB::definition().size() as u64 == 0 {
            FormatResult::Maybe(Some(0.666))
        } else {
            FormatResult::No(format!(
                "Size is not a multiple of {}",
                SplatB::definition().size()
            ))
        }
    }

    fn load(path: &Path) -> Result<Vec<SplatB>> {
        SplatB::load_fast(path)
    }

    fn save(_splats: &[SplatB], _path: &Path) -> Result<()> {
        panic!("Not implemented");
    }
}

impl SplatB {
    fn load_fast(path: &Path) -> Result<Vec<SplatB>> {
        let data = std::fs::read(path)?;
        let chunk_size = SplatB::definition().size();
        let splats = data
            .chunks_exact(chunk_size)
            .map(|chunk| {
                let mut reader = std::io::Cursor::new(chunk);
                SplatB {
                    position: read_vector3_f32(&mut reader).unwrap(),
                    scale: read_vector3_f32(&mut reader).unwrap(),
                    color: read_vector4_u8(&mut reader).unwrap(),
                    rotation: read_vector4_u8(&mut reader).unwrap(),
                }
            })
            .collect();
        Ok(splats)
    }
}