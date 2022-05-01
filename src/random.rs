use rand::Rng;

pub fn gen_random() {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    println!("{}", random_number);
}
