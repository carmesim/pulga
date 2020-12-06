mod exofetch;

use colored::Colorize; // This trait allows us to use colors on Strings and &strs

fn main() {
    let user_data = exofetch::get_user_data();
    println!("{}: {}", "username".cyan(), user_data.username.red());
    println!("{}: {}", "hostname".cyan(), user_data.hostname.red());
    println!("{}: {}", "device name".cyan(), user_data.devicename.red());
    println!("{}: {}", "home dir.".cyan(), user_data.hmd.red());
    println!("{}: {}", "platform".cyan(), user_data.platform.red());
    println!("{}: {}", "distro".cyan(), user_data.distro.red());
    println!("{}: {}", "desktop env.".cyan(), user_data.desk_env.red());
    println!(
        "{}: {}/{}",
        "memory usage".cyan(),
        user_data.used_memory.red(),
        user_data.total_memory.red()
    );
}
