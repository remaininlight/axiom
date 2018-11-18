use codegen::util;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::StructType;
use inkwell::values::{IntValue, PointerValue};

#[derive(Debug, Clone)]
pub struct AnyValue {
    pub val: PointerValue,
}

impl AnyValue {
    pub fn get_type(context: &Context) -> StructType {
        // TODO Just returning Number structure for now. Can this infer what its correct type should be from context?
        // Maybe return space to hold largest possible type?
        context.struct_type(
            &[&context.f32_type().vec_type(2), &context.i8_type()],
            false,
        )
    }

    pub fn new(val: PointerValue) -> Self {
        AnyValue { val }
    }

    pub fn copy_to(&self, builder: &mut Builder, module: &Module, other: &AnyValue) {
        util::copy_ptr(builder, module, self.val, other.val)
    }

    pub fn get_value_ptr(&self) -> PointerValue {
        self.val
    }

    pub fn get_form_ptr(&self, builder: &mut Builder) -> PointerValue {
        unsafe { builder.build_struct_gep(&self.val, 1, "any.form.ptr") }
    }

    pub fn get_form(&self, builder: &mut Builder) -> IntValue {
        let vec = self.get_form_ptr(builder);
        builder.build_load(&vec, "any.form").into_int_value()
    }
}
