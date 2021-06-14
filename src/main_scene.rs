use gdnative::{api::PathFollow, prelude::*};
use rand::Rng;

use crate::mob::Mob;

#[derive(NativeClass)]
#[inherit(Node)]
#[user_data(user_data::MutexData<Main>)]
#[register_with(Self::register_main)]
pub struct Main {
    #[property]
    mob: Ref<PackedScene>,
}

#[methods]
impl Main {
    fn register_main(_builder: &ClassBuilder<Self>) {}

    fn new(_owner: &Node) -> Self {
        Self {
            mob: PackedScene::new().into_shared(),
        }
    }

    #[export]
    fn on_mob_timer_timeout(&self, owner: &Node) {
        let mob_scene: Ref<KinematicBody, _> = instance_scene(&self.mob);
        let spawner = unsafe {
            owner.get_node_as::<Spatial>("Spawner")
            .unwrap()
        };
        let spawn_location = spawner.transform().origin;

        let target = unsafe { owner.get_node_as::<Spatial>("ShootAt").unwrap() };
        let target_position = target.transform().origin;

        let mob_scene = unsafe { mob_scene.into_shared().assume_safe() };
        owner.add_child(mob_scene, false);

        let mob = mob_scene.cast_instance::<Mob>().unwrap();
        mob.map_mut(|mob, mob_owner| {
            mob.initialize(
                &mob_owner,
                &spawn_location,
                &target_position,
            );
        })
        .unwrap();
    }
}

/// Root here is needs to be the same type (or a parent type) of the node that you put in the child
///   scene as the root. For instance Spatial is used for this example.
fn instance_scene<Root>(scene: &Ref<PackedScene, Shared>) -> Ref<Root, Unique>
where
    Root: gdnative::GodotObject<RefKind = ManuallyManaged> + SubClass<Node>,
{
    let scene = unsafe { scene.assume_safe() };

    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .expect("should be able to instance scene");

    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .expect("root node type should be correct")
}
