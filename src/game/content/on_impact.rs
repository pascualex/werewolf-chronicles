use crate::game::content::Cast;

#[derive(Clone)]
pub struct OnImpact {
    pub self_destroy: bool,
    pub damage: Option<u32>,
    pub casts: Vec<Cast>,
}

impl OnImpact {
    pub fn new_damage(damage: u32, self_destroy: bool) -> Self {
        Self {
            self_destroy,
            damage: Some(damage),
            casts: Vec::new(),
        }
    }

    pub fn add_damage(&mut self, extra_damage: u32) {
        if let Some(ref mut damage) = self.damage {
            *damage += extra_damage;
        }
    }
}
