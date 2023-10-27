use usb_device::{endpoint::EndpointType, UsbDirection};

use crate::buffer::Buffer;

pub struct Endpoint {
    index: u8,
    ep_type: Option<EndpointType>,
    claimed_in: bool,
    claimed_out: bool,
    rx_buffer: Option<Buffer>,
    tx_buffer: Option<Buffer>,
    rx_len: Option<usize>,
}

impl Endpoint {
    pub const EP_BUF_SIZE: usize = 64;

    pub fn new(index: u8) -> Self {
        Self {
            index,
            ep_type: None,
            claimed_in: false,
            claimed_out: false,
            rx_buffer: None,
            tx_buffer: None,
            rx_len: None,
        }
    }

    pub fn is_claimed(&self, ep_dir: UsbDirection) -> bool {
        match ep_dir {
            UsbDirection::Out => self.claimed_out,
            UsbDirection::In => self.claimed_in,
        }
    }

    pub fn rx_en(&self) -> bool {
        self.is_claimed(UsbDirection::Out)
    }

    pub fn tx_en(&self) -> bool {
        self.is_claimed(UsbDirection::In)
    }

    pub fn rx_len(&self) -> Option<usize> {
        self.rx_len
    }

    pub fn set_rx_len(&mut self, len: Option<usize>) {
        self.rx_len = len;
    }

    pub fn bytes_needed_for_claim(&self) -> usize {
        // Endpoint zero has a shared in/out buffer so only allocate only
        // allocate if neither direction has been claimed
        match (self.index, self.claimed_in, self.claimed_out) {
            (0, true, _) | (0, _, true) => 0,
            _ => Self::EP_BUF_SIZE,
        }
    }

    pub fn claim(&mut self, ep_dir: UsbDirection) {
        assert!(!self.is_claimed(ep_dir));
        match ep_dir {
            UsbDirection::Out => self.claimed_out = true,
            UsbDirection::In => self.claimed_in = true,
        }
    }

    pub fn ep_type(&self) -> Option<EndpointType> {
        self.ep_type
    }

    pub fn set_ep_type(&mut self, ep_type: EndpointType) {
        assert!(self.ep_type == None || self.ep_type == Some(ep_type));
        self.ep_type = Some(ep_type);
    }

    pub fn set_rx_buffer(&mut self, buffer: Buffer) {
        assert!(self.rx_en());
        assert!(buffer.len() == 64);
        assert!(self.rx_buffer.is_none());
        self.rx_buffer = Some(buffer);
    }

    pub fn read(&self, buffer: &mut [u8]) -> usize {
        self.rx_buffer.as_ref().unwrap().volatile_read(buffer)
    }

    pub fn set_tx_buffer(&mut self, buffer: Buffer) {
        assert!(self.tx_en());
        assert!(buffer.len() == 64);
        assert!(self.tx_buffer.is_none());
        self.tx_buffer = Some(buffer);
    }

    pub fn write(&mut self, buffer: &[u8]) -> usize {
        let buf = if self.index == 0 {
            &mut self.rx_buffer
        } else {
            &mut self.tx_buffer
        };

        buf.as_mut().unwrap().volatile_write(buffer)
    }
}
