use gdnative::prelude::*;

use crate::mob::Mob;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
#[user_data(user_data::MutexData<Player>)]
pub struct Player {
    #[property(default = 14.0)]
    speed: f32,
    #[property(default = 75.0)]
    fall_acceleration: f32,
    #[property(default = 20.0)]
    jump_impulse: f32,
    #[property(default = 16.0)]
    bounce_impulse: f32,
    velocity: Vector3,
}

#[methods]
impl Player {
    fn new(_owner: &KinematicBody) -> Self {
        Self {
            speed: 14.0,
            fall_acceleration: 75.0,
            jump_impulse: 20.0,
            bounce_impulse: 16.0,
            velocity: Vector3::ZERO,
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody, delta: f32) {
        let input = Input::godot_singleton();
        let mut direction = Vector3::ZERO;

        if input.is_action_pressed("move_left") {
            direction.x -= 1.0;
        }
        if input.is_action_pressed("move_right") {
            direction.x += 1.0;
        }
        if input.is_action_pressed("move_forward") {
            direction.z -= 1.0;
        }
        if input.is_action_pressed("move_back") {
            direction.z += 1.0;
        }

        if direction != Vector3::ZERO {
            direction = direction.normalized();
            let pivot = unsafe { owner.get_node_as::<Spatial>("Pivot").unwrap() };
            pivot.look_at(owner.translation() + direction, Vector3::UP);
        }

        // Ground velocity
        self.velocity.x = direction.x * self.speed;
        self.velocity.z = direction.z * self.speed;
        // Jumping
        if owner.is_on_floor() && input.is_action_just_pressed("jump") {
            self.velocity.y += self.jump_impulse;
        }

        // Vertical velocity
        self.velocity.y -= self.fall_acceleration * delta;

        // Moving the character
        self.velocity = owner.move_and_slide(
            self.velocity,
            Vector3::UP,
            // Defaults listed in the documentation
            false,
            4,
            0.785398,
            true,
        );

        for index in 0..owner.get_slide_count() {
            let collision = owner.get_slide_collision(index).unwrap();
            let collision = unsafe { collision.assume_safe() };
            let collider = collision.collider().unwrap();
            let collider = unsafe { collider.assume_safe() };
            let collider = collider.cast::<Node>().unwrap();
            if collider.is_in_group("mob") {
                let mob = collider.cast::<KinematicBody>().unwrap();
                let mob = mob.cast_instance::<Mob>().unwrap();
                mob.map_mut(|mob, owner| {
                    mob.squash(&owner);
                })
                .unwrap();
                self.velocity.y = self.bounce_impulse;
            }
        }
    }
}
