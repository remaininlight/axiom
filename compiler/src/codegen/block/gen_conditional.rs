use super::BlockContext;
use inkwell::values::PointerValue;
use inkwell::{FloatPredicate};

pub fn gen_conditional_statement(
    index: usize,
    condition: usize,
    consequence: usize,
    alternative: usize,
    node: &mut BlockContext,
) -> PointerValue {

    // TODO Check builder is not being copied by value
    let builder = node.ctx.b;
    let zero_const = node.ctx.context.f64_type().const_float(0.0);

    // From https://github.com/TheDan64/inkwell/blob/master/examples/kaleidoscope/main.rs#L999
    // TODO Why is float created here?
    // TODO Find a way to invoke a compile_expr function
    //let condition = self.compile_expr(condition)?;
    let condition = node.get_statement(condition);
    let condition = builder.build_float_compare(FloatPredicate::ONE, condition, zero_const, "ifcond");

    let then_bb = node
        .ctx
        .context
        .append_basic_block(&node.ctx.func, "then");
    let else_bb = node
        .ctx
        .context
        .append_basic_block(&node.ctx.func, "else");
    let cont_bb = node
        .ctx
        .context
        .append_basic_block(&node.ctx.func, "ifcont");

    //node.ctx.context.b.build_conditional_branch(cond, &then_bb, &else_bb);
    builder.build_conditional_branch(condition, &then_bb, &else_bb);

    // build then block
    builder.position_at_end(&then_bb);
    let then_val = node.get_statement(consequence);
    builder.build_unconditional_branch(&cont_bb);

    let then_bb = builder.get_insert_block().unwrap();

    // build else block
    builder.position_at_end(&else_bb);
    let else_val = node.get_statement(alternative);
    builder.build_unconditional_branch(&cont_bb);

    let else_bb = builder.get_insert_block().unwrap();

    // emit merge block
    builder.position_at_end(&cont_bb);

    let phi = builder.build_phi(node.ctx.context.f64_type(), "iftmp");

    phi.add_incoming(&[
        (&then_val, &then_bb),
        (&else_val, &else_bb)
    ]);

    phi.as_basic_value().into_pointer_value()
}
