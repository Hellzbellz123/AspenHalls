use bevy::{
    core_pipeline::fullscreen_vertex_shader::FULLSCREEN_SHADER_HANDLE,
    prelude::{Handle, Image, Vec3},
    reflect::{TypePath, TypeUuid},
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, PrimitiveState, RenderPipelineDescriptor,
            ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey},
};
use bevy_tiling_background::ScrollingBackground;

// TODO:
// fix scaling with camera zoom, related too camera view_projection in shader code

/// background material that is tiled and scaled across the whole screen.
/// moves with the camera, make sure `NoFrustumCulling` component is added else
/// you will get strange behavior
#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath, Default)]
#[uuid = "09756d79-32e9-4dc4-bb95-b373370815e3"]
pub struct ScaledBackgroundMaterial {
    /// how much image moves relative too camera
    #[uniform(0)]
    pub movement_scale: f32,
    /// 3 extra f32 for wasm padding because wasm wants 16byte structs
    #[uniform(0)]
    pub _wasm_padding: Vec3,
    /// This image must have its [`SamplerDescriptor`] address_mode_* fields set to
    /// [`AddressMode::Repeat`].
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for ScaledBackgroundMaterial {
    fn vertex_shader() -> ShaderRef {
        FULLSCREEN_SHADER_HANDLE.typed().into()
    }
    fn fragment_shader() -> ShaderRef {
        "packs/asha/shaders/scaledbg.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _: &MeshVertexBufferLayout,
        _: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive = PrimitiveState::default();
        descriptor.vertex.entry_point = "fullscreen_vertex_shader".into();
        Ok(())
    }
}

impl ScrollingBackground for ScaledBackgroundMaterial {
    fn set_movement(&mut self, movement: f32) {
        self.movement_scale = movement;
    }
}
