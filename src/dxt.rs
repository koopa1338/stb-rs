#[rustfmt::skip]
static OMATCH_5: [(u8, u8); 256] = [
   ( 0,  0), ( 0,  0), ( 0,  1), ( 0,  1), ( 1,  0), ( 1,  0), ( 1,  0), ( 1,  1),
   ( 1,  1), ( 1,  1), ( 1,  2), ( 0,  4), ( 2,  1), ( 2,  1), ( 2,  1), ( 2,  2),
   ( 2,  2), ( 2,  2), ( 2,  3), ( 1,  5), ( 3,  2), ( 3,  2), ( 4,  0), ( 3,  3),
   ( 3,  3), ( 3,  3), ( 3,  4), ( 3,  4), ( 3,  4), ( 3,  5), ( 4,  3), ( 4,  3),
   ( 5,  2), ( 4,  4), ( 4,  4), ( 4,  5), ( 4,  5), ( 5,  4), ( 5,  4), ( 5,  4),
   ( 6,  3), ( 5,  5), ( 5,  5), ( 5,  6), ( 4,  8), ( 6,  5), ( 6,  5), ( 6,  5),
   ( 6,  6), ( 6,  6), ( 6,  6), ( 6,  7), ( 5,  9), ( 7,  6), ( 7,  6), ( 8,  4),
   ( 7,  7), ( 7,  7), ( 7,  7), ( 7,  8), ( 7,  8), ( 7,  8), ( 7,  9), ( 8,  7),
   ( 8,  7), ( 9,  6), ( 8,  8), ( 8,  8), ( 8,  9), ( 8,  9), ( 9,  8), ( 9,  8),
   ( 9,  8), (10,  7), ( 9,  9), ( 9,  9), ( 9, 10), ( 8, 12), (10,  9), (10,  9),
   (10,  9), (10, 10), (10, 10), (10, 10), (10, 11), ( 9, 13), (11, 10), (11, 10),
   (12,  8), (11, 11), (11, 11), (11, 11), (11, 12), (11, 12), (11, 12), (11, 13),
   (12, 11), (12, 11), (13, 10), (12, 12), (12, 12), (12, 13), (12, 13), (13, 12),
   (13, 12), (13, 12), (14, 11), (13, 13), (13, 13), (13, 14), (12, 16), (14, 13),
   (14, 13), (14, 13), (14, 14), (14, 14), (14, 14), (14, 15), (13, 17), (15, 14),
   (15, 14), (16, 12), (15, 15), (15, 15), (15, 15), (15, 16), (15, 16), (15, 16),
   (15, 17), (16, 15), (16, 15), (17, 14), (16, 16), (16, 16), (16, 17), (16, 17),
   (17, 16), (17, 16), (17, 16), (18, 15), (17, 17), (17, 17), (17, 18), (16, 20),
   (18, 17), (18, 17), (18, 17), (18, 18), (18, 18), (18, 18), (18, 19), (17, 21),
   (19, 18), (19, 18), (20, 16), (19, 19), (19, 19), (19, 19), (19, 20), (19, 20),
   (19, 20), (19, 21), (20, 19), (20, 19), (21, 18), (20, 20), (20, 20), (20, 21),
   (20, 21), (21, 20), (21, 20), (21, 20), (22, 19), (21, 21), (21, 21), (21, 22),
   (20, 24), (22, 21), (22, 21), (22, 21), (22, 22), (22, 22), (22, 22), (22, 23),
   (21, 25), (23, 22), (23, 22), (24, 20), (23, 23), (23, 23), (23, 23), (23, 24),
   (23, 24), (23, 24), (23, 25), (24, 23), (24, 23), (25, 22), (24, 24), (24, 24),
   (24, 25), (24, 25), (25, 24), (25, 24), (25, 24), (26, 23), (25, 25), (25, 25),
   (25, 26), (24, 28), (26, 25), (26, 25), (26, 25), (26, 26), (26, 26), (26, 26),
   (26, 27), (25, 29), (27, 26), (27, 26), (28, 24), (27, 27), (27, 27), (27, 27),
   (27, 28), (27, 28), (27, 28), (27, 29), (28, 27), (28, 27), (29, 26), (28, 28),
   (28, 28), (28, 29), (28, 29), (29, 28), (29, 28), (29, 28), (30, 27), (29, 29),
   (29, 29), (29, 30), (29, 30), (30, 29), (30, 29), (30, 29), (30, 30), (30, 30),
   (30, 30), (30, 31), (30, 31), (31, 30), (31, 30), (31, 30), (31, 31), (31, 31),
];

