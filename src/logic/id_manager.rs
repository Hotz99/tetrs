pub fn next_unique_id(used_ids: &mut Vec<bool>, pent_id: &u8) -> u16 {
    // next_id = pent_id + multiple of 12
    let mut next_id = pent_id.clone() as u16;

    while used_ids[next_id as usize] {
        next_id += 12;
    }

    // if next_id is larger than 12 bits
    if next_id & 0xF000 != 0 {
        for i in 0..used_ids.len() {
            used_ids[i] = false;
        }

        return next_unique_id(used_ids, pent_id);
    }

    used_ids[next_id as usize] = true;

    next_id
}

// composite_id (16 bits) = pent_id (4 bits) + unique_id (12 bits)
pub fn create_composite_id(pent_id: &u8, unique_id: &u16) -> u16 {
    ((pent_id.clone() as u16) << 12) | (unique_id.clone() & 0x0FFF) // extract 12 bits
}

pub fn get_pent_id(composite_id: u16) -> u8 {
    (composite_id >> 12) as u8
}

pub fn get_unique_id(composite_id: u16) -> u16 {
    composite_id & 0x0FFF
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

    // write a test for create_composite_id
    #[test]
    fn test_create_composite_id() {
        for x in 0..12 {
            for y in 0..4096 {
                let composite_id = create_composite_id(&x, &y);
                assert_eq!(get_pent_id(composite_id), x);
                assert_eq!(get_unique_id(composite_id), y);
            }
        }
    }

    #[test]
    fn test_get_unique_id() {
        let composite_id = create_composite_id(&1, &2);

        assert_eq!(get_pent_id(composite_id), 1);
        assert_eq!(get_unique_id(composite_id), 2);
    }

    #[test]
    fn test_get_pent_id() {
        assert_eq!(get_pent_id(0), 0);
        assert_eq!(get_pent_id(258), 1);
        assert_eq!(get_pent_id(65535), 255);
    }
}
