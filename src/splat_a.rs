use crate::SplatFormat;
use crate::*;
use anyhow::Result;
use derive_new::new as New;
use nalgebra::Quaternion;
use nalgebra::{Vector3, Vector4};
use ply::ply::*;
use ply_rs as ply;
use ply_rs::parser;
use std::path::Path;

#[derive(Debug, Default, New)]
pub struct SplatA {
    pub position: Vector3<f32>,       // 3 elements
    pub normal: Option<Vector3<f32>>, // 3 elements (optional)
    pub f_dc: Vector3<f32>,           // 3 elements
    pub f_rest: Option<Vec<f32>>,     // 45 elements (optional)
    pub opacity: f32,                 // 1 element
    pub scale: Vector3<f32>,          // 3 elements
    pub rot: Vector4<f32>,            // 4 elements
}

impl SplatFormat for SplatA {
    fn is_format(path: &Path) -> FormatResult {
        if path.extension().unwrap() == "ply" {
            // TODO: too simple
            let f = std::fs::File::open(path).unwrap();
            let mut f = std::io::BufReader::new(f);
            let splat_parser = parser::Parser::<SplatA>::new();
            let header = splat_parser.read_header(&mut f).unwrap();
            if SplatA::is_splat_a(&header) {
                FormatResult::Yes
            } else {
                FormatResult::No("Not a SplatA file".to_string())
            }
        } else {
            FormatResult::No("Not a PLY file".to_string())
        }
    }
    fn load(path: &Path) -> Result<Vec<UberSplat>> {
        let f = std::fs::File::open(path).unwrap();
        let mut f = std::io::BufReader::new(f);
        let splat_parser = parser::Parser::<SplatA>::new();
        let header = splat_parser.read_header(&mut f).unwrap();

        // Depending on the header, read the data into our structs..
        let mut splat_list = Vec::new();
        for (_ignore_key, element) in &header.elements {
            // we could also just parse them in sequence, but the file format might change
            match element.name.as_ref() {
                "vertex" => {
                    splat_list = splat_parser
                        .read_payload_for_element(&mut f, element, &header)
                        .unwrap();
                }
                _ => panic!("Enexpeced element!"),
            }
        }

        let splats: Vec<UberSplat> = splat_list.iter().map(|splat| splat.into()).collect();
        Ok(splats)
    }
    fn save(_: &[UberSplat], _: &Path) -> Result<()> {
        panic!("Not implemented");
    }
}

impl SplatA {
    pub fn is_splat_a(header: &Header) -> bool {
        if header.elements.len() != 1 {
            println!("header.elements.len() != 1");
            return false;
        }
        let Some(element) = header.elements.get("vertex") else {
            println!("header.elements.get(\"vertex\")");
            return false;
        };
        let required_properties = vec![
            "x", "y", "z", "f_dc_0", "f_dc_1", "f_dc_2", "opacity", "scale_0", "scale_1",
            "scale_2", "rot_0", "rot_1", "rot_2", "rot_3",
        ];
        for name in required_properties {
            let Some(property) = element.properties.get(name) else {
                println!("element.properties.get(\"{}\")", name);
                return false;
            };
            if property.data_type != PropertyType::Scalar(ScalarType::Float) {
                println!("property.data_type != PropertyType::Scalar(ScalarType::Float)");
                return false;
            }
        }
        let normal_properties = ["nx", "ny", "nz"];
        let has_normals = normal_properties.iter().all(|name| {
            let property = element.properties.get(name.to_owned());
            property.is_some()
                && property.unwrap().data_type == PropertyType::Scalar(ScalarType::Float)
        });
        if !has_normals {
            println!("!has_normals");
            return false;
        }
        let higher_order_spherical_harmonics_names = (0..45)
            .map(|i| format!("f_rest_{}", i))
            .collect::<Vec<String>>();
        let has_higher_order_spherical_harmonics =
            higher_order_spherical_harmonics_names.iter().all(|name| {
                let property = element.properties.get(name);
                property.is_some()
                    && property.unwrap().data_type == PropertyType::Scalar(ScalarType::Float)
            });
        if !has_higher_order_spherical_harmonics {
            panic!("!has_higher_order_spherical_harmonics");
        }
        true
    }
}

