#[derive(Clone)]
pub struct Weapon {
    pub name: String,
    pub damage: u16,
    pub is_used : bool
    
}
pub struct WeaponManager {
    count : usize,
    pub availible_weapons : Vec<Weapon>
}

impl WeaponManager {
    pub fn new() -> WeaponManager {
        WeaponManager {
            count : 0,
            availible_weapons : Vec::new()
        }
    }

    pub fn add_weapon(&mut self, new_weapon : Weapon){
        self.availible_weapons.push(new_weapon);
        self.count = self.availible_weapons.len();
    }
}