/// Running sum of the source pixels covered by one cell.
#[derive(Clone, Copy)]
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

    pub fn add(&mut self, r: u8, g: u8, b: u8) {
        self.r_sum += r as u32;
        self.g_sum += g as u32;
        self.b_sum += b as u32;
        self.px_sum += 1;
    }

    /// HSL lightness of the average color: the midpoint of its brightest and
    /// darkest channel.
    pub fn lightness(&self) -> u8 {
        if self.px_sum == 0 {
            return 0;
        }

        let max = self.r_sum.max(self.g_sum).max(self.b_sum);
        let min = self.r_sum.min(self.g_sum).min(self.b_sum);

        ((max + min) / (self.px_sum * 2)) as u8
    }

    /// The average color of the cell, as `[r, g, b]`.
    pub fn average(&self) -> [u8; 3] {
        if self.px_sum == 0 {
            return [0; 3];
        }

        [
            (self.r_sum / self.px_sum) as u8,
            (self.g_sum / self.px_sum) as u8,
            (self.b_sum / self.px_sum) as u8,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_cell_reads_as_black() {
        let px = Px::new();

        assert_eq!(px.lightness(), 0);
        assert_eq!(px.average(), [0, 0, 0]);
    }

    #[test]
    fn it_averages_the_pixels_it_was_given() {
        let mut px = Px::new();

        px.add(0, 0, 0);
        px.add(100, 200, 40);

        assert_eq!(px.average(), [50, 100, 20]);
        // Lightness of the average color: (max 100 + min 20) / 2.
        assert_eq!(px.lightness(), 60);
    }
}
