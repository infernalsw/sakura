//! Sakura is a game engine, or hopefully will be one day.

use sakura_runtime::RuntimeBuilder;
use sakura_winit::ViewportBuilder;

/// A game instance.
pub struct Game {
    script_runtime_builder: RuntimeBuilder,
    viewport_builder: ViewportBuilder,
}

impl Game {
    /// Create a new game instance.
    pub fn new() -> Self {
        Self {
            script_runtime_builder: RuntimeBuilder::default(),
            viewport_builder: ViewportBuilder::default(),
        }
    }

    /// Start the game.
    pub fn run(self) {
        // scripting runtime
        self.script_runtime_builder
            .with_rust_module("sakura_math", sakura_math::module)
            .build()
            .expect("failed to build scripting runtime")
            .process_events();

        // tokio runtime
        let runtime = tokio::runtime::Runtime::new().expect("failed to initialize tokio runtime");

        // gpu instance
        let gpu_instance = runtime.block_on(sakura_gpu::Instance::default().initialize());
    }
}
