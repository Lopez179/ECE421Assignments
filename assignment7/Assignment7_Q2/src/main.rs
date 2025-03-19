use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::sync::Arc;
fn main()
{
    let mut sample_data = Arc::new(Mutex::new(vec![1, 81, 107]));

    for i in 0..10
    {
        let sample_data = sample_data.clone();
        thread::spawn(move || { sample_data.lock().unwrap()[0] += i; }); // fails here
    }
    thread::sleep(Duration::from_millis(50));
}