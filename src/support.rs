use byteorder::{LittleEndian, ReadBytesExt};
use half::f16;
use nalgebra::{Vector3, Vector4};
use std::io::{self, Read};

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

pub fn vector3_from_f32(v: Vector3<f32>) -> Vector3<f16> {
    Vector3::new(f16::from_f32(v.x), f16::from_f32(v.y), f16::from_f32(v.z))
}

pub fn vector4_from_f32(v: Vector4<f32>) -> Vector4<f16> {
    Vector4::new(
        f16::from_f32(v.x),
        f16::from_f32(v.y),
        f16::from_f32(v.z),
        f16::from_f32(v.w),
    )
}
