#[rustfmt::skip]
static OMATCH_5: [(u16, u16); 256] = [
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
static OMATCH_6: [(u16, u16); 256] = [
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

static MIDPOINTS5: [f64; 32] = [
    0.015686, 0.047059, 0.078431, 0.111765, 0.145098, 0.176471, 0.207843, 0.241176, 0.274510,
    0.305882, 0.337255, 0.370588, 0.403922, 0.435294, 0.466667, 0.5, 0.533333, 0.564706, 0.596078,
    0.629412, 0.662745, 0.694118, 0.725490, 0.758824, 0.792157, 0.823529, 0.854902, 0.888235,
    0.921569, 0.952941, 0.984314, 1.0,
];

static MIDPOINTS6: [f64; 64] = [
    0.007843, 0.023529, 0.039216, 0.054902, 0.070588, 0.086275, 0.101961, 0.117647, 0.133333,
    0.149020, 0.164706, 0.180392, 0.196078, 0.211765, 0.227451, 0.245098, 0.262745, 0.278431,
    0.294118, 0.309804, 0.325490, 0.341176, 0.356863, 0.372549, 0.388235, 0.403922, 0.419608,
    0.435294, 0.450980, 0.466667, 0.482353, 0.500000, 0.517647, 0.533333, 0.549020, 0.564706,
    0.580392, 0.596078, 0.611765, 0.627451, 0.643137, 0.658824, 0.674510, 0.690196, 0.705882,
    0.721569, 0.737255, 0.754902, 0.772549, 0.788235, 0.803922, 0.819608, 0.835294, 0.850980,
    0.866667, 0.882353, 0.898039, 0.913725, 0.929412, 0.945098, 0.960784, 0.976471, 0.992157, 1.0,
];

pub enum CompressionMode {
    Normal,
    Dither,
    HighQual,
}

#[derive(Clone, Copy)]
pub enum Rounding {
    Biased,
    Unbiased,
}

pub fn mul_8_bit(a: isize, b: isize) -> isize {
    let t = a * b + 128;
    (t + (t >> 8)) >> 8
}

pub fn from_16_bit(data: &mut [u8; 4], value: u16) {
    let rv = (value & 0xf800) >> 11;
    let gv = (value & 0x070e) >> 5;
    let bv = value & 0x001f;
    data[0] = ((rv * 33) >> 2) as u8;
    data[1] = ((gv * 65) >> 4) as u8;
    data[2] = ((bv * 33) >> 2) as u8;
    data[3] = 0;
}

pub fn as_16_bit(r: isize, g: isize, b: isize) -> u16 {
    let r = mul_8_bit(r, 31) << 11;
    let g = mul_8_bit(g, 63) << 5;
    let b = mul_8_bit(b, 31);

    (r + g + b) as u16
}

pub fn lerp13(a: u8, b: u8, rounding: Rounding) -> isize {
    match rounding {
        Rounding::Biased => a as isize + mul_8_bit((b - a) as isize, 0x55),
        Rounding::Unbiased => ((2 * a + b) / 3) as isize,
    }
}

pub fn lerp13_rgb(data: &mut [u8; 3], p1: &[u8; 2], p2: &[u8; 2], rounding: Rounding) {
    data[0] = lerp13(p1[0], p2[0], rounding) as u8;
    data[1] = lerp13(p1[1], p2[1], rounding) as u8;
    data[2] = lerp13(p1[1], p2[1], rounding) as u8;
}

pub fn eval_colors(color: &[u8; 16], c0: u16, c1: u16, rounding: Rounding) {
    // unwraps are ok here as the function signature only accepts a u8 array of the needed len
    let c2: [u8; 2] = color[..2].try_into().unwrap();
    let c2_offset_4: [u8; 2] = color[4..6].try_into().unwrap();
    let mut c3_offset_8: [u8; 3] = color[8..11].try_into().unwrap();
    let mut c3_offset_12: [u8; 3] = color[12..15].try_into().unwrap();
    let mut c4: [u8; 4] = color[..4].try_into().unwrap();
    let mut c4_offset_4: [u8; 4] = color[4..8].try_into().unwrap();

    from_16_bit(&mut c4, c0);
    from_16_bit(&mut c4_offset_4, c1);
    lerp13_rgb(&mut c3_offset_8, &c2, &c2_offset_4, rounding);
    lerp13_rgb(&mut c3_offset_12, &c2_offset_4, &c2, rounding);
}

pub fn match_colors_block(block: &[u8; 63], color: &[u8; 7]) -> usize {
    let dirr = color[0] - color[4];
    let dirg = color[1] - color[5];
    let dirb = color[2] - color[6];
    let mut dots = [0; 16];
    let mut stops = [0; 4];
    for (i, dot) in dots.iter_mut().enumerate() {
        let idx = i * 4;
        *dot = block[idx] * dirr + block[idx + 1] * dirg + block[idx + 2] * dirb;
    }

    for (i, stop) in stops.iter_mut().enumerate() {
        let idx = i * 4;
        *stop = block[idx] * dirr + block[idx + 1] * dirg + block[idx + 2] * dirb;
    }

    let c0_point = (stops[1] + stops[3]) as usize;
    let half_point = (stops[3] + stops[2]) as usize;
    let c3_point = (stops[2] + stops[0]) as usize;

    let mut mask = 0;
    for i in (0..15).rev() {
        let dot = dots[i] as usize * 2;
        mask <<= 2;
        if dot < half_point {
            if dot < c0_point {
                mask |= 1;
            } else {
                mask |= 3;
            }
        } else if dot < c3_point {
            mask |= 2;
        }
    }

    mask
}

pub fn quantize5(x: f64) -> u16 {
    let x = x.clamp(0.0, 1.0);
    let mut q = (x * 31.0) as u16;
    if x > MIDPOINTS5[q as usize] {
        q += 1;
    }

    q
}

pub fn quantize6(x: f64) -> u16 {
    let x = x.clamp(0.0, 1.0);
    let mut q = (x * 63.0) as u16;
    if x > MIDPOINTS6[q as usize] {
        q += 1;
    }

    q
}

pub fn refine_block(block: &[u8; 67], pmax16: u16, pmin16: u16, mask: usize) -> bool {
    const W1TAB: [usize; 4] = [3, 0, 2, 1];
    const PROD: [usize; 4] = [0x90000, 0x000900, 0x040102, 0x010402];

    #[allow(unused_assignments)]
    let mut min16: u16 = 0;
    #[allow(unused_assignments)]
    let mut max16: u16 = 0;

    let mut cm = mask;

    if (mask ^ (mask << 2)) < 4 {
        let mut r = 8;
        let mut b = 8;
        let mut g = 8;

        for i in 0..16 {
            r += block[i * 4];
            g += block[i * 4 + 1];
            b += block[i * 4 + 2];
        }

        r >>= 4;
        g >>= 4;
        b >>= 4;

        max16 =
            (OMATCH_5[r as usize].0 << 11) | (OMATCH_6[g as usize].0 << 5) | OMATCH_5[b as usize].0;
        min16 =
            (OMATCH_5[r as usize].1 << 11) | (OMATCH_6[g as usize].1 << 5) | OMATCH_5[b as usize].1;
    } else {
        let mut at1_r = 0;
        let mut at1_g = 0;
        let mut at1_b = 0;
        let mut at2_r = 0;
        let mut at2_g = 0;
        let mut at2_b = 0;
        let mut akku = 0;
        for i in 0..16 {
            cm >>= 2;
            let step = cm & 3;
            let w1 = W1TAB[step];
            let r = block[i * 4];
            let g = block[i * 4 + 1];
            let b = block[i * 4 + 2];

            akku += PROD[step];

            at1_r += w1 * r as usize;
            at1_g += w1 * g as usize;
            at1_b += w1 * b as usize;
            at2_r += r as usize;
            at2_g += g as usize;
            at2_b += b as usize;
        }

        at2_r = 3 * at2_r - at1_r;
        at2_g = 3 * at2_g - at1_g;
        at2_b = 3 * at2_b - at1_b;

        let xx = akku >> 16;
        let yy = (akku >> 8) & 0xff;
        let xy = akku & 0xff;

        let f = 3f64 / 255f64 / (xx * yy - xy * xy) as f64;

        max16 = quantize5((at1_r * yy - at2_r * xy) as f64 * f) << 11;
        max16 |= quantize6((at1_g * yy - at2_g * xy) as f64 * f) << 5;
        max16 |= quantize5((at1_b * yy - at2_b * xy) as f64 * f);

        min16 = quantize5((at2_r * xx - at1_r * xy) as f64 * f) << 11;
        min16 |= quantize6((at2_g * xx - at1_g * xy) as f64 * f) << 5;
        min16 |= quantize5((at2_b * xx - at1_b * xy) as f64 * f);
    }

    pmin16 != min16 || pmax16 != max16
}
