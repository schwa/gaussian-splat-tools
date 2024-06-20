# gaussian-splat-tools

## Installation

```sh
cargo install --git https://github.com/schwa/gaussian-splat-tools
```

## Usage

### Detect the type of a splat file

```sh
gaussian-splat-tools guess-format -i train.splat
```

### Convert splat file from one format to another

```sh
gaussian-splat-tools convert -i train.splat -o train.splatc
```

## Splat Formats

### Splat A

The splat format defined by original paper and stored in a `.ply` file. The `.ply` header looks like this:

```plaintext
ply
format binary_little_endian 1.0
element vertex 3
property float x
property float y
property float z
property float nx
property float ny
property float nz
property float f_dc_0
property float f_dc_1
property float f_dc_2
property float f_rest_0
...
property float f_rest_44
property float opacity
property float scale_0
property float scale_1
property float scale_2
property float rot_0
property float rot_1
property float rot_2
property float rot_3
end_header
```

### Splat B

The splat format used by `.splat` files. A rust definition of this format is:

```rust
struct SplatB {
    position: Vector3<f32>,
    scale: Vector3<f32>,
    color: Vector4<u8>,
    rotation: Vector4<u8>, // Quaternion stored w, x, y, z.
}
```

### Splat C

The splat format defined as the in-memory representation with [MetalSplat](https://github.com/scier/MetalSplatter) project. I'm using `.splatc` as the file extension for this format. A rust definition of this format is:

```rust
pub struct SplatC {
    pub position: Vector3<f16>,
    pub color: Vector4<f16>,
    pub cov_a: Vector3<f16>,
    pub cov_b: Vector3<f16>,
}
```

## License

MIT
