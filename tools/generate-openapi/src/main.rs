fn main() -> eyre::Result<()> {
    let current_dir = std::env::current_dir()?;
    let path = current_dir.join("openapi.json");

    let schema = oxidrive_web::openapi_schema().to_pretty_json()?;

    std::fs::write(&path, schema)?;
    eprintln!("schema written to {}", path.display());

    Ok(())
}
