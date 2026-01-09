# LVR2-Rust

A Rust implementation of the LVR2 (Las Vegas Reconstruction 2) library for surface reconstruction from point cloud data.

```
         /\
        /  \               ##          ##      ##    #######         ######
       /    \              ##          ##      ##    ##     ##     ##      ##
      /      \             ##           ##    ##     ##      ##            ##
     /________\            ##           ##    ##     ##     ##            ##
    /\        /\           ##            ##  ##      #######             ##
   /  \      /  \          ##            ##  ##      ##    ##          ##
  /    \    /    \         ##             ####       ##     ##       ##
 /      \  /      \        ##########      ##        ##      ##    ##########
/________\/________\
```

## Overview

This is a Rust port of the LVR2 library, providing tools for:

- Point cloud loading and processing (PLY, PTS, XYZ formats)
- Surface normal estimation
- Marching Cubes surface reconstruction
- Mesh optimization and smoothing
- PLY mesh export

## Building

```bash
cd rust_lvr2
cargo build --release
```

## Usage

### Command Line Tool

```bash
# Basic reconstruction
cargo run --release --bin lvr2_reconstruct -- input.pts

# With custom options
cargo run --release --bin lvr2_reconstruct -- input.pts -o output.ply --voxelsize 5.0 --kn 15
```

### As a Library

```rust
use lvr2::prelude::*;
use lvr2::reconstruction::{ReconstructionOptions, reconstruct};
use lvr2::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load point cloud
    let points = io::load_points("scan.pts")?;
    
    // Configure reconstruction
    let options = ReconstructionOptions {
        voxel_size: 10.0,
        kn: 10,
        ki: 10,
        kd: 5,
        ..Default::default()
    };
    
    // Reconstruct surface
    let mesh = reconstruct(&points, &options)?;
    
    // Save result
    io::save_mesh("output.ply", &mesh)?;
    
    println!("Reconstructed {} vertices, {} faces", 
             mesh.num_vertices(), mesh.num_faces());
    
    Ok(())
}
```

## Module Structure

- `geometry` - Geometric primitives (vectors, normals, bounding boxes, matrices)
- `types` - Core data structures (PointBuffer, MeshBuffer, Channel)
- `io` - File I/O (PLY, PTS/XYZ formats)
- `reconstruction` - Surface reconstruction algorithms (Marching Cubes, normal estimation)
- `algorithm` - Mesh processing (smoothing, simplification)
- `util` - Utilities (progress bars, timing)

## Dependencies

- `nalgebra` - Linear algebra
- `kiddo` - KD-tree for nearest neighbor search
- `rayon` - Parallel processing
- `ply-rs` - PLY file format support
- `clap` - Command line argument parsing
- `thiserror` - Error handling
- `log` / `env_logger` - Logging

## Status

This is an initial Rust port focusing on core functionality:

- [x] Basic geometry types (Vec3, Normal, BoundingBox, Matrix4, Plane)
- [x] Point buffer with attributes
- [x] Mesh buffer with attributes
- [x] PLY file reading/writing
- [x] PTS/XYZ file reading
- [x] KD-tree based spatial search
- [x] Normal estimation (PCA)
- [x] Hash grid for spatial hashing
- [x] Marching Cubes reconstruction
- [x] Laplacian mesh smoothing
- [ ] Planar Marching Cubes (PMC)
- [ ] Dual Marching Cubes (DMC)
- [ ] Hole filling
- [ ] Region growing
- [ ] Mesh simplification (QEM)
- [ ] Texture generation
- [ ] HDF5 support
- [ ] CUDA acceleration

## Comparison with C++ Version

The C++ LVR2 library is a comprehensive toolkit with ~1000 source files. This Rust port focuses on the core reconstruction pipeline:

| Feature | C++ LVR2 | Rust LVR2 |
|---------|----------|-----------|
| Point cloud I/O | PLY, PTS, XYZ, H5 | PLY, PTS, XYZ |
| Mesh I/O | PLY, OBJ, STL | PLY |
| Reconstruction | MC, PMC, DMC, MT | MC |
| GPU Support | CUDA | - |
| Visualization | Qt Viewer | - |

## License

BSD 3-Clause License (same as original LVR2)

## Citation

If you use this library, please cite the original LVR2 paper:

```bibtex
@inproceedings{wiemann2018,
  author={Wiemann, Thomas and Mitschke, Isaak and Mock, Alexander and Hertzberg, Joachim},
  booktitle={2018 Second IEEE International Conference on Robotic Computing (IRC)}, 
  title={{Surface Reconstruction from Arbitrarily Large Point Clouds}}, 
  year={2018},
  pages={278-281},
  doi={10.1109/IRC.2018.00059}
}
```
