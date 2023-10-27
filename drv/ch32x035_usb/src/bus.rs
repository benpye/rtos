use core::{array, ops::Deref};

use ch32x0::ch32x035 as device;
use spin::mutex::SpinMutex;
use usb_device::{
    bus::PollResult,
    endpoint::{EndpointAddress, EndpointType},
    UsbDirection, UsbError,
};

use crate::endpoint::Endpoint;
use crate::pac::usbfs;

struct UsbPeripheral(device::USBFS);

impl UsbPeripheral {
    const NUM_ENDPOINTS: usize = 8;

    pub fn ep_ctrl_h(&self, index: usize) -> &usbfs::EP_CTRL_H {
        match index {
            0 => &self.ep0_ctrl_h,
            1 => &self.ep1_ctrl_h,
            2 => &self.ep2_ctrl_h,
            3 => &self.ep3_ctrl_h,
            4 => &self.ep4_ctrl_h,
            5 => &self.ep5_ctrl_h,
            6 => &self.ep6_ctrl_h,
            7 => &self.ep7_ctrl_h,
            _ => unimplemented!(),
        }
    }

    pub fn ep_tx_len(&self, index: usize) -> &usbfs::EP_TX_LEN {
        match index {
            0 => &self.ep0_tx_len,
            1 => &self.ep1_tx_len,
            2 => &self.ep2_tx_len,
            3 => &self.ep3_tx_len,
            4 => &self.ep4_tx_len,
            5 => &self.ep5_tx_len,
            6 => &self.ep6_tx_len,
            7 => &self.ep7_tx_len,
            _ => unimplemented!(),
        }
    }

    pub fn ep_dma(&self, index: usize) -> &usbfs::EP_DMA {
        match index {
            0 => &self.ep0_dma,
            1 => &self.ep1_dma,
            2 => &self.ep2_dma,
            3 => &self.ep3_dma,
            5 => &self.ep5_dma,
            6 => &self.ep6_dma,
            7 => &self.ep7_dma,
            _ => unimplemented!(),
        }
    }
}

impl Deref for UsbPeripheral {
    type Target = usbfs::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        // Safety: Transmuting the public crate's RegisterBlock to our private
        // version is safe as they are equivalent safe abstractions over the
        // peripheral.
        unsafe { core::mem::transmute(&*self.0) }
    }
}

struct Driver<const BUF_LEN: usize> {
    usbfs: UsbPeripheral,
    endpoints: [Endpoint; UsbPeripheral::NUM_ENDPOINTS],
    allocator: crate::buffer::Allocator,
    buffer_capacity: usize,
    pending_out: u16,
    pending_setup: bool,
}

impl<const BUF_LEN: usize> Driver<BUF_LEN> {
    pub fn new(
        usbfs: device::USBFS,
        buffer: &'static crate::buffer::EndpointMemory<BUF_LEN>,
    ) -> Self {
        Self {
            usbfs: UsbPeripheral(usbfs),
            endpoints: array::from_fn(|i| Endpoint::new(i as u8)),
            allocator: buffer.allocator().unwrap(),
            buffer_capacity: BUF_LEN,
            pending_out: 0,
            pending_setup: false,
        }
    }
}

