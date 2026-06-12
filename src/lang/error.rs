use std::fmt;

use logos::Span;

#[derive(Debug)]
pub struct LangError {
	pub message: String,
	pub span: Option<Span>,
	pub source: String,
}

fn line_col(source: &str, pos: usize) -> (usize, usize) {
	let mut row = 1;
	let mut col = 1;

	for ch in source[..pos].chars() {
		if ch == '\n' {
			row += 1;
			col = 1;
		} else {
			col += 1;
		}
	}

	(row, col)
}

impl fmt::Display for LangError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		match &self.span {
			Some(span) => {
				let (row, col) = line_col(&self.source, span.start);

				let line = self.source.lines().nth(row.saturating_sub(1)).unwrap_or("");

				let caret_count = (span.end - span.start).max(1);

				let gutter = row.to_string().len();

				writeln!(f, "{}", self.message)?;
				writeln!(f, "---> {}:{}", row, col)?;
				writeln!(f, "{:>width$} |", "", width = gutter)?;
				writeln!(f, "{:>width$} | {}", row, line, width = gutter)?;
				writeln!(
					f,
					"{:>width$} | {}{}",
					"",
					" ".repeat(col - 1),
					"^".repeat(caret_count),
					width = gutter,
				)?;
			},

			None => {
				writeln!(f, "{}", self.message)?;
			},
		}

		Ok(())
	}
}
