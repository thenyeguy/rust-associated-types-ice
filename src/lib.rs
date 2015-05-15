mod types;
pub struct Container {
    pub value: Option<Box<types::TestTrait<Input=f32, Output=f32>>>
}