impl<const BUF_LEN: usize> Driver<BUF_LEN> {
    fn alloc_ep(
        &mut self,
        ep_dir: usb_device::UsbDirection,
        ep_addr: Option<usb_device::endpoint::EndpointAddress>,
        ep_type: usb_device::endpoint::EndpointType,
        max_packet_size: u16,
        _interval: u8,
    ) -> usb_device::Result<usb_device::endpoint::EndpointAddress> {
        let ep_addr = ep_addr.or(if ep_type == EndpointType::Control {
            Some(EndpointAddress::from_parts(0, ep_dir))
        } else {
            None
        });

        // We cannot support packet sizes beyond 64 bytes as that is the
        // maximum size of the DMA buffer.
        if max_packet_size > 64 {
            return Err(UsbError::Unsupported);
        }

        if let Some(ep_addr) = ep_addr {
            // If a specific endpoint was specified ensure it is free
            let ep = &mut self.endpoints[ep_addr.index()];

            assert!(ep_type != EndpointType::Control || ep_addr.index() == 0);

            if ep.is_claimed(ep_dir) {
                return Err(UsbError::InvalidEndpoint);
            }

            // Endpoint zero may only be a control endpoint
            if ep_addr.index() == 0 && ep_type != EndpointType::Control {
                return Err(UsbError::Unsupported);
            }

            // Ensure we have enough memory
            let bytes_required = ep.bytes_needed_for_claim();
            if bytes_required > self.buffer_capacity {
                return Err(UsbError::EndpointMemoryOverflow);
            }

            if ep.ep_type() == None {
                ep.set_ep_type(ep_type);
            } else if ep.ep_type() != Some(ep_type) {
                // The endpoint has already been allocated in the other
                // direction but as a different type
                return Err(UsbError::InvalidEndpoint);
            }

            self.buffer_capacity -= bytes_required;
            ep.claim(ep_dir);

            return Ok(ep_addr);
        } else {
            // Otherwise find the next free endpoint.
            for i in 1..UsbPeripheral::NUM_ENDPOINTS {
                let ep_addr = EndpointAddress::from_parts(i, ep_dir);
                let ep = &mut self.endpoints[i];

                assert!(ep_type != EndpointType::Control);

                // The endpoint must not be claimed for the given direction
                if ep.is_claimed(ep_dir) {
                    continue;
                }

                // Ensure we have enough memory - we can fail immediately here
                // as this requirement is the same for all endpoints
                let bytes_required = ep.bytes_needed_for_claim();
                if bytes_required > self.buffer_capacity {
                    return Err(UsbError::EndpointMemoryOverflow);
                }

                // The endpoint type must be unset or matching
                match ep.ep_type() {
                    None => ep.set_ep_type(ep_type),
                    Some(t) => {
                        if t != ep_type {
                            continue;
                        }
                    }
                }

                self.buffer_capacity -= bytes_required;
                ep.claim(ep_dir);

                return Ok(ep_addr);
            }
        }

        // No free endpoints.
        Err(UsbError::EndpointOverflow)
    }

    fn ep_reset_all(&mut self) {
        for idx in 0..UsbPeripheral::NUM_ENDPOINTS {
            self.usbfs.ep_tx_len(idx).write(|w| w.t_len().variant(0));
            self.usbfs.ep_ctrl_h(idx).write(|w| {
                w.r_res()
                    .variant(if self.endpoints[idx].rx_en() {
                        usbfs::ep_ctrl_h::R_RES_A::Ack
                    } else {
                        usbfs::ep_ctrl_h::R_RES_A::Nak
                    })
                    .t_res()
                    .nak()
            })
        }

        self.pending_out = 0;
        self.pending_setup = false;

        for ep in self.endpoints.iter_mut() {
            ep.set_rx_len(None);
        }
    }

