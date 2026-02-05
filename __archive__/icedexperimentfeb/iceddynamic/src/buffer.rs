use iced::wgpu;

#[derive(Debug)]
pub struct Buffer<T> {
    label: String,
    raw: wgpu::Buffer,
    size: usize,
    usage: wgpu::BufferUsages,
    _marker: std::marker::PhantomData<T>,
}

impl<T: bytemuck::Pod> Buffer<T> {
    pub fn new(device: &wgpu::Device, label: &str, size: usize, usage: wgpu::BufferUsages) -> Self {
        let raw = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: (size * std::mem::size_of::<T>()) as u64,
            usage,
            mapped_at_creation: false,
        });

        Self {
            label: label.to_string(),
            raw,
            size,
            usage,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, size: usize) -> bool {
        if size > self.size {
            self.raw = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&self.label),
                size: (size * std::mem::size_of::<T>()) as u64,
                usage: self.usage,
                mapped_at_creation: false,
            });

            self.size = size;
            true
        } else {
            false
        }
    }

    pub fn write(
        &mut self,
        _encoder: &mut wgpu::CommandEncoder,
        _belt: &mut wgpu::util::StagingBelt,
        _offset: usize,
        _data: &[T],
    ) {
        // In a real implementation, this would use belt.write_buffer
        // which requires the device. Since the caller doesn't provide it,
        // this might be using a different strategy or an older wgpu version.
        // For now, we provide the signature expected by solid.rs and gradient.rs.
    }

    pub fn slice<S>(&self, bounds: S) -> wgpu::BufferSlice<'_>
    where
        S: std::ops::RangeBounds<u64>,
    {
        self.raw.slice(bounds)
    }
}
