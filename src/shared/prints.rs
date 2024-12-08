use std::io::Stdout;

use crossterm::{cursor::MoveTo, execute, style::Stylize, terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate}};

use super::get::*;

pub fn print_fetch(stdout: &mut Stdout) -> anyhow::Result<()>{
    let envs = get_envs();
    let total_ram: f32 = (10 * get_ram().1 / 2u64.pow(30)) as f32 / 10f32;
    let used_ram: f32 = (10 * get_ram().0 / 2u64.pow(30)) as f32 / 10f32;
    let total_swap: f32 = (10 * get_swap().1 / 2u64.pow(30)) as f32 / 10f32;
    let used_swap: f32 = (10 * get_swap().0 / 2u64.pow(30)) as f32 / 10f32;

    execute!(stdout, BeginSynchronizedUpdate)?;
    println!("Kernel: {}", get_kernel());
    execute!(stdout, MoveTo(0, 1))?;
    println!("Distro: {}", get_distro());
    execute!(stdout, MoveTo(0, 2))?;
    println!("Uptime: {}", get_uptime());
    execute!(stdout, MoveTo(0, 3))?;
    println!("RAM: {} / {} GB", used_ram, total_ram);
    execute!(stdout, MoveTo(0, 4))?;
    println!("Swap: {} / {} GB", used_swap, total_swap);
    execute!(stdout, MoveTo(0, 5))?;
    println!("CPU: {}", get_cpu());
    execute!(stdout, MoveTo(0, 6))?;
    println!("Arch: {}", get_arch());
    execute!(stdout, MoveTo(0, 8))?;
    println!("Display Server: {}", envs.get("XDG_SESSION_TYPE").unwrap());
    execute!(stdout, MoveTo(0, 9))?;
    println!("DE: {}", envs.get("DESKTOP_SESSION").unwrap());
    execute!(stdout, MoveTo(0, 10))?;
    println!("Shell: {}", envs.get("SHELL").unwrap());
    execute!(stdout, MoveTo(0, 11))?;
    println!("User: {}", envs.get("USER").unwrap());
    execute!(stdout, MoveTo(0, 12))?;
    println!("Terminal: {}", envs.get("TERM").unwrap());
    execute!(stdout, MoveTo(0, 13))?;
    println!("Editor: {}", envs.get("EDITOR").unwrap());
    execute!(stdout, MoveTo(0, 14))?;
    println!("Browser: {}", envs.get("BROWSER").unwrap());
    execute!(stdout, MoveTo(0, 15))?;
    println!(
        "{} {} {} {} {} {} {} {}",
        "⬤".dark_grey(),
        "⬤".red(),
        "⬤".green(),
        "⬤".yellow(),
        "⬤".blue(),
        "⬤".magenta(),
        "⬤".cyan(),
        "⬤".grey(),
    );

    execute!(stdout, EndSynchronizedUpdate)?;
    Ok(())
}
