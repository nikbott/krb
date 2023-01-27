use krb::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run::run()?;
    Ok(())
}
