use gdnative::prelude::*;
mod hello;
mod spin;

fn init(handle: InitHandle) {
    handle.add_class::<hello::HelloWorld>();
    handle.add_class::<spin::Spin>();
}

godot_init!(init);