//! A lint rule for using double quoted strings.

use wdl_ast::AstNode;
use wdl_ast::Diagnostic;
use wdl_ast::Diagnostics;
use wdl_ast::Document;
use wdl_ast::Span;
use wdl_ast::SupportedVersion;
use wdl_ast::SyntaxElement;
use wdl_ast::SyntaxKind;
use wdl_ast::VisitReason;
use wdl_ast::Visitor;
use wdl_ast::v1::Expr;
use wdl_ast::v1::LiteralExpr;
use wdl_ast::v1::LiteralStringKind;

use crate::Rule;
use crate::Tag;
use crate::TagSet;

/// The identifier for the double quotes rule.
const ID: &str = "DoubleQuotes";

/// Creates a "use double quotes" diagnostic.
fn use_double_quotes(span: Span) -> Diagnostic {
    Diagnostic::note("string defined with single quotes")
        .with_rule(ID)
        .with_highlight(span)
        .with_fix("change the string to use double quotes")
}

/// Detects strings that are not defined with double quotes.
#[derive(Default, Debug, Clone, Copy)]
pub struct DoubleQuotesRule;

impl Rule for DoubleQuotesRule {
    fn id(&self) -> &'static str {
        ID
    }

    fn description(&self) -> &'static str {
        "Ensures that strings are defined using double quotes."
    }

    fn explanation(&self) -> &'static str {
        "All strings should be defined using double quotes. There is no semantic difference \
         between single and double quotes in WDL, but double quotes should be used exclusively to \
         ensure consistency and avoid any confusion."
    }

    fn tags(&self) -> TagSet {
        TagSet::new(&[Tag::Clarity, Tag::Style])
    }

    fn exceptable_nodes(&self) -> Option<&'static [wdl_ast::SyntaxKind]> {
        Some(&[
            SyntaxKind::VersionStatementNode,
            SyntaxKind::TaskDefinitionNode,
            SyntaxKind::WorkflowDefinitionNode,
            SyntaxKind::StructDefinitionNode,
            SyntaxKind::MetadataSectionNode,
            SyntaxKind::ParameterMetadataSectionNode,
            SyntaxKind::BoundDeclNode,
            SyntaxKind::LiteralStringNode,
        ])
    }
}

impl Visitor for DoubleQuotesRule {
    type State = Diagnostics;

    fn document(
        &mut self,
        _: &mut Self::State,
        reason: VisitReason,
        _: &Document,
        _: SupportedVersion,
    ) {
        if reason == VisitReason::Exit {
            return;
        }

        // Reset the visitor upon document entry
        *self = Default::default();
    }

    fn expr(&mut self, state: &mut Self::State, reason: VisitReason, expr: &Expr) {
        if reason == VisitReason::Exit {
            return;
        }

        if let Expr::Literal(LiteralExpr::String(s)) = expr {
            if s.kind() == LiteralStringKind::SingleQuoted {
                state.exceptable_add(
                    use_double_quotes(s.span()),
                    SyntaxElement::from(expr.inner().clone()),
                    &self.exceptable_nodes(),
                );
            }
        }
    }
}
