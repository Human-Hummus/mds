

#[macro_export]
macro_rules! info {
    ($($args:expr),*) =>{
        println!("{} > {}{}", color::Fg(color::LightBlue), format!($($args),*), color::Fg(color::Reset))
    }
}
#[macro_export]
macro_rules! trivial {
    ($($args:expr),*) =>{
        println!("{} ! Trivial Warning: {}{}", color::Fg(color::Yellow), format!($($args),*), color::Fg(color::Reset))
    }
}
#[macro_export]
macro_rules! warn {
    ($($args:expr),*) =>{
        println!("{} ! Warning: {}{}", color::Fg(color::Yellow), format!($($args),*), color::Fg(color::Reset))
    }
}
#[macro_export]
macro_rules! error {
    ($($args:expr),*) =>{
        println!("{} ! Error: {}{}", color::Fg(color::Red), format!($($args),*), color::Fg(color::Reset))
    }
}
#[macro_export]
macro_rules! fatal {
    ($($args:expr),*) =>{
        {
            println!("{} ☠ Error: {}{}", color::Fg(color::Red), format!($($args),*), color::Fg(color::Reset));
            std::process::exit(1);
        }
    }
}