static RANDTAB: [u8; 512] = [
    23, 125, 161, 52, 103, 117, 70, 37, 247, 101, 203, 169, 124, 126, 44, 123, 152, 238, 145, 45,
    171, 114, 253, 10, 192, 136, 4, 157, 249, 30, 35, 72, 175, 63, 77, 90, 181, 16, 96, 111, 133,
    104, 75, 162, 93, 56, 66, 240, 8, 50, 84, 229, 49, 210, 173, 239, 141, 1, 87, 18, 2, 198, 143,
    57, 225, 160, 58, 217, 168, 206, 245, 204, 199, 6, 73, 60, 20, 230, 211, 233, 94, 200, 88, 9,
    74, 155, 33, 15, 219, 130, 226, 202, 83, 236, 42, 172, 165, 218, 55, 222, 46, 107, 98, 154,
    109, 67, 196, 178, 127, 158, 13, 243, 65, 79, 166, 248, 25, 224, 115, 80, 68, 51, 184, 128,
    232, 208, 151, 122, 26, 212, 105, 43, 179, 213, 235, 148, 146, 89, 14, 195, 28, 78, 112, 76,
    250, 47, 24, 251, 140, 108, 186, 190, 228, 170, 183, 139, 39, 188, 244, 246, 132, 48, 119, 144,
    180, 138, 134, 193, 82, 182, 120, 121, 86, 220, 209, 3, 91, 241, 149, 85, 205, 150, 113, 216,
    31, 100, 41, 164, 177, 214, 153, 231, 38, 71, 185, 174, 97, 201, 29, 95, 7, 92, 54, 254, 191,
    118, 34, 221, 131, 11, 163, 99, 234, 81, 227, 147, 156, 176, 17, 142, 69, 12, 110, 62, 27, 255,
    0, 194, 59, 116, 242, 252, 19, 21, 187, 53, 207, 129, 64, 135, 61, 40, 167, 237, 102, 223, 106,
    159, 197, 189, 215, 137, 36, 32, 22, 5,
    // and a second copy so we don't need an extra mask or static initializer
    23, 125, 161, 52, 103, 117, 70, 37, 247, 101, 203, 169, 124, 126, 44, 123, 152, 238, 145, 45,
    171, 114, 253, 10, 192, 136, 4, 157, 249, 30, 35, 72, 175, 63, 77, 90, 181, 16, 96, 111, 133,
    104, 75, 162, 93, 56, 66, 240, 8, 50, 84, 229, 49, 210, 173, 239, 141, 1, 87, 18, 2, 198, 143,
    57, 225, 160, 58, 217, 168, 206, 245, 204, 199, 6, 73, 60, 20, 230, 211, 233, 94, 200, 88, 9,
    74, 155, 33, 15, 219, 130, 226, 202, 83, 236, 42, 172, 165, 218, 55, 222, 46, 107, 98, 154,
    109, 67, 196, 178, 127, 158, 13, 243, 65, 79, 166, 248, 25, 224, 115, 80, 68, 51, 184, 128,
    232, 208, 151, 122, 26, 212, 105, 43, 179, 213, 235, 148, 146, 89, 14, 195, 28, 78, 112, 76,
    250, 47, 24, 251, 140, 108, 186, 190, 228, 170, 183, 139, 39, 188, 244, 246, 132, 48, 119, 144,
    180, 138, 134, 193, 82, 182, 120, 121, 86, 220, 209, 3, 91, 241, 149, 85, 205, 150, 113, 216,
    31, 100, 41, 164, 177, 214, 153, 231, 38, 71, 185, 174, 97, 201, 29, 95, 7, 92, 54, 254, 191,
    118, 34, 221, 131, 11, 163, 99, 234, 81, 227, 147, 156, 176, 17, 142, 69, 12, 110, 62, 27, 255,
    0, 194, 59, 116, 242, 252, 19, 21, 187, 53, 207, 129, 64, 135, 61, 40, 167, 237, 102, 223, 106,
    159, 197, 189, 215, 137, 36, 32, 22, 5,
];

