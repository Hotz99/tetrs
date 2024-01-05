pub struct IdManager {
    id_pool: Vec<u8>,
    next_id: u8,
}

impl IdManager {
    pub fn new() -> IdManager {
        IdManager {
            id_pool: Vec::new(),
            next_id: 0,
        }
    }

    pub fn next_piece_id(&mut self) -> u8 {
        self.id_pool.pop().unwrap_or_else(|| {
            let id = self.next_id;
            self.next_id = self.next_id.wrapping_add(1);
            id
        })
    }

    // game::update() calls this when a piece is removed from the field to return its id to the pool
    pub fn return_id(&mut self, id: u8) {
        self.id_pool.push(id);
    }
}

pub fn create_composite_id(pent_id: &u8, piece_id: &u8) -> u16 {
    ((pent_id.clone() as u16) << 8) | (piece_id.clone() as u16)
}

pub fn get_pent_id(composite_id: u16) -> u8 {
    ((composite_id >> 8) & 0xFF) as u8
}

pub fn get_piece_id(composite_id: u16) -> u8 {
    (composite_id & 0xFF) as u8
}

pub fn char_to_id(c: char) -> u8 {
    match c {
        'X' => 0,
        'I' => 1,
        'Z' => 2,
        'T' => 3,
        'U' => 4,
        'V' => 5,
        'W' => 6,
        'Y' => 7,
        'L' => 8,
        'P' => 9,
        'N' => 10,
        'F' => 11,
        _ => 255,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_piece_id() {
        let composite_id = create_composite_id(&1, &2);

        assert_eq!(get_pent_id(composite_id), 1);
        assert_eq!(get_piece_id(composite_id), 2);
    }

    #[test]
    fn test_get_pent_id() {
        assert_eq!(get_pent_id(0), 0);
        assert_eq!(get_pent_id(258), 1);
        assert_eq!(get_pent_id(65535), 255);
    }
}
