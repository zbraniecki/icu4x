mod collector;
pub mod scope;

use collector::*;
pub use scope::Scope;

use super::ast;
use super::parser::slice::Slice;
use super::types::{MessagePart, VariableType};
use crate::resolver::scope::Variables;
use crate::types::MarkupElement;
use crate::MF2Function;
use std::borrow::Cow;

// MV - message value type
// MSGSV - messages value type
// MPV - message parts value type
pub struct Resolver<MV> {
    p1: std::marker::PhantomData<MV>,
}

// 'm - message lifetime
// 'mv - message value lifetime
// 'varsm - variables map lifetime
// 'varsv - variables values lifetime
// 'msgsm - messages map lifetime
// 'msgsmv - messages map value lifetime
// 'msgsv - messages value lifetime
// 'scope - scope lifetime
// 'mpv - message parts value lifetime
impl<'b, 'm, 'mv, 'varsm, 'varsv, 'mf, 'scope, 'mpv, MV> Resolver<MV>
where
    MV: Slice<'mv>,
    'mv: 'mpv,
    'varsv: 'mpv,
    'b: 'mpv,
    // 'varsm: 'varsv,
{
    pub fn resolve_to_parts(
        msg: &'m ast::Message<MV>,
        scope: &'scope Scope<'b, 'mf, 'varsm, 'varsv>,
    ) -> Vec<Box<dyn MessagePart<'mpv> + 'mpv>> {
        let mut collector = MessagePartsList::new();
        Self::resolve_message_to_collector(msg, scope, &mut collector);
        collector.0
    }

    pub fn resolve_to_string(
        msg: &'m ast::Message<MV>,
        scope: &'scope Scope<'b, 'mf, 'varsm, 'varsv>,
    ) -> Cow<'mpv, str> {
        let mut collector = MessageString::new();
        Self::resolve_message_to_collector(msg, scope, &mut collector);
        collector.0
    }

    pub fn resolve_to_sink<W: std::fmt::Write>(
        msg: &'m ast::Message<MV>,
        scope: &'scope Scope<'b, 'mf, 'varsm, 'varsv>,
        sink: W,
    ) {
        let mut collector = MessageSink::new(sink);
        Self::resolve_message_to_collector(msg, scope, &mut collector);
    }

    fn resolve_message_to_collector<C>(
        msg: &'m ast::Message<MV>,
        scope: &'scope Scope<'b, 'mf, 'varsm, 'varsv>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<'mpv>,
    {
        let value = &msg.value;
        let pattern = match value {
            ast::MessageValue::Pattern(pattern) => pattern,
            ast::MessageValue::Select(_) => todo!(),
        };
        for pe in &pattern.body {
            Self::resolve_pattern_element(pe, scope, collector);
        }
    }

    fn resolve_pattern_element<C>(
        pe: &'m ast::PatternElement<MV>,
        scope: &'scope Scope<'b, 'mf, 'varsm, 'varsv>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<'mpv>,
    {
        match pe {
            ast::PatternElement::Text(s) => {
                let s = s.as_cow();
                collector.push_part(&s);
            }
            ast::PatternElement::Placeholder(p) => Self::resolve_placeholder(p, scope, collector),
        }
    }

    fn resolve_placeholder<C>(
        placeholder: &'m ast::Placeholder<MV>,
        scope: &'scope Scope<'b, 'mf, 'varsm, 'varsv>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<'mpv>,
    {
        match placeholder {
            ast::Placeholder::Markup { name, options } => todo!(),
            ast::Placeholder::MarkupEnd { name } => todo!(),
            ast::Placeholder::Expression(e) => Self::resolve_expression(e, scope, collector),
        }
    }

    fn resolve_expression<C>(
        exp: &'m ast::Expression<MV>,
        scope: &'scope Scope<'b, 'mf, 'varsm, 'varsv>,
        collector: &mut C,
    ) where
        C: MessagePartCollector<'mpv>,
    {
        match exp {
            ast::Expression::Operand {
                operand,
                annotation,
            } => match operand {
                ast::Operand::Literal(l) => {
                    let s: Cow<'mv, str> = l.value.as_cow();
                    let part: Box<dyn MessagePart<'mpv>> = VariableType::to_part(&s);
                    collector.push_part(part.as_ref())
                }
                ast::Operand::Variable(v) => {
                    let var = Self::get_variable(v, scope).unwrap();
                    if let Some(annotation) = annotation {
                        let func = scope
                            .mf
                            .functions
                            .get(annotation.function.as_str())
                            .unwrap();
                        // let func = Self::get_function(&annotation.function, scope).unwrap();
                        let result: Vec<Box<dyn MessagePart<'mpv> + 'mpv>> = func(var, scope.mf);
                        // for part in result {
                        //     collector.push_part(part.as_ref());
                        // }
                    } else {
                        let part: Box<dyn MessagePart<'mpv>> = var.to_part();
                        collector.push_part(part.as_ref());
                    }
                }
            },
            ast::Expression::Annotation(_) => todo!(),
        }
    }

    fn get_variable(
        variable: &'m MV,
        scope: &'scope Scope<'b, 'mf, 'varsm, 'varsv>,
    ) -> Option<&'varsm dyn VariableType<'varsv>> {
        scope.variables.and_then(|vars| vars.get(variable.as_str()))
    }

    // fn resolve_variable<C, V>(
    //     var: &Box<dyn VariableType<'varsv> + 'varsv>,
    //     scope: &'scope Scope<'b, 'mf, 'varsm, 'varsv>,
    //     collector: &mut C,
    // ) where
    //     C: MessagePartCollector<'mpv>,
    //     V: Slice<'varsv>,
    // {
    //     let part = var.to_part().as_ref();
    //     collector.push_part(part);
    // }
}

#[cfg(test)]
mod test {
    use super::super::parser::Parser;
    use super::super::types::{MessagePart, VariableType};
    use super::ast;
    use super::{Resolver, Scope};
    use crate::resolver::scope::Variables;
    use crate::MessageFormat;
    use icu_locid::locale;
    use smallvec::SmallVec;
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[test]
    fn sanity_check() {
        let mf = MessageFormat::new(locale!("en-US"));
        let source = "{Hello World}";
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();

        let scope = Scope::new(&mf, None);
        let string = Resolver::resolve_to_string(&msg, &scope);

        assert_eq!(string, "Hello World");
    }

    #[test]
    fn stay_borrowed_check() {
        let mf = MessageFormat::new(locale!("en-US"));

        let msg = ast::Message {
            declarations: Default::default(),
            value: ast::MessageValue::Pattern(ast::Pattern {
                body: SmallVec::from_vec(vec![ast::PatternElement::Text("Hello World")]),
            }),
        };

        let scope = Scope::new(&mf, None);
        let string = Resolver::resolve_to_string(&msg, &scope);

        assert!(matches!(string, Cow::Borrowed("Hello World")));

        let scope = Scope::new(&mf, None);
        let parts = Resolver::resolve_to_parts(&msg, &scope);

        assert_eq!(parts.get(0).unwrap().to_cow(), "Hello World");
        assert_eq!(parts.len(), 1);

        let mut sink = String::new();
        let scope = Scope::new(&mf, None);
        Resolver::resolve_to_sink(&msg, &scope, &mut sink);

        assert_eq!(sink, "Hello World");
    }

    // #[test]
    // fn lifetimes_check() {
    //     let mf = MessageFormat::new(locale!("en"));
    //
    //     let parser = Parser::new("{Hello World{$name}{$creature}}");
    //     let msg = parser.parse().unwrap();
    //     // let parser = Parser::new("{Dragon}");
    //     // let creature_msg = parser.parse().unwrap();
    //     // let mut msgs = HashMap::new();
    //     // msgs.insert("dragon".to_string(), &creature_msg);
    //
    //     let mut variables = HashMap::new();
    //     variables.insert("name".into(), VariableType::String("John"));
    //     variables.insert("creature".into(), VariableType::MessageReference("dragon"));
    //     let mut variables = Variables::new();
    //     variables.insert("name".to_string(), "John".to_string());
    //     let scope = Scope::new(&mf, Some(&variables));
    //     let parts = Resolver::resolve_to_parts(&msg, &scope);
    //
    //     assert_eq!(
    //         parts,
    //         vec![
    //             MessagePart::Literal("Hello World"),
    //             MessagePart::Literal("John"),
    //             MessagePart::Literal("Dragon"),
    //         ]
    //     );
    //
    //     let parser = Parser::new("{{$name}}");
    //     let msg = parser.parse().unwrap();
    //     let string = Resolver::<_, _, &str>::resolve_to_string(&msg, &scope);
    //     assert!(matches!(string, Cow::Borrowed("John")));
    //
    //     let parser = Parser::new("{{$creature}}");
    //     let msg = parser.parse().unwrap();
    //     let string = Resolver::<_, _, &str>::resolve_to_string(&msg, &scope);
    //     assert!(matches!(string, Cow::Borrowed("Dragon")));
    // }

    #[test]
    fn allocate_check() {
        let mf = MessageFormat::new(locale!("en-US"));

        let msg = ast::Message {
            declarations: Default::default(),
            value: ast::MessageValue::Pattern(ast::Pattern {
                body: SmallVec::from_vec(vec![
                    ast::PatternElement::Text("Hello "),
                    ast::PatternElement::Text("World"),
                ]),
            }),
        };

        let scope = Scope::new(&mf, None);
        let string = Resolver::resolve_to_string(&msg, &scope);

        assert_eq!(string, Cow::<str>::Owned(String::from("Hello World")));

        let scope = Scope::new(&mf, None);
        let parts = Resolver::resolve_to_parts(&msg, &scope);

        let expected = vec!["Hello ", "World"];
        for (part, expected) in parts.iter().zip(expected) {
            if let Cow::Owned(_) = part.to_cow() {
                panic!();
            }
            assert_eq!(part.to_cow(), expected);
        }
    }

    #[test]
    fn variable_check() {
        let mf = MessageFormat::new(locale!("en-US"));

        let source = "{{$name}}";
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();

        let mut variables = Variables::new();
        variables.insert("name".into(), "John".to_string());
        let scope = Scope::new(&mf, Some(&variables));
        let string = Resolver::resolve_to_string(&msg, &scope);

        assert_eq!(string, "John");
    }
}