impl ply_rs::ply::PropertyAccess for SplatA {
    fn new() -> Self {
        SplatA::default()
    }
    fn set_property(&mut self, key: String, property: ply_rs::ply::Property) {
        fn update_vector(v: Option<Vector3<f32>>, index: usize, value: f32) -> Vector3<f32> {
            match v {
                Some(mut v) => {
                    v[index] = value;
                    v
                }
                None => {
                    let mut v = Vector3::new(0.0, 0.0, 0.0);
                    v[index] = value;
                    v
                }
            }
        }

        fn update_array(v: Option<Vec<f32>>, _index: usize, value: f32) -> Vec<f32> {
            match v {
                Some(mut v) => {
                    v.push(value);
                    v
                }
                None => {
                    vec![value]
                }
            }
        }

        match (key.as_ref(), property) {
            ("x", ply_rs::ply::Property::Float(v)) => self.position.x = v,
            ("y", ply_rs::ply::Property::Float(v)) => self.position.y = v,
            ("z", ply_rs::ply::Property::Float(v)) => self.position.z = v,
            ("f_dc_0", ply_rs::ply::Property::Float(v)) => self.f_dc.x = v,
            ("f_dc_1", ply_rs::ply::Property::Float(v)) => self.f_dc.y = v,
            ("f_dc_2", ply_rs::ply::Property::Float(v)) => self.f_dc.z = v,
            ("opacity", ply_rs::ply::Property::Float(v)) => self.opacity = v,
            ("scale_0", ply_rs::ply::Property::Float(v)) => self.scale.x = v,
            ("scale_1", ply_rs::ply::Property::Float(v)) => self.scale.y = v,
            ("scale_2", ply_rs::ply::Property::Float(v)) => self.scale.z = v,
            ("rot_0", ply_rs::ply::Property::Float(v)) => self.rot.x = v,
            ("rot_1", ply_rs::ply::Property::Float(v)) => self.rot.y = v,
            ("rot_2", ply_rs::ply::Property::Float(v)) => self.rot.z = v,
            ("rot_3", ply_rs::ply::Property::Float(v)) => self.rot.w = v,
            ("nx", ply_rs::ply::Property::Float(v)) => {
                self.normal = Some(update_vector(self.normal, 0, v))
            }
            ("ny", ply_rs::ply::Property::Float(v)) => {
                self.normal = Some(update_vector(self.normal, 1, v))
            }
            ("nz", ply_rs::ply::Property::Float(v)) => {
                self.normal = Some(update_vector(self.normal, 2, v))
            }
            (name, ply_rs::ply::Property::Float(v)) => {
                if name.starts_with("f_rest_") {
                    let Some(index) = name.strip_prefix("f_rest_") else {
                        panic!("Vertex: f_rest index out of bounds: {}", name);
                    };
                    let index = index.parse::<usize>().unwrap();
                    if index >= 45 {
                        panic!("Vertex: f_rest index out of bounds: {}", index);
                    }
                    self.f_rest = Some(update_array(self.f_rest.clone(), index, v));
                }
            }

            // (_, _) => println!("Vertex: Unexpected key/value combination: key: {}", key),
            (k, _) => panic!("Vertex: Unexpected key/value combination: key: {}", k),
        }
    }
}

// SplatScenePoint(position: SIMD3<Float>(2.0078542, 0.08104886, 1.3285047), normal: Optional(SIMD3<Float>(0.0, 0.0, 0.0)), color: SplatIO.SplatScenePoint.Color.sphericalHarmonic(1.1078603, 0.76136017, 0.06554606, [-0.0057055317, -0.007687452, -0.0043917783, -0.0037030454, -0.010418807, 0.06328091, 0.0073155826, -0.0065571507, 0.012642727, 0.006464842, -0.022115033, -0.015164005, 0.03302637, -0.023272093, -0.016852863, 0.006827223, 0.0045320853, -0.005715641, 0.017861754, -0.01044909, 0.0427432, 0.011729195, -0.0037403249, 0.009896112, -0.008849159, -0.00833527, -0.028211955, 0.03504748, 0.014180188, 0.011458171, 0.024590509, 0.01905671, -0.016029052, 0.02723091, -0.025627526, 0.03138132, 0.031709813, 0.0045662196, 0.018008206, -0.022949746, 0.006410549, -0.033924486, 0.029564247, 0.038887653, 0.028849693]), opacity: SplatIO.SplatScenePoint.Opacity.logitFloat(0.84990764), scale: SplatIO.SplatScenePoint.Scale.exponent(-3.8865843, -5.659209, -4.7688828), rotation: simd_quatf(real: 1.0344028, imag: SIMD3<Float>(-0.19919053, -0.10477345, -0.014542822)))

impl From<&SplatA> for UberSplat {
    fn from(splat: &SplatA) -> Self {
        // TODO: FIXME this is all totally invented.
        let color = Color::SphericalHarmonic(splat.f_dc, splat.f_rest.clone().unwrap());
        let opacity = Opacity::LogitFloat(splat.opacity);
        let scale = Scale::Exponent(splat.scale);
        let rotation = Quaternion::new(splat.rot.w, splat.rot.x, splat.rot.y, splat.rot.z);
        UberSplat::new(
            splat.position,
            splat.normal,
            color,
            opacity,
            scale,
            rotation,
        )
    }
}
