use crate::kernel::KernelRequirement;
use crate::firmware::FirmwareRequirement;

pub trait HardwareProfile {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn required_kernel_configs(&self) -> &[KernelRequirement];
    fn required_firmware(&self) -> &[FirmwareRequirement];
    
    fn recommended_kernel_configs(&self) -> &[KernelRequirement] { &[] }
    fn recommended_firmware(&self) -> &[FirmwareRequirement] { &[] }
}

pub mod nuc;
pub mod thinkpad;
pub mod desktop;
pub mod framework;
pub mod mini_pc;

pub fn get_all_profiles() -> Vec<Box<dyn HardwareProfile>> {
    vec![
        Box::new(nuc::IntelNucProfile),
        Box::new(thinkpad::ThinkPadProfile),
        Box::new(desktop::DesktopProfile),
        Box::new(framework::FrameworkProfile),
        Box::new(mini_pc::AmdMiniPcProfile),
    ]
}
