use crate::enums::weapon_set::WeaponSet;

#[derive(Debug)]
pub struct WeaponSetData<T: Copy> {
    primary: T,
    secondary: T,
}

impl<T: Copy> WeaponSetData<T> {
    pub fn new(primary: T, secondary: T) -> Self {
        Self { primary, secondary }
    }

    pub fn get(&self, weaponset: WeaponSet) -> T {
        match weaponset {
            WeaponSet::Primary => self.primary,
            WeaponSet::Secondary => self.secondary,
        }
    }
}
