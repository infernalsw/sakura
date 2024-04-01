use std::error::Error;

///! Sakura GPU is a library for creating and managing a [wgpu] GPU context.

/// A wrapper around [wgpu::Instance].
#[derive(Default)]
pub struct Instance {
    inner: InstanceStage,
}

impl Instance {
    /// Initialise the GPU context.
    pub async fn initialize(self) -> Result<Instance, Box<dyn Error>> {
        self.load_adapter().await?.load_device().await
    }

    /// Get the device.
    pub fn device(&self) -> &wgpu::Device {
        match &self.inner {
            InstanceStage::DeviceQueue(_, _, device, _) => device,
            _ => panic!("Instance not initialised correctly."),
        }
    }

    /// Get the queue.
    pub fn queue(&self) -> &wgpu::Queue {
        match &self.inner {
            InstanceStage::DeviceQueue(_, _, _, queue) => queue,
            _ => panic!("Instance not initialised correctly."),
        }
    }

    /// Get the adapter.
    pub fn adapter(&self) -> &wgpu::Adapter {
        match &self.inner {
            InstanceStage::Adapter(_, adapter) => adapter,
            _ => panic!("Instance not initialised correctly."),
        }
    }

    /// Load a suitable adapter for the instance.
    async fn load_adapter(mut self) -> Result<Instance, Box<dyn Error>> {
        let instance = match self.inner {
            InstanceStage::Instance(instance) => instance,
            _ => return Err("Instance not initialised correctly.".into()),
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        self.inner = InstanceStage::Adapter(instance, adapter);

        Ok(self)
    }

    /// Load device.
    async fn load_device(mut self) -> Result<Instance, Box<dyn Error>> {
        let (instance, adapter) = match self.inner {
            InstanceStage::Adapter(instance, adapter) => (instance, adapter),
            _ => return Err("Adapter not initialised correctly.".into()),
        };

        // request device, todo: investigate options for selection
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        self.inner = InstanceStage::DeviceQueue(instance, adapter, device, queue);

        Ok(self)
    }
}

/// An enum representing the different stages of the GPU context initialisation.
enum InstanceStage {
    Instance(wgpu::Instance),
    Adapter(wgpu::Instance, wgpu::Adapter),
    DeviceQueue(wgpu::Instance, wgpu::Adapter, wgpu::Device, wgpu::Queue),
}

impl Default for InstanceStage {
    fn default() -> Self {
        Self::Instance(wgpu::Instance::default())
    }
}
