use clap::{Parser, ValueEnum};
use keystone_engine::{Arch, Keystone, KeystoneError, Mode, OptionType, OptionValue};

mod formats;
use formats::OutputFormat;

#[derive(Debug, ValueEnum, Clone)]
enum Architecture {
    Arm,
    ArmThumb,
    Arm64,
    Mips32,
    Mips64,
    Ppc32,
    Ppc64,
    Sparc32,
    Sparc64,
    Sysz,
    X86_16,
    X86_32,
    X86_64,
}

#[derive(Parser)]
struct Args {
    /// The instruction set of the assembly
    pub arch: Architecture,

    /// The code to assemble
    pub code: String,

    /// How to format the output
    #[clap(short, long, value_enum, default_value_t = OutputFormat::SpacedHexdump)]
    pub format: OutputFormat,
}

fn get_keystone(arch: Architecture) -> Result<Keystone, KeystoneError> {
    match arch {
        Architecture::Arm => Keystone::new(Arch::ARM, Mode::ARM),
        Architecture::ArmThumb => Keystone::new(Arch::ARM, Mode::THUMB),
        Architecture::Arm64 => Keystone::new(Arch::ARM64, Mode::ARM),
        Architecture::Mips32 => Keystone::new(Arch::MIPS, Mode::MIPS32),
        Architecture::Mips64 => Keystone::new(Arch::MIPS, Mode::MIPS64),
        Architecture::Ppc32 => Keystone::new(Arch::PPC, Mode::PPC32),
        Architecture::Ppc64 => Keystone::new(Arch::PPC, Mode::PPC64),
        Architecture::Sparc32 => Keystone::new(Arch::SPARC, Mode::SPARC32),
        Architecture::Sparc64 => Keystone::new(Arch::SPARC, Mode::SPARC64),
        Architecture::Sysz => Keystone::new(Arch::SYSTEMZ, Mode::LITTLE_ENDIAN),
        Architecture::X86_16 => {
            let engine = Keystone::new(Arch::X86, Mode::MODE_16)?;
            let _ = engine.option(OptionType::SYNTAX, OptionValue::SYNTAX_INTEL);
            Ok(engine)
        }
        Architecture::X86_32 => {
            let engine = Keystone::new(Arch::X86, Mode::MODE_32)?;
            let _ = engine.option(OptionType::SYNTAX, OptionValue::SYNTAX_INTEL);
            Ok(engine)
        }
        Architecture::X86_64 => {
            let engine = Keystone::new(Arch::X86, Mode::MODE_64)?;
            let _ = engine.option(OptionType::SYNTAX, OptionValue::SYNTAX_INTEL);
            Ok(engine)
        }
    }
}

fn main() {
    let args = Args::parse();

    let keystone = match get_keystone(args.arch) {
        Ok(ks) => ks,
        Err(why) => {
            eprintln!("Failed to build assembler: {why}");
            return;
        }
    };

    let bytes = match keystone.asm(args.code, 0) {
        Ok(b) => b.bytes,
        Err(why) => {
            eprintln!("Could not assemble code: {why}");
            return;
        }
    };

    let byte_str = match args.format.format_sequence(&bytes) {
        Ok(s) => s,
        Err(why) => {
            eprintln!("Could not format the result: {why}");
            return;
        }
    };

    println!("{byte_str}");
}
