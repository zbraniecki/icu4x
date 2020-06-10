//! A single Plural Rule is an expression which tests the value of [`PluralOperands`]
//! against a condition. If the condition is truthful, then the [`PluralCategory`]
//! to which the Rule is assigned should be used.
//!
//! # Examples
//!
//! In this example we're going to examine the AST, parsing and resolving of a
//! set of English Cardinal Plural Rules.
//!
//! A CLDR JSON source contains the following entry:
//!
//! ```json
//! {
//!   "one": "i = 1 and v = 0 @integer 1",
//!   "other": " @integer 0, 2~16, 100, 1000, 10000, 100000, 1000000, … @decimal 0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …"
//! }
//! ```
//!
//! When the user provides a number for which the Plural Category is to be selected,
//! the system will examin a rule for each category in order, and stop on the first
//! category which matches.
//!
//! In our example, the user provided an input value `1`.
//! That value expanded into [`PluralOperands`] looks like this:
//!
//! ```
//! PluralOperands {
//!     n: 1_f64,
//!     i: 1,
//!     v: 0,
//!     w: 0,
//!     f: 0,
//!     t: 0
//! }
//! ```
//!
//! Now, the system will parse the first rule, assigned to [`PluralCategory`] `one`, and
//! test if it matches.
//!
//! The value of the rule is:
//!
//! ```text
//! i = 1 and v = 0 @integer 1
//! ```
//!
//! The Rule contains a [`Condition`] `i = 1 and v = 0` and a [`Sample`] integer `1`.
//!
//! When parsed, the resulting AST will look like this:
//!
//! ```
//! let input = "i = 1 and v = 0 @integer 1";
//!
//! let ast = parse(input.as_bytes());
//! assert_eq!(ast, Rule {
//!     condition: Condition(Box::new([
//!         AndCondition(Box::new([
//!             Relation {
//!                 expression: Expression {
//!                     operand: Operand::I,
//!                     modulus: None,
//!                 },
//!                 operator: Operator::Eq,
//!                 range_list: RangeList(Box::new([
//!                     RangeListItem::Value(
//!                         Value(1)
//!                     )
//!                 ]))
//!             },
//!             Relation {
//!                 expression: Expression {
//!                     operand: Operand::V,
//!                     modulus: None,
//!                 },
//!                 operator: Operator::Eq,
//!                 range_list: RangeList(Box::new([
//!                     RangeListItem::Value(
//!                         Value(0)
//!                     )
//!                 ]))
//!             },
//!         ])),
//!     ])),
//!     samples: Some(Samples {
//!         integer: Some(SampleList {
//!             sample_ranges: Box::new([
//!                 SampleRange {
//!                     lower_val: DecimalValue {
//!                         integer: Value(1),
//!                         decimal: None,
//!                     },
//!                     upper_val: None,
//!                 }
//!             ]),
//!             ellipsis: None,
//!         }),
//!         decimal: None,
//!     }),
//! });
//! ```
//!
//! Finally, we can pass this AST (in fact, just the [`Condition`] node),
//! to a resolver alongside the [`PluralOperands`] to test if the Rule
//! matches:
//!
//! ```
//! assert_eq!(resolve(ast, operands), true);
//! ```
//!
//! Since the rule for category `one` matches, we will return this category.
//! Otherwise, we'd test the next rule, in this case `other`, which has an
//! empty [`Condition`] meaning that it'll match all operands.
//!
//! In result if `i` is `1` and `v` is `0` we will return `one`, otherwise `other`.
//!
//! For other locales, there are more `Categories` and more complicated Rules.
pub mod ast;
pub mod lexer;
pub mod parser;
pub mod resolver;
