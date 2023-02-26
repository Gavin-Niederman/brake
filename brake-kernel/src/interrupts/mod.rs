mod idt;
mod pics;

pub unsafe fn init() {
    idt::init_idt();
    pics::init_pics();
}