use crate::abi::call::{ArgAbi, FnAbi, Reg, Uniform};
use crate::abi::{HasDataLayout, Size, TyAbiInterface};
use crate::spec::HasTargetSpec;

const WORD_SIZE: u64 = 32;

fn classify_ret<'a, Ty, C>(_cx: &C, arg: &mut ArgAbi<'a, Ty>) -> bool
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    let total = arg.layout.size;

    if total.bits() > (4 * WORD_SIZE) {
        arg.make_indirect();
        return true;
    }

    if arg.layout.is_aggregate() {
        if total.bits() <= WORD_SIZE {
            arg.cast_to(Reg::i32());
        } else if total.bits() <= (2 * WORD_SIZE) {
            arg.cast_to(Uniform { unit: Reg::i32(), total: Size::from_bits(2 * WORD_SIZE) });
        } else if total.bits() <= (3 * WORD_SIZE) {
            arg.cast_to(Uniform { unit: Reg::i32(), total: Size::from_bits(3 * WORD_SIZE) });
        } else {
            arg.cast_to(Uniform { unit: Reg::i32(), total: Size::from_bits(4 * WORD_SIZE) });
        }
        return false;
    }

    arg.extend_integer_width_to(WORD_SIZE);
    false
}

fn classify_arg<'a, Ty, C>(_cx: &C, arg: &mut ArgAbi<'a, Ty>, avail_gprs: &mut u64)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    let total = arg.layout.size;

    if total.bits() > (4 * WORD_SIZE) {
        arg.make_indirect();
        *avail_gprs = avail_gprs.saturating_sub(1);
        return;
    }

    if total.bits() > (3 * WORD_SIZE) {
        if arg.layout.is_aggregate() {
            arg.cast_to(Uniform { unit: Reg::i32(), total: Size::from_bits(4 * WORD_SIZE) });
        }

        *avail_gprs = avail_gprs.saturating_sub(4);
        return;
    } else if total.bits() > (2 * WORD_SIZE) {
        if arg.layout.is_aggregate() {
            arg.cast_to(Uniform { unit: Reg::i32(), total: Size::from_bits(3 * WORD_SIZE) });
        }

        *avail_gprs = avail_gprs.saturating_sub(3);
        return;
    } else if total.bits() > WORD_SIZE {
        if arg.layout.is_aggregate() {
            arg.cast_to(Uniform { unit: Reg::i32(), total: Size::from_bits(2 * WORD_SIZE) });
        }

        *avail_gprs = avail_gprs.saturating_sub(2);
        return;
    }

    if arg.layout.is_aggregate() {
        arg.cast_to(Reg::i32());
        *avail_gprs = avail_gprs.saturating_sub(1);
        return;
    }

    if *avail_gprs > 0 {
        arg.extend_integer_width_to(WORD_SIZE);
        *avail_gprs -= 1;
    }
}

pub fn compute_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout + HasTargetSpec,
{
    let mut avail_gprs = 8;

    if !fn_abi.ret.is_ignore() && classify_ret(cx, &mut fn_abi.ret) {
        avail_gprs -= 1;
    }

    for arg in fn_abi.args.iter_mut().filter(|arg| !arg.is_ignore()) {
        classify_arg(cx, arg, &mut avail_gprs);
    }
}
