use anyhow::Result;
use clap::{Parser, Subcommand};
use hardware_compat::{profiles, HardwareCompatChecker};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        short,
        long,
        default_value = ".artifacts/out/leviso/kernel-build/.config"
    )]
    kernel_config: PathBuf,

    #[arg(
        short = 'F',
        long,
        default_value = ".artifacts/out/leviso/squashfs-root/usr/lib/firmware"
    )]
    firmware_dir: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify hardware compatibility against all profiles
    Verify {
        /// Specific profile to check
        #[arg(short, long)]
        profile: Option<String>,
    },
    /// List all available hardware profiles
    Profiles,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let checker = HardwareCompatChecker::new(cli.kernel_config, cli.firmware_dir);

    match cli.command {
        Commands::Verify { profile } => {
            let all_profiles = profiles::get_all_profiles();
            let mut results = Vec::new();

            if let Some(profile_name) = profile {
                let p = all_profiles
                    .iter()
                    .find(|p| {
                        p.name()
                            .to_lowercase()
                            .contains(&profile_name.to_lowercase())
                    })
                    .ok_or_else(|| anyhow::anyhow!("Profile '{}' not found", profile_name))?;

                let report = checker.verify_profile(p.as_ref())?;
                report.print_summary();
                results.push(report);
            } else {
                for p in all_profiles {
                    let report = checker.verify_profile(p.as_ref())?;
                    report.print_summary();
                    results.push(report);
                }
            }

            let failed = results.iter().any(|r| r.has_critical_failures());
            if failed {
                std::process::exit(1);
            }
        }
        Commands::Profiles => {
            println!("Available Hardware Profiles:");
            for p in profiles::get_all_profiles() {
                println!("  - {:<30} {}", p.name(), p.description());
            }
        }
    }

    Ok(())
}
