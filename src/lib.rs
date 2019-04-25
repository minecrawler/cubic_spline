mod from_raw;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod test;

///! Interpolation methods for computation of cubic spline points
///! within the range of a discrete set of known points.

/// Collection for calculate spline points
pub struct Spline();

impl Spline {
  /// Calculates flat vector of point from known points
  /// # Example
  /// ```
  /// use cubic_spline::Spline;
  ///
  /// static TENSION: f64 = 0.5;
  /// static NUM_OF_SEGMENTS: u32 = 16;
  ///
  /// let points = vec![10.0, 200.0, 256.0, 390.0, 512.0, 10.0, 778.0, 200.0];
  ///
  /// let spline_points = Spline::from_flatten_points(&points, TENSION, NUM_OF_SEGMENTS);
  ///
  /// assert_eq!(spline_points.len(), 102);
  /// ```
  pub fn from_flatten_points(points: &[f64], tension: f64, num_of_segments: u32) -> Vec<f64> {
    from_raw::get_curve_points(points, tension, num_of_segments)
  }

  // pub fn points_from_tuples(
  //   points: &[(f64, f64)],
  //   tension: f64,
  //   num_of_segments: u32,
  // ) -> Vec<(f64, f64)> {
  //   // from_raw::get_curve_points(points, tension, num_of_segments)
  //   Vec::new()
  // }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn getCurvePoints(pts: Vec<f64>, tension: f64, num_of_segments: u32) -> Vec<f64> {
  let points = vec![10.0, 200.0, 256.0, 390.0, 512.0, 10.0, 778.0, 200.0];
  Spline::points_from_raw(&pts, tension, num_of_segments)
}