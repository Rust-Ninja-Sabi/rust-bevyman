use bevy::prelude::*;

#[derive(PartialEq)]
enum Gameobject {
    FoodObject,
    WallObject,
    PowerObject,
    Bevyman,
    GhostObject,
    Empty
}

struct Gamegrid {
    value: [[Gameobject;11];10], //col row
    min_x: f32,
    max_x: f32,
    min_z: f32,
    max_z: f32
}
impl Default for Gamegrid{
    fn default() -> Self {
        use self::Gameobject::*;
        Self {
            value:[[WallObject,WallObject,WallObject,WallObject,WallObject,FoodObject,WallObject,WallObject,WallObject,WallObject,WallObject],
                [WallObject,PowerObject,FoodObject,FoodObject,FoodObject,FoodObject,FoodObject,FoodObject,FoodObject,FoodObject,WallObject],
                [WallObject,FoodObject,WallObject,FoodObject,WallObject,WallObject,WallObject,FoodObject,WallObject,FoodObject,WallObject],
                [WallObject,FoodObject,FoodObject,FoodObject,FoodObject,FoodObject,FoodObject,FoodObject,FoodObject,FoodObject,WallObject],
                [WallObject,FoodObject,WallObject,FoodObject,WallObject,WallObject,WallObject,FoodObject,WallObject,FoodObject,WallObject],
                [FoodObject,FoodObject,FoodObject,FoodObject,WallObject,GhostObject,WallObject,FoodObject,FoodObject,FoodObject,FoodObject],
                [WallObject,FoodObject,WallObject,FoodObject,GhostObject,GhostObject,GhostObject,FoodObject,WallObject,FoodObject,WallObject],
                [WallObject,FoodObject,WallObject,WallObject,WallObject,FoodObject,WallObject,WallObject,WallObject,FoodObject,WallObject],
                [WallObject,FoodObject,FoodObject,FoodObject,FoodObject,Bevyman,FoodObject,FoodObject,FoodObject,PowerObject,WallObject],
                [WallObject,WallObject,WallObject,WallObject,WallObject,FoodObject,WallObject,WallObject,WallObject,WallObject,WallObject]],
            min_x: 0.0,
            max_x: 10.0,
            min_z: 0.0,
            max_z: 9.0,
        }
    }
}

impl Gamegrid {
    fn to3d(&self, x:usize, y:usize, height:f32)->Vec3{
        Vec3::new(x as f32 * 1.0, height, y as f32 * 1.0)
    }

    fn to_map_x(&self, x:f32, _y:f32, _z:f32)->usize {
        x.round() as usize
    }

    fn to_map_y(&self, _x:f32, _y:f32, z:f32)->usize {
        z.round() as usize
    }
}

struct Score {
    foodcounter:i32,
    points:i32,
    times:i32,
}
impl Default for Score{
    fn default() -> Self {
        Self {
            foodcounter:0,
            points:0,
            times:3
        }
    }
}

#[derive(Component)]
struct Ghost;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Food;

#[derive(Component)]
struct Power;

#[derive(Component)]
struct Wall;

fn main() {
    App::new()
        //add config resources
        .insert_resource(Msaa {samples: 4})
        .insert_resource(WindowDescriptor{
            title: "bevyman".to_string(),
            width: 640.0,
            height: 400.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(Gamegrid::default())
        .insert_resource(Score::default())
        //bevy itself
        .add_plugins(DefaultPlugins)
        //resources
        //.insert_resource(Score{value:0})
        // if it implements `Default` or `FromWorld`
        //.init_resource::<MyFancyResource>()
        // events:
        //.add_event::<LevelUpEvent>()
        // system once
        .add_startup_system(setup)
        // system frame
        .add_system(move_player)
        .run();
}

fn setup(
    mut commands: Commands,
    gamegrid: Res<Gamegrid>,
    mut ambient_light: ResMut<AmbientLight>,
    mut score: ResMut<Score>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    ambient_light.color = Color::WHITE;

    //camera
    commands.spawn_bundle(PerspectiveCameraBundle{
        transform: Transform::from_xyz(5.0,10.0,12.0).looking_at(Vec3::new(5.,0.,5.), Vec3::Y),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle{
        point_light: PointLight{
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(5.0, 7.0, 5.0),
        ..Default::default()
    });

    // plane
    commands.spawn_bundle(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Plane{size: 11.0})),
        material:materials.add(Color::rgb(0.8,0.8,0.8).into()),
        transform: Transform::from_xyz(5., 0., 5.),
        ..Default::default()
    });

    // gamegrid
    for (y, row) in gamegrid.value.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            match col {
                Gameobject::Bevyman => {
                    // player
                    //commands.spawn_bundle((
                    //    Transform::from_xyz(0.0, 0.0, 0.0),
                    //    GlobalTransform::identity(),
                    //    ))
                    //    .with_children(|parent| {
                    //        parent.spawn_scene(asset_server.load("models/bevyman.gltf#Scene0"));
                    //    })
                    //    .insert(Player);
                    let start = gamegrid.to3d(x,y,0.5);
                    commands.spawn_bundle(PbrBundle{
                        mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.50, subdivisions: 32, })),
                        material:materials.add(Color::YELLOW.into()),
                        transform: Transform::from_translation(start),
                        ..Default::default()
                    })
                    .insert(Player);
                },
                Gameobject::FoodObject => {
                    commands.spawn_bundle(PbrBundle{
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25})),
                        material:materials.add(Color::GREEN.into()),
                        transform: Transform::from_translation(gamegrid.to3d(x,y,0.5)),
                        ..Default::default()
                    })
                        .insert(Food);
                    score.foodcounter +=1;
                }
                Gameobject::PowerObject => {
                    commands.spawn_bundle(PbrBundle{
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25})),
                        material:materials.add(Color::PINK.into()),
                        transform: Transform::from_translation(gamegrid.to3d(x,y,0.5)),
                        ..Default::default()
                    })
                        .insert(Power);
                    score.foodcounter +=1;
                }
                Gameobject::WallObject => {
                    commands.spawn_bundle(PbrBundle{
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0})),
                        material:materials.add(Color::GRAY.into()),
                        transform: Transform::from_translation(gamegrid.to3d(x,y,0.5)),
                        ..Default::default()
                    })
                        .insert(Wall);
                }
                Gameobject::GhostObject => {
                    commands.spawn_bundle(PbrBundle{
                        mesh: meshes.add(Mesh::from(shape::Capsule {
                            radius: 0.5,
                            depth: 1.0,
                            ..Default::default()
                        })),
                        material:materials.add(Color::BLUE.into()),
                        transform: Transform::from_translation(gamegrid.to3d(x,y,0.5)),
                        ..Default::default()
                    })
                    .insert(Ghost);
                }
                _ => {}
            }
        }
    }
}

const PLAYER_SPEED:f32=1.0;

fn move_player(
    time:Res<Time>,
    keyboard_input:Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, With<Player>)>
){
    for (mut transform,_) in query.iter_mut() {
        let mut direction = Vec3::new(0.,0.,0.);
        if keyboard_input.pressed(KeyCode::Left) {
            direction = Vec3::new(-1.0,0.,0.)
        } else if keyboard_input.pressed(KeyCode::Right) {
            direction = Vec3::new(1.0,0.,0.)
        } else if keyboard_input.pressed(KeyCode::Up) {
            direction = Vec3::new(0.,0.,-1.)
        } else if keyboard_input.pressed(KeyCode::Down) {
            direction = Vec3::new(0.,0.,1.)
        }
        transform.translation = transform.translation + direction * PLAYER_SPEED * time.delta_seconds();
    }
}
