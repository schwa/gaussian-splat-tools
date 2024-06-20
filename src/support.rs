use byteorder::{LittleEndian, ReadBytesExt};
use half::f16;
use nalgebra::{Quaternion, Vector3, Vector4};
use std::io::{self, Read};

#[allow(dead_code)]
pub fn read_vector3_f16<R: Read>(reader: &mut R) -> io::Result<Vector3<f16>> {
    Ok(Vector3::new(
        f16::from_bits(reader.read_u16::<LittleEndian>().unwrap()),
        f16::from_bits(reader.read_u16::<LittleEndian>().unwrap()),
        f16::from_bits(reader.read_u16::<LittleEndian>().unwrap()),
    ))
}

#[allow(dead_code)]
pub fn read_vector4_f16<R: Read>(reader: &mut R) -> io::Result<Vector4<f16>> {
    Ok(Vector4::new(
        f16::from_bits(reader.read_u16::<LittleEndian>().unwrap()),
        f16::from_bits(reader.read_u16::<LittleEndian>().unwrap()),
        f16::from_bits(reader.read_u16::<LittleEndian>().unwrap()),
        f16::from_bits(reader.read_u16::<LittleEndian>().unwrap()),
    ))
}

pub fn read_vector3_f32<R: Read>(reader: &mut R) -> io::Result<Vector3<f32>> {
    Ok(Vector3::new(
        reader.read_f32::<LittleEndian>()?,
        reader.read_f32::<LittleEndian>()?,
        reader.read_f32::<LittleEndian>()?,
    ))
}

pub fn read_vector4_u8<R: Read>(reader: &mut R) -> io::Result<Vector4<u8>> {
    Ok(Vector4::new(
        reader.read_u8()?,
        reader.read_u8()?,
        reader.read_u8()?,
        reader.read_u8()?,
    ))
}

#[allow(dead_code)]
pub fn vector3_from_f32(v: Vector3<f32>) -> Vector3<f16> {
    Vector3::new(f16::from_f32(v.x), f16::from_f32(v.y), f16::from_f32(v.z))
}

#[allow(dead_code)]
pub fn vector3_from_f16(v: Vector3<f16>) -> Vector3<f32> {
    Vector3::new(v.x.to_f32(), v.y.to_f32(), v.z.to_f32())
}

#[allow(dead_code)]
pub fn vector4_from_f32(v: Vector4<f32>) -> Vector4<f16> {
    Vector4::new(
        f16::from_f32(v.x),
        f16::from_f32(v.y),
        f16::from_f32(v.z),
        f16::from_f32(v.w),
    )
}

#[allow(dead_code)]
pub fn vector4_from_f16(v: Vector4<f16>) -> Vector4<f32> {
    Vector4::new(v.x.to_f32(), v.y.to_f32(), v.z.to_f32(), v.w.to_f32())
}

#[allow(dead_code)]
pub fn quaternion_from_u8(rotation: Vector4<u8>) -> Quaternion<f32> {
    Quaternion::new(
        (rotation[1] as f32 / 255.0) - 0.5,
        (rotation[2] as f32 / 255.0) - 0.5,
        (rotation[3] as f32 / 255.0) - 0.5,
        (rotation[0] as f32 / 255.0) - 0.5,
    )
}

#[allow(dead_code)]
pub fn color_and_alpha_from_u8(color: Vector4<u8>) -> (Vector3<f32>, f32) {
    let alpha = color[3] as f32 / 255.0;
    let color = Vector3::new(
        color[0] as f32 / 255.0,
        color[1] as f32 / 255.0,
        color[2] as f32 / 255.0,
    );
    (color, alpha)
}

#[allow(dead_code)]
pub fn srgb_to_linear(rgb: Vector3<f32>) -> Vector3<f32> {
    rgb.map(|v| v.powf(2.2))
}
