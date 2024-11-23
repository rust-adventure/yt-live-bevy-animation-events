use bevy::{
    animation::AnimationTargetId,
    input::common_conditions::input_toggle_active,
    pbr::CascadeShadowConfigBuilder, prelude::*,
    scene::SceneInstanceReady,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(
                input_toggle_active(true, KeyCode::Escape),
            ),
        )
        .add_systems(Startup, setup)
        // .add_systems(Update, debug)
        .add_observer(enable_animations)
        .run();
}

const WAVE_MASK: u32 = 5;
fn enable_animations(
    _trigger: Trigger<SceneInstanceReady>,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut AnimationPlayer)>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut commands: Commands,
) {
    for (entity, mut player) in &mut query {
        // Create the nodes.
        let mut animation_graph = AnimationGraph::new();
        let blend_node = animation_graph
            .add_additive_blend(1.0, animation_graph.root);

        // everything other than the walking bones
        let wave_mask_bones = [
            AnimationTargetId::from_names(
                [
                    Name::new("Armature"),
                    Name::new("BoneArmR"),
                ]
                .iter(),
            ),
            AnimationTargetId::from_names(
                [
                    Name::new("Armature"),
                    Name::new("BoneBody"),
                ]
                .iter(),
            ),
            AnimationTargetId::from_names(
                [
                    Name::new("Armature"),
                    Name::new("BoneArmL"),
                ]
                .iter(),
            ),
        ];
        for target in wave_mask_bones {
            animation_graph.add_target_to_mask_group(
                target, WAVE_MASK,
            );
        }

        // everything other than the waving bones
        let walk_mask_bones = [
            AnimationTargetId::from_names(
                [
                    Name::new("Armature"),
                    Name::new("BoneLegR"),
                ]
                .iter(),
            ),
            AnimationTargetId::from_names(
                [
                    Name::new("Armature"),
                    Name::new("BoneBody"),
                ]
                .iter(),
            ),
            AnimationTargetId::from_names(
                [
                    Name::new("Armature"),
                    Name::new("BoneLegL"),
                ]
                .iter(),
            ),
            AnimationTargetId::from_names(
                [
                    Name::new("Armature"),
                    Name::new("BoneArmL"),
                ]
                .iter(),
            ),
        ];
        for target in walk_mask_bones {
            animation_graph
                .add_target_to_mask_group(target, 8);
        }

        animation_graph.add_clip_with_mask(
            asset_server.load(
                GltfAssetLabel::Animation(0)
                    .from_asset("character.glb"),
            ),
            32,
            1.,
            blend_node,
        );

        animation_graph.add_clip_with_mask(
            asset_server.load(
                GltfAssetLabel::Animation(1)
                    .from_asset("character.glb"),
            ),
            256,
            1.,
            blend_node,
        );
        dbg!(&animation_graph);

        // Add the graph.
        let handle = animation_graphs.add(animation_graph);

        // Save the assets in a resource.
        // commands.insert_resource(ExampleAnimationGraph(handle));

        commands.entity(entity).insert((
            AnimationGraphHandle(handle.clone()),
            // ExampleAnimationWeights::default(),
        ));
        // for &node_index in &CLIP_NODE_INDICES {
        player.play(0.into()).repeat();
        player.play(1.into()).repeat();
        player.play(2.into()).repeat();
        player.play(3.into()).repeat();
        // }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10., 7., 10.)
            .looking_at(Vec3::new(0.0, 2., 0.0), Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(500000.0, 500000.0),
            ),
        ),
        MeshMaterial3d(
            materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ),
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

    commands.spawn((
        Name::new("Character"),
        SceneRoot(
            asset_server.load(
                GltfAssetLabel::Scene(0)
                    .from_asset("character.glb"),
            ),
        ),
    ));
}
