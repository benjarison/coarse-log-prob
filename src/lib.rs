use std::convert::{From, Into};
use std::cmp::{PartialOrd, Ordering};

///
/// Represents a log probability using "half" precision, which is backed by a u16 value. Log
/// probabilities span the range [-87.33655, 0], where the lower bound is taken from the value
/// `f32::MIN_POSITIVE.ln()`. This representation of log probabilities requires half the
/// amount of storage of a single precision f32 value, and is useful in cases where low precision
/// can be tolerated.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoarseLogProb(u16);

impl CoarseLogProb {

    ///
    /// The smallest representable log probability, which is approximately equal to the value
    /// ```f32::MIN_POSITIVE.ln()```
    ///
    pub const MIN: CoarseLogProb = CoarseLogProb(u16::MAX);

    ///
    /// Represents the unity probability value (1 in real space, 0 in log space)
    ///
    pub const UNITY: CoarseLogProb = CoarseLogProb(0);

    // Minimum value represented as an f32 value
    const MIN_FLOAT_VAL: f32 = -87.33655f32;
    // Unit of increment for log probability
    const INCREMENT: f32 = 0.0013326703;
    // Inverse of minimum float value
    const INV_MFV: f32 = -0.01144996;
}

impl From<f32> for CoarseLogProb {

    ///
    /// Converts an f32 value into a `CoarseLogProb`. Note that values greater than zero will
    /// automatically get mapped to `CoarseLogProb::UNITY`, and values less than
    /// `CoarseLogProb::MIN` will automatically get mapped to `CoarseLogProb::MIN`.
    ///
    /// # Arguments
    ///
    /// * `value` - an f32 value to be converted to a `CoarseLogProb`
    ///
    fn from(value: f32) -> CoarseLogProb {
        if value < CoarseLogProb::MIN_FLOAT_VAL {
            CoarseLogProb::MIN
        } else if value >= 0f32 {
            CoarseLogProb::UNITY
        } else {
            let int = (value * CoarseLogProb::INV_MFV * u16::MAX as f32).round() as u16;
            CoarseLogProb(int)
        }
    }
}

impl Into<f32> for CoarseLogProb {

    ///
    /// Converts a `CoarseLogProb` into an f32 value
    ///
    fn into(self) -> f32 {
        0f32 - (self.0 as f32 * CoarseLogProb::INCREMENT)
    }
}

impl PartialOrd for CoarseLogProb {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 < other.0 {
            Some(Ordering::Greater)
        } else if self.0 > other.0 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CoarseLogProb;

    #[test]
    fn test_from_f32() {
        assert_eq!(CoarseLogProb::from(0f32), CoarseLogProb(0));
        assert_eq!(CoarseLogProb::from(-10.0f32), CoarseLogProb(7504));
        assert_eq!(CoarseLogProb::from(-87.33655f32), CoarseLogProb(65535));
        assert_eq!(CoarseLogProb::from(-100f32), CoarseLogProb(65535));
    }
}