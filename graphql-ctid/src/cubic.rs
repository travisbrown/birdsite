#[derive(Debug, Clone, Copy)]
pub struct Cubic {
    /// curves = [x1, y1, x2, y2]
    pub curves: [f64; 4],
}

impl Cubic {
    pub fn new(curves: [f64; 4]) -> Self {
        Self { curves }
    }

    /// Equivalent to Python's get_value(time)
    pub fn get_value(&self, time: f64) -> f64 {
        let [x1, y1, x2, y2] = self.curves;

        // Endpoint handling (extrapolate with gradient)
        if time <= 0.0 {
            let mut start_gradient = 0.0;
            if x1 > 0.0 {
                start_gradient = y1 / x1;
            } else if y1 == 0.0 && x2 > 0.0 {
                start_gradient = y2 / x2;
            }
            return start_gradient * time;
        }

        if time >= 1.0 {
            let mut end_gradient = 0.0;
            if x2 < 1.0 {
                end_gradient = (y2 - 1.0) / (x2 - 1.0);
            } else if x2 == 1.0 && x1 < 1.0 {
                end_gradient = (y1 - 1.0) / (x1 - 1.0);
            }
            return 1.0 + end_gradient * (time - 1.0);
        }

        // Invert x(t) via binary search on t in [0,1]
        let mut start = 0.0f64;
        let mut end = 1.0f64;
        let mut mid = 0.0f64;
        const EPS: f64 = 1e-5;

        // Safety guard to avoid infinite looping on pathological cases
        for _ in 0..64 {
            mid = (start + end) * 0.5;
            let x_est = Self::calculate(x1, x2, mid);
            if (time - x_est).abs() < EPS {
                return Self::calculate(y1, y2, mid);
            }
            if x_est < time {
                start = mid;
            } else {
                end = mid;
            }
            if (end - start) < EPS {
                break;
            }
        }

        Self::calculate(y1, y2, mid)
    }

    /// Equivalent to Python's `calculate(a, b, m)`:
    /// 3*a*(1-m)^2*m + 3*b*(1-m)*m^2 + m^3
    #[inline]
    fn calculate(a: f64, b: f64, m: f64) -> f64 {
        let one_minus = 1.0 - m;
        3.0 * a * one_minus * one_minus * m + 3.0 * b * one_minus * m * m + m * m * m
    }
}
