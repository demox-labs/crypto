use miden_crypto::Felt;
use miden_crypto::dsa::rpo_falcon512::SecretKey;
use miden_crypto::rand::RpoRandomCoin;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Log a &str the console in the browser or console.log in nodejs
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

struct BenchmarkStats {
    min: f64,
    max: f64,
    avg: f64,
    times: Vec<f64>,
}

impl BenchmarkStats {
    fn new() -> Self {
        Self {
            min: f64::MAX,
            max: f64::MIN,
            avg: 0.0,
            times: Vec::new(),
        }
    }

    fn add_time(&mut self, time: f64) {
        self.min = self.min.min(time);
        self.max = self.max.max(time);
        self.times.push(time);
        self.avg = self.times.iter().sum::<f64>() / self.times.len() as f64;
    }
}

#[wasm_bindgen]
pub fn benchmark_secret_key_generation_thread_rng() -> JsValue {
    let mut stats = BenchmarkStats::new();
    const ITERATIONS: usize = 25;

    for i in 0..ITERATIONS {
        let start = now();
        let mut rng = rand::thread_rng();
        let _secret_key = SecretKey::with_rng(&mut rng);
        let end = now();

        let time = end - start;
        stats.add_time(time);
        log(&format!("Thread RNG iteration {}: {:.2} ms", i + 1, time));
    }

    let results = js_sys::Object::new();
    js_sys::Reflect::set(&results, &"min".into(), &stats.min.into()).unwrap();
    js_sys::Reflect::set(&results, &"max".into(), &stats.max.into()).unwrap();
    js_sys::Reflect::set(&results, &"avg".into(), &stats.avg.into()).unwrap();
    js_sys::Reflect::set(
        &results,
        &"times".into(),
        &js_sys::Array::from_iter(stats.times.into_iter().map(|t| JsValue::from_f64(t))),
    )
    .unwrap();
    results.into()
}

#[wasm_bindgen]
pub fn benchmark_secret_key_generation_os_rng() -> JsValue {
    let mut stats = BenchmarkStats::new();
    const ITERATIONS: usize = 25;

    for i in 0..ITERATIONS {
        let start = now();
        let mut rng = StdRng::from_os_rng();
        let _secret_key = SecretKey::with_rng(&mut rng);
        let end = now();

        let time = end - start;
        stats.add_time(time);
        log(&format!("OS RNG iteration {}: {:.2} ms", i + 1, time));
    }

    let results = js_sys::Object::new();
    js_sys::Reflect::set(&results, &"min".into(), &stats.min.into()).unwrap();
    js_sys::Reflect::set(&results, &"max".into(), &stats.max.into()).unwrap();
    js_sys::Reflect::set(&results, &"avg".into(), &stats.avg.into()).unwrap();
    js_sys::Reflect::set(
        &results,
        &"times".into(),
        &js_sys::Array::from_iter(stats.times.into_iter().map(|t| JsValue::from_f64(t))),
    )
    .unwrap();
    results.into()
}

#[wasm_bindgen]
pub fn benchmark_secret_key_generation_felt_rng() -> JsValue {
    let mut stats = BenchmarkStats::new();
    const ITERATIONS: usize = 25;

    for i in 0..ITERATIONS {
        let start = now();
        let mut rng = StdRng::from_os_rng();
        let coin_seed: [u64; 4] = rng.random();
        let mut rng = &mut Box::new(RpoRandomCoin::new(coin_seed.map(Felt::new).into()));
        let _secret_key = SecretKey::with_rng(&mut rng);
        let end = now();

        let time = end - start;
        stats.add_time(time);
        log(&format!("OS RNG iteration {}: {:.2} ms", i + 1, time));
    }

    let results = js_sys::Object::new();
    js_sys::Reflect::set(&results, &"min".into(), &stats.min.into()).unwrap();
    js_sys::Reflect::set(&results, &"max".into(), &stats.max.into()).unwrap();
    js_sys::Reflect::set(&results, &"avg".into(), &stats.avg.into()).unwrap();
    js_sys::Reflect::set(
        &results,
        &"times".into(),
        &js_sys::Array::from_iter(stats.times.into_iter().map(|t| JsValue::from_f64(t))),
    )
    .unwrap();
    results.into()
}

#[wasm_bindgen]
pub fn run_all_benchmarks() -> JsValue {
    log("Starting benchmarks...");

    let thread_rng_stats = benchmark_secret_key_generation_thread_rng();
    let os_rng_stats = benchmark_secret_key_generation_os_rng();
    let felt_rng_stats = benchmark_secret_key_generation_felt_rng();
    let results = js_sys::Object::new();
    js_sys::Reflect::set(&results, &"thread_rng".into(), &thread_rng_stats).unwrap();
    js_sys::Reflect::set(&results, &"os_rng".into(), &os_rng_stats).unwrap();
    js_sys::Reflect::set(&results, &"felt_rng".into(), &felt_rng_stats).unwrap();
    log("Benchmarks completed!");
    results.into()
}
