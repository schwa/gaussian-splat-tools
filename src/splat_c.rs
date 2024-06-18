use crate::splat_format::*;
use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use derive_new::new as New;
use half::f16;
use nalgebra::{Vector3, Vector4};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[repr(C, packed)]
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
    type BinarySplat = SplatC;

    fn definition() -> SplatDefinition {
        SplatDefinition::new(vec![
            Property::new("position".to_string(), false, Storage::Half3),
            Property::new("color".to_string(), false, Storage::Half4),
            Property::new("cov_a".to_string(), false, Storage::Half3),
            Property::new("cov_b".to_string(), false, Storage::Half3),
        ])
    }

    fn is_format(path: &Path) -> FormatResult {
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

        fn load(_path: &Path) -> Result<Vec<SplatC>> {
        panic!("Not implemented");
    }

    fn save(splats: &[SplatC], path: &Path) -> Result<()> {
        // Open the file in write mode
        let mut file = File::create(path)?;

        // Iterate over each splat and write it to the file
        for splat in splats {
            let bytes: &[u8] = bytemuck::bytes_of(splat);
            file.write_all(bytes)?;
        }

        Ok(())
    }
}
