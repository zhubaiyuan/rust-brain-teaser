use std::convert::TryFrom;

struct ZeroToTen(i32);

impl TryFrom<i32> for ZeroToTen {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 || value > 10 {
            Err("Value must be between 0 and 10")
        } else {
            Ok(Self(value))
        }
    }
}
