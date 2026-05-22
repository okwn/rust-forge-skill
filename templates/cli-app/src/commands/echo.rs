use anyhow::Result;

pub fn run(text: String, uppercase: bool) -> Result<()> {
    let output = if uppercase {
        text.to_uppercase()
    } else {
        text
    };

    println!("{}", output);
    Ok(())
}