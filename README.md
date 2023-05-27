# Short Formats for Short Durations

| `present(seconds: f64)` | formatted result |
| ------------- | ------------- |
| `present(0.01)` | `010ms` |
| `present(1.0)` | `1.00s` |
| `present(10.0)` | `10.0s` |
| `present(100.0)` | `1m40s` |
| `present(1000.0)` | `0h17m` |
| `present(10000.0)` | `2h47m` |
| `present(100000.0)` | `1d04h` |
| `present(1000000.0)` | `01w5d` |
| `present(10000000.0)` | `16w4d` |
| `present(100000000.0)` | `3y09w` |
| `present(1000000000.0)` | `0032y` |
| `present(10000000000.0)` | `0317y` |

## Formats & Ranges Covered

| format | smallest unit multiple | range multiple | values covered |
| ------------- | ------------- | ------------- | ------------- |
| `000ms` |          |             | up to 00.9995000000s |
| `0.00s` | 10×      |   10×       | up to 09.9950000000s |
| `00.0s` | 10×      |   10×       | up to 99.950000000s (about 1½ minutes) |
| `0m00s` | 10×      |    6×       | up to 09m59.5s |
| `0h00m` | 60×      |   60×       | up to 09h59m30s |
| `0d00h` | 60×      |   24×       | up to 9d23h30m |
| `00w0d` | 24×      |   70×       | up to 99w6d12h (about 1 year, 9 months) |
| `0y00w` |  7×      |    5.21775× | up to 9y51w4d02h54m36s |
| `0000y` | 52.1775× | 1000×       | up to 9999y26w0d14h54m36s |
| `10ky+` |          |             | values outside representation range |

As regular expressions:

| format | matcher |
| ------------- | ------------- |
| `000ms` | `([0-9][0-9][0-9])ms` |
| `0.00s` | `([0-9][.][0-9][0-9])s` |
| `00.0s` | `([0-9][0-9][.][0-9])s` |
| `0m00s` | `([0-9])m([0-9][0-9])s` |
| `0h00m` | `([0-9])h([0-9][0-9])m` |
| `0d00h` | `([0-9])d([0-9][0-9])h` |
| `00w0d` | `([0-9][0-9])w([0-9])d` |
| `0y00w` | `([0-9])y([0-9][0-9])w` |
| `0000y` | `([0-9][0-9][0-9][0-9])y` |
| `10ky+` | `10ky[+]` |

## The Rounding Rules for Years

For units longer than a second, there is some ambiguity as to their actual
duration. In UTC timekeeping, a "leap second" is occasionally
introduced into the last minute (and thus last hour and day) of a year; and
even if we set aside the leap second and adopt fixed minutes, hours and days
(the "SI Day" of 86400 seconds), there is also the matter of leap years, of
which there are 97 every 400 years.

The average Gregorian year is 365.2425 SI days, which provides a simple basis
for our calculations that is quite accurate for many purposes, and certainly
for giving a sense of time elapsed when many years have passed; but the year
is nevertheless a complex case for rounding because it is a non-integer number
of days and a non-integer number of weeks (52 weeks is 364 days).

We can ask two questions to come up with rounding rules for years:

1. When is a time between `1y51m` and `2y00w` closer to  `2y00w` ?
2. When is a time between `0019y` and `0020y` closer to  `0020y` ?

The answer to the first question helps us to set the upper limit for the
format `0y00w`, while the answer to second helps us to set the upper limit for
the format `0000y`, as well as helping us to round values:

1. From `1y51w0d` to `2y00w0d` we have 712152 seconds or `8d05h49m12s` . The
   point where we get closer to `2y00w` is 356076 seconds or `4d02h54m36s`.
2. From `0019y00w0d` to `0020y00w0d` there are 31556952 seconds. Half this
   value is 15778476 seconds, or `26w0d14h54m36s`.
