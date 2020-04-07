use futures::executor::block_on;

async fn get_string() -> String {
    return "s".to_owned();
}

async fn collect_strings(n: i32) -> Vec<String> {
    let batch_size = 16;
    let mut jobs_left = n;
    let mut result: Vec<String> = vec![];
    while jobs_left > 0 {
        let mut fut = vec![];
        let iterations = std::cmp::min(jobs_left, batch_size);
        for _ in 0..iterations {
            fut.push(get_string());
        }
        result.extend(futures::future::join_all(fut).await);
        jobs_left = jobs_left - iterations;
    }
    result
}

fn main() {
    let result = block_on(collect_strings(18));
    for r in result {
        println!("{}", r);
    }
}
