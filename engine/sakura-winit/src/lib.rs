//! sakura-winit
//!
//! A wrapper library around winit and wgpu to make it easier to create windows and handle input.

use std::{error::Error, sync::Arc};

use winit::{dpi::LogicalSize, event_loop::EventLoopWindowTarget, window::WindowBuilder};

pub struct Viewport<'window> {
    window: Arc<winit::window::Window>,
    background: wgpu::Color,
    surface: wgpu::Surface<'window>,
    config: wgpu::SurfaceConfiguration,
}

/// A builder for creating a [Viewport] instance.
pub struct ViewportBuilder {
    inner: WindowBuilder,
    color: wgpu::Color,
}

impl Default for ViewportBuilder {
    fn default() -> Self {
        Self {
            inner: winit::window::WindowBuilder::new(),
            color: wgpu::Color::BLACK,
        }
    }
}

impl ViewportBuilder {
    /// Apply a function to the inner `WindowBuilder`.
    pub fn with(self, f: impl FnOnce(WindowBuilder) -> WindowBuilder) -> Self {
        Self {
            inner: f(self.inner),
            ..self
        }
    }

    /// Set the dimensions of the window. This is not guaranteed to be the final size of the window,
    /// as the window manager may impose its own constraints.
    pub fn dimensions(self, width: u32, height: u32) -> Self {
        self.with(|w| w.with_inner_size(LogicalSize::new(width, height)))
    }

    /// Set the title of the window.
    pub fn title(self, title: &str) -> Self {
        self.with(|w| w.with_title(title))
    }

    /// Set the background color of the window.
    pub fn color(self, color: wgpu::Color) -> Self {
        Self { color, ..self }
    }

    /// Initialize the window with the given title.
    pub fn build<'a, T>(
        self,
        event_loop: &'a EventLoopWindowTarget<T>,
        instance: &'a wgpu::Instance,
        adapter: &'a wgpu::Adapter,
        device: &'a wgpu::Device,
    ) -> Result<Viewport<'a>, Box<dyn Error>>
    where
        T: 'static,
    {
        // create window
        let window = Arc::new(self.inner.build(event_loop)?);
        let size = window.inner_size();

        // create surface
        let surface = instance.create_surface(window.clone())?;
        let config = surface
            .get_default_config(adapter, size.width, size.height)
            .ok_or("No default config found")?;
        surface.configure(device, &config);

        // return
        Ok(Viewport {
            window,
            background: self.color,
            surface,
            config,
        })
    }
}

impl Viewport<'_> {
    /// Resize the window.
    fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(device, &self.config);
    }
}
