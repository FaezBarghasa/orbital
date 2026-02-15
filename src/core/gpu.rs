// GPU abstraction layer with buffer import support for zero-copy presentation.

use crate::core::display::Display;
use crate::core::rect::Rect;

/// Represents an imported GPU buffer for zero-copy rendering.
///
/// In the future this will wrap a DMA-BUF handle or shared memory fd.
/// Currently it stores the essential metadata for buffer identity and layout.
pub struct GpuBuffer {
    /// File descriptor for the shared memory or DMA-BUF.
    pub fd: usize,
    /// Buffer width in pixels.
    pub width: i32,
    /// Buffer height in pixels.
    pub height: i32,
    /// Pixel format (e.g., ARGB8888 = 0, XRGB8888 = 1).
    pub format: u32,
    /// Row stride in bytes.
    pub stride: u32,
}

pub struct Gpu {
    /// Currently imported buffer for zero-copy presentation, if any.
    imported_buffer: Option<GpuBuffer>,
}

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            imported_buffer: None,
        }
    }

    /// Import an external buffer for zero-copy presentation.
    ///
    /// This stores the buffer metadata. In a full DRM/KMS implementation,
    /// this would call `drmPrimeFDToHandle` + `drmModeAddFB2` to create
    /// a scanout-capable framebuffer object.
    pub fn import_buffer(
        &mut self,
        fd: usize,
        width: i32,
        height: i32,
        format: u32,
        stride: u32,
    ) -> &GpuBuffer {
        self.imported_buffer = Some(GpuBuffer {
            fd,
            width,
            height,
            format,
            stride,
        });
        self.imported_buffer.as_ref().unwrap()
    }

    /// Release any imported buffer.
    pub fn release_buffer(&mut self) {
        self.imported_buffer = None;
    }

    /// Check if there is an active imported buffer.
    pub fn has_imported_buffer(&self) -> bool {
        self.imported_buffer.is_some()
    }

    /// Get a reference to the imported buffer, if any.
    pub fn imported_buffer(&self) -> Option<&GpuBuffer> {
        self.imported_buffer.as_ref()
    }

    /// Render from an imported buffer to a display region.
    ///
    /// In a full implementation, this would use the GPU to blit the
    /// imported buffer to the display framebuffer without an intermediate
    /// CPU copy. Currently this is a no-op placeholder; the actual
    /// zero-copy path is handled by the scanout module which remaps
    /// the display framebuffer directly.
    pub fn render_from_buffer(&self, _buffer: &GpuBuffer, _display: &mut Display, _rect: &Rect) {
        // When DRM/KMS integration is available, this will call
        // drmModeSetPlane or equivalent to present the imported buffer
        // on the display's primary plane.
    }

    /// Standard render path. If an imported buffer is available, uses
    /// zero-copy presentation; otherwise falls back to the compositor's
    /// software blit.
    pub fn render(&self) {
        if let Some(buffer) = &self.imported_buffer {
            // Zero-copy path: the imported buffer is already mapped as
            // the display scanout source.  Nothing to do here since the
            // scanout module handles the framebuffer remapping.
            let _ = buffer;
        }
        // The normal compositor blit path is handled by Window::draw()
        // and Compositor::redraw_windows() in the scheme's redraw cycle.
    }
}
