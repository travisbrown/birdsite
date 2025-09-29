const EPSILON: f64 = 1e-5;

#[derive(Debug, Clone, Copy)]
pub struct Cubic {
    values: [f64; 4],
}

impl Cubic {
    pub(super) const fn new(values: [f64; 4]) -> Self {
        Self { values }
    }

    pub(super) fn value(&self, time: f64) -> f64 {
        let [x1, y1, x2, y2] = self.values;

        // Endpoint handling (extrapolate with gradient).
        if time <= 0.0 {
            let start_gradient = if x1 > 0.0 {
                y1 / x1
            } else if y1 == 0.0 && x2 > 0.0 {
                y2 / x2
            } else {
                0.0
            };

            start_gradient * time
        } else if time >= 1.0 {
            let end_gradient = if x2 < 1.0 {
                (y2 - 1.0) / (x2 - 1.0)
            } else if x2 == 1.0 && x1 < 1.0 {
                (y1 - 1.0) / (x1 - 1.0)
            } else {
                0.0
            };

            end_gradient.mul_add(time - 1.0, 1.0)
        } else {
            // We invert `x(t)` via binary search on `time` in `[0, 1]`.
            let mut start: f64 = 0.0;
            let mut end: f64 = 1.0;
            let mut mid: f64 = 0.0;

            // Safety guard to avoid infinite looping on pathological cases.
            for _ in 0..64 {
                mid = (start + end) * 0.5;

                let x_est = calculate(x1, x2, mid);

                if (time - x_est).abs() < EPSILON {
                    return calculate(y1, y2, mid);
                }

                if x_est < time {
                    start = mid;
                } else {
                    end = mid;
                }

                if (end - start) < EPSILON {
                    break;
                }
            }

            calculate(y1, y2, mid)
        }
    }
}

/// `3 * a * (1 - m)^2 * m + 3 * b * (1 - m) * m^2 + m^3`
fn calculate(a: f64, b: f64, m: f64) -> f64 {
    let one_minus = 1.0 - m;
    (m * m).mul_add(
        m,
        (3.0 * a * one_minus * one_minus).mul_add(m, 3.0 * b * one_minus * m * m),
    )
}
