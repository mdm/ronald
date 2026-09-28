use eframe::egui;
use web_time::{Duration, Instant};

use crate::colors;

const WINDOW: Duration = Duration::from_millis(500);

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct PerfSample {
    pub emulated_fps: f32,
    pub host_fps: f32,
    pub load: f32,
}

pub struct PerfStats {
    window_start: Instant,
    busy: Duration,
    emulated_frames: u32,
    host_frames: u32,
    latest: PerfSample,
}

impl Default for PerfStats {
    fn default() -> Self {
        Self {
            window_start: Instant::now(),
            busy: Duration::ZERO,
            emulated_frames: 0,
            host_frames: 0,
            latest: PerfSample::default(),
        }
    }
}

impl PerfStats {
    pub fn record_frame(&mut self, busy: Duration, emulated_frames: u32) {
        self.busy += busy;
        self.emulated_frames += emulated_frames;
        self.host_frames += 1;

        let elapsed = self.window_start.elapsed();
        if elapsed < WINDOW {
            return;
        }

        let seconds = elapsed.as_secs_f64();
        let latest = if seconds > 0.0 {
            PerfSample {
                emulated_fps: (self.emulated_frames as f64 / seconds) as f32,
                host_fps: (self.host_frames as f64 / seconds) as f32,
                load: (self.busy.as_secs_f64() / seconds) as f32,
            }
        } else {
            self.latest
        };

        *self = Self {
            latest,
            ..Default::default()
        }
    }

    pub fn latest(&self) -> PerfSample {
        self.latest
    }
}

pub fn draw_perf_stats(ui: &mut egui::Ui, overlay_rect: egui::Rect, sample: PerfSample) {
    let mut child_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(overlay_rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
    );

    child_ui.spacing_mut().item_spacing.x = 8.0;
    child_ui.add_space(8.0);

    let load_color = if sample.load > 1.0 {
        colors::LIGHT_RED
    } else if sample.load > 0.9 {
        colors::DARK_ORANGE
    } else {
        colors::WHITE
    };
    child_ui.label(
        egui::RichText::new(format!("{:>3.0}% load", 100.0 * sample.load))
            .monospace()
            .color(load_color),
    );
    child_ui.label(
        egui::RichText::new(format!(
            "{:>3.1} fps · {:>3.1} host · ",
            sample.emulated_fps, sample.host_fps
        ))
        .monospace()
        .color(colors::WHITE),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close_window(
        stats: &mut PerfStats,
        busy: Duration,
        emulated_frames: u32,
        host_frames: u32,
    ) -> PerfSample {
        for _ in 0..host_frames {
            stats.record_frame(busy, emulated_frames);
        }

        stats.window_start = Instant::now() - WINDOW;
        stats.record_frame(Duration::ZERO, 0);

        stats.latest()
    }

    #[test]
    fn no_sample_before_window_elapses() {
        let mut stats = PerfStats::default();
        stats.record_frame(Duration::from_millis(5), 1);

        assert_eq!(stats.latest(), PerfSample::default());
    }

    #[test]
    fn idle_frames_report_zero_except_host_rate() {
        let mut stats = PerfStats::default();
        let sample = close_window(&mut stats, Duration::ZERO, 0, 10);

        assert_eq!(sample.emulated_fps, 0.0);
        assert_eq!(sample.load, 0.0);
        assert!(sample.host_fps > 0.0, "host_fps was {}", sample.host_fps);
    }

    #[test]
    fn accumulators_reset_between_windows() {
        let mut stats = PerfStats::default();
        let first = close_window(&mut stats, Duration::from_millis(5), 1, 25);
        assert!(first.load > 0.0, "load was {}", first.load);
        assert!(
            first.emulated_fps > 0.0,
            "emulated_fps was {}",
            first.emulated_fps
        );

        // A second window of nothing but idle frames must not inherit the first.
        let second = close_window(&mut stats, Duration::ZERO, 0, 0);

        assert_eq!(second.load, 0.0);
        assert_eq!(second.emulated_fps, 0.0);
    }
}

#[cfg(test)]
mod gui_tests {
    use super::*;

    use egui_kittest::{Harness, kittest::Queryable};

    pub const FULL_SPEED: PerfSample = PerfSample {
        emulated_fps: 50.1,
        host_fps: 60.0,
        load: 0.37,
    };

    pub fn harness(sample: PerfSample) -> Harness<'static> {
        let app = move |ui: &mut egui::Ui| {
            let rect = egui::Rect::from_min_size(ui.max_rect().min, egui::Vec2::new(640.0, 40.0));
            ui.painter()
                .rect_filled(rect, egui::CornerRadius::default(), colors::BLACK);
            draw_perf_stats(ui, rect, sample);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        harness
    }

    #[test]
    fn metrics_are_rendered() {
        let harness = harness(FULL_SPEED);

        harness.get_by_label_contains("50.1 fps");
        harness.get_by_label_contains("60.0 host");
        harness.get_by_label_contains("37% load");
    }
}
