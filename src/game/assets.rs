#[rustfmt::skip]
pub static PLAYER_SPRITE: [i32; 25] = [
    0, 0, 1, 0, 0,
    0, 1, 1, 1, 0,
    0, 1, 1, 1, 0,
    1, 1, 1, 1, 1,
    1, 1, 1, 1, 1,
];

#[rustfmt::skip]
pub static INVADER_SPRITE: [i32; 88] = [
    0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0,
    0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1,
    1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1,
    0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0,
];

#[rustfmt::skip]
pub static INVADER_2_SPRITE: [i32; 88] = [
    0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0,
    1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1,
    1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1,
    1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0,
    0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0,
];

#[rustfmt::skip]
pub static SQUID_SPRITE: [i32; 80] = [
    0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
    0, 0, 0, 1, 1, 1, 1, 0, 0, 0,
    0, 0, 1, 1, 1, 1, 1, 1, 0, 0,
    0, 1, 1, 0, 1, 1, 0, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
    0, 0, 1, 0, 1, 1, 0, 1, 0, 0,
    0, 1, 0, 1, 0, 0, 1, 0, 1, 0,
];

#[rustfmt::skip]
pub static SQUID_2_SPRITE: [i32; 80] = [
    0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
    0, 0, 0, 1, 1, 1, 1, 0, 0, 0,
    0, 0, 1, 1, 1, 1, 1, 1, 0, 0,
    0, 1, 1, 0, 1, 1, 0, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 0, 1, 0, 1, 1, 0, 1, 0, 0,
    0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
    0, 0, 1, 0, 0, 0, 0, 1, 0, 0,
];

#[rustfmt::skip]
pub static GOLIATH_SPRITE: [i32; 12*8] = [
    0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0,
    0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0,
    0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0,
];

#[rustfmt::skip]
pub static GOLIATH_2_SPRITE: [i32; 12*8] = [
    0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0,
    0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0,
    1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
];

#[rustfmt::skip]
pub static EXPLOSION_SPRITE: [i32; 13*9] = [
    0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0,
    0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0,
    1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
    0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0,
    0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0,
];
