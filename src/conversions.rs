use crate::*;
use nalgebra::{Matrix3, Quaternion, UnitQuaternion, Vector3, Vector4};

impl From<&SplatB> for SplatC {
    fn from(s: &SplatB) -> Self {
        let color = Vector4::new(
            s.color[0] as f32 / 255.0,
            s.color[1] as f32 / 255.0,
            s.color[2] as f32 / 255.0,
            s.color[3] as f32 / 255.0,
        );
        let rotation = UnitQuaternion::from_quaternion(Quaternion::new(
            (s.rotation[1] as f32 / 255.0) - 0.5,
            (s.rotation[2] as f32 / 255.0) - 0.5,
            (s.rotation[3] as f32 / 255.0) - 0.5,
            (s.rotation[0] as f32 / 255.0) - 0.5,
        ))
        .to_rotation_matrix();
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
