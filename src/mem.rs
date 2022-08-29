const WASM_PAGE_SIZE: u64 = 65536;

pub fn get_heap_memory_size() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        (core::arch::wasm32::memory_size(0) as u64) * WASM_PAGE_SIZE
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}
