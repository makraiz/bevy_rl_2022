use crate::prelude::*;

//Events
pub struct SufferDamage {
    pub amount: Vec<i32>,
    pub target: Entity,
    pub attacker_name: Name,
}

//Systems
pub fn damage_system(mut dam: EventReader<SufferDamage>, mut query: Query<(&mut CombatStats, Entity, &Name)>) {
    for damage in dam.iter() {
        for (mut stats, tgt, target_name) in query.iter_mut() {
            if tgt == damage.target {
                let dmg = damage.amount.iter().sum::<i32>() - stats.defense;
                stats.hp -= dmg;
                println!("{} hits {} for {} damage!", damage.attacker_name.name, target_name.name, dmg);
            }
        }
    }
}

pub fn bring_out_your_dead(mut commands: Commands, query: Query<(Entity, &CombatStats)>) {
    let mut dead = Vec::new();
    for (ent, stats) in query.iter() {
        if stats.hp <= 0 {dead.push(ent)}
    }
    for death in dead {
        commands.entity(death).despawn();
    }
}