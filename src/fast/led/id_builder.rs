use super::anim::LedId;

pub fn linear(expansion_id: String, port: u8, count: u8) -> Vec<LedId> {
    let mut ids: Vec<LedId> = Vec::new();
    for i in 0..count {
        ids.push(LedId {
            expansion_id: expansion_id.clone(),
            port,
            index: i,
        });
    }
    return ids;
}

pub fn linear_rev(expansion_id: String, port: u8, count: u8) -> Vec<LedId> {
    let mut ids: Vec<LedId> = Vec::new();
    for i in (0..count).rev() {
        ids.push(LedId {
            expansion_id: expansion_id.clone(),
            port,
            index: i,
        });
    }
    return ids;
}

pub fn linear_alternate(expansion_id: String, port: u8, count: u8) -> Vec<LedId> {
    let mut ids: Vec<LedId> = Vec::new();
    for i in 0..count {
        if i % 2 == 0 {
            ids.push(LedId {
                expansion_id: expansion_id.clone(),
                port,
                index: i,
            });
        } else {
            ids.push(LedId {
                expansion_id: expansion_id.clone(),
                port,
                index: count - i,
            });
        }
    }
    return ids;
}