#[rustfmt::skip]
static OMATCH_6: [(u8, u8); 256] = [
   ( 0,  0), ( 0,  1), ( 1,  0), ( 1,  1), ( 1,  1), ( 1,  2), ( 2,  1), ( 2,  2),
   ( 2,  2), ( 2,  3), ( 3,  2), ( 3,  3), ( 3,  3), ( 3,  4), ( 4,  3), ( 4,  4),
   ( 4,  4), ( 4,  5), ( 5,  4), ( 5,  5), ( 5,  5), ( 5,  6), ( 6,  5), ( 6,  6),
   ( 6,  6), ( 6,  7), ( 7,  6), ( 7,  7), ( 7,  7), ( 7,  8), ( 8,  7), ( 8,  8),
   ( 8,  8), ( 8,  9), ( 9,  8), ( 9,  9), ( 9,  9), ( 9, 10), (10,  9), (10, 10),
   (10, 10), (10, 11), (11, 10), ( 8, 16), (11, 11), (11, 12), (12, 11), ( 9, 17),
   (12, 12), (12, 13), (13, 12), (11, 16), (13, 13), (13, 14), (14, 13), (12, 17),
   (14, 14), (14, 15), (15, 14), (14, 16), (15, 15), (15, 16), (16, 14), (16, 15),
   (17, 14), (16, 16), (16, 17), (17, 16), (18, 15), (17, 17), (17, 18), (18, 17),
   (20, 14), (18, 18), (18, 19), (19, 18), (21, 15), (19, 19), (19, 20), (20, 19),
   (20, 20), (20, 20), (20, 21), (21, 20), (21, 21), (21, 21), (21, 22), (22, 21),
   (22, 22), (22, 22), (22, 23), (23, 22), (23, 23), (23, 23), (23, 24), (24, 23),
   (24, 24), (24, 24), (24, 25), (25, 24), (25, 25), (25, 25), (25, 26), (26, 25),
   (26, 26), (26, 26), (26, 27), (27, 26), (24, 32), (27, 27), (27, 28), (28, 27),
   (25, 33), (28, 28), (28, 29), (29, 28), (27, 32), (29, 29), (29, 30), (30, 29),
   (28, 33), (30, 30), (30, 31), (31, 30), (30, 32), (31, 31), (31, 32), (32, 30),
   (32, 31), (33, 30), (32, 32), (32, 33), (33, 32), (34, 31), (33, 33), (33, 34),
   (34, 33), (36, 30), (34, 34), (34, 35), (35, 34), (37, 31), (35, 35), (35, 36),
   (36, 35), (36, 36), (36, 36), (36, 37), (37, 36), (37, 37), (37, 37), (37, 38),
   (38, 37), (38, 38), (38, 38), (38, 39), (39, 38), (39, 39), (39, 39), (39, 40),
   (40, 39), (40, 40), (40, 40), (40, 41), (41, 40), (41, 41), (41, 41), (41, 42),
   (42, 41), (42, 42), (42, 42), (42, 43), (43, 42), (40, 48), (43, 43), (43, 44),
   (44, 43), (41, 49), (44, 44), (44, 45), (45, 44), (43, 48), (45, 45), (45, 46),
   (46, 45), (44, 49), (46, 46), (46, 47), (47, 46), (46, 48), (47, 47), (47, 48),
   (48, 46), (48, 47), (49, 46), (48, 48), (48, 49), (49, 48), (50, 47), (49, 49),
   (49, 50), (50, 49), (52, 46), (50, 50), (50, 51), (51, 50), (53, 47), (51, 51),
   (51, 52), (52, 51), (52, 52), (52, 52), (52, 53), (53, 52), (53, 53), (53, 53),
   (53, 54), (54, 53), (54, 54), (54, 54), (54, 55), (55, 54), (55, 55), (55, 55),
   (55, 56), (56, 55), (56, 56), (56, 56), (56, 57), (57, 56), (57, 57), (57, 57),
   (57, 58), (58, 57), (58, 58), (58, 58), (58, 59), (59, 58), (59, 59), (59, 59),
   (59, 60), (60, 59), (60, 60), (60, 60), (60, 61), (61, 60), (61, 61), (61, 61),
   (61, 62), (62, 61), (62, 62), (62, 62), (62, 63), (63, 62), (63, 63), (63, 63),
];

pub enum CompressionMode {
    Normal,
    Dither,
    HighQual,
}

pub enum Rounding {
    Biased,
    Unbiased,
}

pub fn mul_8_bit(a: isize, b: isize) -> isize {
    let t = a * b + 128;
    (t + (t >> 8)) >> 8
}

pub fn from_16_bit(value: u16) -> [u8; 4] {
    let rv = (value & 0xf800) >> 11;
    let gv = (value & 0x070e) >> 5;
    let bv = (value & 0x001f) >> 0;
    let out: [u8; 4] = [
        ((rv * 33) >> 2) as u8,
        ((gv * 65) >> 4) as u8,
        ((bv * 33) >> 2) as u8,
        0,
    ];

    out
}

pub fn as_16_bit(r: isize, g: isize, b: isize) -> u16 {
    ((mul_8_bit(r, 31) << 11 + mul_8_bit(g, 63) << 5) + mul_8_bit(b, 31)) as u16
}

pub fn lerp13(a: isize, b: isize, rounding: Rounding) -> isize {
    match rounding {
        Rounding::Biased => a + mul_8_bit(b - a, 0x55),
        Rounding::Unbiased => (2 * a + b) / 3,
    }
}
