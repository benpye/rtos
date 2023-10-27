pub struct Server<const MAX_INPUT: usize, T>(T);

pub trait Notify {
    fn receive_notifications(&mut self, notifications: u32);
}

pub trait Dispatch<const MAX_INPUT: usize> {
    fn dispatch_call(&mut self, sender: u8, data: [usize; MAX_INPUT], len: usize);
}

impl<const MAX_INPUT: usize, T> Server<MAX_INPUT, T>
where
    T: Dispatch<MAX_INPUT> + Notify,
{
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn listen(&mut self) -> ! {
        loop {
            let syscall::ReceiveResult {
                notifications,
                sender,
                len,
                data,
            } = syscall::sys_receive();
            if sender != u8::MAX {
                self.0.dispatch_call(sender, data, len as usize);
            }

            if notifications != 0 {
                self.0.receive_notifications(notifications);
            }
        }
    }
}
