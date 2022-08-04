use crate::prelude::*;

//Systems
pub fn melee_combat_system(mut commands: Commands, attacker_query: Query<(Entity, &CombatStats, &WantsToMelee, &Name)>, mut defender_query: Query<(Entity, &CombatStats, &Name, Option<&mut SufferDamage>)>) {
    for (atk_ent, atk_stats, atk_target, atk_name) in attacker_query.iter() {
        if atk_stats.hp > 0 {
            for (def_ent, def_stats, def_name, damages) in defender_query.iter_mut() {
                if atk_target.target == def_ent && def_stats.hp > 0 {
                    let damage = i32::max(0, atk_stats.power - def_stats.defense);
                    if damage == 0 {
                        println!("{} is unable to hurt {}", atk_name.name, def_name.name);
                    } else {
                        println!("{} hits {} for {} damage!", atk_name.name, def_name.name, damage);
                        match damages {
                            Some(mut d) => d.new_damage(damage),
                            None => {commands.entity(def_ent).insert(SufferDamage{amount: vec![damage]});},
                        }
                    }
                }
            }
        }
        commands.entity(atk_ent).remove::<WantsToMelee>();
    }
}

pub fn damage_system(mut commands: Commands, mut query: Query<(Entity, &SufferDamage, &mut CombatStats)>) {
    for (ent, dam, mut stats) in query.iter_mut() {
        stats.hp -= dam.amount.iter().sum::<i32>();
        commands.entity(ent).remove::<SufferDamage>();
    }
}


/*  This system was meant to work with an event, but it was breaking other systems.  
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
*/

pub fn bring_out_your_dead(mut commands: Commands, query: Query<(Entity, &CombatStats)>) {
    let mut dead = Vec::new();
    for (ent, stats) in query.iter() {
        if stats.hp <= 0 {dead.push(ent)}
    }
    for death in dead {
        commands.entity(death).despawn();
    }
}