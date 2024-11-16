use bevy::{
    pbr::CascadeShadowConfigBuilder, prelude::*,
    scene::SceneInstanceReady,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_observer(enable_animations)
        .run();
}

fn enable_animations(
    trigger: Trigger<SceneInstanceReady>,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut AnimationPlayer)>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut commands: Commands,
) {
    for (entity, mut player) in &mut query {
        info!("adding graph");
        // Create the nodes.
        let mut animation_graph = AnimationGraph::new();
        animation_graph.add_clip(
            asset_server.load(
                GltfAssetLabel::Animation(0)
                    .from_asset("character.glb"),
            ),
            1.0,
            animation_graph.root,
        );

        // Add the graph.
        let handle = animation_graphs.add(animation_graph);

        // Save the assets in a resource.
        // commands.insert_resource(ExampleAnimationGraph(handle));

        commands.entity(entity).insert((
            AnimationGraphHandle(handle.clone()),
            // ExampleAnimationWeights::default(),
        ));
        // for &node_index in &CLIP_NODE_INDICES {
        player.play(1.into()).repeat();
        // }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut AnimationPlayer)>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10., 10., 10.)
            .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .build(),
    ));

    commands.spawn(SceneRoot(
        asset_server.load(
            GltfAssetLabel::Scene(0)
                .from_asset("character.glb"),
        ),
    ));
}

// fn setup_assets_programmatically(
//     commands: &mut Commands,
//     asset_server: &mut AssetServer,
// ) {
// }

// fn init_animations(
//     mut commands: Commands,
//     mut query: Query<(Entity, &mut AnimationPlayer)>,
//     animation_graph: Res<ExampleAnimationGraph>,
//     mut done: Local<bool>,
// ) {
//     if *done {
//         return;
//     }

//     for (entity, mut player) in query.iter_mut() {
//         commands.entity(entity).insert((
//             AnimationGraphHandle(animation_graph.0.clone()),
//             ExampleAnimationWeights::default(),
//         ));
//         for &node_index in &CLIP_NODE_INDICES {
//             player.play(node_index.into()).repeat();
//         }

//         *done = true;
//     }
// }
