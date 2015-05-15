mod types;
pub type Object = Option<Box<types::TestTrait<Input=f32, Output=f32>>>;
