use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

pub struct ProgressHandle {
    bar: ProgressBar,
}

impl ProgressHandle {
    pub fn set_message(&self, message: impl Into<String>) {
        self.bar.set_message(message.into());
    }

    pub fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }

    pub fn set_position(&self, position: u64) {
        self.bar.set_position(position);
    }
}

impl Drop for ProgressHandle {
    fn drop(&mut self) {
        self.bar.finish_and_clear();
    }
}

pub fn spinner(message: &str) -> ProgressHandle {
    let bar = ProgressBar::new_spinner();
    bar.set_draw_target(ProgressDrawTarget::stdout());
    bar.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .expect("spinner progress template should be valid"),
    );
    bar.set_style(
        bar.style()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    bar.set_message(message.to_string());
    bar.enable_steady_tick(std::time::Duration::from_millis(120));
    ProgressHandle { bar }
}

pub fn progress_bar(len: usize, message: &str) -> ProgressHandle {
    let bar = ProgressBar::new(len as u64);
    bar.set_draw_target(ProgressDrawTarget::stdout());
    bar.set_style(
        ProgressStyle::with_template("{bar:40.cyan/blue} {percent:>3}% {msg}")
            .expect("bar progress template should be valid")
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );
    bar.set_message(message.to_string());
    ProgressHandle { bar }
}
