pub struct Px {
    r_sum: u32,
    g_sum: u32,
    b_sum: u32,
    px_sum: u32,
}

impl Px {
    pub fn new() -> Px {
        Px {
            r_sum: 0,
            g_sum: 0,
            b_sum: 0,
            px_sum: 0,
        }
    }

    pub fn add_px(&mut self, r: u8, g: u8, b: u8) {
        self.r_sum += r as u32;
        self.g_sum += g as u32;
        self.b_sum += b as u32;
        self.px_sum += 1;
    }

    pub fn get_ligthness(&self) -> u8 {
        let mut max = self.r_sum;
        let mut min = self.r_sum;

        if self.g_sum > max {
            max = self.g_sum;
        }

        if self.g_sum < min {
            min = self.g_sum;
        }

        if self.b_sum > max {
            max = self.b_sum;
        }

        if self.b_sum < min {
            min = self.b_sum;
        }

        ((max + min) / (self.px_sum * 2)) as u8
    }

    pub fn avg_r(&self) -> u8 {
        (self.r_sum / self.px_sum) as u8
    }

    pub fn avg_g(&self) -> u8 {
        (self.g_sum / self.px_sum) as u8
    }

    pub fn avg_b(&self) -> u8 {
        (self.b_sum / self.px_sum) as u8
    }
}
