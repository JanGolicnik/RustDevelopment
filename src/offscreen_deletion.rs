use bevy::prelude::*;

use crate::ResolutionSettings;

#[derive(Component)]
pub struct OffscreenDeletion;

pub struct OffscreenDeletionPlugin;

impl Plugin for OffscreenDeletionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, offscreen_deletion_system);
    }
}

fn offscreen_deletion_system(
    mut commands: Commands,
    query: Query<(&Transform, Entity), With<OffscreenDeletion>>,
    resolution_settings: Res<ResolutionSettings>,
) {
    query.for_each(|(transform, entity)| {
        //intentionally far offscreen just cuz
        if transform.translation.x < -resolution_settings.x {
            commands.entity(entity).despawn();
        }
    });
}
