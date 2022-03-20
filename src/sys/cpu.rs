use raw_cpuid::CpuId;
use crate::println;

pub fn init() {
    let cpuid = CpuId::new();

    if let Some(vendor_info) = cpuid.get_vendor_info() {
        println!("CPU {}", vendor_info);
    }

    if let Some(processor_brand_string) = cpuid.get_processor_brand_string() {
        println!("CPU {}", processor_brand_string.as_str().trim());
    }

    if let Some(processor_frequency_info) = cpuid.get_processor_frequency_info() {
        let processor_base_frequency = processor_frequency_info.processor_base_frequency();
        println!("CPU {} MHz", processor_base_frequency);
    }
}
