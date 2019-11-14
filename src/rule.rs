/*
 * Copyright (c) 2019 Mike White
 *
 * @Script: rule.rs
 * @Author: Mike White
 * @Email: botahamec@outlook.com
 * @Create At: 2019-11-14 18:38:14
 * @Last Modified By: Mike White
 * @Last Modified At: 2019-11-14 18:43:53
 * @Description: Rules for the parameters of a function
 */

// what the rule is looking for
enum RuleType {
	EQUAL(_),
	GREATER(usize),
	LESS(usize),
	AND(RuleDuo),
	OR(RuleDuo),
	XOR(RuleDuo)
	NOT(FunctionRule),
	CONTAIN(Vec<_>)
}

pub struct FunctionRule {
	variable: String,
	type: RuleType
}

pub struct RuleDuo {
	FunctionRule,
	FunctionRule
}