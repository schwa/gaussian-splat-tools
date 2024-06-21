use derive_new::new as New;
use nalgebra::{Matrix3, Quaternion, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use tabled::Tabled;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Color {
    SphericalHarmonic(f32, f32, f32, [f32; 15]),
    FirstOrderSphericalHarmonic(Vector3<f32>),
    LinearFloat(Vector3<f32>),
    LinearU8(Vector3<u8>),
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Color::SphericalHarmonic(_, _, _, _) => write!(f, "SphericalHarmonic"),
            Color::FirstOrderSphericalHarmonic(_) => write!(f, "FirstOrderSphericalHarmonic"),
            Color::LinearFloat(v) => write!(f, "LinearFloat({})", v),
            Color::LinearU8(v) => write!(f, "LinearU8({})", v),
        }
    }
}

impl Color {
    pub fn to_linear_float(&self) -> Vector3<f32> {
        match self {
            Color::SphericalHarmonic(_, _, _, _) => panic!("SphericalHarmonic is not supported"),
            Color::FirstOrderSphericalHarmonic(_) => {
                panic!("FirstOrderSphericalHarmonic is not supported")
            }
            Color::LinearFloat(v) => *v,
            Color::LinearU8(uint8) => Vector3::new(
                uint8[0] as f32 / 255.0,
                uint8[1] as f32 / 255.0,
                uint8[2] as f32 / 255.0,
            ),
        }
    }

    pub fn to_linear_u8(&self) -> Vector3<u8> {
        match self {
            Color::SphericalHarmonic(_, _, _, _) => panic!("SphericalHarmonic is not supported"),
            Color::FirstOrderSphericalHarmonic(_) => {
                panic!("FirstOrderSphericalHarmonic is not supported")
            }
            Color::LinearFloat(float) => Vector3::new(
                (float[0] * 255.0) as u8,
                (float[1] * 255.0) as u8,
                (float[2] * 255.0) as u8,
            ),
            Color::LinearU8(uint8) => *uint8,
        }
    }
}

// MARK: -

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Opacity {
    LinearFloat(f32),
    LinearU8(u8),
    LogitFloat(f32),
}

impl Display for Opacity {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Opacity::LinearFloat(float) => write!(f, "LinearFloat({})", float),
            Opacity::LinearU8(uint8) => write!(f, "LinearU8({})", uint8),
            Opacity::LogitFloat(float) => write!(f, "LogitFloat({})", float),
        }
    }
}

impl Opacity {
    #[allow(dead_code)]
    pub fn to_linear_float(&self) -> f32 {
        match self {
            Opacity::LinearFloat(float) => *float,
            Opacity::LinearU8(uint8) => *uint8 as f32 / 255.0,
            Opacity::LogitFloat(_) => panic!("LogitFloat is not supported"),
        }
    }

    pub fn to_linear_u8(&self) -> u8 {
        match self {
            Opacity::LinearFloat(float) => (*float * 255.0) as u8,
            Opacity::LinearU8(uint8) => *uint8,
            Opacity::LogitFloat(_) => panic!("LogitFloat is not supported"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Scale {
    Exponent(Vector3<f32>),
    LinearFloat(Vector3<f32>),
}

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Scale::Exponent(v) => write!(f, "Exponent({})", v),
            Scale::LinearFloat(v) => write!(f, "LinearFloat({})", v),
        }
    }
}

impl Scale {
    pub fn to_linear_float(&self) -> Vector3<f32> {
        match self {
            Scale::Exponent(_) => panic!("Not supported"),
            Scale::LinearFloat(v) => *v,
        }
    }
}

// MARK: -

#[derive(Debug, Clone, New, PartialEq, Serialize, Deserialize, Tabled)]
#[tabled(rename_all = "CamelCase")]

pub struct UberSplat {
    pub position: Vector3<f32>,
    #[tabled(display_with = "display_option")]
    pub normal: Option<Vector3<f32>>,
    pub color: Color,
    pub opacity: Opacity,
    pub scale: Scale,
    pub rotation: Quaternion<f32>,
}

impl UberSplat {
    pub fn to_cov(&self) -> (Vector3<f32>, Vector3<f32>) {
        let rotation = UnitQuaternion::from_quaternion(self.rotation).to_rotation_matrix();
        let scale = Matrix3::from_diagonal(&self.scale.to_linear_float());
        let transform = rotation * scale;
        let cov3d = transform * transform.transpose();
        let cov_a = Vector3::new(cov3d[(0, 0)], cov3d[(0, 1)], cov3d[(0, 2)]);
        let cov_b = Vector3::new(cov3d[(1, 1)], cov3d[(1, 2)], cov3d[(2, 2)]);
        (cov_a, cov_b)
    }
}

fn display_option(o: &Option<Vector3<f32>>) -> String {
    match o {
        Some(s) => format!("{}", s),
        None => "-".to_string(),
    }
}
