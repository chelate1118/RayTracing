# Ray Tracing

3D graphics project written in Rust, based on ray tracing.
Load map from json file.

Supported objects:
- Plane (infinite)
- Sphere

Supported light sources:
- Sun
- Light Sphere

## Run Command

Make sure cargo is in path environment.

```console
$ cargo run --release -- [json file name]
```

## Example

![map2](https://user-images.githubusercontent.com/94269897/236137125-bfa9beca-2a60-43c1-8563-2b9545736bd9.png)

## Upcoming features

1. Denoising
2. Cube object
3. Glass material
4. CUDA GPU support
