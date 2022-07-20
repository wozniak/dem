use dem::*;

fn main() -> anyhow::Result<()> {
    let demo = dem::Demo::open("c:\\users\\wozniak\\documents\\08-1163.dem")?;

    dbg!(demo);

    println!("press enter...");
    let mut str = String::new();
    std::io::stdin().read_line(&mut str)?;

    Ok(())
}
