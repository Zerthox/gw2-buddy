use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::AsRefStr;

/// Applied buff.
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
pub enum Buff {
    #[strum(serialize = "Quick")]
    Quickness = 1187,

    #[strum(serialize = "Alac")]
    Alacrity = 30328,

    #[strum(serialize = "Arc Power")]
    ArcanePower = 5582,

    #[strum(serialize = "Spider")]
    SpiderVenom = 13036,

    #[strum(serialize = "Skale")]
    SkaleVenom = 13054,

    #[strum(serialize = "Devourer")]
    DevourerVenom = 13094,

    #[strum(serialize = "Basi")]
    BasiliskVenom = 13133,

    #[strum(serialize = "Soul Stone")]
    SouleStoneVenom = 49038,

    #[strum(serialize = "Dwarf")]
    #[num_enum(alternatives = [33330])]
    RiteOfTheGreatDwarf = 26596,

    #[strum(serialize = "AoJ")]
    AshesOfTheJust = 41957,

    #[strum(serialize = "Bear")]
    BearStance = 40045,

    #[strum(serialize = "Dolyak")]
    DolyakStance = 41815,

    #[strum(serialize = "Vulture")]
    VultureStance = 44651,

    #[strum(serialize = "Moa")]
    MoaStance = 45038,

    #[strum(serialize = "Griffon")]
    GriffonStance = 46280,

    #[strum(serialize = "OWP")]
    OneWolfPack = 44139,
}
