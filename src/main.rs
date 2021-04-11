use bevy::prelude::*;

pub struct HelloPlugin;

struct GreetTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_system(hello_world.system());
    }
}

fn hello_world(time: Res<Time>, mut timer: ResMut<GreetTimer>) {
    if timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    println!("hello world");
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
