use bevy::{prelude::*, reflect::TypeRegistry, utils::Duration};
use bevy_inspector_egui::WorldInspectorPlugin;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(print_ent_count.exclusive_system())
        .add_startup_system(create_player)
        .add_system(load_on_l.exclusive_system())
        .add_system(save_on_s.exclusive_system())
        .add_system(print_ent_count.exclusive_system())
        .run();
}

fn print_ent_count(world: &mut World) {
    println!("{}", world.entities().len());
}

fn create_player(mut commands: Commands, asset_server:Res<AssetServer>) {
    let player = commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("awesomeface.png"),
        //transform: Transform {
            //scale: Vec3::splat(0.01),
            //..Default::default()
        //},
        ..Default::default()
    });
    let mut cam = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(cam);
}

fn load_on_l(world: &mut World) {
    let keyboard = world.get_resource::<Input<KeyCode>>().unwrap();
    if keyboard.just_pressed(KeyCode::L) {

        world.clear_entities();

        let scene_handle: Handle<DynamicScene> ={   
            let asset_server=  world.get_resource::<AssetServer>().unwrap();
            let test: Handle<Image> = asset_server.load("awesomeface.png");
            asset_server.load("savefile.scn.ron")
        };

        let mut scene_spawner =  world.get_resource_mut::<SceneSpawner>().unwrap(); 
        scene_spawner.spawn_dynamic(scene_handle);
    }
}

fn save_on_s(world: &mut World) {
    let keyboard = world.get_resource::<Input<KeyCode>>().unwrap();
    if keyboard.just_pressed(KeyCode::P) {

        let type_registry = world.get_resource::<TypeRegistry>().unwrap();
        let scene = DynamicScene::from_world(&world, type_registry);

        //info!("{}", scene.serialize_ron(type_registry).unwrap());
        let path = Path::new("assets/savefile.scn.ron");
        let mut file = File::create(path).unwrap();
        write!(file, "{}", scene.serialize_ron(type_registry).unwrap());

    }

}