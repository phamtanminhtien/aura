use crate::compiler::ast::*;
use crate::compiler::frontend::formatter::Formatter;

pub(crate) fn format_doc(f: &mut Formatter, doc: &Option<DocComment>) {
    if let Some(d) = doc {
        match d {
            DocComment::Line(content) => {
                for line in content.lines() {
                    f.indent();
                    f.result.push_str("///");
                    if !line.is_empty() && !line.starts_with(' ') {
                        f.result.push(' ');
                    }
                    f.result.push_str(line.trim_end());
                    f.result.push('\n');
                }
            }
            DocComment::Block(content) => {
                f.indent();
                f.result.push_str("/**");

                let lines: Vec<&str> = content.lines().collect();
                if lines.len() <= 1 {
                    f.result.push_str(content);
                } else {
                    for (i, line) in lines.iter().enumerate() {
                        if i == 0 && line.trim().is_empty() {
                            f.result.push('\n');
                            continue;
                        }

                        if i == lines.len() - 1 && line.trim().is_empty() {
                            continue;
                        }

                        f.indent();
                        let trimmed = line.trim_start();
                        if trimmed.starts_with('*') {
                            f.result.push_str(" ");
                            f.result.push_str(trimmed);
                        } else {
                            f.result.push_str(trimmed);
                        }
                        f.result.push('\n');
                    }
                    f.indent();
                    f.result.push(' ');
                }
                f.result.push_str("*/\n");
            }
        }
    }
}
