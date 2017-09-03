// silly-loop.rs
pub fn silly_loop() {
    for _ in 1..1_000_000__000 {}
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    pub fn test_silly_loop() {
        ::silly_loop();
    }
}
