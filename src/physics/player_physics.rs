use super::{GameObject, PhysicsSystem};
use ggez::graphics::Rect;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra::Point2;
use ggez::{input, Context};

const MOVE_FORCE: f32 = 2.0;
const JUMP_FORCE: f32 = -8.5;
const FRICTION: f32 = 0.15;

pub struct PlayerPhysics {
    velocity: Point2<f32>,
    affected_by_gravity: bool,
}

impl PlayerPhysics {
    pub fn new() -> PlayerPhysics {
        PlayerPhysics {
            velocity: Point2::new(0.0, 0.0),
            affected_by_gravity: true,
        }
    }

    fn handle_input(&mut self, context: &mut Context) {
        if input::keyboard::is_key_pressed(context, KeyCode::A) {
            self.velocity.x -= MOVE_FORCE;
        } else if input::keyboard::is_key_pressed(context, KeyCode::S) {
            self.velocity.x += MOVE_FORCE;
        }

        if input::keyboard::is_key_pressed(context, KeyCode::Space) && self.on_ground() {
            self.velocity.y += JUMP_FORCE;
            self.affected_by_gravity = true;
        }
    }

    fn on_ground(&self) -> bool {
        !self.affected_by_gravity
    }

    fn stay_in_arena(&mut self, location: &mut Rect, (arena_width, arena_height): (f32, f32)) {
        if location.y + location.h > arena_height {
            self.affected_by_gravity = false;
            self.velocity.y = 0.0;
            location.y = arena_height - location.h;
        }

        if location.x < 0.0 {
            location.x = 0.0;
        } else if location.x + location.w > arena_width {
            location.x = arena_width - location.w;
        }
    }

    fn get_colliding_with(
        &self,
        collidable_game_objects: &Vec<GameObject>,
        location: &Rect,
    ) -> Option<GameObject> {
        for other_game_object in collidable_game_objects {
            if other_game_object.location.overlaps(location) {
                return Some(other_game_object.clone());
            }
        }
        None
    }
}

impl PhysicsSystem for PlayerPhysics {
    fn update(
        &mut self,
        location: &mut Rect,
        arena: (f32, f32),
        gravity_force: f32,
        context: &mut Context,
        collidable_game_objects: &Vec<GameObject>,
    ) {
        self.handle_input(context);
        self.stay_in_arena(location, arena);

        if let Some(game_object) = self.get_colliding_with(collidable_game_objects, location) {
            println!("We collided with a game object!");
        }

        if self.affected_by_gravity {
            self.velocity.y += gravity_force;
        }
        location.x += self.velocity.x;
        location.y += self.velocity.y;

        if self.velocity.x != 0.0 {
            let opposite_velocity = self.velocity.x * -1.0;
            let speed_decrease = opposite_velocity * FRICTION;
            self.velocity.x += speed_decrease
        }
    }
}