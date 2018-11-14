use super::{Function, FunctionContext, VarArgs};
use codegen::values::{NumValue};
use inkwell::values::PointerValue;
use inkwell::{FloatPredicate};
//use inkwell::IntPredicate;
use mir::block;


use codegen::{intrinsics, util};
use std::f32::consts;

pub struct IfFunction {}
impl Function for IfFunction {
    fn function_type() -> block::Function { block::Function::If }

    fn gen_call(
        func: &mut FunctionContext,
        args: &[PointerValue],
        _varargs: Option<VarArgs>,
        result: PointerValue,
    ) {

        let result_num = NumValue::new(result);

        let condition_num = NumValue::new(args[0]);
        let condition_vec = condition_num.get_vec(func.ctx.b);
        let left_index = func.ctx.context.i32_type().const_int(0, false);
        let condition_left = func
            .ctx
            .b
            .build_extract_element(&condition_vec, &left_index, "condition.left")
            .into_float_value();
        let condition_bool = func.ctx.b.build_float_compare(
            FloatPredicate::OEQ,
            condition_left,
            func.ctx.context.f32_type().const_float(1.0),
            "conditionbool",
        );

        let then_num = NumValue::new(args[1]);
        let then_vec = then_num.get_vec(func.ctx.b);
        let then_form = then_num.get_form(func.ctx.b);

        let alternative_num = NumValue::new(args[1]);
        let alternative_vec = alternative_num.get_vec(func.ctx.b);

        let left_index = func.ctx.context.i32_type().const_int(0, false);
        let result_vec = func.ctx.b.build_select(condition_bool,
                                                 then_vec,
                                                 alternative_vec,
                                                 "resultvec").into_vector_value();

        result_num.set_form(func.ctx.b, &then_form);
        result_num.set_vec(func.ctx.b, &result_vec);
    }
}
