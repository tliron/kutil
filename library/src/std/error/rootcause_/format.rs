use {
    rootcause::{ReportRef, handlers::*, hooks::report_formatter::*, markers::*},
    std::fmt,
};

///
#[derive(Debug)]
pub struct SimpleReportFormatter;

impl ReportFormatter for SimpleReportFormatter {
    fn format_reports(
        &self,
        reports: &[ReportRef<'_, Dynamic, Uncloneable, Local>],
        formatter: &mut fmt::Formatter,
        _function: FormattingFunction,
    ) -> fmt::Result {
        for (i, report) in reports.iter().enumerate() {
            if i > 0 {
                writeln!(formatter)?;
            }
            format_indented(*report, 0, formatter)?;
        }
        Ok(())
    }
}

fn format_indented(
    report: ReportRef<'_, Dynamic, Uncloneable, Local>,
    indentation: usize,
    formatter: &mut fmt::Formatter,
) -> fmt::Result {
    for _ in 0..indentation {
        write!(formatter, "  ")?;
    }
    writeln!(formatter, "{}:", report.format_current_context_unhooked())?;
    // TODO: Also format the attachments
    for subreport in report.children() {
        format_indented(subreport.into_uncloneable(), indentation + 1, formatter)?;
    }
    Ok(())
}
