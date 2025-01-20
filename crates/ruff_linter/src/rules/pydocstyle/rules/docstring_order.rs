use crate::checkers::ast::Checker;
use crate::docstrings::Docstring;
use crate::registry::Rule;
use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_text_size::Ranged;

/// ## What it does
/// Checks Numpy docstring sections follow the correct order.
///
/// ## Why is this bad?
/// Using a consistent order for sections makes it easier to find information in the docstring.
///
/// ## Example
/// ```python
/// def is_positive(x: int) -> bool:
///     """Returns whether the value is a positive number.
///
///     Returns
///     -------
///     out : bool
///         whether the value of x is positive.
///
///     The extended summary should be just below the short summary to
///     provide additional context.
///
///     Parameters
///     ----------
///     x : int
///         a number
///     """
/// ```
///
/// Use instead:
/// ```python
/// def is_positive(x: int) -> bool:
///     """Returns whether the value is a positive number.
///
///     The extended summary should be just below the short summary to
///     provide additional context.
///
///     Parameters
///     ----------
///     x : int
///         a number
///
///     Returns
///     -------
///     out : bool
///         whether the value of x is positive.
///     """
///
/// ## References
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html#sections)
#[derive(ViolationMetadata)]
pub(crate) struct OutOfOrderDocstring {
    sections: Vec<String>,
}

impl Violation for OutOfOrderDocstring {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Docstring sections do not follow numpy order: {:?}",
            self.sections
        )
    }
}

/// D420
pub(crate) fn out_of_order_docstring(checker: &Checker, docstring: &Docstring) -> bool {
    if !docstring.body().trim().is_empty() {
        return true;
    }

    if checker.enabled(Rule::OutOfOrderDocstring) {
        let diagnostic = OutOfOrderDocstring {
            sections: vec![
                "Summary".to_string(),
                "Extended Summary".to_string(),
                "Parameters".to_string(),
                "Returns".to_string(),
            ],
        };
        checker.report_diagnostic(Diagnostic::new(diagnostic, docstring.range()));
    }
    false
}
