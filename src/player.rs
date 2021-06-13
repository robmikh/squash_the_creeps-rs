use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
#[user_data(user_data::MutexData<Player>)]
#[register_with(Self::register_player)]
pub struct Player {
    #[property(default = 14.0)]
    speed: f32,
    #[property(default = 75.0)]
    fall_acceleration: f32,
    velocity: Vector3,
}

#[methods]
impl Player {
    fn register_player(_builder: &ClassBuilder<Self>) {
    }

    fn new(_owner: &KinematicBody) -> Self {
        Self {
            speed: 14.0,
            fall_acceleration: 75.0,
            velocity: Vector3::zero(),
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody, delta: f32) {
        let input = Input::godot_singleton();
        let mut direction = Vector3::zero();

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

        if direction != Vector3::zero() {
            direction = direction.normalize();
            let pivot = unsafe {
                owner.get_node_as::<Spatial>("Pivot").unwrap()
            };
            pivot.look_at(owner.translation() + direction, Vector3::new(0.0, 1.0, 0.0));
        }

        // Ground velocity
        self.velocity.x = direction.x * self.speed;
        self.velocity.z = direction.z * self.speed;
        // Vertical velocity
        self.velocity.y -= self.fall_acceleration * delta;
        // Moving the character
        owner.move_and_slide(
            self.velocity, 
            Vector3::new(0.0, 1.0, 0.0),
            // Defaults listed in the documentation
            false,
            4,
            -0.785398,
            true
        );
    }
}