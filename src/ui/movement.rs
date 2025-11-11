use bevy::prelude::*;

use crate::{board::{Board, MainBoard, PossibleMovement}, interactive::BoardPos, ui::Cursor};

const DELAY_FOR_LAYER_TO_SHOW : f32 = 0.05;

#[derive(Message)]
pub struct ShowMovementUi {
    position: UVec2,
    layer: u32,
}

impl From<PossibleMovement> for ShowMovementUi {
    fn from(value: PossibleMovement) -> Self {
        Self {
            position: value.position,
            layer: value.layer
        }
    }
}

#[derive(Resource, Default)]
pub struct ShownPositions {
    pub movement: Vec<Entity>
    // attack
}

impl ShownPositions {
    fn reset(&mut self, cmds: &mut Commands) {
        for elem in self.movement.iter() {
            cmds.entity(*elem).despawn()
        }
        self.movement = Vec::default();
    }
}

#[derive(Component)]
struct MovementOption;

#[derive(Component, Deref, DerefMut)]
pub struct Delay(Timer);


pub fn apply_visibility_delayed(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Delay)>,
    time: Res<Time>
) {
    for (entity, mut delay) in entities.iter_mut(){
        delay.tick(time.delta());
        if delay.is_finished() {
            let mut entity = commands.entity(entity);
            entity.remove::<Delay>();
            entity.insert(Visibility::Inherited);
        }
    }
}

pub fn on_shown_movement(
    mut commands: Commands,
    board_entity: Single<Entity, With<MainBoard>>,
    mut on_show: MessageReader<ShowMovementUi>,
    mut current: ResMut<ShownPositions>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){

    for msg in on_show.read() {
        let board_helper = BoardPos::from(msg.position);
        commands.entity(board_entity.entity()).with_children(|parent| {
            let new_entity = parent.spawn((
                MovementOption,
                Transform::from_translation(board_helper.get_screen_pos(0)+vec3(1.0, 1.0, 0.0)),
                Mesh2d(meshes.add(Rectangle::new(30., 30.))),
                MeshMaterial2d(materials.add(Color::linear_rgba(0., 0., 1., 0.3))),
                Visibility::Hidden,
                Delay(Timer::from_seconds(msg.layer as f32 * DELAY_FOR_LAYER_TO_SHOW, TimerMode::Once))
            )).id();
            current.movement.push(new_entity);
        });
    }
}


pub fn on_click_cursor(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    board: Res<Board>,
    cursor: Single<&Cursor>,
    mut shown: ResMut<ShownPositions>,
    mut movement_writer: MessageWriter<ShowMovementUi>) {
        if mouse.just_pressed(MouseButton::Left){
            let pos = cursor.position;
            shown.reset(&mut commands);
            let Some(unit) = board.units.get(&pos) else {
                return
            };
            let possible_movements = unit.get_movements(pos,&board);
            movement_writer.write_batch(possible_movements.into_iter().map(|mov| mov.into()));
        }
}
