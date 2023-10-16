use bevy::prelude::*;

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationIndices {
    pub index: usize,
    pub sprite_indices: Vec<usize>,
}