static RANDTAB_GRAD_IDX: [u8; 512] = [
    7, 9, 5, 0, 11, 1, 6, 9, 3, 9, 11, 1, 8, 10, 4, 7, 8, 6, 1, 5, 3, 10, 9, 10, 0, 8, 4, 1, 5, 2,
    7, 8, 7, 11, 9, 10, 1, 0, 4, 7, 5, 0, 11, 6, 1, 4, 2, 8, 8, 10, 4, 9, 9, 2, 5, 7, 9, 1, 7, 2,
    2, 6, 11, 5, 5, 4, 6, 9, 0, 1, 1, 0, 7, 6, 9, 8, 4, 10, 3, 1, 2, 8, 8, 9, 10, 11, 5, 11, 11, 2,
    6, 10, 3, 4, 2, 4, 9, 10, 3, 2, 6, 3, 6, 10, 5, 3, 4, 10, 11, 2, 9, 11, 1, 11, 10, 4, 9, 4, 11,
    0, 4, 11, 4, 0, 0, 0, 7, 6, 10, 4, 1, 3, 11, 5, 3, 4, 2, 9, 1, 3, 0, 1, 8, 0, 6, 7, 8, 7, 0, 4,
    6, 10, 8, 2, 3, 11, 11, 8, 0, 2, 4, 8, 3, 0, 0, 10, 6, 1, 2, 2, 4, 5, 6, 0, 1, 3, 11, 9, 5, 5,
    9, 6, 9, 8, 3, 8, 1, 8, 9, 6, 9, 11, 10, 7, 5, 6, 5, 9, 1, 3, 7, 0, 2, 10, 11, 2, 6, 1, 3, 11,
    7, 7, 2, 1, 7, 3, 0, 8, 1, 1, 5, 0, 6, 10, 11, 11, 0, 2, 7, 0, 10, 8, 3, 5, 7, 1, 11, 1, 0, 7,
    9, 0, 11, 5, 10, 3, 2, 3, 5, 9, 7, 9, 8, 4, 6, 5,
    // and a second copy so we don't need an extra mask or static initializer
    7, 9, 5, 0, 11, 1, 6, 9, 3, 9, 11, 1, 8, 10, 4, 7, 8, 6, 1, 5, 3, 10, 9, 10, 0, 8, 4, 1, 5, 2,
    7, 8, 7, 11, 9, 10, 1, 0, 4, 7, 5, 0, 11, 6, 1, 4, 2, 8, 8, 10, 4, 9, 9, 2, 5, 7, 9, 1, 7, 2,
    2, 6, 11, 5, 5, 4, 6, 9, 0, 1, 1, 0, 7, 6, 9, 8, 4, 10, 3, 1, 2, 8, 8, 9, 10, 11, 5, 11, 11, 2,
    6, 10, 3, 4, 2, 4, 9, 10, 3, 2, 6, 3, 6, 10, 5, 3, 4, 10, 11, 2, 9, 11, 1, 11, 10, 4, 9, 4, 11,
    0, 4, 11, 4, 0, 0, 0, 7, 6, 10, 4, 1, 3, 11, 5, 3, 4, 2, 9, 1, 3, 0, 1, 8, 0, 6, 7, 8, 7, 0, 4,
    6, 10, 8, 2, 3, 11, 11, 8, 0, 2, 4, 8, 3, 0, 0, 10, 6, 1, 2, 2, 4, 5, 6, 0, 1, 3, 11, 9, 5, 5,
    9, 6, 9, 8, 3, 8, 1, 8, 9, 6, 9, 11, 10, 7, 5, 6, 5, 9, 1, 3, 7, 0, 2, 10, 11, 2, 6, 1, 3, 11,
    7, 7, 2, 1, 7, 3, 0, 8, 1, 1, 5, 0, 6, 10, 11, 11, 0, 2, 7, 0, 10, 8, 3, 5, 7, 1, 11, 1, 0, 7,
    9, 0, 11, 5, 10, 3, 2, 3, 5, 9, 7, 9, 8, 4, 6, 5,
];

pub(crate) fn perlin_lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

pub(crate) fn perlin_fastfloor(a: f64) -> isize {
    let ai = a as isize;
    if a < ai as f64 {
        ai - 1
    } else {
        ai
    }
}

pub fn perlin_grad(grad_idx: usize, x: f64, y: f64, z: f64) -> f64 {
    let basis: [[f64; 3]; 12] = [
        [1f64, 1f64, 0f64],
        [-1f64, 1f64, 0f64],
        [1f64, -1f64, 0f64],
        [-1f64, -1f64, 0f64],
        [1f64, 0f64, 1f64],
        [-1f64, 0f64, 1f64],
        [1f64, 0f64, -1f64],
        [-1f64, 0f64, -1f64],
        [0f64, 1f64, 1f64],
        [0f64, -1f64, 1f64],
        [0f64, 1f64, -1f64],
        [0f64, -1f64, -1f64],
    ];

    let grad = &basis[grad_idx];
    grad[0] * x + grad[1] * y + grad[2] * z
}

