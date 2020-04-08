use futures::executor::block_on;
//use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::time::Duration;

async fn get_string(input: i32) -> String {
    let mut rng = thread_rng();
    let sleep_time = rng.gen_range(1, 5);
    async_std::task::sleep(Duration::from_secs(sleep_time)).await;
    return format!("{} slept {}", input.to_string(), sleep_time);
}

async fn collect_strings(n: i32) -> Vec<String> {
    let batch_size = 16;
    let mut jobs_left = n;
    let mut result: Vec<String> = vec![];
    while jobs_left > 0 {
        let mut fut = vec![];
        let iterations = std::cmp::min(jobs_left, batch_size);
        for i in 0..iterations {
            fut.push(get_string(i));
        }
        result.extend(futures::future::join_all(fut).await);
        jobs_left = jobs_left - iterations;
    }
    result
}

fn main() {
    for r in block_on(collect_strings(18)) {
        println!("{}", r);
    }
}