    pub fn enable(&mut self) {
        self.usbfs.base_ctrl.reset();

        self.usbfs.ep4_1_mod.write(|w| {
            w.ep1_r_en()
                .bit(self.endpoints[1].rx_en())
                .ep1_t_en()
                .bit(self.endpoints[1].tx_en())
                .ep4_r_en()
                .bit(self.endpoints[4].rx_en())
                .ep4_t_en()
                .bit(self.endpoints[4].tx_en())
        });

        self.usbfs.ep2_3_mod.write(|w| {
            w.ep3_r_en()
                .bit(self.endpoints[3].rx_en())
                .ep3_t_en()
                .bit(self.endpoints[3].tx_en())
                .ep2_r_en()
                .bit(self.endpoints[2].rx_en())
                .ep2_t_en()
                .bit(self.endpoints[2].tx_en())
        });

        self.usbfs.ep567_mod.write(|w| {
            w.ep7_r_en()
                .bit(self.endpoints[7].rx_en())
                .ep7_t_en()
                .bit(self.endpoints[7].tx_en())
                .ep6_r_en()
                .bit(self.endpoints[6].rx_en())
                .ep6_t_en()
                .bit(self.endpoints[6].tx_en())
                .ep5_r_en()
                .bit(self.endpoints[5].rx_en())
                .ep5_t_en()
                .bit(self.endpoints[5].tx_en())
        });

        // Unwrap should never fail here as the remaining memory is tracked on
        // each call to alloc_ep
        let mut allocate_buffer = |idx: usize| {
            let ep = &mut self.endpoints[idx];
            let mut dma_addr = None;
            if ep.tx_en() && idx != 0 {
                let mut buf = self.allocator.allocate(64).unwrap();
                let buf_addr = buf.as_ptr_mut();
                ep.set_tx_buffer(buf);
                dma_addr = Some(buf_addr as u16);
            }
            if ep.rx_en() || idx == 0 {
                let mut buf = self.allocator.allocate(64).unwrap();
                let buf_addr = buf.as_ptr_mut();
                ep.set_rx_buffer(buf);
                let _ = dma_addr.insert(buf_addr as u16);
            }
            dma_addr.unwrap_or(0)
        };

        // The allocation order is important, endpoints 0 and 4 use a single
        // register to specify the DMA buffer for both endpoints.
        let _buf_addr4 = allocate_buffer(4);
        for idx in [0, 1, 2, 3, 5, 6, 7] {
            let buf_addr = allocate_buffer(idx);
            self.usbfs
                .ep_dma(idx)
                .write(|w| w.buf_addr().variant(buf_addr));
        }

        self.ep_reset_all();

        self.usbfs.dev_addr.write(|w| w.usb_addr().variant(0));

        const SYS_MODE_UC_DEV_PU_EN: u8 = 0b10;

        self.usbfs.base_ctrl.write(|w| {
            w.sys_mode()
                .variant(SYS_MODE_UC_DEV_PU_EN)
                .int_busy()
                .set_bit()
                .rst_sie()
                .clear_bit()
                .clr_all()
                .clear_bit()
                .dma_en()
                .set_bit()
        });

        self.usbfs.int_fg.write(|w| {
            w.bus_rst()
                .set_bit()
                .transfer()
                .set_bit()
                .suspend()
                .set_bit()
                .hst_sof()
                .set_bit()
                .fifo_ov()
                .set_bit()
        });

        self.usbfs
            .dev_ctrl
            .write(|w| w.pd_dis().set_bit().port_en().set_bit());

        self.usbfs.int_en.write(|w| {
            w.suspend()
                .set_bit()
                .transfer()
                .set_bit()
                .bus_rst()
                .set_bit()
        });
    }

    fn poll(&mut self) -> usb_device::bus::PollResult {
        let int_flag = self.usbfs.int_fg.read();
        let int_stat = self.usbfs.int_st.read();

        let mut in_complete = 0;

        if int_flag.bus_rst().bit_is_set() {
            self.usbfs.int_fg.write(|w| w.bus_rst().set_bit());

            return PollResult::Reset;
        } else if int_flag.transfer().bit_is_set() {
            let ep_num = int_stat.endp().bits() as usize;
            let rx_len = self.usbfs.rx_len.read().rx_len().bits();

            match int_stat.token().variant() {
                usbfs::int_st::TOKEN_A::Out => {
                    if int_stat.tog_ok().bit() {
                        self.pending_out |= 1 << ep_num;
                        self.endpoints[ep_num].set_rx_len(Some(rx_len as usize));

                        // NAK future OUT transfers until the packet has been read.
                        self.usbfs.ep_ctrl_h(ep_num).modify(|_, w| w.r_res().nak());
                    }
                }
                usbfs::int_st::TOKEN_A::Sof => {
                    // Ignore SOF packet transfers
                }
                usbfs::int_st::TOKEN_A::In => {
                    in_complete = 1 << ep_num;

                    // NAK future IN transfers until new data is ready.
                    if ep_num == 0 {
                        // Endpoint 0 has a shared TX/RX DMA buffer, so we
                        // set RX to ACK again as well.
                        self.usbfs
                            .ep0_ctrl_h
                            .modify(|_, w| w.t_res().nak().r_res().ack());
                    } else {
                        self.usbfs
                            .ep_ctrl_h(ep_num as usize)
                            .modify(|_, w| w.t_res().nak());
                    }
                }
                usbfs::int_st::TOKEN_A::Setup => {
                    self.pending_setup = true;
                    self.endpoints[0].set_rx_len(Some(8));

                    self.usbfs
                        .ep0_ctrl_h
                        .modify(|_, w| w.t_res().nak().r_res().nak());
                }
            }

            self.usbfs.int_fg.write(|w| w.transfer().set_bit());
        } else if int_flag.suspend().bit_is_set() {
            self.usbfs.int_fg.write(|w| w.suspend().set_bit());

            return PollResult::Suspend;
        }

        if self.pending_setup || self.pending_out != 0 || in_complete != 0 {
            PollResult::Data {
                ep_out: self.pending_out,
                ep_in_complete: in_complete,
                ep_setup: if self.pending_setup { 1 } else { 0 },
            }
        } else {
            PollResult::None
        }
    }

