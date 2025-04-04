use crossterm::style::{Color, Stylize};

pub fn summon_crab() {
    println!("{}", "🦀 You have summoned Ferris!".with(Color::Yellow));
    println!(
        "{}",
        r#"
       _     _
     _/ \~~~/ \_
 ,----(   o   o   )
/      \__   ^__/
\_|      (___)
  |    |_|_|
  \_\  |_|_|
   (_)   (_)"#
            .with(Color::Cyan)
    );
}

pub fn panic_mode() {
    println!("{}", "System panic!!!".with(Color::Red));
    println!("Dumping fake memory... 💾");
    for i in (1..=3).rev() {
        println!("Rebooting in {}...", i);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

pub fn help() {
    println!("Available commands:");
    println!("- login <username>");
    println!("- logout");
    println!("- summon crab");
    println!("- panic");
    println!("- crabgpt <question>");
    println!("- encrypt <text>");
    println!("- decrypt <base64>");
    println!("- crabtop");
    println!("- play hangman");
    println!("- exit");
}
