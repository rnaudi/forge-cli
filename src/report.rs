use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ok,
    Warn,
    Err,
}

impl Status {
    pub fn label(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Warn => "WARN",
            Self::Err => "ERR",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Config,
    Tools,
    Secrets,
    Registries,
}

impl Category {
    fn all() -> [Self; 4] {
        [Self::Config, Self::Tools, Self::Secrets, Self::Registries]
    }

    fn heading(self) -> &'static str {
        match self {
            Self::Config => "Config",
            Self::Tools => "Tools",
            Self::Secrets => "Secrets",
            Self::Registries => "Registries",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult {
    pub category: Category,
    pub status: Status,
    pub name: String,
    pub detail: String,
}

impl CheckResult {
    pub fn ok(category: Category, name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(Status::Ok, category, name, detail)
    }

    pub fn warn(category: Category, name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(Status::Warn, category, name, detail)
    }

    pub fn err(category: Category, name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(Status::Err, category, name, detail)
    }

    pub fn new(
        status: Status,
        category: Category,
        name: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            category,
            status,
            name: name.into(),
            detail: detail.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadinessReport {
    results: Vec<CheckResult>,
}

impl ReadinessReport {
    pub fn new(results: Vec<CheckResult>) -> Self {
        Self { results }
    }

    pub fn results(&self) -> &[CheckResult] {
        &self.results
    }

    pub fn exit_code(&self) -> u8 {
        if self
            .results
            .iter()
            .any(|result| result.status == Status::Err)
        {
            1
        } else {
            0
        }
    }

    pub fn render(&self) -> String {
        let mut output = String::from("Forge readiness report\n");

        if self.results.is_empty() {
            output.push_str("\nNo checks configured.\n");
        } else {
            for category in Category::all() {
                let category_results = self
                    .results
                    .iter()
                    .filter(|result| result.category == category);

                let mut wrote_heading = false;
                for result in category_results {
                    if !wrote_heading {
                        let _ = writeln!(output, "\n{}:", category.heading());
                        wrote_heading = true;
                    }

                    let _ = writeln!(
                        output,
                        "  {} {} - {}",
                        result.status.label(),
                        result.name,
                        result.detail
                    );
                }
            }
        }

        let ok_count = self.count(Status::Ok);
        let warn_count = self.count(Status::Warn);
        let err_count = self.count(Status::Err);

        let _ = writeln!(
            output,
            "\nSummary: {ok_count} OK, {warn_count} WARN, {err_count} ERR"
        );

        output
    }

    fn count(&self, status: Status) -> usize {
        self.results
            .iter()
            .filter(|result| result.status == status)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warning_only_report_exits_successfully() {
        let report = ReadinessReport::new(vec![CheckResult::warn(
            Category::Secrets,
            "OPTIONAL_TOKEN",
            "environment variable is not set or is empty",
        )]);

        assert_eq!(report.exit_code(), 0);
    }

    #[test]
    fn error_report_exits_non_zero() {
        let report = ReadinessReport::new(vec![CheckResult::err(
            Category::Tools,
            "cargo",
            "command was not found on PATH",
        )]);

        assert_eq!(report.exit_code(), 1);
    }

    #[test]
    fn render_includes_statuses_and_summary() {
        let report = ReadinessReport::new(vec![
            CheckResult::ok(Category::Tools, "cargo", "command `cargo` found on PATH"),
            CheckResult::warn(
                Category::Secrets,
                "OPTIONAL_TOKEN",
                "environment variable is not set or is empty",
            ),
        ]);

        let rendered = report.render();

        assert!(rendered.contains("OK cargo"));
        assert!(rendered.contains("WARN OPTIONAL_TOKEN"));
        assert!(rendered.contains("Summary: 1 OK, 1 WARN, 0 ERR"));
    }
}
