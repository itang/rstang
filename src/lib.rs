#[macro_export]
macro_rules! time {
    ($arg:block) => (
        {
            extern crate time as libtime;
            let start = libtime::get_time();

            let ret = {
                $arg
            };

            {
                let elapsed = libtime::get_time() - start;
                println!("Elapsed time: {} msecs", elapsed.num_milliseconds());
            }

            ret
        }
    )
}

#[cfg(test)]
mod test {
    #[test]
    fn test_time() {
        use std::thread;
        use std::time::Duration;
        time!({
            thread::sleep(Duration::from_millis(100));
        });
    }
}
