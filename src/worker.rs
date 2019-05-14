extern crate redis;

use worker::redis::Commands;
use config::ConnectionConfig;
use config::QueueConfig;

pub fn main(thread_number: usize, config: ConnectionConfig, queue: QueueConfig, other_queues: Vec<QueueConfig>) {
    println!("#{} using {}", thread_number, queue);
    for _i in 1..10 {
        if pop_and_process(thread_number, &config, &queue, true) {
            continue;
        }
        if pop_and_process(thread_number, &config, &queue, false) {
            continue;
        }

        for i in 0..other_queues.len() {
            if pop_and_process(thread_number, &config, &other_queues[i], true) {
                break;
            }
            if pop_and_process(thread_number, &config, &other_queues[i], false) {
                break;
            }
        }
    }
}

fn pop_and_process(thread_number: usize, config: &ConnectionConfig, queue: &QueueConfig, priority: bool) -> bool {
    let queue_name = match priority {
        true => queue.get_priority_queue_name(),
        false => queue.get_default_queue_name(),
    };

    let pulled_value = pop_from_queue(&config, &queue_name);
    let pull_result = pulled_value.is_ok();

    match pulled_value {
        Ok(value) => println!("#{} pull from {}: {}", thread_number, queue_name, value.1),
        Err(value) => println!("#{} pull from {}: {}", thread_number, queue_name, value),
    }

    return pull_result;
}

fn pop_from_queue(config: &ConnectionConfig, queue_name: &String) -> redis::RedisResult<(String, isize)> {
    let connection_string = config.get_connection_string();
    let client = redis::Client::open(connection_string.as_str())?;
    let connection = client.get_connection()?;
    connection.blpop(queue_name, config.timeout)
}