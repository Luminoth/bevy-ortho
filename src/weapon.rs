use crate::inventory;

#[derive(Debug)]
pub struct Weapon {
    // TODO: weapon data
}

impl Weapon {
    pub fn new(item: inventory::InventoryItem) -> Self {
        match item {
            inventory::InventoryItem::Weapon => Self {},
            _ => unreachable!(),
        }
    }
}
