use std::{
    thread::JoinHandle, 
    sync::{ Mutex, Arc, mpsc::{ Sender, Receiver }}
};

enum Message
{
    NewJob(Job),
    Terminate
}

trait FnBox
{
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F
{
    fn call_box(self: Box<Self>) // signature could have been like this also: self: Box<F>
    // since the method is implemented on type F 
    {
        (*self)()
    }
}

type Job = Box< dyn FnBox + Send + 'static >;

struct Worker
{
    id: usize,
    thread: Option<JoinHandle<()>>
}

impl Worker
{
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self
    {
        let thread = std::thread::spawn( move || {
            loop
            {
                let message = receiver.lock().unwrap().recv().unwrap();
                println!("Got the job with the id {}....going to execute eh!", id);

                match message
                {
                    Message::NewJob(job) => job.call_box(),
                    Message::Terminate => break
                }
                
            }
        });
        
        Self { id, thread: Some(thread) }
    }
}

pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: Sender<Message>
}

impl Drop for ThreadPool
{
    fn drop(&mut self)
    {
        for _ in &mut self.workers
        {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers
        {
            if let Some(thread) = worker.thread.take()
            {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool
{
    pub fn new(pool_size: usize) -> Self
    {
        // We are checking the pool size
        // if the pool size is lesser than 1 we will make our program panic.
        assert!(pool_size > 0); // To check for the said condition, if not true, then thread will panic.
        let mut workers = Vec::with_capacity(pool_size);
        
        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..pool_size
        {
            workers.push(Worker::new(id, receiver.clone()));
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        // We will send the closure to the Worker
        // Let's create a closure to send over the channel
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }    
}