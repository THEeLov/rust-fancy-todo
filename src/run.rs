use crate::error;

pub fn run() -> error::Result<()> {
    let stdin = std::io::stdin();

    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
    }

    Ok(())
}
