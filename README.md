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

### Example

```sh
$ gaussian-splat-tools --help
Usage: gaussian-splat-tools [COMMAND]

Commands:
  formats       List the formats supported by this tool
  info          Get info about a gaussian splat file
  convert       Convert a gaussian splat file to another format
  guess-format  Guess the format of a gaussian splat file
  reduce        Reduce the number of splats in a gaussian splat file
  shuffle       Shuffle the splats in a gaussian splat file
  dump          Dump the splats in a gaussian splat file
  ply-to-ascii  Convert a ply file to ascii
  dump-ply      Dump a ply file
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

$ gaussian-splat-tools formats
Supported formats:
SplatA: The original `.ply` based splat format as defined by - <https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/> - with optional normals and spherical harmonics. XXX bytes pers splat.
SplatB: `.splat` format as used by antimatter15's splat viewer: <https://github.com/antimatter15/splat>. 32 bytes per splat.
SplatC: `.splatc` half float format as defined by Sean Cier's MetalSplatter <https://github.com/scier/MetalSplatter> project. 26 bytes per splat.

$ gaussian-splat-tools info -i garden.splat
Format: SplatB / `.splat` format as used by antimatter15's splat viewer: <https://github.com/antimatter15/splat>. 32 bytes per splat.
Size: 186.71 MB
# Splats: 5834784
Min position: [[-44.790844, -34.003365, -24.122707]]
Max position: [[57.472885, 9.3217745, 35.236153]]
Avg position: [[-0.33897668, 0.401791, 1.7059005]]
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
