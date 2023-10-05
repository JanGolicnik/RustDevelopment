use bevy::prelude::*;

#[derive(Component)]
pub struct YSort(pub f32);

pub struct YSortPlugin;

impl Plugin for YSortPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ysort_system);
    }
}

fn ysort_system(mut query: Query<(&mut Transform, &YSort)>) {
    query.for_each_mut(|(mut transform, ysort)| {
        transform.translation.z =
            ysort.0 - (1.0f32 / (1.0f32 + (2.0f32.powf(-0.01 * transform.translation.y))));
    })
}
