use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Label)]
#[user_data(user_data::MutexData<ScoreLabel>)]
pub struct ScoreLabel {
    score: i64,
}

#[methods]
impl ScoreLabel {
    fn new(_owner: &Label) -> Self {
        Self { score: 0 }
    }

    #[export]
    fn on_mob_squashed(&mut self, owner: &Label) {
        self.score += 1;
        owner.set_text(format!("Score: {}", self.score));
    }
}
