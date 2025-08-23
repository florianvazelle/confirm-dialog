// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use slint::ComponentHandle;
use std::error::Error;
use std::process::Command;

/// Simple app that shows a confirm/cancel dialog with auto-timeout
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Timeout in seconds before auto-confirm
    #[arg(short, long, default_value_t = 15)]
    timeout: u64,

    /// Command to execute when confirmed (optional)
    #[arg(short, long)]
    command: Option<String>,
}

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let timeout_secs = args.timeout;
    let command = args.command;

    let dialog = ConfirmDialog::new()?;

    {
        let dialog_weak = dialog.as_weak();
        dialog.on_ok_clicked(move || {
            println!("Confirmed");
            if let Some(cmd) = command.as_ref() {
                run_command(cmd);
            }
            if let Some(d) = dialog_weak.upgrade() {
                d.hide().unwrap();
            }
        });
    }

    {
        let dialog_weak = dialog.as_weak();
        dialog.on_cancel_clicked(move || {
            println!("Cancelled");
            if let Some(d) = dialog_weak.upgrade() {
                d.hide().unwrap();
            }
        });
    }

    // Start countdown
    let dialog_weak = dialog.as_weak();
    start_countdown(dialog_weak, timeout_secs as i32);

    dialog.show()?;
    dialog.run()?;

    Ok(())
}

fn start_countdown(dialog_weak: slint::Weak<ConfirmDialog>, seconds: i32) {
    if let Some(d) = dialog_weak.upgrade() {
        if seconds > 0 {
            d.set_remaining_time(seconds);
            slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                start_countdown(dialog_weak, seconds - 1);
            });
        } else {
            println!("Auto-confirmed after timeout");
            d.invoke_ok_clicked();
        }
    }
}

fn run_command(cmd: &str) {
    println!("Executing command: {}", cmd);
    // Split command into program and args (basic handling)
    let mut parts = cmd.split_whitespace();
    if let Some(program) = parts.next() {
        let args: Vec<&str> = parts.collect();
        match Command::new(program).args(&args).spawn() {
            Ok(_) => println!("Command started successfully"),
            Err(e) => eprintln!("Failed to run command: {}", e),
        }
    }
}
