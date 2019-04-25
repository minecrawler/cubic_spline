# cubic_spline

[![Crates.io](https://img.shields.io/crates/v/cubic_spline.svg)](https://crates.io/crates/cubic_spline/)

Interpolation methods for computation of cubic spline points within the range of a discrete set of known points.

[Online documentation](https://docs.rs/cubic_spline/0.6.0/cubic_spline/)

##### Example
```rust
use cubic_spline::Spline;

static TENSION: f64 = 0.5;
static NUM_OF_SEGMENTS: u32 = 16;

let points = vec![10.0, 200.0, 256.0, 390.0, 512.0, 10.0, 778.0, 200.0];

let spline_points = Spline::from_flatten_points(&points, TENSION, NUM_OF_SEGMENTS);

assert_eq!(spline_points.len(), 102);
```

##### Enjoy using!

### License

This module is [MIT licensed](./LICENSE).

