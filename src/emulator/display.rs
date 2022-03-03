use super::emulator::Result;

#[derive(Debug, Default)]
pub struct Display {
    pub pixel_map: Vec<Vec<bool>>,
    pub height: u8,
    pub width: u8
}

pub type Collision = bool;

impl Display {
    pub fn resize(&mut self, width: u8, height: u8) {
        let cur_height = self.pixel_map.len();
        let mut cur_width = 0;
        if cur_height > 0 {
            cur_width = self.pixel_map[0].len();
        }

        let ydiff = height as i16 - cur_height as i16;
        let xdiff = width as i16 - cur_width as i16;

        if (xdiff, ydiff) == (0, 0) { return; }

        if ydiff > 0 {
            for _i in 0..ydiff {
                self.pixel_map.push(vec![false; width as usize]);
            }
        }
        if ydiff < 0 {
            for _i in 0..(-1 * ydiff) as usize {
                self.pixel_map.pop();
            }
        }
        for row in &mut self.pixel_map {
            if row.len() as u8 == width { continue; }
            if xdiff > 0 {
                row.append(&mut vec![false; xdiff as usize]);
            }
            if xdiff < 0 {
                for _i in 0..(-1 * xdiff) as usize {
                    row.push(false);
                }
            }
        }
    }

    fn byte_to_bits(byte: u8) -> [bool; 8] {
        [
            byte & 0b10000000 != 0,
            byte & 0b01000000 != 0,
            byte & 0b00100000 != 0,
            byte & 0b00010000 != 0,
            byte & 0b00001000 != 0,
            byte & 0b00000100 != 0,
            byte & 0b00000010 != 0,
            byte & 0b00000001 != 0
        ]
    }

    fn draw_bit_at(&mut self, x: u8, y: u8, bit: bool) -> Result<Collision> {
        let px = self.pixel_map[(y % self.height) as usize][(x % self.width) as usize];
        let new = px ^ bit;
        let collision = if px == true && new == false { true } else { false };
        self.pixel_map[(y % self.height) as usize][(x % self.width) as usize] = new;
        Ok(collision)
    }

    fn draw_byte_at(&mut self, x: u8, y: u8, byte: u8) -> Result<Collision> {
        let bits = Display::byte_to_bits(byte);
        let mut collision = false;
        for i in 0..8 {
            collision |= self.draw_bit_at(x + i, y, bits[i as usize])?;
        }
        Ok(collision)
    }

    pub fn draw_bytes_at(&mut self, x: u8, y: u8, bytes: &[u8]) -> Result<Collision> {
        let mut collision = false;
        for i in 0..bytes.len() {
            collision |= self.draw_byte_at(x, y + i as u8, bytes[i])?;
        }
        Ok(collision)
    }

    pub fn clear(&mut self) {
        self.pixel_map = vec![vec![false; self.width.into()]; self.height.into()];
    }
}