    fn read(
        &mut self,
        ep_addr: usb_device::endpoint::EndpointAddress,
        buf: &mut [u8],
    ) -> usb_device::Result<usize> {
        let ep = &mut self.endpoints[ep_addr.index()];
        if !ep.rx_en() {
            return Err(UsbError::InvalidEndpoint);
        }

        if let Some(rx_len) = ep.rx_len() {
            if rx_len <= buf.len() {
                ep.read(&mut buf[..rx_len]);
            }

            ep.set_rx_len(None);

            self.pending_out &= !(1 << ep_addr.index());

            if ep_addr.index() == 0 && self.pending_setup {
                self.pending_setup = false;
                self.usbfs
                    .ep0_ctrl_h
                    .modify(|_, w| w.r_tog().set_bit().t_tog().clear_bit().r_res().ack());
            } else {
                self.usbfs
                    .ep_ctrl_h(ep_addr.index())
                    .modify(|r, w| w.r_tog().bit(!r.r_tog().bit()).r_res().ack());
            }

            if rx_len > buf.len() {
                return Err(UsbError::BufferOverflow);
            }

            Ok(rx_len)
        } else {
            Err(UsbError::WouldBlock)
        }
    }

    fn write(
        &mut self,
        ep_addr: usb_device::endpoint::EndpointAddress,
        buf: &[u8],
    ) -> usb_device::Result<usize> {
        let ep_num = ep_addr.index();
        let ep = &mut self.endpoints[ep_num];

        if !ep.tx_en() {
            return Err(UsbError::InvalidEndpoint);
        }

        if buf.len() > 64 {
            return Err(UsbError::BufferOverflow);
        }

        // EP0 has shared TX/RX and so we must block for pending RX if this TX
        // has len >0, otherwise block if a TX is pending.
        let blocked = (ep_num == 0 && (ep.rx_len().is_some() && buf.len() > 0))
            || self.usbfs.ep_ctrl_h(ep_num).read().t_res().is_ack();

        if blocked {
            return Err(UsbError::WouldBlock);
        }

        let bytes_written = ep.write(buf);

        if ep_num == 0 {
            self.usbfs
                .ep0_tx_len
                .write(|w| w.t_len().variant(bytes_written as u8));

            // Due to shared TX/RX buffers we must NAK RX until TX is
            // complete if TX len > 0.
            self.usbfs.ep0_ctrl_h.modify(|r, w| {
                w.t_res()
                    .ack()
                    .r_res()
                    .variant(if buf.len() > 0 {
                        usbfs::ep_ctrl_h::R_RES_A::Nak
                    } else {
                        usbfs::ep_ctrl_h::R_RES_A::Ack
                    })
                    .t_tog()
                    .bit(!r.t_tog().bit())
            });
        } else {
            self.usbfs
                .ep_tx_len(ep_num)
                .write(|w| w.t_len().variant(bytes_written as u8));

            self.usbfs
                .ep_ctrl_h(ep_num)
                .modify(|r, w| w.t_res().ack().t_tog().bit(!r.t_tog().bit()));
        }

        Ok(bytes_written)
    }

    fn set_stalled(&mut self, ep_addr: usb_device::endpoint::EndpointAddress, stalled: bool) {
        let ep_num = ep_addr.index();
        match (stalled, ep_addr.direction()) {
            (true, UsbDirection::Out) => {
                self.usbfs
                    .ep_ctrl_h(ep_num)
                    .modify(|_, w| w.r_res().stall());
            }
            (true, UsbDirection::In) => {
                self.usbfs
                    .ep_ctrl_h(ep_num)
                    .modify(|_, w| w.t_res().stall());
            }
            (false, UsbDirection::Out) => {
                // Reset to NAK if there is still data to be read, otherwise ACK.
                if self.endpoints[ep_num].rx_len().is_some() {
                    self.usbfs.ep_ctrl_h(ep_num).modify(|_, w| w.r_res().nak());
                } else {
                    self.usbfs.ep_ctrl_h(ep_num).modify(|_, w| w.r_res().ack());
                }
            }
            (false, UsbDirection::In) => {
                self.usbfs.ep_ctrl_h(ep_num).modify(|_, w| w.t_res().nak());
            }
        }
    }