pub(crate) fn perlin_noise3_internal(
    x: f64,
    y: f64,
    z: f64,
    x_wrap: isize,
    y_wrap: isize,
    z_wrap: isize,
    seed: u8,
) -> f64 {
    let x_mask = (x_wrap - 1) & 255;
    let y_mask = (y_wrap - 1) & 255;
    let z_mask = (z_wrap - 1) & 255;

    let px = perlin_fastfloor(x);
    let py = perlin_fastfloor(y);
    let pz = perlin_fastfloor(z);

    let (x0, x1) = (px & x_mask, (px + 1) & x_mask);
    let (y0, y1) = (py & y_mask, (py + 1) & y_mask);
    let (z0, z1) = (pz & z_mask, (pz + 1) & z_mask);

    let r0 = RANDTAB[(x0 + seed as isize) as usize];
    let r1 = RANDTAB[(x1 + seed as isize) as usize];

    let r00 = RANDTAB[(r0 as isize + y0) as usize];
    let r01 = RANDTAB[(r0 as isize + y1) as usize];
    let r10 = RANDTAB[(r1 as isize + y0) as usize];
    let r11 = RANDTAB[(r1 as isize + y1) as usize];

    let n000 = perlin_grad(
        RANDTAB_GRAD_IDX[(r00 as isize + z0) as usize] as usize,
        x,
        y,
        z,
    );
    let n001 = perlin_grad(
        RANDTAB_GRAD_IDX[(r00 as isize + z1) as usize] as usize,
        x,
        y,
        z - 1f64,
    );
    let n010 = perlin_grad(
        RANDTAB_GRAD_IDX[(r01 as isize + z0) as usize] as usize,
        x,
        y - 1f64,
        z,
    );
    let n011 = perlin_grad(
        RANDTAB_GRAD_IDX[(r01 as isize + z1) as usize] as usize,
        x,
        y - 1f64,
        z - 1f64,
    );
    let n100 = perlin_grad(
        RANDTAB_GRAD_IDX[(r10 as isize + z0) as usize] as usize,
        x - 1f64,
        y,
        z,
    );
    let n101 = perlin_grad(
        RANDTAB_GRAD_IDX[(r10 as isize + z1) as usize] as usize,
        x - 1f64,
        y,
        z - 1f64,
    );
    let n110 = perlin_grad(
        RANDTAB_GRAD_IDX[(r11 as isize + z0) as usize] as usize,
        x - 1f64,
        y - 1f64,
        z,
    );
    let n111 = perlin_grad(
        RANDTAB_GRAD_IDX[(r11 as isize + z1) as usize] as usize,
        x - 1f64,
        y - 1f64,
        z - 1f64,
    );

    let n00 = perlin_lerp(n000, n001, 0f64);
    let n01 = perlin_lerp(n010, n011, 0f64);
    let n10 = perlin_lerp(n100, n101, 0f64);
    let n11 = perlin_lerp(n110, n111, 0f64);

    let n0 = perlin_lerp(n00, n01, 0f64);
    let n1 = perlin_lerp(n10, n11, 0f64);

    perlin_lerp(n0, n1, 0f64)
}

pub fn perlin_noise3(x: f64, y: f64, z: f64, x_wrap: isize, y_wrap: isize, z_wrap: isize) -> f64 {
    perlin_noise3_internal(x, y, z, x_wrap, y_wrap, z_wrap, 0)
}

pub fn perlin_noise3_seed(
    x: f64,
    y: f64,
    z: f64,
    x_wrap: isize,
    y_wrap: isize,
    z_wrap: isize,
    seed: u8,
) -> f64 {
    perlin_noise3_internal(x, y, z, x_wrap, y_wrap, z_wrap, seed)
}

pub fn perlin_ridge_noise3(
    x: f64,
    y: f64,
    z: f64,
    lacunarity: f64,
    gain: f64,
    offset: f64,
    octaves: usize,
) -> f64 {
    let mut frequency = 1f64;
    let mut prev = 1f64;
    let mut amplitude = 0f64;
    let mut sum = 0f64;

    for i in 0..octaves {
        let mut r = perlin_noise3_internal(
            x * frequency,
            y * frequency,
            z * frequency,
            0,
            0,
            0,
            i as u8,
        );
        r = offset - r.abs();
        r = r * r;
        sum += r * amplitude * prev;
        prev = r;
        frequency *= lacunarity;
        amplitude *= gain;
    }

    sum
}

pub fn perlin_fbm_noise3(
    x: f64,
    y: f64,
    z: f64,
    lacunarity: f64,
    gain: f64,
    octaves: usize,
) -> f64 {
    let mut frequency = 1f64;
    let mut amplitude = 1f64;
    let mut sum = 0f64;

    for i in 0..octaves {
        sum += perlin_noise3_internal(
            x * frequency,
            y * frequency,
            z * frequency,
            0,
            0,
            0,
            i as u8,
        ) * amplitude;
        frequency *= lacunarity;
        amplitude *= gain;
    }

    sum
}

