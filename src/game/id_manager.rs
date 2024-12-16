pub struct IdManager {
    used_ids: Vec<bool>,
}

impl IdManager {
    pub fn default() -> Self {
        Self {
            used_ids: vec![false; 4096],
        }
    }

    pub fn next_unique_id(&mut self, pent_id: u8) -> u16 {
        // next_id = pent_id + multiple of 12
        let mut next_id = pent_id as usize;

        loop {
            // if next_id is larger than 12 bits
            if (next_id & 0xF000) != 0 {
                self.used_ids.fill(false);
                return pent_id as u16;
            } else if !self.used_ids[next_id] {
                self.used_ids[next_id] = true;
                return next_id as u16;
            }
            next_id += 12;
        }
    }
}
