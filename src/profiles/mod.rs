use crate::firmware::FirmwareRequirement;
use crate::kernel::KernelRequirement;

pub trait HardwareProfile {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn required_kernel_configs(&self) -> &[KernelRequirement];
    fn required_firmware(&self) -> &[FirmwareRequirement];

    fn recommended_kernel_configs(&self) -> &[KernelRequirement] {
        &[]
    }
    fn recommended_firmware(&self) -> &[FirmwareRequirement] {
        &[]
    }
}

// === Compact Systems ===
pub mod mini_pc;
pub mod nuc;

// === Laptops - Business/Premium ===
pub mod dell_xps;
pub mod framework;
pub mod surface;
pub mod thinkpad;

// === Laptops - Gaming ===
pub mod gaming_laptop;

// === Laptops - Linux-First ===
pub mod linux_laptop;

// === Desktops ===
pub mod desktop;
pub mod gaming_desktop;

// === Workstations & Servers ===
pub mod homelab;
pub mod workstation;

// === Specialized ===
pub mod steam_deck;

pub fn get_all_profiles() -> Vec<Box<dyn HardwareProfile>> {
    vec![
        // Compact Systems
        Box::new(nuc::IntelNucProfile),
        Box::new(mini_pc::AmdMiniPcProfile),
        // Laptops - Business/Premium
        Box::new(thinkpad::ThinkPadProfile),
        Box::new(dell_xps::DellXpsProfile),
        Box::new(framework::FrameworkProfile),
        Box::new(surface::SurfaceProfile),
        // Laptops - Gaming
        Box::new(gaming_laptop::GamingLaptopProfile),
        // Laptops - Linux-First
        Box::new(linux_laptop::LinuxLaptopProfile),
        // Desktops
        Box::new(desktop::DesktopProfile),
        Box::new(gaming_desktop::GamingDesktopProfile),
        // Workstations & Servers
        Box::new(workstation::WorkstationProfile),
        Box::new(homelab::HomelabProfile),
        // Specialized
        Box::new(steam_deck::SteamDeckProfile),
    ]
}

/// Get profiles by category
pub fn get_profiles_by_category(category: &str) -> Vec<Box<dyn HardwareProfile>> {
    match category.to_lowercase().as_str() {
        "compact" | "mini" => vec![
            Box::new(nuc::IntelNucProfile),
            Box::new(mini_pc::AmdMiniPcProfile),
        ],
        "laptop" | "laptops" => vec![
            Box::new(thinkpad::ThinkPadProfile),
            Box::new(dell_xps::DellXpsProfile),
            Box::new(framework::FrameworkProfile),
            Box::new(surface::SurfaceProfile),
            Box::new(gaming_laptop::GamingLaptopProfile),
            Box::new(linux_laptop::LinuxLaptopProfile),
        ],
        "desktop" | "desktops" => vec![
            Box::new(desktop::DesktopProfile),
            Box::new(gaming_desktop::GamingDesktopProfile),
        ],
        "gaming" => vec![
            Box::new(gaming_desktop::GamingDesktopProfile),
            Box::new(gaming_laptop::GamingLaptopProfile),
            Box::new(steam_deck::SteamDeckProfile),
        ],
        "server" | "workstation" => vec![
            Box::new(workstation::WorkstationProfile),
            Box::new(homelab::HomelabProfile),
        ],
        _ => get_all_profiles(),
    }
}
