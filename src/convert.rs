pub fn flatten_to_tuples(pts: &[f64]) -> Vec<(f64, f64)> {
  let len = (pts.len() / 2) + 1;
  let mut result = Vec::with_capacity(len);
  for (ind, x_or_y) in pts.iter().enumerate() {
    if ind % 2 != 0 {
      continue;
    }
    let x = *x_or_y;
    if let Some(y) = pts.get(ind + 1) {
      result.push((x, *y));
    } else {
      break;
    }
  }
  result
}

pub fn tuples_to_flatten(tuples: &[(f64, f64)]) -> Vec<f64> {
  let mut result = Vec::with_capacity(tuples.len() * 2);
  for (x, y) in tuples {
    result.push(*x);
    result.push(*y);
  }
  result
}
