use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use hacc::{Image, Preloader, TryRead};

#[derive(Clone, ValueEnum)]
enum Mode {
    Preloader,
    LK,
    DA,
}

#[derive(Parser)]
struct Cli {
    /// Input file
    #[arg(short, long)]
    input: PathBuf,
    /// Output directory
    #[arg(short, long)]
    output: PathBuf,

    /// Parser mode
    #[arg(short, long)]
    mode: Mode,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.input.is_file() {
        anyhow::bail!("Input doesn't exist");
    }

    if !cli.output.is_dir() {
        fs::create_dir(&cli.output)?;
    }

    let data = fs::read(cli.input)?;

    let out = cli.output;
    match cli.mode {
        Mode::Preloader => {
            let pl = Preloader::try_read(&data)?;

            let fi = pl.gfh().file_info();
            println!("Base: {:#x}", fi.load_addr() + fi.jump_offset());

            fs::write(out.join("pl.bin"), pl.content())?;
        }
        Mode::LK => {
            let lk = Image::new(&data);

            for p in lk.partitions() {
                let name = p.header.name();
                println!("Partition {}:", name);
                println!("\tBase: {:#x}", p.header.addr());
                println!();

                fs::write(out.join(format!("{}.bin", name)), p.content)?;

                for c in lk.get_part_certs(name) {
                    fs::write(out.join(format!("{}-{}.bin", name, c.header.name())), c.content)?;
                }
            }
        }
        _ => todo!(),
    }

    Ok(())
}
