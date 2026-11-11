#[allow(unused_imports)]
use cortex_m;
use embedded_alloc::LlffHeap;

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

pub fn get() -> &'static LlffHeap { return &HEAP; }
