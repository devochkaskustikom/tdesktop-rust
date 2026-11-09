//! User interface on Slint. Prototype for phase 4: long list and Hindi text.

use std::rc::Rc;

slint::include_modules!();

const ROW_COUNT: usize = 1000;
const WINDOW_ROWS: usize = 40;
const SAMPLE_TEXT: &str = "नमस्ते दुनिया · Привет · Hello";

/// Opens the prototype window and runs the event loop until it is closed.
pub fn run_prototype() -> Result<(), slint::PlatformError> {
    let rows = Rc::new(sample_rows());
    let window = AppWindow::new()?;
    window.set_sample_text(SAMPLE_TEXT.into());
    window.set_total_rows(ROW_COUNT as i32);
    show_window(&window, &rows, 0);

    let weak = window.as_weak();
    window.on_window_needed(move |first| {
        if let Some(window) = weak.upgrade() {
            show_window(&window, &rows, first.max(0) as usize);
        }
    });

    window.run()
}

/// Instantiates only the rows around the viewport, starting at `first`.
fn show_window(window: &AppWindow, rows: &[ChatRow], first: usize) {
    let start = first.min(rows.len().saturating_sub(WINDOW_ROWS));
    let end = (start + WINDOW_ROWS).min(rows.len());
    window.set_window_rows(slint::ModelRc::new(slint::VecModel::from(rows[start..end].to_vec())));
    window.set_window_first(start as i32);
}

fn sample_rows() -> Vec<ChatRow> {
    (1..=ROW_COUNT)
        .map(|index| ChatRow {
            initials: format!("{}", index % 100).into(),
            name: format!("Chat {index}").into(),
            last: format!("Last message in chat {index}").into(),
        })
        .collect()
}
