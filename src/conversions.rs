use crate::*;
use nalgebra::{Matrix3, UnitQuaternion, Vector3, Vector4};

impl From<&SplatB> for SplatBAlt {
    fn from(value: &SplatB) -> Self {
        SplatBAlt {
            position: value.position,
            scale: value.scale,
            color: value.color,
            rotation: quaternion_from_u8(value.rotation),
        }
    }
}

impl From<&SplatB> for SplatC {
    fn from(s: &SplatB) -> Self {
        let splat_b_alt = SplatBAlt::from(s);
        SplatC::from(&splat_b_alt)
    }
}

impl From<&SplatBAlt> for SplatC {
    fn from(s: &SplatBAlt) -> Self {
        let (color, alpha) = color_and_alpha_from_u8(s.color);
        let color = srgb_to_linear(color);
        let color = Vector4::new(color.x, color.y, color.z, alpha);

        let rotation = UnitQuaternion::from_quaternion(s.rotation).to_rotation_matrix();
        let scale = Matrix3::from_diagonal(&s.scale);
        let transform = rotation * scale;
        let cov3d = transform * transform.transpose();
        let cov_a = Vector3::new(cov3d[(0, 0)], cov3d[(0, 1)], cov3d[(0, 2)]);
        let cov_b = Vector3::new(cov3d[(1, 1)], cov3d[(1, 2)], cov3d[(2, 2)]);

        SplatC {
            position: vector3_from_f32(s.position),
            color: vector4_from_f32(color),
            cov_a: vector3_from_f32(cov_a),
            cov_b: vector3_from_f32(cov_b),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;
    use half::f16;
    use nalgebra::{Quaternion, Vector3, Vector4};

    #[test]
    fn test_splat_b_to_splat_c_conversion() {
        let splat_b_alt = SplatBAlt::new(
            Vector3::new(1.0320082, 6.9903774, 0.9209764),
            Vector3::new(0.3529712, 0.1467299, 0.48865178),
            Vector4::new(255, 249, 235, 252),
            Quaternion::from_parts(
                -0.67954236,
                Vector3::new(0.28900078, 0.28900078, -0.6092449),
            ),
        );

        let splat_c = SplatC::from(&splat_b_alt);
        assert_eq!(
            splat_c.position,
            Vector3::new(
                f16::from_f32(1.0322266),
                f16::from_f32(6.9921875),
                f16::from_f32(0.92089844)
            )
        );
        assert_eq!(
            splat_c.color,
            Vector4::new(
                f16::from_f32(1.0),
                f16::from_f32(0.94873047),
                f16::from_f32(0.8354492),
                f16::from_f32(0.98828125)
            )
        );
        assert_eq!(
            splat_c.cov_a,
            Vector3::new(
                f16::from_f32(0.14294434),
                f16::from_f32(0.0027160645),
                f16::from_f32(-0.10736084)
            )
        );
        assert_eq!(
            splat_c.cov_b,
            Vector3::new(
                f16::from_f32(0.12390137),
                f16::from_f32(0.010047913),
                f16::from_f32(0.11804199)
            )
        );
    }
}
