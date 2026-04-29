use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

pub fn spinner(enabled: bool, message: &str) -> ProgressBar {
    if !enabled {
        return ProgressBar::hidden();
    }

    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stdout());
    progress.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .expect("spinner progress template should be valid"),
    );
    progress.set_message(message.to_string());
    progress.enable_steady_tick(std::time::Duration::from_millis(120));
    progress
}

pub fn progress_bar(enabled: bool, len: usize, message: &str) -> ProgressBar {
    if !enabled {
        return ProgressBar::hidden();
    }

    let progress = ProgressBar::with_draw_target(Some(len as u64), ProgressDrawTarget::stdout());
    progress.set_style(
        ProgressStyle::with_template("{bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .expect("bar progress template should be valid")
            .progress_chars("##-"),
    );
    progress.set_message(message.to_string());
    progress
}
