#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use embassy_stm32 as _;
    #[test]
    fn first_test() -> Result<(), &'static str> {
        Ok(())
    }
}
