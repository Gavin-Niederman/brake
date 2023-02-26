use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::gdt;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub unsafe fn init_idt() {
    IDT.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::tss::DOUBLE_FAULT_IST_INDEX);
    IDT.load()
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, err_code: u64) -> !
{
    panic!("Double Fault! error code:{err_code}\n Stack Frame:\n{:#?}", stack_frame);
}