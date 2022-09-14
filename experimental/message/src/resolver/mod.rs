mod collector;

use super::ast;
use super::parser::slice::Slice;
use super::types::{MessagePart, VariableType};

pub trait MFFunction {
    fn resolve_to_parts<'r>(
        &self,
        variable: Option<&dyn VariableType<'r>>,
        parts: &mut Vec<Box<dyn MessagePart<'r> + 'r>>,
    );
    fn resolve_to_write(&self) -> Box<dyn MessagePart>;
}

pub struct Resolver {
    pub function: Option<Box<dyn MFFunction>>,
}

impl<'r> Resolver {
    pub fn resolve_to_parts<S: Slice<'r>>(
        &self,
        ast: &[ast::PatternElement<S>],
        parts: &mut Vec<Box<dyn MessagePart<'r> + 'r>>,
    ) {
        for pe in ast {
            match pe {
                ast::PatternElement::Text(s) => {
                    let c = s.as_cow();
                    parts.push(Box::new(c));
                }
                ast::PatternElement::Placeholder(p) => match p {
                    ast::Placeholder::Markup { name, options } => todo!(),
                    ast::Placeholder::MarkupEnd { name } => todo!(),
                    ast::Placeholder::Expression(e) => {
                        let (v, a) = match e {
                            ast::Expression::Operand {
                                operand,
                                annotation,
                            } => {
                                let v = match operand {
                                    ast::Operand::Literal(l) => Some(l.value.as_cow()),
                                    ast::Operand::Variable(v) => None,
                                };
                                (v, annotation.as_ref())
                            }
                            ast::Expression::Annotation(a) => (None, Some(a)),
                        };
                        if let Some(a) = a {
                            if let Some(f) = &self.function {
                                f.resolve_to_parts(v.as_ref(), parts)
                            }
                        }
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct NumberFormat {}

    impl MFFunction for NumberFormat {
        fn resolve_to_write(&self) -> Box<dyn MessagePart> {
            todo!()
        }

        fn resolve_to_parts<'r>(
            &self,
            variable: Option<&dyn VariableType<'r>>,
            parts: &mut Vec<Box<dyn MessagePart<'r> + 'r>>,
        ) {
            if let Some(variable) = variable {
                parts.push(Box::new(String::from("STRONG")));
                let part = variable.to_part();
                parts.push(part);
                parts.push(Box::new(String::from("/STRONG")));
            }
        }
    }

    #[test]
    fn literal_pass() {
        let pes = vec![
            ast::PatternElement::Text("foo"),
            ast::PatternElement::Placeholder(ast::Placeholder::Expression(
                ast::Expression::Operand {
                    operand: ast::Operand::Literal(ast::Literal { value: "literal" }),
                    annotation: None,
                },
            )),
        ];

        let resolver = Resolver { function: None };

        let mut parts = vec![];
        resolver.resolve_to_parts(&pes, &mut parts);
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn from_fn_test() {
        let pes: Vec<ast::PatternElement<String>> = vec![];

        let resolver = Resolver {
            function: Some(Box::new(NumberFormat {})),
        };

        let mut parts = vec![];
        resolver.resolve_to_parts(&pes, &mut parts);
        assert_eq!(parts.len(), 3);
    }
}
