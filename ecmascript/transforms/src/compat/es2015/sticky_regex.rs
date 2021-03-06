use ast::*;
use crate::util::ExprFactory;
use std::iter;
use swc_common::{Fold, FoldWith};

/// Compile ES2015 sticky regex to an ES5 RegExp constructor
///
///# Example
///## In
///
/// ```js
/// /o+/y;
/// ```
///
///## Out
///
/// ```js
/// new RegExp("o+", "y")
/// ```
#[derive(Debug, Clone, Copy)]
pub struct StickyRegex;

impl Fold<Expr> for StickyRegex {
    fn fold(&mut self, e: Expr) -> Expr {
        let e = e.fold_children(self);

        match e {
            Expr::Lit(Lit::Regex(Regex { exp, flags, span })) => {
                if flags
                    .as_ref()
                    .map(|s| s.value.contains("y"))
                    .unwrap_or(false)
                {
                    let str_lit = |s: Str| box Expr::Lit(Lit::Str(s));
                    let span = mark!(span);

                    return Expr::New(NewExpr {
                        callee: box quote_ident!(span, "RegExp").into(),
                        args: Some(
                            iter::once(str_lit(exp).as_arg())
                                .into_iter()
                                .chain(flags.map(|flags| str_lit(flags).as_arg()))
                                .collect(),
                        ),
                        span,
                    });
                } else {
                    return Expr::Lit(Lit::Regex(Regex { exp, flags, span }));
                }
            }
            _ => e,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test!(
        StickyRegex,
        babel_basic,
        "var re = /o+/y;",
        "var re = new RegExp('o+', 'y');"
    );

    test!(
        StickyRegex,
        babel_ignore_non_sticky,
        "var re = /o+/;",
        "var re = /o+/;"
    );
}
