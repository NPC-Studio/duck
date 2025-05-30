use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    FileId,
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Call, Expr, ExprKind},
};

#[derive(Debug, PartialEq)]
pub struct DrawSprite;
impl Lint for DrawSprite {
    fn explanation() -> &'static str {
        "Projects that implement their own rendering backend may wish to be restrictive around when and where the `draw_sprite` functions are called."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "draw_sprite"
    }
}

impl EarlyExprPass for DrawSprite {
    fn visit_expr_early(expr: &Expr, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Call(Call { left, .. }) = expr.kind() {
            if let ExprKind::Identifier(identifier) = left.kind() {
                if gm_draw_sprite_functions().contains(&identifier.lexeme.as_str()) {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message(format!("Use of `{}`", identifier.lexeme))
                            .with_labels(vec![
                                Label::primary(left.file_id(), left.span())
                                    .with_message("replace this call with your API's ideal function"),
                            ]),
                    );
                }
            }
        }
    }
}

fn gm_draw_sprite_functions() -> &'static [&'static str] {
    &[
        "draw_sprite",
        "draw_sprite_ext",
        "draw_sprite_general",
        "draw_sprite_part",
        "draw_sprite_part_ext",
        "draw_sprite_pos",
        "draw_sprite_stretched",
        "draw_sprite_stretched_ext",
        "draw_sprite_tiled",
        "draw_sprite_tiled_ext",
    ]
}
