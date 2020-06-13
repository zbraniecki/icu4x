use super::ast;
use crate::operands::PluralOperands;
use crate::PluralCategory;

pub fn select(
    rules: &[(PluralCategory, ast::Condition)],
    operands: &PluralOperands,
) -> PluralCategory {
    for (category, rule) in rules {
        if matches(rule, operands) {
            return *category;
        }
    }
    PluralCategory::Other
}

pub fn matches(condition: &ast::Condition, operands: &PluralOperands) -> bool {
    condition
        .0
        .iter()
        .any(|c| matches_and_condition(c, operands))
}

fn matches_and_condition(condition: &ast::AndCondition, operands: &PluralOperands) -> bool {
    condition.0.iter().all(|r| matches_relation(r, operands))
}

fn matches_relation(relation: &ast::Relation, operands: &PluralOperands) -> bool {
    let exp = calculate_expression(&relation.expression, operands);
    matches_range(&relation.range_list, exp, &relation.operator)
}

fn calculate_expression(expression: &ast::Expression, operands: &PluralOperands) -> u64 {
    match expression.operand {
        ast::Operand::N => operands.n as u64,
        ast::Operand::I => operands.i,
        ast::Operand::F => operands.f,
        ast::Operand::V => operands.v as u64,
        ast::Operand::W => operands.w as u64,
        ast::Operand::T => operands.t,
    }
}

fn matches_range(range: &ast::RangeList, value: u64, operator: &ast::Operator) -> bool {
    range
        .0
        .iter()
        .any(|item| matches_range_item(item, value, operator))
}

fn matches_range_item(item: &ast::RangeListItem, value: u64, operator: &ast::Operator) -> bool {
    match item {
        ast::RangeListItem::Value(n) => match operator {
            ast::Operator::Eq => n.0 as u64 == value,
            ast::Operator::NotEq => n.0 as u64 != value,
            _ => unimplemented!(),
        },
        ast::RangeListItem::Range((start, end)) => match operator {
            ast::Operator::Eq => value >= start.0 as u64 && value <= end.0 as u64,
            ast::Operator::NotEq => value < start.0 as u64 || value > end.0 as u64,
            _ => unimplemented!(),
        },
    }
}
