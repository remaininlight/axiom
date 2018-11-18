use super::{Function, FunctionContext, VarArgs};
use codegen::values::{AnyValue, NumValue};
use inkwell::values::PointerValue;
use inkwell::{FloatPredicate};
use mir::block;

pub struct IfFunction {}
impl Function for IfFunction {
    fn function_type() -> block::Function { block::Function::If }

    fn gen_call(
        func: &mut FunctionContext,
        args: &[PointerValue],
        _varargs: Option<VarArgs>,
        result: PointerValue,
    ) {

        let condition_num = NumValue::new(args[0]);
        let condition_vec = condition_num.get_vec(func.ctx.b);
        let left_index = func.ctx.context.i32_type().const_int(0, false);
        let condition_left = func.ctx
                                 .b
                                 .build_extract_element(&condition_vec, &left_index, "condition.left")
                                 .into_float_value();
        let condition_bool = func.ctx.b.build_float_compare(
            FloatPredicate::ONE,
            condition_left,
            func.ctx.context.f32_type().const_float(0.0),
            "conditionbool",
        );

        let then_any = AnyValue::new(args[1]);
        let alternative_any = AnyValue::new(args[2]);
        let left_index = func.ctx.context.i32_type().const_int(0, false);
        let evaluated = func.ctx.b.build_select(condition_bool,
                                                then_any.get_value_ptr(),
                                                alternative_any.get_value_ptr(),
                                                "evaluated");
        let evaluated_ptr = evaluated.as_pointer_value();
        let evaluated_any = AnyValue::new(*evaluated_ptr);
        let result_any = AnyValue::new(result);

        evaluated_any.copy_to(func.ctx.b, func.ctx.module, &result_any);
    }
}
