#![allow(unused_imports)]

// TODO: convert all source files too use ahprelude
// prelude then items not in prelude that are still needed in multiple spots.
// if import is only used in one file it can stay in that file ig
// non bevy based deps
pub use rand;
pub use serde;

/// imports for components and resources specifically for aspen halls
pub mod game {
    #[cfg(feature = "develop")]
    /// holds features/tools for inspecting state of application
    pub mod inspect {
        pub use crate::game_tools::debug_plugin::DebugPlugin;
        pub use bevy_inspector_egui::prelude::{InspectorOptions, ReflectInspectorOptions};
    }
    #[cfg(feature = "develop")]
    pub use inspect::*;

    pub use crate::{
        console::{
            command_systems::{spawnenemy_command, spawnweapon_command, teleport_player_command},
            commands::{SpawnEnemyCommand, SpawnWeaponCommand, TeleportPlayerCommand},
        },
        consts::*,
        game::{
            actors::{
                ai::components::{
                    AIChaseAction, AIChaseConfig, AIShootAction, AIShootConfig, AIWanderAction,
                    AIWanderConfig, ActorType, ChaseScore, Type,
                },
                animation::components::{ActorAnimationType, AnimState, AnimationSheet},
                combat::components::{
                    BarrelPointTag, CurrentlySelectedWeapon, DamageType, Weapon, WeaponColliderTag,
                    WeaponSlots, WeaponSocket, WeaponStats,
                },
                components::{
                    ActorCombatStats, ActorDerivedAttributes, ActorPrimaryAttributes,
                    ActorSecondaryAttributes, ActorTertiaryAttributes, Player, ProjectileStats,
                    TimeToLive,
                },
                spawners::components::{EnemyType, SpawnActorEvent, Spawner, WeaponType},
            },
            audio::{ActorSoundTimer, AmbienceSoundChannel, GameSoundChannel, MusicSoundChannel},
            input::action_maps::{self},
            TimeInfo,
        },
        // AudioHandles,
        loading::{
            assets::{
                ActorTextureHandles, AudioHandles, InitAssetHandles, MapAssetHandles,
                SingleTileTextureHandles, TouchControlAssetHandles,
            },
            config::{
                save_load::save_settings, ConfigFile, DifficultyScales, GameDifficulty,
                GeneralSettings, RenderSettings, SoundSettings, WindowSettings,
            },
            splashscreen::{MainCamera, OnlySplashScreen, SplashPlugin, SplashTimer},
        },
        utilities::{despawn_with, lerp, set_window_icon, GetEither, GetEitherMut},
        AppState,
    };
}

/// external and internal plugins from aspen halls and bevy
pub mod plugins {
    #[cfg(feature = "develop")]
    pub use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

    pub use bevy::{
        diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
        log::LogPlugin as BevyLogPlugin,
    };
    pub use bevy_debug_text_overlay::OverlayPlugin;
    pub use bevy_kira_audio::AudioPlugin;
    pub use bevy_mod_logfu::LogPlugin as LogFuPlugin;
    pub use bevy_rapier2d::render::RapierDebugRenderPlugin;
    pub use big_brain::BigBrainPlugin;

    pub use crate::{
        console::QuakeConPlugin,
        game::{interface::InterfacePlugin, AspenHallsPlugin},
        loading::{splashscreen::SplashPlugin, AppAssetLoadingPlugin},
    };
}

/// bevy engine and external plugins are imported under this module
///
/// modules are namespaces!!! 😜
pub mod engine {
    // bevy plugins with weird names

    pub use big_brain;
    pub use leafwing_input_manager;
    // bevy and bevy plugins
    pub use bevy;
    pub use bevy_common_assets;
    pub use bevy_console;
    pub use bevy_ecs_ldtk;
    pub use bevy_kira_audio;
    pub use bevy_rapier2d;

    pub use bevy_ecs_ldtk::prelude::*;
    pub use bevy_kira_audio::prelude::{
        Audio, AudioApp, AudioChannel, AudioCommandError, AudioControl, AudioEasing, AudioEmitter,
        AudioInstance, AudioInstanceAssetsExt, AudioReceiver, AudioSettings, AudioSource,
        AudioTween, DynamicAudioChannel, DynamicAudioChannels, FadeIn, FadeOut, Frame, MainTrack,
        PlayAudioCommand, PlaybackState, Sound as KiraSound, SoundData, SpacialAudio,
        StaticSoundData, StaticSoundSettings, TweenCommand, Volume,
    };

    pub use serde::{Deserialize, Serialize};

