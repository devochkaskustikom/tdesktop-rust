//! User interface on Slint. Prototype for phase 4: long list and Hindi text.

slint::include_modules!();

const ROW_COUNT: usize = 1000;
const SAMPLE_TEXT: &str = "नमस्ते दुनिया · Привет · Hello";

/// Opens the prototype window and runs the event loop until it is closed.
pub fn run_prototype() -> Result<(), slint::PlatformError> {
    let window = AppWindow::new()?;
    window.set_sample_text(SAMPLE_TEXT.into());
    window.set_rows(slint::ModelRc::new(slint::VecModel::from(sample_rows())));
    window.run()
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
