use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Morton encoding helpers
// ---------------------------------------------------------------------------

/// Spread the bits of a `u32` into alternating positions of a `u64`.
/// Bit i of `x` goes to bit 2i of the result.
#[inline]
fn spread_bits(x: u32) -> u64 {
    let mut x = x as u64;
    x = (x | (x << 16)) & 0x0000_FFFF_0000_FFFF;
    x = (x | (x << 8)) & 0x00FF_00FF_00FF_00FF;
    x = (x | (x << 4)) & 0x0F0F_0F0F_0F0F_0F0F;
    x = (x | (x << 2)) & 0x3333_3333_3333_3333;
    x = (x | (x << 1)) & 0x5555_5555_5555_5555;
    x
}

/// Compact alternating bits of a `u64` back into a `u32`.
/// Inverse of `spread_bits`.
#[inline]
fn compact_bits(mut x: u64) -> u32 {
    x &= 0x5555_5555_5555_5555;
    x = (x | (x >> 1)) & 0x3333_3333_3333_3333;
    x = (x | (x >> 2)) & 0x0F0F_0F0F_0F0F_0F0F;
    x = (x | (x >> 4)) & 0x00FF_00FF_00FF_00FF;
    x = (x | (x >> 8)) & 0x0000_FFFF_0000_FFFF;
    x = (x | (x >> 16)) & 0x0000_0000_FFFF_FFFF;
    x as u32
}

/// Interleave bits of `x` and `y` to produce a Morton (Z-order) code.
/// Bits of `x` occupy even positions; bits of `y` occupy odd positions.
#[inline]
pub fn morton_encode(x: u32, y: u32) -> u64 {
    spread_bits(x) | (spread_bits(y) << 1)
}

/// Decode a Morton code back into its `(x, y)` components.
#[inline]
pub fn morton_decode(key: u64) -> (u32, u32) {
    (compact_bits(key), compact_bits(key >> 1))
}

// ---------------------------------------------------------------------------
// SpcKey
// ---------------------------------------------------------------------------

/// A Morton-encoded 2D spatial key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SpcKey(u64);

impl SpcKey {
    /// Construct from a raw Morton-encoded value.
    #[inline]
    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Encode `(x, y)` as a Morton key.
    #[inline]
    pub fn from_xy(x: u32, y: u32) -> Self {
        Self(morton_encode(x, y))
    }

    /// Decode this key back to `(x, y)`.
    #[inline]
    pub fn to_xy(self) -> (u32, u32) {
        morton_decode(self.0)
    }

    /// The raw `u64` Morton value.
    #[inline]
    pub fn raw(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for SpcKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y) = self.to_xy();
        write!(f, "SpcKey({}, {})", x, y)
    }
}

// ---------------------------------------------------------------------------
// SpcRegion
// ---------------------------------------------------------------------------

/// An axis-aligned bounding box in 2D SPC space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpcRegion {
    pub min_x: u32,
    pub min_y: u32,
    pub max_x: u32,
    pub max_y: u32,
}

impl SpcRegion {
    /// Create a new region. Panics in debug mode if `min > max` on either axis.
    pub fn new(min_x: u32, min_y: u32, max_x: u32, max_y: u32) -> Self {
        debug_assert!(min_x <= max_x, "min_x must be <= max_x");
        debug_assert!(min_y <= max_y, "min_y must be <= max_y");
        Self { min_x, min_y, max_x, max_y }
    }

    /// Test whether a [`SpcKey`] falls within this region (inclusive on all sides).
    pub fn contains_key(&self, key: SpcKey) -> bool {
        let (x, y) = key.to_xy();
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    /// Return the `(min_key, max_key)` Morton-encoded corners of this region.
    ///
    /// **Note:** Morton ranges are not contiguous — a range scan must also
    /// apply `contains_key` to filter keys outside the true rectangular region.
    pub fn key_range(&self) -> (SpcKey, SpcKey) {
        (
            SpcKey::from_xy(self.min_x, self.min_y),
            SpcKey::from_xy(self.max_x, self.max_y),
        )
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn morton_round_trip() {
        for (x, y) in [(0, 0), (1, 0), (0, 1), (255, 255), (u32::MAX, u32::MAX)] {
            assert_eq!(morton_decode(morton_encode(x, y)), (x, y));
        }
    }

    #[test]
    fn spc_key_round_trip() {
        let key = SpcKey::from_xy(42, 99);
        assert_eq!(key.to_xy(), (42, 99));
    }

    #[test]
    fn spc_region_contains() {
        let region = SpcRegion::new(0, 0, 10, 10);
        assert!(region.contains_key(SpcKey::from_xy(5, 5)));
        assert!(region.contains_key(SpcKey::from_xy(0, 0)));
        assert!(region.contains_key(SpcKey::from_xy(10, 10)));
        assert!(!region.contains_key(SpcKey::from_xy(11, 5)));
        assert!(!region.contains_key(SpcKey::from_xy(5, 11)));
    }

    #[test]
    fn spc_region_key_range_corners() {
        let region = SpcRegion::new(1, 2, 3, 4);
        let (lo, hi) = region.key_range();
        assert_eq!(lo.to_xy(), (1, 2));
        assert_eq!(hi.to_xy(), (3, 4));
    }
}
