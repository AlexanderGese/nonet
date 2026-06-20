#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub cells: [u8; 81], // 0 = empty
}

impl Board {
    pub fn empty() -> Self {
        Board { cells: [0; 81] }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let mut cells = [0u8; 81];
        let mut i = 0;
        for ch in s.chars() {
            let v = match ch {
                '.' | '0' | '_' => 0,
                '1'..='9' => ch as u8 - b'0',
                c if c.is_whitespace() => continue,
                _ => return None,
            };
            if i >= 81 {
                return None;
            }
            cells[i] = v;
            i += 1;
        }
        (i == 81).then_some(Board { cells })
    }

    pub fn to_line(&self) -> String {
        self.cells
            .iter()
            .map(|&c| if c == 0 { '.' } else { (b'0' + c) as char })
            .collect()
    }

    #[inline]
    pub fn get(&self, r: usize, c: usize) -> u8 {
        self.cells[r * 9 + c]
    }

    // which values are already taken in this cell's row/col/box, as a bitmask
    pub fn used_mask(&self, idx: usize) -> u16 {
        let (r, c) = (idx / 9, idx % 9);
        let (br, bc) = (r / 3 * 3, c / 3 * 3);
        let mut m = 0u16;
        for k in 0..9 {
            m |= 1 << self.cells[r * 9 + k];
            m |= 1 << self.cells[k * 9 + c];
            m |= 1 << self.cells[(br + k / 3) * 9 + (bc + k % 3)];
        }
        m
    }

    pub fn clue_count(&self) -> usize {
        self.cells.iter().filter(|&&c| c != 0).count()
    }

    pub fn is_valid(&self) -> bool {
        for unit in 0..9 {
            let (mut rows, mut cols, mut boxes) = (0u16, 0u16, 0u16);
            for k in 0..9 {
                let (br, bc) = (unit / 3 * 3, unit % 3 * 3);
                let triples = [
                    (self.cells[unit * 9 + k], &mut rows),
                    (self.cells[k * 9 + unit], &mut cols),
                    (self.cells[(br + k / 3) * 9 + (bc + k % 3)], &mut boxes),
                ];
                for (v, set) in triples {
                    if v != 0 {
                        if *set & (1 << v) != 0 {
                            return false;
                        }
                        *set |= 1 << v;
                    }
                }
            }
        }
        true
    }
}