    pub use bevy::{
        app::{
            App, DynamicPlugin, First, FixedUpdate, Last, Main, Plugin, PluginGroup, PostStartup,
            PostUpdate, PreStartup, PreUpdate, Startup, StateTransition, Update,
        },
        core::prelude::{
            DebugName, FrameCountPlugin, Name, TaskPoolOptions, TaskPoolPlugin,
            TypeRegistrationPlugin,
        },
        core_pipeline::{
            clear_color::ClearColorConfig,
            tonemapping::{DebandDither, Tonemapping},
        },
        ecs::prelude::{
            Bundle, Component, Entity, RemovedComponents,
            {
                apply_deferred, apply_state_transition, IntoSystemConfigs, IntoSystemSet,
                IntoSystemSetConfigs, NextState, OnEnter, OnExit, OnTransition, Schedule,
                Schedules, State, States, SystemSet,
                {
                    any_component_removed, any_with_component, not, on_event, run_once, Condition,
                    {in_state, state_changed, state_exists, state_exists_and_equals},
                    {
                        resource_added, resource_changed, resource_changed_or_removed,
                        resource_equals, resource_exists, resource_exists_and_changed,
                        resource_removed,
                    },
                },
            },
            {Added, AnyOf, Changed, Or, QueryState, With, Without},
            {AppTypeRegistry, ReflectComponent, ReflectResource},
            {
                Commands, Deferred, In, IntoSystem, Local, NonSend, NonSendMut, ParallelCommands,
                ParamSet, Query, ReadOnlySystem, Res, ResMut, Resource, System,
                SystemParamFunction,
            },
            {DetectChanges, DetectChangesMut, Mut, Ref}, {EntityRef, FromWorld, World},
            {Event, EventReader, EventWriter, Events},
        },
        hierarchy::prelude::*,
        input::{
            prelude::{
                Axis, Input, MouseButton,
                {
                    Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType,
                    Gamepads,
                },
                {KeyCode, ScanCode}, {TouchInput, Touches},
            },
            InputSystem,
        },
        log::{debug_span, error_span, info_span, trace_span, warn_span, Level},
        math::{
            ivec2, ivec3,
            prelude::{
                {
                    BVec2, BVec3, BVec4, EulerRot, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, Quat,
                    Ray, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
                },
                {
                    CubicBSpline, CubicBezier, CubicCardinalSpline, CubicGenerator, CubicHermite,
                    CubicSegment,
                },
            },
            vec2, vec3, IRect, Rect as FRect, URect,
        },
        prelude::{bevy_main, *},
        reflect::prelude::{
            reflect_trait, FromReflect, GetField, GetTupleStructField, Reflect, ReflectDefault,
            ReflectDeserialize, ReflectFromReflect, ReflectSerialize, Struct, TupleStruct,
        },
        render::{
            camera::ScalingMode,
            primitives::Frustum,
            texture::{CompressedImageFormats, ImageType},
        },
        time::{
            prelude::{Fixed as FixedTime, Real as RealTime, Time, Timer, TimerMode},
            TimeSystem,
        },
        transform::prelude::*,
        utils::{dbg, default, error, info, warn, Duration},
        window::{
            prelude::{
                CursorEntered, CursorIcon, CursorLeft, CursorMoved, FileDragAndDrop, Ime,
                MonitorSelection, ReceivedCharacter, Window, WindowMoved, WindowPlugin,
                WindowPosition, WindowResizeConstraints, *,
            },
            CompositeAlphaMode, PresentMode, WindowFocused, WindowMode, WindowResized,
            WindowResolution, WindowScaleFactorChanged,
        },
    };

    pub use big_brain::{
        // big brain common imports
        prelude::{
            Action as Task, ActionBuilder as TaskBuilder, ActionSpan as TaskSpan,
            ActionState as TaskState, Actor as Brain, AllOrNothing, BigBrainSet, ChebyshevDistance,
            ConcurrentMode, Concurrently, EvaluatingScorer, Evaluator, FirstToScore, FixedScore,
            HasThinker, Highest, LinearEvaluator, Measure, MeasuredScorer, Picker, PowerEvaluator,
            ProductOfScorers, Score, Scorer, ScorerBuilder, ScorerSpan, SigmoidEvaluator, Steps,
            SumOfScorers, Thinker, WeightedProduct, WeightedSum, WinningScorer,
        },
        thinker::ThinkerBuilder,
    };

    pub use bevy_asset_loader::{
        prelude::*, standard_dynamic_asset::StandardDynamicAssetCollection,
    };

    pub use bevy_rapier2d::prelude::{Real, *};

    pub use leafwing_input_manager::{
        //leafwing common imports
        plugin::InputManagerSystem,
        prelude::{
            ActionState, ActionStateDriver, Actionlike, ClashStrategy, DeadZoneShape, DualAxis,
            InputManagerBundle, InputManagerPlugin, InputMap, MockInput, Modifier,
            MouseWheelAxisType, MouseWheelDirection, QwertyScanCode, SingleAxis, ToggleActions,
            UserInput, VirtualDPad,
        },
    };

    pub use bevy_mod_debugdump::{
        render_graph, render_graph_dot, schedule_graph, schedule_graph_dot,
    };
    pub use bevy_prototype_lyon::{
        draw as svg_draw,
        entity::ShapeBundle as SvgBundle,
        prelude::{Fill, FillOptions, GeometryBuilder},
        shapes as svg_shapes,
    };
}
