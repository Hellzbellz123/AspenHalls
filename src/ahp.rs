#![allow(unused_imports)]

// TODO: convert all source files too use ahprelude
// prelude then items not in prelude that are still needed in multiple spots.
// if import is only used in one file it can stay in that file ig
// non bevy based deps
pub use rand;
pub use serde;

pub mod aspen_lib {
    #[cfg(feature = "inspect")]
    pub mod inspect {
        pub use crate::dev_tools::debug_plugin::DebugPlugin;
        pub use bevy_inspector_egui::prelude::{InspectorOptions, ReflectInspectorOptions};
    }
    #[cfg(feature = "inspect")]
    pub use inspect::*;

    pub use crate::{
        console::{
            command_systems::{spawnenemy_command, spawnweapon_command, teleport_player_command},
            commands::{SpawnEnemyCommand, SpawnWeaponCommand, TeleportPlayerCommand},
        },
        game::{
            input::action_maps::{self, Gameplay},
            actors::{
                ai::components::{AIChaseConfig, AIShootConfig, AIWanderConfig, ActorType},
                animation::components::{AnimState, AnimationSheet},
                components::{
                    ActorCombatStats, ActorDerivedAttributes, ActorPrimaryAttributes,
                    ActorSecondaryAttributes, ActorTertiaryAttributes, ProjectileStats, TimeToLive,
                },
            },
            audio::{AmbienceSoundChannel, MusicSoundChannel, GameSoundChannel, WalkingSoundTimer},
        },
        loading::{
            assets::{
                ActorTextureHandles, AudioHandles, InitAssetHandles, MapAssetHandles,
                SingleTileTextureHandles, TouchControlAssetHandles,
            },
            config::{
                ConfigFile, DifficultyScales, GameDifficulty, GeneralSettings, RenderSettings,
                SoundSettings, WindowSettings,
            },
            splashscreen::{MainCameraTag, OnlySplashScreen, SplashPlugin, SplashTimer},
        },
        utilities::{despawn_with, lerp, set_window_icon, GetEither, GetEitherMut},
        AppStage,
    };
}

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
        AudioInstance, AudioInstanceAssetsExt, AudioPlugin, AudioReceiver, AudioSettings,
        AudioSource, AudioTween, DynamicAudioChannel, DynamicAudioChannels, FadeIn, FadeOut, Frame,
        MainTrack, PlayAudioCommand, PlaybackState, Sound as KiraSound, SoundData, SpacialAudio,
        StaticSoundData, StaticSoundSettings, TweenCommand, Volume,
    };

    pub use serde::{Deserialize, Serialize};

    pub use bevy::{
        core_pipeline::{
            clear_color::ClearColorConfig,
            tonemapping::{DebandDither, Tonemapping},
        },
        ecs::reflect::ReflectResource,
        input::InputSystem,
        log::LogPlugin,
        prelude::*,
        reflect::Reflect,
        render::{
            camera::ScalingMode,
            primitives::Frustum,
            texture::{CompressedImageFormats, ImageType},
        },
        window::{PresentMode, WindowMode, WindowResized, WindowResolution},
    };

    pub use big_brain::{
        // big brain common imports
        prelude::{
            Action, ActionBuilder, ActionSpan, ActionState as BBActionState, Actor, AllOrNothing,
            BigBrainPlugin, BigBrainSet, ChebyshevDistance, ConcurrentMode, Concurrently,
            EvaluatingScorer, Evaluator, FirstToScore, FixedScore, HasThinker, Highest,
            LinearEvaluator, Measure, MeasuredScorer, Picker, PowerEvaluator, ProductOfScorers,
            Score, Scorer, ScorerBuilder, ScorerSpan, SigmoidEvaluator, Steps, SumOfScorers,
            Thinker, WeightedProduct, WeightedSum, WinningScorer,
        },
        thinker::ThinkerBuilder,
    };

    pub use bevy_asset_loader::{
        prelude::*, standard_dynamic_asset::StandardDynamicAssetCollection,
    };

    pub use bevy_rapier2d::prelude::*;

    pub use leafwing_input_manager::{
        //leafwing common imports
        plugin::InputManagerSystem,
        prelude::{
            ActionState as LIMActionState, ActionStateDriver, Actionlike, ClashStrategy,
            DeadZoneShape, DualAxis, InputManagerBundle, InputManagerPlugin, InputMap, MockInput,
            Modifier, MouseWheelAxisType, MouseWheelDirection, QwertyScanCode, SingleAxis,
            ToggleActions, UserInput, VirtualDPad,
        },
    };
}
