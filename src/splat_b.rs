use crate::*;
use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use derive_new::new as New;
use nalgebra::{Quaternion, Vector3, Vector4};
use std::fs::File;
use std::io::Write;
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
}

impl SplatFormat for SplatB {

    fn is_format(path: &Path) -> FormatResult {
        let extension = path.extension().unwrap().to_str().unwrap();
        if extension != "splat" {
            return FormatResult::No("Extension is not splat".to_string());
        }
        if !path.exists() {
            return FormatResult::Maybe(Some(0.333));
        }
        let size = std::fs::metadata(path).unwrap().len();
        if size % 32 as u64 == 0 {
            FormatResult::Maybe(Some(0.666))
        } else {
            FormatResult::No(format!(
                "Size is not a multiple of 32"

            ))
        }
    }

    fn load(path: &Path) -> Result<Vec<UberSplat>> {
        let data = std::fs::read(path)?;
        let chunk_size = 32;
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
            .map(UberSplat::from)
            .collect();
        Ok(splats)
    }

    fn save(splats: &[UberSplat], path: &Path) -> Result<()> {
        let mut file = File::create(path)?;
        for splat in splats {
            let splat: SplatB = splat.to_owned().into();
            let bytes: &[u8] = bytemuck::bytes_of(&splat);
            file.write_all(bytes)?;
        }
        Ok(())
    }
}

impl From<SplatB> for UberSplat {
    fn from(splat: SplatB) -> Self {
        // TODO: SO UGLY
        let color = splat.color.xyz();
        let color = Vector3::new(
            color.x as f32 / 255.0,
            color.y as f32 / 255.0,
            color.z as f32 / 255.0,
        );
        let color = srgb_to_linear(color);
        let color = Vector3::new(
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
        );
        let color = Color::LinearU8(color);
        let opacity = Opacity::LinearU8(splat.color.w);
        let scale = Scale::LinearFloat(splat.scale);
        let rotation = Quaternion::new(
            splat.rotation[1] as f32 / 255.0 - 0.5,
            splat.rotation[2] as f32 / 255.0 - 0.5,
            splat.rotation[3] as f32 / 255.0 - 0.5,
            splat.rotation[0] as f32 / 255.0 - 0.5,
        );
        UberSplat::new(splat.position, None, color, opacity, scale, rotation)
    }
}

impl From<UberSplat> for SplatB {
    fn from(uber_splat: UberSplat) -> Self {
        let rgb = uber_splat.color.to_linear_u8();
        let alpha = uber_splat.opacity.to_linear_u8();
        let color = Vector4::new(rgb.x, rgb.y, rgb.z, alpha);
        let scale = uber_splat.scale.to_linear_float();
        let imag = uber_splat
            .rotation
            .imag()
            .iter()
            .map(|v| ((v + 0.5) * 255.0) as u8)
            .collect::<Vec<u8>>();
        let real = (uber_splat.rotation.scalar() + 0.5 * 255.0) as u8;
        let rotation = Vector4::new(real, imag[0], imag[1], imag[2]);
        SplatB::new(uber_splat.position, scale, color, rotation)
    }
}
