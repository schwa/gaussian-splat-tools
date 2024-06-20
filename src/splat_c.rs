use crate::*;
use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use core::panic;
use derive_new::new as New;
use half::f16;
use nalgebra::{Vector3, Vector4};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[repr(C)]
#[derive(Debug, Clone, New, Copy)]
pub struct SplatC {
    pub position: Vector3<f16>,
    pub color: Vector4<f16>,
    pub cov_a: Vector3<f16>,
    pub cov_b: Vector3<f16>,
}

unsafe impl Zeroable for SplatC {}
unsafe impl Pod for SplatC {}

#[cfg(test)]
#[test]
fn test_splat_c_size() {
    assert_eq!(std::mem::size_of::<SplatC>(), 26);
    assert_eq!(SplatC::definition().size(), 26);
}

impl SplatFormat for SplatC {
    fn definition() -> SplatDefinition {
        SplatDefinition::new(vec![
            Property::new("position".to_string(), false, Storage::Half3),
            Property::new("color".to_string(), false, Storage::Half4),
            Property::new("cov_a".to_string(), false, Storage::Half3),
            Property::new("cov_b".to_string(), false, Storage::Half3),
        ])
    }

    fn is_format(path: &Path) -> FormatResult {
        if !path.exists() {
            let extension = path.extension().unwrap().to_str().unwrap();
            if extension == "splatc" {
                return FormatResult::Maybe(Some(0.333));
            }
            return FormatResult::No("Extension is not splatc".to_string());
        }

        let size = std::fs::metadata(path).unwrap().len();
        if size % SplatC::definition().size() as u64 == 0 {
            FormatResult::Maybe(Some(0.666))
        } else {
            FormatResult::No(format!(
                "Size is not a multiple of {}",
                SplatC::definition().size()
            ))
        }
    }

    fn load(_: &Path) -> Result<Vec<UberSplat>> {
        panic!("Not implemented");
    }

    fn save(splats: &[UberSplat], path: &Path) -> Result<()> {
        let mut file = File::create(path)?;
        for splat in splats {
            let splat: SplatC = splat.clone().into();
            let bytes: &[u8] = bytemuck::bytes_of(&splat);
            file.write_all(bytes)?;
        }
        Ok(())
    }
}

impl From<UberSplat> for SplatC {
    fn from(uber_splat: UberSplat) -> Self {
        let position = Vector3::new(
            f16::from_f32(uber_splat.position.x),
            f16::from_f32(uber_splat.position.y),
            f16::from_f32(uber_splat.position.z),
        );
        let rgb = uber_splat.color.to_linear_float();
        let alpha = uber_splat.opacity.to_linear_float();
        let color = Vector4::new(
            f16::from_f32(rgb.x),
            f16::from_f32(rgb.y),
            f16::from_f32(rgb.z),
            f16::from_f32(alpha),
        );
        let cov = uber_splat.to_cov();
        let cov_a = Vector3::new(
            f16::from_f32(cov.0.x),
            f16::from_f32(cov.0.y),
            f16::from_f32(cov.0.z),
        );
        let cov_b = Vector3::new(
            f16::from_f32(cov.1.x),
            f16::from_f32(cov.1.y),
            f16::from_f32(cov.1.z),
        );
        SplatC::new(position, color, cov_a, cov_b)
    }
}
