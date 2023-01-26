use crate::plugins::{
    items::components::{branch::BranchItem, ItemTrait},
    loading::resources::GameAssets,
    player::{components::PlayerCameraComponent, events::SpawnItemEvent},
};
use bevy::prelude::*;

pub fn spawn_item(
    mut commands: Commands,
    mut spawn_item_e: EventReader<SpawnItemEvent>,
    assets: Res<GameAssets>,
    camera_q: Query<&GlobalTransform, With<PlayerCameraComponent>>,
) {
    for _ in spawn_item_e.iter() {
        let far = 1.0;

        let camera_transform = camera_q.single().compute_transform();

        let pos = camera_transform.translation + camera_transform.forward() * far;

        BranchItem.spawn(&mut commands, &assets, Transform::from_translation(pos));
    }
}