    fn is_stalled(&self, ep_addr: usb_device::endpoint::EndpointAddress) -> bool {
        match ep_addr.direction() {
            UsbDirection::Out => self
                .usbfs
                .ep_ctrl_h(ep_addr.index())
                .read()
                .r_res()
                .is_stall(),
            UsbDirection::In => self
                .usbfs
                .ep_ctrl_h(ep_addr.index())
                .read()
                .t_res()
                .is_stall(),
        }
    }

    fn set_device_address(&mut self, addr: u8) {
        self.usbfs.dev_addr.write(|w| w.usb_addr().variant(addr));

        self.usbfs
            .ep0_ctrl_h
            .modify(|_, w| w.r_tog().set_bit().t_tog().set_bit());
    }
}

pub struct UsbBus<const BUF_LEN: usize> {
    // We use FairMutex here as it is backed by an AtomicUsize, rather than an
    // AtomicBool which is implemented more effeciently on riscv targets.
    usb: SpinMutex<Driver<BUF_LEN>>,
}

impl<const BUF_LEN: usize> UsbBus<BUF_LEN> {
    pub fn new(
        usbfs: device::USBFS,
        buffer: &'static crate::buffer::EndpointMemory<BUF_LEN>,
    ) -> Self {
        Self {
            usb: SpinMutex::new(Driver::new(usbfs, buffer)),
        }
    }

    fn get_usb_mut(&mut self) -> &mut Driver<BUF_LEN> {
        self.usb.get_mut()
    }

    fn with_usb_mut<R>(&self, func: impl FnOnce(&mut Driver<BUF_LEN>) -> R) -> R {
        let mut usb = self.usb.lock();
        func(&mut usb)
    }
}

impl<const BUF_LEN: usize> usb_device::bus::UsbBus for UsbBus<BUF_LEN> {
    fn alloc_ep(
        &mut self,
        ep_dir: usb_device::UsbDirection,
        ep_addr: Option<usb_device::endpoint::EndpointAddress>,
        ep_type: usb_device::endpoint::EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> usb_device::Result<usb_device::endpoint::EndpointAddress> {
        let ret = self
            .get_usb_mut()
            .alloc_ep(ep_dir, ep_addr, ep_type, max_packet_size, interval);
        ret
    }

    fn enable(&mut self) {
        self.get_usb_mut().enable();
    }

    fn reset(&self) {
        self.with_usb_mut(|usb| {
            usb.set_device_address(0);
            usb.ep_reset_all();
        });
    }

    fn set_device_address(&self, addr: u8) {
        self.with_usb_mut(|usb| usb.set_device_address(addr))
    }

    fn write(
        &self,
        ep_addr: usb_device::endpoint::EndpointAddress,
        buf: &[u8],
    ) -> usb_device::Result<usize> {
        self.with_usb_mut(|usb| usb.write(ep_addr, buf))
    }

    fn read(
        &self,
        ep_addr: usb_device::endpoint::EndpointAddress,
        buf: &mut [u8],
    ) -> usb_device::Result<usize> {
        self.with_usb_mut(|usb| usb.read(ep_addr, buf))
    }

    fn set_stalled(&self, ep_addr: usb_device::endpoint::EndpointAddress, stalled: bool) {
        self.with_usb_mut(|usb| usb.set_stalled(ep_addr, stalled));
    }

    fn is_stalled(&self, ep_addr: usb_device::endpoint::EndpointAddress) -> bool {
        self.with_usb_mut(|usb| usb.is_stalled(ep_addr))
    }

    fn suspend(&self) {
        // TODO
    }

    fn resume(&self) {
        // TODO
    }

    fn poll(&self) -> usb_device::bus::PollResult {
        self.with_usb_mut(|usb| usb.poll())
    }
}
