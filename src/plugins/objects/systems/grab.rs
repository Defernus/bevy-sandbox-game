use crate::plugins::{
    game_world::resources::GameWorld,
    objects::components::items::{drop_item, grab_item, ItemComponent, ItemGrabbed},
    player::{
        components::{PlayerCameraComponent, PlayerComponent, PlayerHand},
        events::UseGrabPlaceEvent,
        resources::PLAYER_ACCESS_RADIUS,
    },
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn grab(
    mut use_place_grab_e: EventReader<UseGrabPlaceEvent>,
    rapier_context: Res<RapierContext>,
    transform_q: Query<&GlobalTransform, With<PlayerCameraComponent>>,
    player_hand_q: Query<(Entity, &GlobalTransform), With<PlayerHand>>,
    mut commands: Commands,
    player_rigid_body_q: Query<Entity, (With<PlayerComponent>, Without<PlayerHand>)>,
    mut item_grabbed_q: Query<Entity, With<ItemGrabbed>>,
    item_q: Query<Entity, (With<ItemComponent>, Without<ItemGrabbed>)>,
    colliders_q: Query<&Parent, With<Collider>>,
    mut world: ResMut<GameWorld>,
) {
    for _ in use_place_grab_e.iter() {
        for item in item_grabbed_q.iter_mut() {
            let (_, transform) = player_hand_q.single();
            drop_item(
                commands.entity(item),
                transform.compute_transform(),
                &mut world,
            );
        }

        let transform = transform_q.single().compute_transform();
        let ray_origin = transform.translation;
        let dir = transform.forward();

        let player = player_rigid_body_q.single();

        if let Some((entity, _)) = rapier_context.cast_ray(
            ray_origin,
            dir,
            PLAYER_ACCESS_RADIUS,
            false,
            QueryFilter::default().exclude_collider(player),
        ) {
            let parent = match colliders_q.get(entity) {
                Ok(parent) => parent,
                Err(_) => continue,
            };

            match item_q.get(parent.get()) {
                Ok(item) => {
                    let (hand, _) = player_hand_q.single();
                    grab_item(commands.entity(item), hand);
                }
                Err(_) => {}
            }
        }
    }
}
