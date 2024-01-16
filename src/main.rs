extern crate winreg;
use std::io::{self, Write};
use winreg::enums::*;
use winreg::RegKey;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  
    println!("\n{}", r"   _____ ___.              ____.                     .__   ");
    println!("{}", r"    /  _  \_ |__  __ __    |    |____    _____ _____  |  |  ");
    println!("{}", r"   /  /_\  \| __ \|  |  \   |    \__  \  /     \\__  \ |  |  ");
    println!("{}", r"  /    |    \ \_\ \  |  /\__|    |/ __ \|  Y Y  \/ __ \|  |__");
    println!("{}", r"  \____|__  /___  /____/\________(____  /__|_|  (____  /____/");
    println!("{}", r"          \/    \/                    \/      \/     \/      ");


    println!("{}", r"

          --+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

            Theme color changer for windows.


          Help Menu

          0 = Dark Mode

          1 = Light Mode


          Made with love by AbuJamal (Iyad Almsara)

          --+--+--+--+--+--+--+--+--+--+--+--+--+--+

          ");

    print!("{}", "Enter your choice: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let number = input.trim().parse::<u32>();
    match number {
        Ok(0) => {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let path = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize";
            let key = hkcu.open_subkey_with_flags(path, KEY_WRITE)?;

            key.set_value("AppsUseLightTheme", &0u32)?;
            println!("{}", "\nTheme changed to Dark Mode");
            std::thread::sleep(std::time::Duration::from_millis(2000));
        },
        Ok(1) => {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let path = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize";
            let key = hkcu.open_subkey_with_flags(path, KEY_WRITE)?;

            key.set_value("AppsUseLightTheme", &1u32)?;
            println!("{}", "\nTheme changed to Light Mode");
           std::thread::sleep(std::time::Duration::from_millis(2000));
        },
        _ => {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid input")));
        }
    };

    Ok(())
}
