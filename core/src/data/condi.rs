use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::AsRefStr;

/// Condition.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRefStr,
    IntoPrimitive,
    TryFromPrimitive,
)]
#[repr(u32)]
pub enum Condition {
    Blind = 720,
    Crippled = 721,
    Chilled = 722,
    Poison = 723,
    Immobile = 727,
    Bleeding = 736,
    Burning = 737,
    Vulnerability = 738,
    Weakness = 742,
    Fear = 791,
    Confusion = 861,
    Torment = 19426,
    Slow = 26766,
    Taunt = 27705,
}
