#[cfg(test)]
mod test {
    use rand::Rng;

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0.0..1.);
        println!("{}", roll);
    }
}
