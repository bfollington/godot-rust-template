use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Spin;

#[methods]
impl Spin {
    fn new (_owner: &Spatial) -> Self {
        Spin
    }

    #[export]
    fn _ready(&self, _owner: &Spatial) {
        godot_print!("Hello, world!");
    }

    #[export]
    fn _process(&self, owner: &Spatial, delta: f32) {
        let axis = Vector3::new(1.0, 1.0, 1.0).normalize();
        owner.rotate(axis, 0.1);
        godot_print!("Update {}", delta);
    }
}