use anyhow::Result;

#[test]
fn hello_word() -> Result<()> {
    println!("Hello World");
    assert_eq!(1, 1 + 1);
    Ok(())
}
