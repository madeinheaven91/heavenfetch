use std::{collections::HashMap, process::Command};
use sysinfo::System;



pub fn get_kernel() -> String {
    System::kernel_version().unwrap_or("Unknown".into())
}

pub fn get_distro() -> String {
    System::name().unwrap_or("Unknown".into())
}

pub fn get_arch() -> String {
    System::cpu_arch()
}

pub fn get_uptime() -> String {
    let seconds = System::uptime();
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;

    if days > 0 {
        format!(
            "{} days, {} hours, {} minutes, {} seconds",
            days,
            hours % 24,
            minutes % 60,
            seconds % 60
        )
    } else if hours > 0 {
        format!(
            "{} hours, {} minutes, {} seconds",
            hours % 24,
            minutes % 60,
            seconds % 60
        )
    } else if minutes > 0 {
        format!("{} minutes, {} seconds", minutes % 60, seconds % 60)
    } else {
        format!("{} seconds", seconds % 60)
    }
}

pub fn get_ram() -> (u64, u64) {
    let s = System::new_all();
    let total_ram = s.total_memory();
    let used_ram = s.used_memory();
    (used_ram, total_ram)
}

pub fn get_swap() -> (u64, u64) {
    let s = System::new_all();
    let total_swap = s.total_swap();
    let used_swap = s.used_swap();
    (used_swap, total_swap)
}

pub fn get_cpu() -> String {
    let s = System::new_all();

    let cpu = s.cpus();
    let cpu_brand = cpu[0].brand().to_string();
    format!("{} ({} cores)", cpu_brand, cpu.len())
}

pub fn get_envs() -> HashMap<String, String>{
    let output = Command::new("env")
        .output()
        .expect("failed to execute process");
    let binding = String::from_utf8(output.stdout).unwrap();
    let envs = binding.split("\n").collect::<Vec<&str>>();

    let mut envs_map = HashMap::new();
    for env in envs {
        let (key, value) = env.split_once("=").unwrap_or(("", ""));
        envs_map.insert(key.into(), value.into());
    }
    envs_map
}