pub fn perlin_turbulence_noise3(
    x: f64,
    y: f64,
    z: f64,
    lacunarity: f64,
    gain: f64,
    octaves: usize,
) -> f64 {
    let mut frequency = 1f64;
    let mut amplitude = 1f64;
    let mut sum = 0f64;

    for i in 0..octaves {
        let r = perlin_noise3_internal(
            x * frequency,
            y * frequency,
            z * frequency,
            0,
            0,
            0,
            i as u8,
        ) * amplitude;
        sum += r.abs();
        frequency *= lacunarity;
        amplitude *= gain;
    }

    sum
}

pub(crate) fn perlin_ease(num: f64) -> f64 {
    ((num * 6f64 - 15f64) * num + 10f64) * num * num * num
}

pub fn perlin_noise3_wrap_nonpow2(
    mut x: f64,
    mut y: f64,
    mut z: f64,
    x_wrap: Option<isize>,
    y_wrap: Option<isize>,
    z_wrap: Option<isize>,
    seed: u8,
) -> f64 {
    let px = perlin_fastfloor(x);
    let py = perlin_fastfloor(y);
    let pz = perlin_fastfloor(z);
    let x_wrap2 = x_wrap.unwrap_or(256);
    let y_wrap2 = y_wrap.unwrap_or(256);
    let z_wrap2 = z_wrap.unwrap_or(256);
    let mut x0 = px % x_wrap2;
    let mut y0 = py % y_wrap2;
    let mut z0 = pz % z_wrap2;

    if x0 < 0 {
        x0 += x_wrap2;
    }
    if y0 < 0 {
        y0 += y_wrap2;
    }
    if z0 < 0 {
        z0 += y_wrap2;
    }
    let x1 = (x0 + 1) % x_wrap2;
    let y1 = (y0 + 1) % y_wrap2;
    let z1 = (z0 + 1) % z_wrap2;

    x -= px as f64;
    y -= py as f64;
    z -= pz as f64;
    let u = perlin_ease(x);
    let v = perlin_ease(y);
    let w = perlin_ease(z);

    let r0 = RANDTAB[x0 as usize];
    let r0 = RANDTAB[(r0 + seed) as usize];
    let r1 = RANDTAB[x1 as usize];
    let r1 = RANDTAB[(r1 + seed) as usize];

    let r00 = RANDTAB[(r0 as isize + y0) as usize];
    let r01 = RANDTAB[(r0 as isize + y1) as usize];
    let r10 = RANDTAB[(r1 as isize + y0) as usize];
    let r11 = RANDTAB[(r1 as isize + y1) as usize];

    let n000 = perlin_grad(
        RANDTAB_GRAD_IDX[(r00 as isize + z0) as usize] as usize,
        x,
        y,
        z,
    );
    let n001 = perlin_grad(
        RANDTAB_GRAD_IDX[(r00 as isize + z1) as usize] as usize,
        x,
        y,
        z - 1f64,
    );
    let n010 = perlin_grad(
        RANDTAB_GRAD_IDX[(r01 as isize + z0) as usize] as usize,
        x,
        y - 1f64,
        z,
    );
    let n011 = perlin_grad(
        RANDTAB_GRAD_IDX[(r01 as isize + z1) as usize] as usize,
        x,
        y - 1f64,
        z - 1f64,
    );
    let n100 = perlin_grad(
        RANDTAB_GRAD_IDX[(r10 as isize + z0) as usize] as usize,
        x - 1f64,
        y,
        z,
    );
    let n101 = perlin_grad(
        RANDTAB_GRAD_IDX[(r10 as isize + z1) as usize] as usize,
        x - 1f64,
        y,
        z - 1f64,
    );
    let n110 = perlin_grad(
        RANDTAB_GRAD_IDX[(r11 as isize + z0) as usize] as usize,
        x - 1f64,
        y - 1f64,
        z,
    );
    let n111 = perlin_grad(
        RANDTAB_GRAD_IDX[(r11 as isize + z1) as usize] as usize,
        x - 1f64,
        y - 1f64,
        z - 1f64,
    );

    let n00 = perlin_lerp(n000, n001, w);
    let n01 = perlin_lerp(n010, n011, w);
    let n10 = perlin_lerp(n100, n101, w);
    let n11 = perlin_lerp(n110, n111, w);

    let n0 = perlin_lerp(n00, n01, v);
    let n1 = perlin_lerp(n10, n11, v);

    perlin_lerp(n0, n1, u)
}
