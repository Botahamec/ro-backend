/*
 * Copyright (c) 2019 Mike White
 *
 * @Script: rotype.rs
 * @Author: Mike White
 * @Email: botahamec@outlook.com
 * @Create At: 2019-11-09 20:49:44
 * @Last Modified By: Mike White
 * @Last Modified At: 2019-11-14 19:52:57
 * @Description: Ro's typing system is implemented here
 */

use std::collections::HashMap;
use crate::result::RoFunction;

macro_rules! new_ro_type {
	($name: ident, $type: ty) => {
		#[derive(Clone)]
		pub struct $name {
			value: $type
		}
	};
}

// contains the list of properties of a struct
type StructProperties = HashMap<String, RoType>;

// primitive types
new_ro_type!(RoU8, u8);
new_ro_type!(RoU16, u16);
new_ro_type!(RoU32, u32);
new_ro_type!(RoU64, u64);
new_ro_type!(RoI8, i8);
new_ro_type!(RoI16, i16);
new_ro_type!(RoI32, i32);
new_ro_type!(RoI64, i64);
new_ro_type!(RoF32, f32);
new_ro_type!(RoF64, f64);
new_ro_type!(RoBool, bool);
new_ro_type!(FunctionType, RoFunction);

#[derive(Clone)]
pub struct RoStruct {
	pub name: String,
	pub value: StructProperties
}

#[derive(Clone)]
pub struct RoArray {
	pub value: Vec<RoType>,
	pub length: usize // needed for some rules
}

#[derive(Clone)]
pub enum RoType {
	None,
	RoU8(RoU8),
	RoU16(RoU16),
	RoU32(RoU32),
	RoU64(RoU64),
	RoI8(RoI8),
	RoI16(RoI16),
	RoI32(RoI32),
	RoI64(RoI64),
	RoF32(RoF32),
	RoF64(RoF64),
	RoBool(RoBool),
	RoArray(RoArray),
	RoStruct(RoStruct)
}