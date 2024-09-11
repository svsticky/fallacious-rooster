use handlebars::{Handlebars, RenderError};
use serde::Serialize;
use std::fmt;
use std::fmt::Formatter;

pub const TEMPLATE_REPORT_BOARD: &str = include_str!("templates/report_board.hbs");
pub const TEMPLATE_CONFIDENTIAL_ADVISORS: &str =
    include_str!("templates/report_confidential_advisors.hbs");

#[derive(Serialize)]
pub struct ReportTemplate {
    pub message: String,
    pub contact_address: Option<String>,
}

// We don't ever want the report to be printed
impl fmt::Debug for ReportTemplate {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

fn render(template: &str, data: &ReportTemplate) -> Result<String, RenderError> {
    let mut hb = Handlebars::new();
    hb.set_strict_mode(true);

    #[cfg(debug_assertions)]
    hb.set_dev_mode(true);

    hb.render_template(template, data)
}

pub fn render_report_board(data: &ReportTemplate) -> Result<String, RenderError> {
    render(TEMPLATE_REPORT_BOARD, data)
}

pub fn render_report_confidential_advisors(data: &ReportTemplate) -> Result<String, RenderError> {
    render(TEMPLATE_CONFIDENTIAL_ADVISORS, data)
}
