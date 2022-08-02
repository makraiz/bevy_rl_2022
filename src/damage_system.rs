use crate::prelude::*;

//Events
pub struct SufferDamage {
    pub amount: Vec<i32>,
    pub target: Entity,
}

//Systems
pub fn damage_system(mut dam: EventReader<SufferDamage>, mut query: Query<(&mut CombatStats, Entity)>) {
    for damage in dam.iter() {
        for (mut stats, tgt) in query.iter_mut() {
            if tgt == damage.target {
                let dmg: i32 = damage.amount.iter().sum();
                stats.hp -= dmg;
            }
        }
    }
}