use handlebars::{Handlebars, RenderError};
use serde::Serialize;

pub const TEMPLATE_REPORT_BOARD: &str = include_str!("./templates/report_board.handlebars");
pub const TEMPLATE_CONFIDENTIAL_ADVISORS: &str =
    include_str!("./templates/report_confidential_advisors.handlebars");

#[derive(Serialize)]
pub struct ReportTemplate {
    pub message: String,
    pub contact_address: Option<String>,
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
