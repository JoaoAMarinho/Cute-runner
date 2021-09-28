use bevy::prelude::*;

use crate::{TIME_STEP};

// region:    Resources
pub struct Gravity(pub f32);
// endregion:    Resources

// region:    Components
pub struct Velocity(pub Vec2);
pub struct AffectedByGravity(pub bool);
// endregion:    Components

// region:    Plugin
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(gravity_system.system());
    }
}
// endregion:    Plugin

fn gravity_system(
    gravity: Res<Gravity>,
    mut query: Query<(&mut Velocity, &AffectedByGravity)>
    
) {
    for (mut velocity, affected) in query.iter_mut() {
        if !affected.0 {continue;}
        velocity.0.y -= gravity.0 * TIME_STEP;
    }
}