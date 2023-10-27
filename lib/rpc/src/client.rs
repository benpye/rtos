use core::marker::PhantomData;

pub struct Client<T> {
    task_id: u8,
    phantom: PhantomData<T>,
}

impl<T> Client<T> {
    pub fn new(task_id: u8) -> Self {
        Client {
            task_id,
            phantom: PhantomData,
        }
    }

    pub fn task_id(&self) -> u8 {
        self.task_id
    }
}
