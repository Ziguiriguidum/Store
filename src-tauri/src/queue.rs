#[derive(serde::Deserialize)]
#[derive(Debug)]
pub struct IItemQueue {
    id: String,
    path: String,
}

#[derive(serde::Deserialize)]
pub struct IQueue {
    queue: Vec<IItemQueue>,
}

pub trait QueueTrait {    
    fn add(&mut self, id: String, path: String);
    fn remove(&mut self, id: String);
    fn get(&self, id: String) -> Option<&IItemQueue>;
    fn get_all(&self) -> &Vec<IItemQueue>;    
}

impl QueueTrait for IQueue {
    fn add(&mut self, id: String, path: String) {
        self.queue.push(IItemQueue { id, path });
    }

    fn remove(&mut self, id: String) {
        self.queue.retain(|x| x.id != id);
    }

    fn get(&self, id: String) -> Option<&IItemQueue> {
        self.queue.iter().find(|x| x.id == id)
    }

    fn get_all(&self) -> &Vec<IItemQueue> {
        &self.queue
    }
}

pub static mut QUEUE: IQueue = IQueue { queue: Vec::new() };