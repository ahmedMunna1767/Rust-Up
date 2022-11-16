pub fn if_let() {
    let config_max = Some(35i8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
