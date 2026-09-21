use bevy::prelude::*;

use crate::{
    board::{Board, MainBoard, UnitDefinition},
    interactive::BoardPos,
    ui::Cursor,
};

const DELAY_FOR_LAYER_TO_SHOW: f32 = 0.05;


#[derive(Resource, Deref)]
pub struct CurrentSelected(Entity);

#[derive(Component)]
pub struct Selected(UVec2);

#[derive(Component, Clone)]
pub struct MovementOption{
    rel: Entity
}

impl MovementOption {
    fn new(rel: Entity) -> Self {
        Self{
            rel
        }
    }
}

impl Default for MovementOption{
    fn default() -> Self {
        Self {
            rel: Entity::from_raw_u32(0).expect("Movement Option default")
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Default)]
pub struct Delay(Timer);

pub fn apply_visibility_delayed(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Delay)>,
    time: Res<Time>,
) {
    for (entity, mut delay) in entities.iter_mut() {
        delay.tick(time.delta());
        if delay.is_finished() {
            let mut entity = commands.entity(entity);
            entity.remove::<Delay>();
            entity.insert(Visibility::Inherited);
        }
    }
}

pub fn on_drop_movement(
    trigger: On<Remove, Selected>,
    mut commands: Commands,
    options: Query<(Entity,&MovementOption)>,
) {
    let trigger_entity = trigger.entity;
    for (entity, option) in options.iter(){
        if trigger_entity == option.rel {
            commands.entity(entity).despawn();
        }
    }
}

pub fn on_shown_movement(
    trigger: On<Insert, Selected>,
    mut commands: Commands,
    entities_selected: Query<(&UnitDefinition,&BoardPos), With<Selected>>,
    board_entity: Single<Entity, With<MainBoard>>,
    board: Res<Board>,
) {
    let entity = trigger.entity;
    let Ok((unit_definition, pos)) = entities_selected.get(entity) else {
        return;
    };
    let possibilities = unit_definition.get_movements(**pos, &board);
    for position in possibilities {
        let board_helper = BoardPos::from(position.position);
        let time = Timer::from_seconds(
                position.layer as f32 * DELAY_FOR_LAYER_TO_SHOW,
                TimerMode::Once,
            );
        let child= commands.spawn_scene(bsn!{
            MovementOption::new(entity)
            Transform::from_translation(
                board_helper.get_screen_pos(0) + vec3(1.0, 1.0, 0.0),
            )
            Mesh2d(asset_value(Rectangle::new(30., 30.)))
            MeshMaterial2d<ColorMaterial>(asset_value(Color::linear_rgba(0., 0., 1., 0.3)))
            Visibility::Hidden
            Delay(time)
        }).id();
        commands.entity(*board_entity).add_child(child);
    }
}

pub fn on_click_cursor(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    board: Res<Board>,
    current_selected: Option<ResMut<CurrentSelected>>,
    cursor: Single<&Cursor>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let pos = cursor.position;
        let Some(entity) = board.get_entity_at(&pos) else {
            if let Some(current_selected) = current_selected {
            commands.entity(**current_selected).remove::<Selected>();
            commands.remove_resource::<CurrentSelected>();
            }
            return;
        };
        commands.entity(entity).insert(Selected(pos));
        commands.insert_resource(CurrentSelected(entity));
    }
}
