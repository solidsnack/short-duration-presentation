//! The number of seconds in a year is:
//!
//!   31556952
//!
//! Floats of 32 bits can exactly represent powers of 2 less than:
//!
//!   33554432
//!
//! In general, floats between 2ⁿ and 2ⁿ⁺¹ round to a multiple of 2ⁿ⁻²³. At
//! one year, we are rounding to even values; at 10000 years (315569520ks,
//! which is between 2³⁸ and 2³⁹ seconds), we are rounding to 32768 seconds
//! or less than 9 hours, 7 minutes.
//!
//! The overall effect is that a small number of tests require corrections of
//! 1.0 to 16.000001 seconds to get them to work, whereas the all the tests
//! pass in the 64-bit implementation.

const MINUTE: f32 = 60.0;
const HOUR: f32 = 3600.0;
const DAY: f32 = 86400.0;
const WEEK: f32 = DAY * 7.0;
const YEAR: f32 = DAY * 365.2425;
const END_OF_YEAR: f32 = YEAR - (51.0 * WEEK);

pub fn present(seconds: f32) -> String {
    let week_threshold = (99.0 * WEEK) + (6.5 * DAY);
    let years_threshold = (10.0 * YEAR) - (0.5 * END_OF_YEAR);

    match seconds {
        seconds if seconds < 0.0005 => "000ms".into(),
        seconds if seconds < 0.9995 => {
            format!("{:03.0}ms", seconds * 1000.0)
        }
        seconds if seconds < 9.995 => {
            format!("{:01.2}s", seconds)
        }
        seconds if seconds < 99.95 => {
            format!("{:02.1}s", seconds)
        }
        seconds if seconds < (9.0 * MINUTE) + 59.5 => {
            let (m, s) = div_rem_div(seconds, MINUTE, 1.0);
            format!("{:01.0}m{:02.0}s", m, s)
        }
        seconds if seconds < (9.0 * HOUR) + (59.5 * MINUTE) => {
            let (h, m) = div_rem_div(seconds, HOUR, MINUTE);
            format!("{:01.0}h{:02.0}m", h, m)
        }
        seconds if seconds < (9.0 * DAY) + (23.5 * HOUR) => {
            let (d, h) = div_rem_div(seconds, DAY, HOUR);
            format!("{:01.0}d{:02.0}h", d, h)
        }
        seconds if seconds < week_threshold => {
            let (w, d) = div_rem_div(seconds, WEEK, DAY);
            format!("{:02.0}w{:01.0}d", w, d)
        }
        seconds if seconds < years_threshold => {
            let (mut y, mut w) = div_rem_div(seconds, YEAR, WEEK);
            // Special handling for "long week" at end of year.
            if w > 51.0 {
                if YEAR - (w * WEEK) <= (END_OF_YEAR / 2.0) {
                    y += 1.0;
                    w = 0.0;
                } else {
                    w = 51.0;
                }
            }
            format!("{:01.0}y{:02.0}w", y, w)
        }
        seconds if seconds < 9999.5 * YEAR => {
            format!("{:04.0}y", seconds / YEAR)
        }
        _ => "10ky+".into(),
    }
}

fn div_rem_div(n: f32, greater: f32, lesser: f32) -> (f32, f32) {
    let div = (n / greater).floor();
    let rem_div = (n % greater) / lesser;
    (div, rem_div)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_formats() {
        assert_eq!("000ms", present(0.0));
        assert_eq!("016ms", present(0.0156));
        assert_eq!("2.07s", present(2.07));
        assert_eq!("12.3s", present(12.33));
        assert_eq!("2m32s", present(151.67));
        assert_eq!("7h07m", present((7.0 * HOUR) + (7.0 * MINUTE) + 10.0));
        assert_eq!("3d00h", present((3.0 * DAY) + 74.0));
        assert_eq!("01w4d", present(11.0 * DAY));
        assert_eq!("77w1d", present((75.0 * WEEK) + (15.0 * DAY)));
        assert_eq!("2y21w", present((2.0 * YEAR) + (147.0 * DAY)));
        assert_eq!("0082y", present(30000.0 * DAY));
    }

    #[test]
    fn test_year_boundaries() {
        // The year should round up at 51w4d02h54m36s.
        let base = (1.0 * YEAR) + (51.0 * WEEK) + (4.0 * DAY);
        let offset = (2.0 * HOUR) + (54.0 * MINUTE) + 36.0;

        assert_eq!("1y51w", present(base));

        // Correction due to 32-bit precision.
        let correction1 = 1.0;
        assert_eq!("1y51w", present(base + offset - (1.0 + correction1)));
        assert_eq!("2y00w", present(base + offset));

        let almost_ten = (9.0 * YEAR) + (51.0 * WEEK) + (4.0 * DAY);
        assert_eq!("9y51w", present(almost_ten));

        // Correction due to 32-bit precision.
        let correction2 = 16.0000009;
        let correction3 = 16.000001;
        assert_eq!("9y51w", present(almost_ten + offset + correction2));
        assert_eq!("0010y", present(almost_ten + offset + correction3));
    }

    #[test]
    fn test_transitions() {
        assert_eq!("999ms", present(0.9994));
        assert_eq!("1.00s", present(0.9995));

        assert_eq!("9.99s", present(9.9949));
        assert_eq!("10.0s", present(9.995));

        assert_eq!("99.9s", present(99.949));
        assert_eq!("1m40s", present(99.95));

        assert_eq!("9m59s", present(599.49));
        assert_eq!("0h10m", present(599.5));

        assert_eq!("9h59m", present((599.0 * MINUTE) + 29.9));
        assert_eq!("0d10h", present((599.0 * MINUTE) + 30.0));

        assert_eq!("9d23h", present((239.0 * HOUR) + 1799.0));
        assert_eq!("01w3d", present((239.0 * HOUR) + 1800.0));

        // Correction due to 32-bit precision.
        let correction4 = 3.0;
        let base = (699.0 * DAY) + (12.0 * HOUR);
        assert_eq!("99w6d", present(base - (1.0 + correction4)));
        assert_eq!("1y48w", present(base));
    }
}
