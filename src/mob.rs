use gdnative::{api::AnimationPlayer, prelude::*};
use rand::Rng;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
#[user_data(user_data::MutexData<Mob>)]
#[register_with(Self::register_mob)]
pub struct Mob {
    #[property(default = 10.0)]
    min_speed: f32,
    #[property(default = 18.0)]
    max_speed: f32,
    velocity: Vector3,
}

#[methods]
impl Mob {
    fn register_mob(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "squashed",
            args: &[],
        });
    }

    fn new(_owner: &KinematicBody) -> Self {
        Self {
            min_speed: 10.0,
            max_speed: 18.0,
            velocity: Vector3::ZERO,
        }
    }

    pub fn squash(&mut self, owner: &KinematicBody) {
        owner.emit_signal("squashed", &[]);
        unsafe {
            owner.assume_unique().queue_free();
        }
    }

    pub fn initialize(
        &mut self,
        owner: &KinematicBody,
        start_position: &Vector3,
        player_position: &Vector3,
    ) {
        owner.set_translation(*start_position);
        owner.look_at(*player_position, Vector3::UP);
        let mut rng = rand::thread_rng();
        let pi = std::f64::consts::PI;
        owner.rotate_y(rng.gen_range(-pi / 4.0..pi / 4.0));

        let random_speed = rng.gen_range(self.min_speed..self.max_speed);
        self.velocity = Vector3::FORWARD * random_speed;
        let rotation = owner.rotation();
        self.velocity = self.velocity.rotated(Vector3::UP, rotation.y);

        let animation_player = unsafe { owner.get_node_as::<AnimationPlayer>("AnimationPlayer").unwrap() };
        animation_player.set_speed_scale((random_speed / self.min_speed) as f64);
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody, _delta: f32) {
        // Moving the character
        owner.move_and_slide(
            self.velocity,
            // Defaults listed in the documentation
            Vector3::new(0.0, 0.0, 0.0),
            false,
            4,
            0.785398,
            true,
        );
    }

    #[export]
    fn on_visibility_screen_exited(&self, owner: &KinematicBody) {
        unsafe {
            owner.assume_unique().queue_free();
        }
    }
}
