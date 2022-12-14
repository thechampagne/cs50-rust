use std::io::Write;

pub fn get_char(str: &str) -> Result<char, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<char>()?)
}

pub fn get_bool(str: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<bool>()?)
}

pub fn get_i8(str: &str) -> Result<i8, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<i8>()?)
}

pub fn get_i16(str: &str) -> Result<i16, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<i16>()?)
}

pub fn get_i32(str: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<i32>()?)
}

pub fn get_i64(str: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<i64>()?)
}

pub fn get_i128(str: &str) -> Result<i128, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<i128>()?)
}

pub fn get_u8(str: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<u8>()?)
}

pub fn get_u16(str: &str) -> Result<u16, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<u16>()?)
}

pub fn get_u32(str: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<u32>()?)
}

pub fn get_u64(str: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<u64>()?)
}

pub fn get_u128(str: &str) -> Result<u128, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<u128>()?)
}

pub fn get_f32(str: &str) -> Result<f32, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<f32>()?)
}

pub fn get_f64(str: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<f64>()?)
}

pub fn get_isize(str: &str) -> Result<isize, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<isize>()?)
}

pub fn get_usize(str: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim();
    Ok(string.parse::<usize>()?)
}

pub fn get_string(str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut string = String::new();
    print!("{}", str);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut string)?;
    let string = string.trim().to_string();
    Ok(string)
}