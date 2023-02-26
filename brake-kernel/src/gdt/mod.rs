pub mod tss;

use x86_64::{structures::gdt::{GlobalDescriptorTable, Descriptor}, registers::segmentation::{CS, Segment}, instructions::tables::load_tss};

static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

pub unsafe fn init_gdt() {
    let kernel_code_selector = GDT.add_entry(Descriptor::kernel_code_segment());
    let tss_selector = GDT.add_entry(Descriptor::tss_segment(&tss::TSS));
    GDT.load();

    CS::set_reg(kernel_code_selector);
    load_tss(tss_selector);
}