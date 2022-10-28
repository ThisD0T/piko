use bevy::prelude::*;

use rand::prelude::*;

use crate::GameState;

pub struct ColourPlugin;

impl Plugin for ColourPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing))
            .add_startup_system_to_stage(StartupStage::PreStartup, colourscheme_initializer);
    }
}

pub struct ColourScheme {
    pub colour_0: Color,
    pub colour_1: Color,
    pub colour_2: Color,
    pub wall_colour: Color,
}

fn colourscheme_initializer(mut commands: Commands) {
    generate_colourscheme(&mut commands);
}

pub fn generate_colourscheme(commands: &mut Commands) {
    let mut rng = rand::thread_rng();

    let mut hsla: Vec<f32> = Vec::new();

    let mut base_colour = rng.gen_range(0.0..360.0);
    hsla.push(base_colour);
    hsla.push(1.0);
    hsla.push(0.5);
    hsla.push(1.0);

    let colour_0 = Color::hsla(hsla[0], hsla[1], hsla[2], hsla[3]);
    hsla[0] += 180.0;
    if hsla[0] > 360.0 {
        hsla[0] -= 360.0
    }
    let colour_1 = Color::hsla(hsla[0], hsla[1], hsla[2], hsla[3]);
    println!("hsla: {}, {}, {}, {}", hsla[0], hsla[1], hsla[2], hsla[3]);
    hsla[0] += 48.0;
    if hsla[0] > 360.0 {
        hsla[0] -= 360.0
    }
    let colour_2 = Color::hsla(hsla[0], hsla[1], hsla[2], hsla[3]);

    base_colour += 180.0;
    if base_colour > 360.0 {
        base_colour -= 360.0
    }
    let wall_colour = Color::hsla(base_colour, hsla[1] - 0.8, hsla[2] - 0.2, hsla[3]);

    commands.insert_resource(ColourScheme {
        colour_0,
        colour_1,
        colour_2,
        wall_colour,
    });
}
