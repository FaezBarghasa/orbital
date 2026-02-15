// Direct-to-display scanout bypass for fullscreen applications.
//
// When a window fully covers exactly one display, is not transparent,
// and is borderless (typical fullscreen apps), the compositor can
// skip its own blit and instead let the client write directly to the
// display framebuffer.  This eliminates one full-screen memory copy
// per frame.

use crate::core::display::Display;
use crate::window::Window;
use log::info;

/// Tracks whether a display is currently in direct scanout or normal composited mode.
#[derive(Debug, Clone)]
pub enum ScanoutState {
    /// Normal compositor-driven rendering.
    Composited,
    /// The window identified by `window_id` is writing directly to the display.
    DirectScanout { window_id: usize },
}

impl ScanoutState {
    /// Returns `true` when the display is in direct scanout mode.
    pub fn is_direct(&self) -> bool {
        matches!(self, ScanoutState::DirectScanout { .. })
    }
}

/// Check whether a window is eligible for direct scanout on the given display.
///
/// Eligibility criteria:
/// 1. The window must be fullscreen (has a `restore` with `FullScreen` tile position).
/// 2. The window must **not** be transparent (no alpha blending needed).
/// 3. The window must be borderless (no title bar to composite).
/// 4. The window rect must exactly cover the display rect.
/// 5. The window must not be hidden.
pub fn is_eligible(window: &Window, display: &Display) -> bool {
    let is_fullscreen = match &window.restore {
        Some((_, position)) => matches!(position, crate::scheme::TilePosition::FullScreen),
        None => false,
    };
    check_eligibility(
        window.hidden,
        window.transparent,
        window.borderless,
        is_fullscreen,
        window.rect(),
        display.screen_rect(),
    )
}

/// Pure logic check for direct scanout eligibility.
fn check_eligibility(
    hidden: bool,
    transparent: bool,
    borderless: bool,
    is_fullscreen: bool,
    window_rect: crate::core::rect::Rect,
    display_rect: crate::core::rect::Rect,
) -> bool {
    if hidden {
        return false;
    }
    if transparent {
        return false;
    }
    if !borderless {
        return false;
    }
    if !is_fullscreen {
        return false;
    }
    // Window rect must exactly match the display's screen rect
    window_rect.left() == display_rect.left()
        && window_rect.top() == display_rect.top()
        && window_rect.width() == display_rect.width()
        && window_rect.height() == display_rect.height()
}

/// Try to engage direct scanout for a display.
///
/// If the frontmost window on the display is eligible, returns
/// `ScanoutState::DirectScanout`.  Otherwise returns `Composited`.
pub fn try_engage(window_id: usize, window: &Window, display: &Display) -> ScanoutState {
    if is_eligible(window, display) {
        info!(
            "Direct scanout engaged for window {} on display at ({}, {})",
            window_id, display.x, display.y,
        );
        ScanoutState::DirectScanout { window_id }
    } else {
        ScanoutState::Composited
    }
}

/// Disengage direct scanout, returning to compositor-driven rendering.
pub fn disengage(state: &ScanoutState, display: &Display) {
    if let ScanoutState::DirectScanout { window_id } = state {
        info!(
            "Direct scanout disengaged for window {} on display at ({}, {})",
            window_id, display.x, display.y,
        );
    }
}

/// Given the ordered list of window IDs (front-to-back), find the frontmost
/// window for a specific display.  Returns `(window_id, &Window)` if found.
pub fn frontmost_window_for_display<'a>(
    window_order: impl Iterator<Item = usize>,
    windows: &'a std::collections::BTreeMap<usize, Window>,
    display: &Display,
) -> Option<(usize, &'a Window)> {
    let display_rect = display.screen_rect();
    for id in window_order {
        if let Some(window) = windows.get(&id) {
            if !window.hidden {
                let win_rect = window.rect();
                // Check if this window intersects the display
                let intersect = win_rect.intersection(&display_rect);
                if !intersect.is_empty() {
                    return Some((id, window));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::rect::Rect;

    #[test]
    fn test_eligibility_logic() {
        let display_rect = Rect::new(0, 0, 1920, 1080);
        let window_rect = Rect::new(0, 0, 1920, 1080);

        // Ideal eligible case
        assert!(check_eligibility(
            false,
            false,
            true,
            true,
            window_rect,
            display_rect
        ));

        // Hidden
        assert!(!check_eligibility(
            true,
            false,
            true,
            true,
            window_rect,
            display_rect
        ));

        // Transparent
        assert!(!check_eligibility(
            false,
            true,
            true,
            true,
            window_rect,
            display_rect
        ));

        // Not borderless
        assert!(!check_eligibility(
            false,
            false,
            false,
            true,
            window_rect,
            display_rect
        ));

        // Not fullscreen (e.g. maximized or normal)
        assert!(!check_eligibility(
            false,
            false,
            true,
            false,
            window_rect,
            display_rect
        ));

        // Mismatched rects (too small)
        assert!(!check_eligibility(
            false,
            false,
            true,
            true,
            Rect::new(0, 0, 1000, 1000),
            display_rect
        ));

        // Mismatched rects (offset)
        assert!(!check_eligibility(
            false,
            false,
            true,
            true,
            Rect::new(10, 10, 1920, 1080),
            display_rect
        ));
    }
}
