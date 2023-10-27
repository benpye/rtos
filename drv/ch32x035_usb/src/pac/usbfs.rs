#[doc = "USB device registers."]
#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB control register."]
    pub base_ctrl: BASE_CTRL,
    #[doc = "0x01 - USB device physical port control."]
    pub dev_ctrl: DEV_CTRL,
    #[doc = "0x02 - USB interrupt enable register."]
    pub int_en: INT_EN,
    #[doc = "0x03 - USB device address register."]
    pub dev_addr: DEV_ADDR,
    _reserved4: [u8; 0x01],
    #[doc = "0x05 - USB miscellaneous status register."]
    pub mis_st: MIS_ST,
    #[doc = "0x06 - USB interrupt flag register."]
    pub int_fg: INT_FG,
    #[doc = "0x07 - USB interrupt status register."]
    pub int_st: INT_ST,
    #[doc = "0x08 - USB receive length register."]
    pub rx_len: RX_LEN,
    _reserved8: [u8; 0x02],
    #[doc = "0x0c - Endpoint 1 and 4 mode control."]
    pub ep4_1_mod: EP4_1_MOD,
    #[doc = "0x0d - Endpoint 2 and 3 mode control."]
    pub ep2_3_mod: EP2_3_MOD,
    #[doc = "0x0e - Endpoint 5, 6 and 7 mode control."]
    pub ep567_mod: EP567_MOD,
    _reserved11: [u8; 0x01],
    #[doc = "0x10 - Start address of the endpoint buffer."]
    pub ep0_dma: EP_DMA,
    #[doc = "0x14 - Start address of the endpoint buffer."]
    pub ep1_dma: EP_DMA,
    #[doc = "0x18 - Start address of the endpoint buffer."]
    pub ep2_dma: EP_DMA,
    #[doc = "0x1c - Start address of the endpoint buffer."]
    pub ep3_dma: EP_DMA,
    #[doc = "0x20 - Endpoint transmit length."]
    pub ep0_tx_len: EP_TX_LEN,
    #[doc = "0x22 - Endpoint control register."]
    pub ep0_ctrl_h: EP_CTRL_H,
    #[doc = "0x24 - Endpoint transmit length."]
    pub ep1_tx_len: EP_TX_LEN,
    #[doc = "0x26 - Endpoint control register."]
    pub ep1_ctrl_h: EP_CTRL_H,
    #[doc = "0x28 - Endpoint transmit length."]
    pub ep2_tx_len: EP_TX_LEN,
    #[doc = "0x2a - Endpoint control register."]
    pub ep2_ctrl_h: EP_CTRL_H,
    #[doc = "0x2c - Endpoint transmit length."]
    pub ep3_tx_len: EP_TX_LEN,
    #[doc = "0x2e - Endpoint control register."]
    pub ep3_ctrl_h: EP_CTRL_H,
    #[doc = "0x30 - Endpoint transmit length."]
    pub ep4_tx_len: EP_TX_LEN,
    #[doc = "0x32 - Endpoint control register."]
    pub ep4_ctrl_h: EP_CTRL_H,
    _reserved25: [u8; 0x20],
    #[doc = "0x54 - Start address of the endpoint buffer."]
    pub ep5_dma: EP5_DMA,
    #[doc = "0x58 - Start address of the endpoint buffer."]
    pub ep6_dma: EP5_DMA,
    #[doc = "0x5c - Start address of the endpoint buffer."]
    pub ep7_dma: EP5_DMA,
    _reserved28: [u8; 0x04],
    #[doc = "0x64 - Endpoint transmit length."]
    pub ep5_tx_len: EP5_TX_LEN,
    #[doc = "0x66 - Endpoint control register."]
    pub ep5_ctrl_h: EP5_CTRL_H,
    #[doc = "0x68 - Endpoint transmit length."]
    pub ep6_tx_len: EP5_TX_LEN,
    #[doc = "0x6a - Endpoint control register."]
    pub ep6_ctrl_h: EP5_CTRL_H,
    #[doc = "0x6c - Endpoint transmit length."]
    pub ep7_tx_len: EP5_TX_LEN,
    #[doc = "0x6e - Endpoint control register."]
    pub ep7_ctrl_h: EP5_CTRL_H,
    #[doc = "0x70 - Endpoint X control register."]
    pub epx_ctrl: EPX_CTRL,
}
#[doc = "BASE_CTRL (rw) register accessor: USB control register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`base_ctrl::R`].  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`base_ctrl::W`]. You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`base_ctrl`]
module"]
pub type BASE_CTRL = crate::pac::generic::Reg<base_ctrl::BASE_CTRL_SPEC>;
#[doc = "USB control register."]
pub mod base_ctrl {
    #[doc = "Register `BASE_CTRL` reader"]
    pub type R = crate::pac::generic::R<BASE_CTRL_SPEC>;
    #[doc = "Register `BASE_CTRL` writer"]
    pub type W = crate::pac::generic::W<BASE_CTRL_SPEC>;
    #[doc = "Field `DMA_EN` reader - Enables DMA for USB, this bit must be set to 1 in normal transfer mode."]
    pub type DMA_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `DMA_EN` writer - Enables DMA for USB, this bit must be set to 1 in normal transfer mode."]
    pub type DMA_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `CLR_ALL` reader - USB FIFO and interrupt flag clear."]
    pub type CLR_ALL_R = crate::pac::generic::BitReader;
    #[doc = "Field `CLR_ALL` writer - USB FIFO and interrupt flag clear."]
    pub type CLR_ALL_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `RST_SIE` reader - USB protocol processor software reset control."]
    pub type RST_SIE_R = crate::pac::generic::BitReader;
    #[doc = "Field `RST_SIE` writer - USB protocol processor software reset control."]
    pub type RST_SIE_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `INT_BUSY` reader - USB transfer completion interrupt flag not cleared to zero auto pause enable bit."]
    pub type INT_BUSY_R = crate::pac::generic::BitReader;
    #[doc = "Field `INT_BUSY` writer - USB transfer completion interrupt flag not cleared to zero auto pause enable bit."]
    pub type INT_BUSY_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `SYS_MODE` reader - SYS_MODE"]
    pub type SYS_MODE_R = crate::pac::generic::FieldReader;
    #[doc = "Field `SYS_MODE` writer - SYS_MODE"]
    pub type SYS_MODE_W<'a, REG, const O: u8> = crate::pac::generic::FieldWriter<'a, REG, 2, O>;
    #[doc = "Field `LOW_SPEED` reader - USB low speed enable bit."]
    pub type LOW_SPEED_R = crate::pac::generic::BitReader;
    #[doc = "Field `LOW_SPEED` writer - USB low speed enable bit."]
    pub type LOW_SPEED_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `HOST_MODE` reader - USB operating mode selection bits."]
    pub type HOST_MODE_R = crate::pac::generic::BitReader;
    #[doc = "Field `HOST_MODE` writer - USB operating mode selection bits."]
    pub type HOST_MODE_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    impl R {
        #[doc = "Bit 0 - Enables DMA for USB, this bit must be set to 1 in normal transfer mode."]
        #[inline(always)]
        pub fn dma_en(&self) -> DMA_EN_R {
            DMA_EN_R::new((self.bits & 1) != 0)
        }
        #[doc = "Bit 1 - USB FIFO and interrupt flag clear."]
        #[inline(always)]
        pub fn clr_all(&self) -> CLR_ALL_R {
            CLR_ALL_R::new(((self.bits >> 1) & 1) != 0)
        }
        #[doc = "Bit 2 - USB protocol processor software reset control."]
        #[inline(always)]
        pub fn rst_sie(&self) -> RST_SIE_R {
            RST_SIE_R::new(((self.bits >> 2) & 1) != 0)
        }
        #[doc = "Bit 3 - USB transfer completion interrupt flag not cleared to zero auto pause enable bit."]
        #[inline(always)]
        pub fn int_busy(&self) -> INT_BUSY_R {
            INT_BUSY_R::new(((self.bits >> 3) & 1) != 0)
        }
        #[doc = "Bits 4:5 - SYS_MODE"]
        #[inline(always)]
        pub fn sys_mode(&self) -> SYS_MODE_R {
            SYS_MODE_R::new((self.bits >> 4) & 3)
        }
        #[doc = "Bit 6 - USB low speed enable bit."]
        #[inline(always)]
        pub fn low_speed(&self) -> LOW_SPEED_R {
            LOW_SPEED_R::new(((self.bits >> 6) & 1) != 0)
        }
        #[doc = "Bit 7 - USB operating mode selection bits."]
        #[inline(always)]
        pub fn host_mode(&self) -> HOST_MODE_R {
            HOST_MODE_R::new(((self.bits >> 7) & 1) != 0)
        }
    }
    impl W {
        #[doc = "Bit 0 - Enables DMA for USB, this bit must be set to 1 in normal transfer mode."]
        #[inline(always)]
        #[must_use]
        pub fn dma_en(&mut self) -> DMA_EN_W<BASE_CTRL_SPEC, 0> {
            DMA_EN_W::new(self)
        }
        #[doc = "Bit 1 - USB FIFO and interrupt flag clear."]
        #[inline(always)]
        #[must_use]
        pub fn clr_all(&mut self) -> CLR_ALL_W<BASE_CTRL_SPEC, 1> {
            CLR_ALL_W::new(self)
        }
        #[doc = "Bit 2 - USB protocol processor software reset control."]
        #[inline(always)]
        #[must_use]
        pub fn rst_sie(&mut self) -> RST_SIE_W<BASE_CTRL_SPEC, 2> {
            RST_SIE_W::new(self)
        }
        #[doc = "Bit 3 - USB transfer completion interrupt flag not cleared to zero auto pause enable bit."]
        #[inline(always)]
        #[must_use]
        pub fn int_busy(&mut self) -> INT_BUSY_W<BASE_CTRL_SPEC, 3> {
            INT_BUSY_W::new(self)
        }
        #[doc = "Bits 4:5 - SYS_MODE"]
        #[inline(always)]
        #[must_use]
        pub fn sys_mode(&mut self) -> SYS_MODE_W<BASE_CTRL_SPEC, 4> {
            SYS_MODE_W::new(self)
        }
        #[doc = "Bit 6 - USB low speed enable bit."]
        #[inline(always)]
        #[must_use]
        pub fn low_speed(&mut self) -> LOW_SPEED_W<BASE_CTRL_SPEC, 6> {
            LOW_SPEED_W::new(self)
        }
        #[doc = "Bit 7 - USB operating mode selection bits."]
        #[inline(always)]
        #[must_use]
        pub fn host_mode(&mut self) -> HOST_MODE_W<BASE_CTRL_SPEC, 7> {
            HOST_MODE_W::new(self)
        }
        #[doc = r" Writes raw bits to the register."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
    }
    #[doc = "USB control register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`base_ctrl::R`](R).  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`base_ctrl::W`](W). You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct BASE_CTRL_SPEC;
    impl crate::pac::generic::RegisterSpec for BASE_CTRL_SPEC {
        type Ux = u8;
    }
    #[doc = "`read()` method returns [`base_ctrl::R`](R) reader structure"]
    impl crate::pac::generic::Readable for BASE_CTRL_SPEC {}
    #[doc = "`write(|w| ..)` method takes [`base_ctrl::W`](W) writer structure"]
    impl crate::pac::generic::Writable for BASE_CTRL_SPEC {
        const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
        const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    }
    #[doc = "`reset()` method sets BASE_CTRL to value 0x06"]
    impl crate::pac::generic::Resettable for BASE_CTRL_SPEC {
        const RESET_VALUE: Self::Ux = 0x06;
    }
}
#[doc = "DEV_CTRL (rw) register accessor: USB device physical port control.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`dev_ctrl::R`].  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`dev_ctrl::W`]. You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dev_ctrl`]
module"]
pub type DEV_CTRL = crate::pac::generic::Reg<dev_ctrl::DEV_CTRL_SPEC>;
#[doc = "USB device physical port control."]
pub mod dev_ctrl {
    #[doc = "Register `DEV_CTRL` reader"]
    pub type R = crate::pac::generic::R<DEV_CTRL_SPEC>;
    #[doc = "Register `DEV_CTRL` writer"]
    pub type W = crate::pac::generic::W<DEV_CTRL_SPEC>;
    #[doc = "Field `PORT_EN` reader - USB device physical port enable."]
    pub type PORT_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `PORT_EN` writer - USB device physical port enable."]
    pub type PORT_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `GP_FLAG` reader - General flag bit, user-defined."]
    pub type GP_FLAG_R = crate::pac::generic::BitReader;
    #[doc = "Field `GP_FLAG` writer - General flag bit, user-defined."]
    pub type GP_FLAG_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `LOW_SPEED` reader - USB device port low speed enable."]
    pub type LOW_SPEED_R = crate::pac::generic::BitReader;
    #[doc = "Field `LOW_SPEED` writer - USB device port low speed enable."]
    pub type LOW_SPEED_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `DM_PIN` reader - Current UDM pin state."]
    pub type DM_PIN_R = crate::pac::generic::BitReader;
    #[doc = "Field `DP_PIN` reader - Current UDP pin state."]
    pub type DP_PIN_R = crate::pac::generic::BitReader;
    #[doc = "Field `PD_DIS` reader - USB device port UDP/UDM pulldown resistor disable."]
    pub type PD_DIS_R = crate::pac::generic::BitReader;
    #[doc = "Field `PD_DIS` writer - USB device port UDP/UDM pulldown resistor disable."]
    pub type PD_DIS_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    impl R {
        #[doc = "Bit 0 - USB device physical port enable."]
        #[inline(always)]
        pub fn port_en(&self) -> PORT_EN_R {
            PORT_EN_R::new((self.bits & 1) != 0)
        }
        #[doc = "Bit 1 - General flag bit, user-defined."]
        #[inline(always)]
        pub fn gp_flag(&self) -> GP_FLAG_R {
            GP_FLAG_R::new(((self.bits >> 1) & 1) != 0)
        }
        #[doc = "Bit 2 - USB device port low speed enable."]
        #[inline(always)]
        pub fn low_speed(&self) -> LOW_SPEED_R {
            LOW_SPEED_R::new(((self.bits >> 2) & 1) != 0)
        }
        #[doc = "Bit 4 - Current UDM pin state."]
        #[inline(always)]
        pub fn dm_pin(&self) -> DM_PIN_R {
            DM_PIN_R::new(((self.bits >> 4) & 1) != 0)
        }
        #[doc = "Bit 5 - Current UDP pin state."]
        #[inline(always)]
        pub fn dp_pin(&self) -> DP_PIN_R {
            DP_PIN_R::new(((self.bits >> 5) & 1) != 0)
        }
        #[doc = "Bit 7 - USB device port UDP/UDM pulldown resistor disable."]
        #[inline(always)]
        pub fn pd_dis(&self) -> PD_DIS_R {
            PD_DIS_R::new(((self.bits >> 7) & 1) != 0)
        }
    }
    impl W {
        #[doc = "Bit 0 - USB device physical port enable."]
        #[inline(always)]
        #[must_use]
        pub fn port_en(&mut self) -> PORT_EN_W<DEV_CTRL_SPEC, 0> {
            PORT_EN_W::new(self)
        }
        #[doc = "Bit 1 - General flag bit, user-defined."]
        #[inline(always)]
        #[must_use]
        pub fn gp_flag(&mut self) -> GP_FLAG_W<DEV_CTRL_SPEC, 1> {
            GP_FLAG_W::new(self)
        }
        #[doc = "Bit 2 - USB device port low speed enable."]
        #[inline(always)]
        #[must_use]
        pub fn low_speed(&mut self) -> LOW_SPEED_W<DEV_CTRL_SPEC, 2> {
            LOW_SPEED_W::new(self)
        }
        #[doc = "Bit 7 - USB device port UDP/UDM pulldown resistor disable."]
        #[inline(always)]
        #[must_use]
        pub fn pd_dis(&mut self) -> PD_DIS_W<DEV_CTRL_SPEC, 7> {
            PD_DIS_W::new(self)
        }
        #[doc = r" Writes raw bits to the register."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
    }
    #[doc = "USB device physical port control.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`dev_ctrl::R`](R).  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`dev_ctrl::W`](W). You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct DEV_CTRL_SPEC;
    impl crate::pac::generic::RegisterSpec for DEV_CTRL_SPEC {
        type Ux = u8;
    }
    #[doc = "`read()` method returns [`dev_ctrl::R`](R) reader structure"]
    impl crate::pac::generic::Readable for DEV_CTRL_SPEC {}
    #[doc = "`write(|w| ..)` method takes [`dev_ctrl::W`](W) writer structure"]
    impl crate::pac::generic::Writable for DEV_CTRL_SPEC {
        const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
        const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    }
    #[doc = "`reset()` method sets DEV_CTRL to value 0"]
    impl crate::pac::generic::Resettable for DEV_CTRL_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
#[doc = "INT_EN (rw) register accessor: USB interrupt enable register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_en`]
module"]
pub type INT_EN = crate::pac::generic::Reg<int_en::INT_EN_SPEC>;
#[doc = "USB interrupt enable register."]
pub mod int_en {
    #[doc = "Register `INT_EN` reader"]
    pub type R = crate::pac::generic::R<INT_EN_SPEC>;
    #[doc = "Register `INT_EN` writer"]
    pub type W = crate::pac::generic::W<INT_EN_SPEC>;
    #[doc = "Field `BUS_RST` reader - USB bus reset event interrupt."]
    pub type BUS_RST_R = crate::pac::generic::BitReader;
    #[doc = "Field `BUS_RST` writer - USB bus reset event interrupt."]
    pub type BUS_RST_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `TRANSFER` reader - USB transfer (excluding SETUP transaction) completion interrupt."]
    pub type TRANSFER_R = crate::pac::generic::BitReader;
    #[doc = "Field `TRANSFER` writer - USB transfer (excluding SETUP transaction) completion interrupt."]
    pub type TRANSFER_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `SUSPEND` reader - USB bus suspend or wakeup event interrupt."]
    pub type SUSPEND_R = crate::pac::generic::BitReader;
    #[doc = "Field `SUSPEND` writer - USB bus suspend or wakeup event interrupt."]
    pub type SUSPEND_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `SOF_ACT` reader - USB host mode, SOF receive completion interrupt."]
    pub type SOF_ACT_R = crate::pac::generic::BitReader;
    #[doc = "Field `SOF_ACT` writer - USB host mode, SOF receive completion interrupt."]
    pub type SOF_ACT_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `FIFO_OV` reader - FIFO overflow interrupt."]
    pub type FIFO_OV_R = crate::pac::generic::BitReader;
    #[doc = "Field `FIFO_OV` writer - FIFO overflow interrupt."]
    pub type FIFO_OV_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `DEV_NAK` reader - USB device mode, receive NAK interrupt."]
    pub type DEV_NAK_R = crate::pac::generic::BitReader;
    #[doc = "Field `DEV_NAK` writer - USB device mode, receive NAK interrupt."]
    pub type DEV_NAK_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `DEV_SOF` reader - USB device mode, receive SOF interrupt."]
    pub type DEV_SOF_R = crate::pac::generic::BitReader;
    #[doc = "Field `DEV_SOF` writer - USB device mode, receive SOF interrupt."]
    pub type DEV_SOF_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    impl R {
        #[doc = "Bit 0 - USB bus reset event interrupt."]
        #[inline(always)]
        pub fn bus_rst(&self) -> BUS_RST_R {
            BUS_RST_R::new((self.bits & 1) != 0)
        }
        #[doc = "Bit 1 - USB transfer (excluding SETUP transaction) completion interrupt."]
        #[inline(always)]
        pub fn transfer(&self) -> TRANSFER_R {
            TRANSFER_R::new(((self.bits >> 1) & 1) != 0)
        }
        #[doc = "Bit 2 - USB bus suspend or wakeup event interrupt."]
        #[inline(always)]
        pub fn suspend(&self) -> SUSPEND_R {
            SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
        }
        #[doc = "Bit 3 - USB host mode, SOF receive completion interrupt."]
        #[inline(always)]
        pub fn sof_act(&self) -> SOF_ACT_R {
            SOF_ACT_R::new(((self.bits >> 3) & 1) != 0)
        }
        #[doc = "Bit 4 - FIFO overflow interrupt."]
        #[inline(always)]
        pub fn fifo_ov(&self) -> FIFO_OV_R {
            FIFO_OV_R::new(((self.bits >> 4) & 1) != 0)
        }
        #[doc = "Bit 6 - USB device mode, receive NAK interrupt."]
        #[inline(always)]
        pub fn dev_nak(&self) -> DEV_NAK_R {
            DEV_NAK_R::new(((self.bits >> 6) & 1) != 0)
        }
        #[doc = "Bit 7 - USB device mode, receive SOF interrupt."]
        #[inline(always)]
        pub fn dev_sof(&self) -> DEV_SOF_R {
            DEV_SOF_R::new(((self.bits >> 7) & 1) != 0)
        }
    }
    impl W {
        #[doc = "Bit 0 - USB bus reset event interrupt."]
        #[inline(always)]
        #[must_use]
        pub fn bus_rst(&mut self) -> BUS_RST_W<INT_EN_SPEC, 0> {
            BUS_RST_W::new(self)
        }
        #[doc = "Bit 1 - USB transfer (excluding SETUP transaction) completion interrupt."]
        #[inline(always)]
        #[must_use]
        pub fn transfer(&mut self) -> TRANSFER_W<INT_EN_SPEC, 1> {
            TRANSFER_W::new(self)
        }
        #[doc = "Bit 2 - USB bus suspend or wakeup event interrupt."]
        #[inline(always)]
        #[must_use]
        pub fn suspend(&mut self) -> SUSPEND_W<INT_EN_SPEC, 2> {
            SUSPEND_W::new(self)
        }
        #[doc = "Bit 3 - USB host mode, SOF receive completion interrupt."]
        #[inline(always)]
        #[must_use]
        pub fn sof_act(&mut self) -> SOF_ACT_W<INT_EN_SPEC, 3> {
            SOF_ACT_W::new(self)
        }
        #[doc = "Bit 4 - FIFO overflow interrupt."]
        #[inline(always)]
        #[must_use]
        pub fn fifo_ov(&mut self) -> FIFO_OV_W<INT_EN_SPEC, 4> {
            FIFO_OV_W::new(self)
        }
        #[doc = "Bit 6 - USB device mode, receive NAK interrupt."]
        #[inline(always)]
        #[must_use]
        pub fn dev_nak(&mut self) -> DEV_NAK_W<INT_EN_SPEC, 6> {
            DEV_NAK_W::new(self)
        }
        #[doc = "Bit 7 - USB device mode, receive SOF interrupt."]
        #[inline(always)]
        #[must_use]
        pub fn dev_sof(&mut self) -> DEV_SOF_W<INT_EN_SPEC, 7> {
            DEV_SOF_W::new(self)
        }
        #[doc = r" Writes raw bits to the register."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
    }
    #[doc = "USB interrupt enable register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`int_en::R`](R).  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct INT_EN_SPEC;
    impl crate::pac::generic::RegisterSpec for INT_EN_SPEC {
        type Ux = u8;
    }
    #[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
    impl crate::pac::generic::Readable for INT_EN_SPEC {}
    #[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
    impl crate::pac::generic::Writable for INT_EN_SPEC {
        const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
        const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    }
    #[doc = "`reset()` method sets INT_EN to value 0"]
    impl crate::pac::generic::Resettable for INT_EN_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
#[doc = "DEV_ADDR (rw) register accessor: USB device address register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`dev_addr::R`].  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`dev_addr::W`]. You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dev_addr`]
module"]
pub type DEV_ADDR = crate::pac::generic::Reg<dev_addr::DEV_ADDR_SPEC>;
#[doc = "USB device address register."]
pub mod dev_addr {
    #[doc = "Register `DEV_ADDR` reader"]
    pub type R = crate::pac::generic::R<DEV_ADDR_SPEC>;
    #[doc = "Register `DEV_ADDR` writer"]
    pub type W = crate::pac::generic::W<DEV_ADDR_SPEC>;
    #[doc = "Field `USB_ADDR` reader - USB device address."]
    pub type USB_ADDR_R = crate::pac::generic::FieldReader;
    #[doc = "Field `USB_ADDR` writer - USB device address."]
    pub type USB_ADDR_W<'a, REG, const O: u8> = crate::pac::generic::FieldWriter<'a, REG, 7, O>;
    impl R {
        #[doc = "Bits 0:6 - USB device address."]
        #[inline(always)]
        pub fn usb_addr(&self) -> USB_ADDR_R {
            USB_ADDR_R::new(self.bits & 0x7f)
        }
    }
    impl W {
        #[doc = "Bits 0:6 - USB device address."]
        #[inline(always)]
        #[must_use]
        pub fn usb_addr(&mut self) -> USB_ADDR_W<DEV_ADDR_SPEC, 0> {
            USB_ADDR_W::new(self)
        }
        #[doc = r" Writes raw bits to the register."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
    }
    #[doc = "USB device address register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`dev_addr::R`](R).  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`dev_addr::W`](W). You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct DEV_ADDR_SPEC;
    impl crate::pac::generic::RegisterSpec for DEV_ADDR_SPEC {
        type Ux = u8;
    }
    #[doc = "`read()` method returns [`dev_addr::R`](R) reader structure"]
    impl crate::pac::generic::Readable for DEV_ADDR_SPEC {}
    #[doc = "`write(|w| ..)` method takes [`dev_addr::W`](W) writer structure"]
    impl crate::pac::generic::Writable for DEV_ADDR_SPEC {
        const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
        const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    }
    #[doc = "`reset()` method sets DEV_ADDR to value 0"]
    impl crate::pac::generic::Resettable for DEV_ADDR_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
#[doc = "MIS_ST (r) register accessor: USB miscellaneous status register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`mis_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mis_st`]
module"]
pub type MIS_ST = crate::pac::generic::Reg<mis_st::MIS_ST_SPEC>;
#[doc = "USB miscellaneous status register."]
pub mod mis_st {
    #[doc = "Register `MIS_ST` reader"]
    pub type R = crate::pac::generic::R<MIS_ST_SPEC>;
    #[doc = "Field `DEV_ATTACH` reader - USB device attach status for the port in USB host mode."]
    pub type DEV_ATTACH_R = crate::pac::generic::BitReader;
    #[doc = "Field `DM_LEVEL` reader - DM_LEVEL"]
    pub type DM_LEVEL_R = crate::pac::generic::BitReader;
    #[doc = "Field `SUSPEND` reader - USB suspend."]
    pub type SUSPEND_R = crate::pac::generic::BitReader;
    #[doc = "Field `BUS_RST` reader - USB bus reset."]
    pub type BUS_RST_R = crate::pac::generic::BitReader;
    #[doc = "Field `R_FIFO_RDY` reader - USB receive FIFO data ready."]
    pub type R_FIFO_RDY_R = crate::pac::generic::BitReader;
    #[doc = "Field `SIE_FREE` reader - USB protocol handler free."]
    pub type SIE_FREE_R = crate::pac::generic::BitReader;
    #[doc = "Field `SOF_ACT` reader - SOF packet transfer status in USB host mode."]
    pub type SOF_ACT_R = crate::pac::generic::BitReader;
    #[doc = "Field `SOF_PRES` reader - SOF packet presage status in USB host mode."]
    pub type SOF_PRES_R = crate::pac::generic::BitReader;
    impl R {
        #[doc = "Bit 0 - USB device attach status for the port in USB host mode."]
        #[inline(always)]
        pub fn dev_attach(&self) -> DEV_ATTACH_R {
            DEV_ATTACH_R::new((self.bits & 1) != 0)
        }
        #[doc = "Bit 1 - DM_LEVEL"]
        #[inline(always)]
        pub fn dm_level(&self) -> DM_LEVEL_R {
            DM_LEVEL_R::new(((self.bits >> 1) & 1) != 0)
        }
        #[doc = "Bit 2 - USB suspend."]
        #[inline(always)]
        pub fn suspend(&self) -> SUSPEND_R {
            SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
        }
        #[doc = "Bit 3 - USB bus reset."]
        #[inline(always)]
        pub fn bus_rst(&self) -> BUS_RST_R {
            BUS_RST_R::new(((self.bits >> 3) & 1) != 0)
        }
        #[doc = "Bit 4 - USB receive FIFO data ready."]
        #[inline(always)]
        pub fn r_fifo_rdy(&self) -> R_FIFO_RDY_R {
            R_FIFO_RDY_R::new(((self.bits >> 4) & 1) != 0)
        }
        #[doc = "Bit 5 - USB protocol handler free."]
        #[inline(always)]
        pub fn sie_free(&self) -> SIE_FREE_R {
            SIE_FREE_R::new(((self.bits >> 5) & 1) != 0)
        }
        #[doc = "Bit 6 - SOF packet transfer status in USB host mode."]
        #[inline(always)]
        pub fn sof_act(&self) -> SOF_ACT_R {
            SOF_ACT_R::new(((self.bits >> 6) & 1) != 0)
        }
        #[doc = "Bit 7 - SOF packet presage status in USB host mode."]
        #[inline(always)]
        pub fn sof_pres(&self) -> SOF_PRES_R {
            SOF_PRES_R::new(((self.bits >> 7) & 1) != 0)
        }
    }
    #[doc = "USB miscellaneous status register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`mis_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct MIS_ST_SPEC;
    impl crate::pac::generic::RegisterSpec for MIS_ST_SPEC {
        type Ux = u8;
    }
    #[doc = "`read()` method returns [`mis_st::R`](R) reader structure"]
    impl crate::pac::generic::Readable for MIS_ST_SPEC {}
    #[doc = "`reset()` method sets MIS_ST to value 0"]
    impl crate::pac::generic::Resettable for MIS_ST_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
#[doc = "INT_FG (rw) register accessor: USB interrupt flag register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`int_fg::R`].  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`int_fg::W`]. You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_fg`]
module"]
pub type INT_FG = crate::pac::generic::Reg<int_fg::INT_FG_SPEC>;
#[doc = "USB interrupt flag register."]
pub mod int_fg {
    #[doc = "Register `INT_FG` reader"]
    pub type R = crate::pac::generic::R<INT_FG_SPEC>;
    #[doc = "Register `INT_FG` writer"]
    pub type W = crate::pac::generic::W<INT_FG_SPEC>;
    #[doc = "Field `BUS_RST` reader - In USB device mode, USB bus reset event interrupt flag bit, write 1 to clear."]
    pub type BUS_RST_R = crate::pac::generic::BitReader;
    #[doc = "Field `BUS_RST` writer - In USB device mode, USB bus reset event interrupt flag bit, write 1 to clear."]
    pub type BUS_RST_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `TRANSFER` reader - USB transfer completion interrupt flag, write 1 to clear."]
    pub type TRANSFER_R = crate::pac::generic::BitReader;
    #[doc = "Field `TRANSFER` writer - USB transfer completion interrupt flag, write 1 to clear."]
    pub type TRANSFER_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `SUSPEND` reader - USB bus suspend or wake-up event interrupt flag, write 1 to clear."]
    pub type SUSPEND_R = crate::pac::generic::BitReader;
    #[doc = "Field `SUSPEND` writer - USB bus suspend or wake-up event interrupt flag, write 1 to clear."]
    pub type SUSPEND_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `HST_SOF` reader - SOF timer interrupt flag in USB host mode, write 1 to clear."]
    pub type HST_SOF_R = crate::pac::generic::BitReader;
    #[doc = "Field `HST_SOF` writer - SOF timer interrupt flag in USB host mode, write 1 to clear."]
    pub type HST_SOF_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `FIFO_OV` reader - USB FIFO overflow interrupt flag, write 1 to clear."]
    pub type FIFO_OV_R = crate::pac::generic::BitReader;
    #[doc = "Field `FIFO_OV` writer - USB FIFO overflow interrupt flag, write 1 to clear."]
    pub type FIFO_OV_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `SIE_FREE` reader - USB protocol handler free."]
    pub type SIE_FREE_R = crate::pac::generic::BitReader;
    #[doc = "Field `TOG_MATCH_SYNC` reader - Toggle bit of received packet matched expected."]
    pub type TOG_MATCH_SYNC_R = crate::pac::generic::BitReader;
    #[doc = "Field `IS_NAK` reader - NAK response interrupt flag bit in USB device mode."]
    pub type IS_NAK_R = crate::pac::generic::BitReader;
    impl R {
        #[doc = "Bit 0 - In USB device mode, USB bus reset event interrupt flag bit, write 1 to clear."]
        #[inline(always)]
        pub fn bus_rst(&self) -> BUS_RST_R {
            BUS_RST_R::new((self.bits & 1) != 0)
        }
        #[doc = "Bit 1 - USB transfer completion interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub fn transfer(&self) -> TRANSFER_R {
            TRANSFER_R::new(((self.bits >> 1) & 1) != 0)
        }
        #[doc = "Bit 2 - USB bus suspend or wake-up event interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub fn suspend(&self) -> SUSPEND_R {
            SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
        }
        #[doc = "Bit 3 - SOF timer interrupt flag in USB host mode, write 1 to clear."]
        #[inline(always)]
        pub fn hst_sof(&self) -> HST_SOF_R {
            HST_SOF_R::new(((self.bits >> 3) & 1) != 0)
        }
        #[doc = "Bit 4 - USB FIFO overflow interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub fn fifo_ov(&self) -> FIFO_OV_R {
            FIFO_OV_R::new(((self.bits >> 4) & 1) != 0)
        }
        #[doc = "Bit 5 - USB protocol handler free."]
        #[inline(always)]
        pub fn sie_free(&self) -> SIE_FREE_R {
            SIE_FREE_R::new(((self.bits >> 5) & 1) != 0)
        }
        #[doc = "Bit 6 - Toggle bit of received packet matched expected."]
        #[inline(always)]
        pub fn tog_match_sync(&self) -> TOG_MATCH_SYNC_R {
            TOG_MATCH_SYNC_R::new(((self.bits >> 6) & 1) != 0)
        }
        #[doc = "Bit 7 - NAK response interrupt flag bit in USB device mode."]
        #[inline(always)]
        pub fn is_nak(&self) -> IS_NAK_R {
            IS_NAK_R::new(((self.bits >> 7) & 1) != 0)
        }
    }
    impl W {
        #[doc = "Bit 0 - In USB device mode, USB bus reset event interrupt flag bit, write 1 to clear."]
        #[inline(always)]
        #[must_use]
        pub fn bus_rst(&mut self) -> BUS_RST_W<INT_FG_SPEC, 0> {
            BUS_RST_W::new(self)
        }
        #[doc = "Bit 1 - USB transfer completion interrupt flag, write 1 to clear."]
        #[inline(always)]
        #[must_use]
        pub fn transfer(&mut self) -> TRANSFER_W<INT_FG_SPEC, 1> {
            TRANSFER_W::new(self)
        }
        #[doc = "Bit 2 - USB bus suspend or wake-up event interrupt flag, write 1 to clear."]
        #[inline(always)]
        #[must_use]
        pub fn suspend(&mut self) -> SUSPEND_W<INT_FG_SPEC, 2> {
            SUSPEND_W::new(self)
        }
        #[doc = "Bit 3 - SOF timer interrupt flag in USB host mode, write 1 to clear."]
        #[inline(always)]
        #[must_use]
        pub fn hst_sof(&mut self) -> HST_SOF_W<INT_FG_SPEC, 3> {
            HST_SOF_W::new(self)
        }
        #[doc = "Bit 4 - USB FIFO overflow interrupt flag, write 1 to clear."]
        #[inline(always)]
        #[must_use]
        pub fn fifo_ov(&mut self) -> FIFO_OV_W<INT_FG_SPEC, 4> {
            FIFO_OV_W::new(self)
        }
        #[doc = r" Writes raw bits to the register."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
    }
    #[doc = "USB interrupt flag register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`int_fg::R`](R).  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`int_fg::W`](W). You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct INT_FG_SPEC;
    impl crate::pac::generic::RegisterSpec for INT_FG_SPEC {
        type Ux = u8;
    }
    #[doc = "`read()` method returns [`int_fg::R`](R) reader structure"]
    impl crate::pac::generic::Readable for INT_FG_SPEC {}
    #[doc = "`write(|w| ..)` method takes [`int_fg::W`](W) writer structure"]
    impl crate::pac::generic::Writable for INT_FG_SPEC {
        const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
        const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    }
    #[doc = "`reset()` method sets INT_FG to value 0x20"]
    impl crate::pac::generic::Resettable for INT_FG_SPEC {
        const RESET_VALUE: Self::Ux = 0x20;
    }
}
#[doc = "INT_ST (r) register accessor: USB interrupt status register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`]
module"]
pub type INT_ST = crate::pac::generic::Reg<int_st::INT_ST_SPEC>;
#[doc = "USB interrupt status register."]
pub mod int_st {
    #[doc = "Register `INT_ST` reader"]
    pub type R = crate::pac::generic::R<INT_ST_SPEC>;
    #[doc = "Field `ENDP` reader - In device mode, the endpoint number of the current USB transfer transaction."]
    pub type ENDP_R = crate::pac::generic::FieldReader;
    #[doc = "Field `TOKEN` reader - In device mode, the token PID identifier of XXb the current USB transfer transaction."]
    pub type TOKEN_R = crate::pac::generic::FieldReader<TOKEN_A>;
    #[doc = "In device mode, the token PID identifier of XXb the current USB transfer transaction.\n\nValue on reset: 0"]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    pub enum TOKEN_A {
        #[doc = "0: `0`"]
        Out = 0,
        #[doc = "1: `1`"]
        Sof = 1,
        #[doc = "2: `10`"]
        In = 2,
        #[doc = "3: `11`"]
        Setup = 3,
    }
    impl From<TOKEN_A> for u8 {
        #[inline(always)]
        fn from(variant: TOKEN_A) -> Self {
            variant as _
        }
    }
    impl crate::pac::generic::FieldSpec for TOKEN_A {
        type Ux = u8;
    }
    impl TOKEN_R {
        #[doc = "Get enumerated values variant"]
        #[inline(always)]
        pub fn variant(&self) -> TOKEN_A {
            match self.bits {
                0 => TOKEN_A::Out,
                1 => TOKEN_A::Sof,
                2 => TOKEN_A::In,
                3 => TOKEN_A::Setup,
                _ => unreachable!(),
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn is_out(&self) -> bool {
            *self == TOKEN_A::Out
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn is_sof(&self) -> bool {
            *self == TOKEN_A::Sof
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn is_in(&self) -> bool {
            *self == TOKEN_A::In
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn is_setup(&self) -> bool {
            *self == TOKEN_A::Setup
        }
    }
    #[doc = "Field `TOG_OK` reader - Toggle bit of received packet matched expected."]
    pub type TOG_OK_R = crate::pac::generic::BitReader;
    #[doc = "Field `SETUP_ACT` reader - SETUP transaction completed."]
    pub type SETUP_ACT_R = crate::pac::generic::BitReader;
    impl R {
        #[doc = "Bits 0:3 - In device mode, the endpoint number of the current USB transfer transaction."]
        #[inline(always)]
        pub fn endp(&self) -> ENDP_R {
            ENDP_R::new(self.bits & 0x0f)
        }
        #[doc = "Bits 4:5 - In device mode, the token PID identifier of XXb the current USB transfer transaction."]
        #[inline(always)]
        pub fn token(&self) -> TOKEN_R {
            TOKEN_R::new((self.bits >> 4) & 3)
        }
        #[doc = "Bit 6 - Toggle bit of received packet matched expected."]
        #[inline(always)]
        pub fn tog_ok(&self) -> TOG_OK_R {
            TOG_OK_R::new(((self.bits >> 6) & 1) != 0)
        }
        #[doc = "Bit 7 - SETUP transaction completed."]
        #[inline(always)]
        pub fn setup_act(&self) -> SETUP_ACT_R {
            SETUP_ACT_R::new(((self.bits >> 7) & 1) != 0)
        }
    }
    #[doc = "USB interrupt status register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct INT_ST_SPEC;
    impl crate::pac::generic::RegisterSpec for INT_ST_SPEC {
        type Ux = u8;
    }
    #[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
    impl crate::pac::generic::Readable for INT_ST_SPEC {}
    #[doc = "`reset()` method sets INT_ST to value 0"]
    impl crate::pac::generic::Resettable for INT_ST_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
#[doc = "RX_LEN (r) register accessor: USB receive length register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`rx_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_len`]
module"]
pub type RX_LEN = crate::pac::generic::Reg<rx_len::RX_LEN_SPEC>;
#[doc = "USB receive length register."]
pub mod rx_len {
    #[doc = "Register `RX_LEN` reader"]
    pub type R = crate::pac::generic::R<RX_LEN_SPEC>;
    #[doc = "Field `RX_LEN` reader - The current number of data bytes received by the USB endpoint."]
    pub type RX_LEN_R = crate::pac::generic::FieldReader;
    impl R {
        #[doc = "Bits 0:6 - The current number of data bytes received by the USB endpoint."]
        #[inline(always)]
        pub fn rx_len(&self) -> RX_LEN_R {
            RX_LEN_R::new((self.bits & 0x7f) as u8)
        }
    }
    #[doc = "USB receive length register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`rx_len::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct RX_LEN_SPEC;
    impl crate::pac::generic::RegisterSpec for RX_LEN_SPEC {
        type Ux = u16;
    }
    #[doc = "`read()` method returns [`rx_len::R`](R) reader structure"]
    impl crate::pac::generic::Readable for RX_LEN_SPEC {}
    #[doc = "`reset()` method sets RX_LEN to value 0"]
    impl crate::pac::generic::Resettable for RX_LEN_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
#[doc = "EP4_1_MOD (rw) register accessor: Endpoint 1 and 4 mode control.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`ep4_1_mod::R`].  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`ep4_1_mod::W`]. You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep4_1_mod`]
module"]
pub type EP4_1_MOD = crate::pac::generic::Reg<ep4_1_mod::EP4_1_MOD_SPEC>;
#[doc = "Endpoint 1 and 4 mode control."]
pub mod ep4_1_mod {
    #[doc = "Register `EP4_1_MOD` reader"]
    pub type R = crate::pac::generic::R<EP4_1_MOD_SPEC>;
    #[doc = "Register `EP4_1_MOD` writer"]
    pub type W = crate::pac::generic::W<EP4_1_MOD_SPEC>;
    #[doc = "Field `EP4_T_EN` reader - Endpoint 4 transmit enable."]
    pub type EP4_T_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP4_T_EN` writer - Endpoint 4 transmit enable."]
    pub type EP4_T_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP4_R_EN` reader - Endpoint 4 receive enable."]
    pub type EP4_R_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP4_R_EN` writer - Endpoint 4 receive enable."]
    pub type EP4_R_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP1_BUF_MOD` reader - Endpoint 1 buffer mode control."]
    pub type EP1_BUF_MOD_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP1_BUF_MOD` writer - Endpoint 1 buffer mode control."]
    pub type EP1_BUF_MOD_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP1_T_EN` reader - Endpoint 1 transmit enable."]
    pub type EP1_T_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP1_T_EN` writer - Endpoint 1 transmit enable."]
    pub type EP1_T_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP1_R_EN` reader - Endpoint 1 receive enable."]
    pub type EP1_R_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP1_R_EN` writer - Endpoint 1 receive enable."]
    pub type EP1_R_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    impl R {
        #[doc = "Bit 2 - Endpoint 4 transmit enable."]
        #[inline(always)]
        pub fn ep4_t_en(&self) -> EP4_T_EN_R {
            EP4_T_EN_R::new(((self.bits >> 2) & 1) != 0)
        }
        #[doc = "Bit 3 - Endpoint 4 receive enable."]
        #[inline(always)]
        pub fn ep4_r_en(&self) -> EP4_R_EN_R {
            EP4_R_EN_R::new(((self.bits >> 3) & 1) != 0)
        }
        #[doc = "Bit 4 - Endpoint 1 buffer mode control."]
        #[inline(always)]
        pub fn ep1_buf_mod(&self) -> EP1_BUF_MOD_R {
            EP1_BUF_MOD_R::new(((self.bits >> 4) & 1) != 0)
        }
        #[doc = "Bit 6 - Endpoint 1 transmit enable."]
        #[inline(always)]
        pub fn ep1_t_en(&self) -> EP1_T_EN_R {
            EP1_T_EN_R::new(((self.bits >> 6) & 1) != 0)
        }
        #[doc = "Bit 7 - Endpoint 1 receive enable."]
        #[inline(always)]
        pub fn ep1_r_en(&self) -> EP1_R_EN_R {
            EP1_R_EN_R::new(((self.bits >> 7) & 1) != 0)
        }
    }
    impl W {
        #[doc = "Bit 2 - Endpoint 4 transmit enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep4_t_en(&mut self) -> EP4_T_EN_W<EP4_1_MOD_SPEC, 2> {
            EP4_T_EN_W::new(self)
        }
        #[doc = "Bit 3 - Endpoint 4 receive enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep4_r_en(&mut self) -> EP4_R_EN_W<EP4_1_MOD_SPEC, 3> {
            EP4_R_EN_W::new(self)
        }
        #[doc = "Bit 4 - Endpoint 1 buffer mode control."]
        #[inline(always)]
        #[must_use]
        pub fn ep1_buf_mod(&mut self) -> EP1_BUF_MOD_W<EP4_1_MOD_SPEC, 4> {
            EP1_BUF_MOD_W::new(self)
        }
        #[doc = "Bit 6 - Endpoint 1 transmit enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep1_t_en(&mut self) -> EP1_T_EN_W<EP4_1_MOD_SPEC, 6> {
            EP1_T_EN_W::new(self)
        }
        #[doc = "Bit 7 - Endpoint 1 receive enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep1_r_en(&mut self) -> EP1_R_EN_W<EP4_1_MOD_SPEC, 7> {
            EP1_R_EN_W::new(self)
        }
        #[doc = r" Writes raw bits to the register."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
    }
    #[doc = "Endpoint 1 and 4 mode control.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`ep4_1_mod::R`](R).  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`ep4_1_mod::W`](W). You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct EP4_1_MOD_SPEC;
    impl crate::pac::generic::RegisterSpec for EP4_1_MOD_SPEC {
        type Ux = u8;
    }
    #[doc = "`read()` method returns [`ep4_1_mod::R`](R) reader structure"]
    impl crate::pac::generic::Readable for EP4_1_MOD_SPEC {}
    #[doc = "`write(|w| ..)` method takes [`ep4_1_mod::W`](W) writer structure"]
    impl crate::pac::generic::Writable for EP4_1_MOD_SPEC {
        const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
        const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    }
    #[doc = "`reset()` method sets EP4_1_MOD to value 0"]
    impl crate::pac::generic::Resettable for EP4_1_MOD_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
#[doc = "EP2_3_MOD (rw) register accessor: Endpoint 2 and 3 mode control.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`ep2_3_mod::R`].  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`ep2_3_mod::W`]. You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep2_3_mod`]
module"]
pub type EP2_3_MOD = crate::pac::generic::Reg<ep2_3_mod::EP2_3_MOD_SPEC>;
#[doc = "Endpoint 2 and 3 mode control."]
pub mod ep2_3_mod {
    #[doc = "Register `EP2_3_MOD` reader"]
    pub type R = crate::pac::generic::R<EP2_3_MOD_SPEC>;
    #[doc = "Register `EP2_3_MOD` writer"]
    pub type W = crate::pac::generic::W<EP2_3_MOD_SPEC>;
    #[doc = "Field `EP2_BUF_MOD` reader - Endpoint 2 buffer mode control."]
    pub type EP2_BUF_MOD_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP2_BUF_MOD` writer - Endpoint 2 buffer mode control."]
    pub type EP2_BUF_MOD_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP2_T_EN` reader - Endpoint 2 transmit enable."]
    pub type EP2_T_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP2_T_EN` writer - Endpoint 2 transmit enable."]
    pub type EP2_T_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP2_R_EN` reader - Endpoint 2 receive enable."]
    pub type EP2_R_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP2_R_EN` writer - Endpoint 2 receive enable."]
    pub type EP2_R_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP3_BUF_MOD` reader - Endpoint 3 buffer mode control."]
    pub type EP3_BUF_MOD_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP3_BUF_MOD` writer - Endpoint 3 buffer mode control."]
    pub type EP3_BUF_MOD_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP3_T_EN` reader - Endpoint 3 transmit enable."]
    pub type EP3_T_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP3_T_EN` writer - Endpoint 3 transmit enable."]
    pub type EP3_T_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP3_R_EN` reader - Endpoint 3 receive enable."]
    pub type EP3_R_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP3_R_EN` writer - Endpoint 3 receive enable."]
    pub type EP3_R_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    impl R {
        #[doc = "Bit 0 - Endpoint 2 buffer mode control."]
        #[inline(always)]
        pub fn ep2_buf_mod(&self) -> EP2_BUF_MOD_R {
            EP2_BUF_MOD_R::new((self.bits & 1) != 0)
        }
        #[doc = "Bit 2 - Endpoint 2 transmit enable."]
        #[inline(always)]
        pub fn ep2_t_en(&self) -> EP2_T_EN_R {
            EP2_T_EN_R::new(((self.bits >> 2) & 1) != 0)
        }
        #[doc = "Bit 3 - Endpoint 2 receive enable."]
        #[inline(always)]
        pub fn ep2_r_en(&self) -> EP2_R_EN_R {
            EP2_R_EN_R::new(((self.bits >> 3) & 1) != 0)
        }
        #[doc = "Bit 4 - Endpoint 3 buffer mode control."]
        #[inline(always)]
        pub fn ep3_buf_mod(&self) -> EP3_BUF_MOD_R {
            EP3_BUF_MOD_R::new(((self.bits >> 4) & 1) != 0)
        }
        #[doc = "Bit 6 - Endpoint 3 transmit enable."]
        #[inline(always)]
        pub fn ep3_t_en(&self) -> EP3_T_EN_R {
            EP3_T_EN_R::new(((self.bits >> 6) & 1) != 0)
        }
        #[doc = "Bit 7 - Endpoint 3 receive enable."]
        #[inline(always)]
        pub fn ep3_r_en(&self) -> EP3_R_EN_R {
            EP3_R_EN_R::new(((self.bits >> 7) & 1) != 0)
        }
    }
    impl W {
        #[doc = "Bit 0 - Endpoint 2 buffer mode control."]
        #[inline(always)]
        #[must_use]
        pub fn ep2_buf_mod(&mut self) -> EP2_BUF_MOD_W<EP2_3_MOD_SPEC, 0> {
            EP2_BUF_MOD_W::new(self)
        }
        #[doc = "Bit 2 - Endpoint 2 transmit enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep2_t_en(&mut self) -> EP2_T_EN_W<EP2_3_MOD_SPEC, 2> {
            EP2_T_EN_W::new(self)
        }
        #[doc = "Bit 3 - Endpoint 2 receive enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep2_r_en(&mut self) -> EP2_R_EN_W<EP2_3_MOD_SPEC, 3> {
            EP2_R_EN_W::new(self)
        }
        #[doc = "Bit 4 - Endpoint 3 buffer mode control."]
        #[inline(always)]
        #[must_use]
        pub fn ep3_buf_mod(&mut self) -> EP3_BUF_MOD_W<EP2_3_MOD_SPEC, 4> {
            EP3_BUF_MOD_W::new(self)
        }
        #[doc = "Bit 6 - Endpoint 3 transmit enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep3_t_en(&mut self) -> EP3_T_EN_W<EP2_3_MOD_SPEC, 6> {
            EP3_T_EN_W::new(self)
        }
        #[doc = "Bit 7 - Endpoint 3 receive enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep3_r_en(&mut self) -> EP3_R_EN_W<EP2_3_MOD_SPEC, 7> {
            EP3_R_EN_W::new(self)
        }
        #[doc = r" Writes raw bits to the register."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
    }
    #[doc = "Endpoint 2 and 3 mode control.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`ep2_3_mod::R`](R).  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`ep2_3_mod::W`](W). You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct EP2_3_MOD_SPEC;
    impl crate::pac::generic::RegisterSpec for EP2_3_MOD_SPEC {
        type Ux = u8;
    }
    #[doc = "`read()` method returns [`ep2_3_mod::R`](R) reader structure"]
    impl crate::pac::generic::Readable for EP2_3_MOD_SPEC {}
    #[doc = "`write(|w| ..)` method takes [`ep2_3_mod::W`](W) writer structure"]
    impl crate::pac::generic::Writable for EP2_3_MOD_SPEC {
        const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
        const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    }
    #[doc = "`reset()` method sets EP2_3_MOD to value 0"]
    impl crate::pac::generic::Resettable for EP2_3_MOD_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
#[doc = "EP567_MOD (rw) register accessor: Endpoint 5, 6 and 7 mode control.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`ep567_mod::R`].  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`ep567_mod::W`]. You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep567_mod`]
module"]
pub type EP567_MOD = crate::pac::generic::Reg<ep567_mod::EP567_MOD_SPEC>;
#[doc = "Endpoint 5, 6 and 7 mode control."]
pub mod ep567_mod {
    #[doc = "Register `EP567_MOD` reader"]
    pub type R = crate::pac::generic::R<EP567_MOD_SPEC>;
    #[doc = "Register `EP567_MOD` writer"]
    pub type W = crate::pac::generic::W<EP567_MOD_SPEC>;
    #[doc = "Field `EP5_T_EN` reader - Endpoint 5 transmit enable."]
    pub type EP5_T_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP5_T_EN` writer - Endpoint 5 transmit enable."]
    pub type EP5_T_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP5_R_EN` reader - Endpoint 5 receive enable."]
    pub type EP5_R_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP5_R_EN` writer - Endpoint 5 receive enable."]
    pub type EP5_R_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP6_T_EN` reader - Endpoint 6 transmit enable."]
    pub type EP6_T_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP6_T_EN` writer - Endpoint 6 transmit enable."]
    pub type EP6_T_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP6_R_EN` reader - Endpoint 6 receive enable."]
    pub type EP6_R_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP6_R_EN` writer - Endpoint 6 receive enable."]
    pub type EP6_R_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP7_T_EN` reader - Endpoint 7 transmit enable."]
    pub type EP7_T_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP7_T_EN` writer - Endpoint 7 transmit enable."]
    pub type EP7_T_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `EP7_R_EN` reader - Endpoint 7 receive enable."]
    pub type EP7_R_EN_R = crate::pac::generic::BitReader;
    #[doc = "Field `EP7_R_EN` writer - Endpoint 7 receive enable."]
    pub type EP7_R_EN_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    impl R {
        #[doc = "Bit 0 - Endpoint 5 transmit enable."]
        #[inline(always)]
        pub fn ep5_t_en(&self) -> EP5_T_EN_R {
            EP5_T_EN_R::new((self.bits & 1) != 0)
        }
        #[doc = "Bit 1 - Endpoint 5 receive enable."]
        #[inline(always)]
        pub fn ep5_r_en(&self) -> EP5_R_EN_R {
            EP5_R_EN_R::new(((self.bits >> 1) & 1) != 0)
        }
        #[doc = "Bit 2 - Endpoint 6 transmit enable."]
        #[inline(always)]
        pub fn ep6_t_en(&self) -> EP6_T_EN_R {
            EP6_T_EN_R::new(((self.bits >> 2) & 1) != 0)
        }
        #[doc = "Bit 3 - Endpoint 6 receive enable."]
        #[inline(always)]
        pub fn ep6_r_en(&self) -> EP6_R_EN_R {
            EP6_R_EN_R::new(((self.bits >> 3) & 1) != 0)
        }
        #[doc = "Bit 4 - Endpoint 7 transmit enable."]
        #[inline(always)]
        pub fn ep7_t_en(&self) -> EP7_T_EN_R {
            EP7_T_EN_R::new(((self.bits >> 4) & 1) != 0)
        }
        #[doc = "Bit 5 - Endpoint 7 receive enable."]
        #[inline(always)]
        pub fn ep7_r_en(&self) -> EP7_R_EN_R {
            EP7_R_EN_R::new(((self.bits >> 5) & 1) != 0)
        }
    }
    impl W {
        #[doc = "Bit 0 - Endpoint 5 transmit enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep5_t_en(&mut self) -> EP5_T_EN_W<EP567_MOD_SPEC, 0> {
            EP5_T_EN_W::new(self)
        }
        #[doc = "Bit 1 - Endpoint 5 receive enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep5_r_en(&mut self) -> EP5_R_EN_W<EP567_MOD_SPEC, 1> {
            EP5_R_EN_W::new(self)
        }
        #[doc = "Bit 2 - Endpoint 6 transmit enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep6_t_en(&mut self) -> EP6_T_EN_W<EP567_MOD_SPEC, 2> {
            EP6_T_EN_W::new(self)
        }
        #[doc = "Bit 3 - Endpoint 6 receive enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep6_r_en(&mut self) -> EP6_R_EN_W<EP567_MOD_SPEC, 3> {
            EP6_R_EN_W::new(self)
        }
        #[doc = "Bit 4 - Endpoint 7 transmit enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep7_t_en(&mut self) -> EP7_T_EN_W<EP567_MOD_SPEC, 4> {
            EP7_T_EN_W::new(self)
        }
        #[doc = "Bit 5 - Endpoint 7 receive enable."]
        #[inline(always)]
        #[must_use]
        pub fn ep7_r_en(&mut self) -> EP7_R_EN_W<EP567_MOD_SPEC, 5> {
            EP7_R_EN_W::new(self)
        }
        #[doc = r" Writes raw bits to the register."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
            self.bits = bits;
            self
        }
    }
    #[doc = "Endpoint 5, 6 and 7 mode control.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`ep567_mod::R`](R).  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`ep567_mod::W`](W). You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct EP567_MOD_SPEC;
    impl crate::pac::generic::RegisterSpec for EP567_MOD_SPEC {
        type Ux = u8;
    }
    #[doc = "`read()` method returns [`ep567_mod::R`](R) reader structure"]
    impl crate::pac::generic::Readable for EP567_MOD_SPEC {}
    #[doc = "`write(|w| ..)` method takes [`ep567_mod::W`](W) writer structure"]
    impl crate::pac::generic::Writable for EP567_MOD_SPEC {
        const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
        const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    }
    #[doc = "`reset()` method sets EP567_MOD to value 0"]
    impl crate::pac::generic::Resettable for EP567_MOD_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
#[doc = "EP_DMA (rw) register accessor: Start address of the endpoint buffer.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`ep_dma::R`].  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`ep_dma::W`]. You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep_dma`]
module"]
pub type EP_DMA = crate::pac::generic::Reg<ep_dma::EP_DMA_SPEC>;
#[doc = "Start address of the endpoint buffer."]
pub mod ep_dma {
    #[doc = "Register `EP%s_DMA` reader"]
    pub type R = crate::pac::generic::R<EP_DMA_SPEC>;
    #[doc = "Register `EP%s_DMA` writer"]
    pub type W = crate::pac::generic::W<EP_DMA_SPEC>;
    #[doc = "Field `BUF_ADDR` reader - Start address of the endpoint buffer, must be 4 byte aligned."]
    pub type BUF_ADDR_R = crate::pac::generic::FieldReader<u16>;
    #[doc = "Field `BUF_ADDR` writer - Start address of the endpoint buffer, must be 4 byte aligned."]
    pub type BUF_ADDR_W<'a, REG, const O: u8> =
        crate::pac::generic::FieldWriter<'a, REG, 15, O, u16>;
    impl R {
        #[doc = "Bits 0:14 - Start address of the endpoint buffer, must be 4 byte aligned."]
        #[inline(always)]
        pub fn buf_addr(&self) -> BUF_ADDR_R {
            BUF_ADDR_R::new((self.bits & 0x7fff) as u16)
        }
    }
    impl W {
        #[doc = "Bits 0:14 - Start address of the endpoint buffer, must be 4 byte aligned."]
        #[inline(always)]
        #[must_use]
        pub fn buf_addr(&mut self) -> BUF_ADDR_W<EP_DMA_SPEC, 0> {
            BUF_ADDR_W::new(self)
        }
        #[doc = r" Writes raw bits to the register."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
    }
    #[doc = "Start address of the endpoint buffer.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`ep_dma::R`](R).  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`ep_dma::W`](W). You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct EP_DMA_SPEC;
    impl crate::pac::generic::RegisterSpec for EP_DMA_SPEC {
        type Ux = u32;
    }
    #[doc = "`read()` method returns [`ep_dma::R`](R) reader structure"]
    impl crate::pac::generic::Readable for EP_DMA_SPEC {}
    #[doc = "`write(|w| ..)` method takes [`ep_dma::W`](W) writer structure"]
    impl crate::pac::generic::Writable for EP_DMA_SPEC {
        const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
        const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    }
    #[doc = "`reset()` method sets EP%s_DMA to value 0"]
    impl crate::pac::generic::Resettable for EP_DMA_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
pub use ep_dma as ep5_dma;
pub use EP_DMA as EP5_DMA;
#[doc = "EP_TX_LEN (rw) register accessor: Endpoint transmit length.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`ep_tx_len::R`].  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`ep_tx_len::W`]. You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep_tx_len`]
module"]
pub type EP_TX_LEN = crate::pac::generic::Reg<ep_tx_len::EP_TX_LEN_SPEC>;
#[doc = "Endpoint transmit length."]
pub mod ep_tx_len {
    #[doc = "Register `EP%s_TX_LEN` reader"]
    pub type R = crate::pac::generic::R<EP_TX_LEN_SPEC>;
    #[doc = "Register `EP%s_TX_LEN` writer"]
    pub type W = crate::pac::generic::W<EP_TX_LEN_SPEC>;
    #[doc = "Field `T_LEN` reader - The number of bytes to be transmitted by the endpoint."]
    pub type T_LEN_R = crate::pac::generic::FieldReader;
    #[doc = "Field `T_LEN` writer - The number of bytes to be transmitted by the endpoint."]
    pub type T_LEN_W<'a, REG, const O: u8> = crate::pac::generic::FieldWriter<'a, REG, 7, O>;
    impl R {
        #[doc = "Bits 0:6 - The number of bytes to be transmitted by the endpoint."]
        #[inline(always)]
        pub fn t_len(&self) -> T_LEN_R {
            T_LEN_R::new((self.bits & 0x7f) as u8)
        }
    }
    impl W {
        #[doc = "Bits 0:6 - The number of bytes to be transmitted by the endpoint."]
        #[inline(always)]
        #[must_use]
        pub fn t_len(&mut self) -> T_LEN_W<EP_TX_LEN_SPEC, 0> {
            T_LEN_W::new(self)
        }
        #[doc = r" Writes raw bits to the register."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
            self.bits = bits;
            self
        }
    }
    #[doc = "Endpoint transmit length.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`ep_tx_len::R`](R).  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`ep_tx_len::W`](W). You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct EP_TX_LEN_SPEC;
    impl crate::pac::generic::RegisterSpec for EP_TX_LEN_SPEC {
        type Ux = u16;
    }
    #[doc = "`read()` method returns [`ep_tx_len::R`](R) reader structure"]
    impl crate::pac::generic::Readable for EP_TX_LEN_SPEC {}
    #[doc = "`write(|w| ..)` method takes [`ep_tx_len::W`](W) writer structure"]
    impl crate::pac::generic::Writable for EP_TX_LEN_SPEC {
        const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
        const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    }
    #[doc = "`reset()` method sets EP%s_TX_LEN to value 0"]
    impl crate::pac::generic::Resettable for EP_TX_LEN_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
pub use ep_tx_len as ep5_tx_len;
pub use EP_TX_LEN as EP5_TX_LEN;
#[doc = "EP_CTRL_H (rw) register accessor: Endpoint control register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`ep_ctrl_h::R`].  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`ep_ctrl_h::W`]. You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep_ctrl_h`]
module"]
pub type EP_CTRL_H = crate::pac::generic::Reg<ep_ctrl_h::EP_CTRL_H_SPEC>;
#[doc = "Endpoint control register."]
pub mod ep_ctrl_h {
    #[doc = "Register `EP%s_CTRL_H` reader"]
    pub type R = crate::pac::generic::R<EP_CTRL_H_SPEC>;
    #[doc = "Register `EP%s_CTRL_H` writer"]
    pub type W = crate::pac::generic::W<EP_CTRL_H_SPEC>;
    #[doc = "Field `T_RES` reader - Endpoint transmitter response control."]
    pub type T_RES_R = crate::pac::generic::FieldReader<T_RES_A>;
    #[doc = "Endpoint transmitter response control.\n\nValue on reset: 0"]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    pub enum T_RES_A {
        #[doc = "0: `0`"]
        Ack = 0,
        #[doc = "1: `1`"]
        Nyet = 1,
        #[doc = "2: `10`"]
        Nak = 2,
        #[doc = "3: `11`"]
        Stall = 3,
    }
    impl From<T_RES_A> for u8 {
        #[inline(always)]
        fn from(variant: T_RES_A) -> Self {
            variant as _
        }
    }
    impl crate::pac::generic::FieldSpec for T_RES_A {
        type Ux = u8;
    }
    impl T_RES_R {
        #[doc = "Get enumerated values variant"]
        #[inline(always)]
        pub fn variant(&self) -> T_RES_A {
            match self.bits {
                0 => T_RES_A::Ack,
                1 => T_RES_A::Nyet,
                2 => T_RES_A::Nak,
                3 => T_RES_A::Stall,
                _ => unreachable!(),
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn is_ack(&self) -> bool {
            *self == T_RES_A::Ack
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn is_nyet(&self) -> bool {
            *self == T_RES_A::Nyet
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn is_nak(&self) -> bool {
            *self == T_RES_A::Nak
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn is_stall(&self) -> bool {
            *self == T_RES_A::Stall
        }
    }
    #[doc = "Field `T_RES` writer - Endpoint transmitter response control."]
    pub type T_RES_W<'a, REG, const O: u8> =
        crate::pac::generic::FieldWriterSafe<'a, REG, 2, O, T_RES_A>;
    impl<'a, REG, const O: u8> T_RES_W<'a, REG, O>
    where
        REG: crate::pac::generic::Writable + crate::pac::generic::RegisterSpec,
        REG::Ux: From<u8>,
    {
        #[doc = "`0`"]
        #[inline(always)]
        pub fn ack(self) -> &'a mut crate::pac::generic::W<REG> {
            self.variant(T_RES_A::Ack)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn nyet(self) -> &'a mut crate::pac::generic::W<REG> {
            self.variant(T_RES_A::Nyet)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn nak(self) -> &'a mut crate::pac::generic::W<REG> {
            self.variant(T_RES_A::Nak)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn stall(self) -> &'a mut crate::pac::generic::W<REG> {
            self.variant(T_RES_A::Stall)
        }
    }
    #[doc = "Field `R_RES` reader - Endpoint receiver response control."]
    pub type R_RES_R = crate::pac::generic::FieldReader<R_RES_A>;
    #[doc = "Endpoint receiver response control.\n\nValue on reset: 0"]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    pub enum R_RES_A {
        #[doc = "0: `0`"]
        Ack = 0,
        #[doc = "1: `1`"]
        Nyet = 1,
        #[doc = "2: `10`"]
        Nak = 2,
        #[doc = "3: `11`"]
        Stall = 3,
    }
    impl From<R_RES_A> for u8 {
        #[inline(always)]
        fn from(variant: R_RES_A) -> Self {
            variant as _
        }
    }
    impl crate::pac::generic::FieldSpec for R_RES_A {
        type Ux = u8;
    }
    impl R_RES_R {
        #[doc = "Get enumerated values variant"]
        #[inline(always)]
        pub fn variant(&self) -> R_RES_A {
            match self.bits {
                0 => R_RES_A::Ack,
                1 => R_RES_A::Nyet,
                2 => R_RES_A::Nak,
                3 => R_RES_A::Stall,
                _ => unreachable!(),
            }
        }
        #[doc = "`0`"]
        #[inline(always)]
        pub fn is_ack(&self) -> bool {
            *self == R_RES_A::Ack
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn is_nyet(&self) -> bool {
            *self == R_RES_A::Nyet
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn is_nak(&self) -> bool {
            *self == R_RES_A::Nak
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn is_stall(&self) -> bool {
            *self == R_RES_A::Stall
        }
    }
    #[doc = "Field `R_RES` writer - Endpoint receiver response control."]
    pub type R_RES_W<'a, REG, const O: u8> =
        crate::pac::generic::FieldWriterSafe<'a, REG, 2, O, R_RES_A>;
    impl<'a, REG, const O: u8> R_RES_W<'a, REG, O>
    where
        REG: crate::pac::generic::Writable + crate::pac::generic::RegisterSpec,
        REG::Ux: From<u8>,
    {
        #[doc = "`0`"]
        #[inline(always)]
        pub fn ack(self) -> &'a mut crate::pac::generic::W<REG> {
            self.variant(R_RES_A::Ack)
        }
        #[doc = "`1`"]
        #[inline(always)]
        pub fn nyet(self) -> &'a mut crate::pac::generic::W<REG> {
            self.variant(R_RES_A::Nyet)
        }
        #[doc = "`10`"]
        #[inline(always)]
        pub fn nak(self) -> &'a mut crate::pac::generic::W<REG> {
            self.variant(R_RES_A::Nak)
        }
        #[doc = "`11`"]
        #[inline(always)]
        pub fn stall(self) -> &'a mut crate::pac::generic::W<REG> {
            self.variant(R_RES_A::Stall)
        }
    }
    #[doc = "Field `T_TOG` reader - Endpoint transmitter toggle bit."]
    pub type T_TOG_R = crate::pac::generic::BitReader;
    #[doc = "Field `T_TOG` writer - Endpoint transmitter toggle bit."]
    pub type T_TOG_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    #[doc = "Field `R_TOG` reader - Endpoint receiver toggle bit."]
    pub type R_TOG_R = crate::pac::generic::BitReader;
    #[doc = "Field `R_TOG` writer - Endpoint receiver toggle bit."]
    pub type R_TOG_W<'a, REG, const O: u8> = crate::pac::generic::BitWriter<'a, REG, O>;
    impl R {
        #[doc = "Bits 0:1 - Endpoint transmitter response control."]
        #[inline(always)]
        pub fn t_res(&self) -> T_RES_R {
            T_RES_R::new((self.bits & 3) as u8)
        }
        #[doc = "Bits 2:3 - Endpoint receiver response control."]
        #[inline(always)]
        pub fn r_res(&self) -> R_RES_R {
            R_RES_R::new(((self.bits >> 2) & 3) as u8)
        }
        #[doc = "Bit 6 - Endpoint transmitter toggle bit."]
        #[inline(always)]
        pub fn t_tog(&self) -> T_TOG_R {
            T_TOG_R::new(((self.bits >> 6) & 1) != 0)
        }
        #[doc = "Bit 7 - Endpoint receiver toggle bit."]
        #[inline(always)]
        pub fn r_tog(&self) -> R_TOG_R {
            R_TOG_R::new(((self.bits >> 7) & 1) != 0)
        }
    }
    impl W {
        #[doc = "Bits 0:1 - Endpoint transmitter response control."]
        #[inline(always)]
        #[must_use]
        pub fn t_res(&mut self) -> T_RES_W<EP_CTRL_H_SPEC, 0> {
            T_RES_W::new(self)
        }
        #[doc = "Bits 2:3 - Endpoint receiver response control."]
        #[inline(always)]
        #[must_use]
        pub fn r_res(&mut self) -> R_RES_W<EP_CTRL_H_SPEC, 2> {
            R_RES_W::new(self)
        }
        #[doc = "Bit 6 - Endpoint transmitter toggle bit."]
        #[inline(always)]
        #[must_use]
        pub fn t_tog(&mut self) -> T_TOG_W<EP_CTRL_H_SPEC, 6> {
            T_TOG_W::new(self)
        }
        #[doc = "Bit 7 - Endpoint receiver toggle bit."]
        #[inline(always)]
        #[must_use]
        pub fn r_tog(&mut self) -> R_TOG_W<EP_CTRL_H_SPEC, 7> {
            R_TOG_W::new(self)
        }
        #[doc = r" Writes raw bits to the register."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
            self.bits = bits;
            self
        }
    }
    #[doc = "Endpoint control register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`ep_ctrl_h::R`](R).  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`ep_ctrl_h::W`](W). You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct EP_CTRL_H_SPEC;
    impl crate::pac::generic::RegisterSpec for EP_CTRL_H_SPEC {
        type Ux = u16;
    }
    #[doc = "`read()` method returns [`ep_ctrl_h::R`](R) reader structure"]
    impl crate::pac::generic::Readable for EP_CTRL_H_SPEC {}
    #[doc = "`write(|w| ..)` method takes [`ep_ctrl_h::W`](W) writer structure"]
    impl crate::pac::generic::Writable for EP_CTRL_H_SPEC {
        const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
        const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    }
    #[doc = "`reset()` method sets EP%s_CTRL_H to value 0"]
    impl crate::pac::generic::Resettable for EP_CTRL_H_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
pub use ep_ctrl_h as ep5_ctrl_h;
pub use EP_CTRL_H as EP5_CTRL_H;
#[doc = "EPX_CTRL (rw) register accessor: Endpoint X control register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`epx_ctrl::R`].  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`epx_ctrl::W`]. You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`epx_ctrl`]
module"]
pub type EPX_CTRL = crate::pac::generic::Reg<epx_ctrl::EPX_CTRL_SPEC>;
#[doc = "Endpoint X control register."]
pub mod epx_ctrl {
    #[doc = "Register `EPX_CTRL` reader"]
    pub type R = crate::pac::generic::R<EPX_CTRL_SPEC>;
    #[doc = "Register `EPX_CTRL` writer"]
    pub type W = crate::pac::generic::W<EPX_CTRL_SPEC>;
    #[doc = "Field `T_EN` reader - Endpoint (8-15) transmit enable."]
    pub type T_EN_R = crate::pac::generic::FieldReader;
    #[doc = "Field `T_EN` writer - Endpoint (8-15) transmit enable."]
    pub type T_EN_W<'a, REG, const O: u8> = crate::pac::generic::FieldWriter<'a, REG, 8, O>;
    #[doc = "Field `R_EN` reader - Endpoint (8-15) receive enable."]
    pub type R_EN_R = crate::pac::generic::FieldReader;
    #[doc = "Field `R_EN` writer - Endpoint (8-15) receive enable."]
    pub type R_EN_W<'a, REG, const O: u8> = crate::pac::generic::FieldWriter<'a, REG, 8, O>;
    #[doc = "Field `T_AF` reader - Transmit endpoint alternate enable."]
    pub type T_AF_R = crate::pac::generic::FieldReader;
    #[doc = "Field `T_AF` writer - Transmit endpoint alternate enable."]
    pub type T_AF_W<'a, REG, const O: u8> = crate::pac::generic::FieldWriter<'a, REG, 7, O>;
    impl R {
        #[doc = "Bits 0:7 - Endpoint (8-15) transmit enable."]
        #[inline(always)]
        pub fn t_en(&self) -> T_EN_R {
            T_EN_R::new((self.bits & 0xff) as u8)
        }
        #[doc = "Bits 8:15 - Endpoint (8-15) receive enable."]
        #[inline(always)]
        pub fn r_en(&self) -> R_EN_R {
            R_EN_R::new(((self.bits >> 8) & 0xff) as u8)
        }
        #[doc = "Bits 17:23 - Transmit endpoint alternate enable."]
        #[inline(always)]
        pub fn t_af(&self) -> T_AF_R {
            T_AF_R::new(((self.bits >> 17) & 0x7f) as u8)
        }
    }
    impl W {
        #[doc = "Bits 0:7 - Endpoint (8-15) transmit enable."]
        #[inline(always)]
        #[must_use]
        pub fn t_en(&mut self) -> T_EN_W<EPX_CTRL_SPEC, 0> {
            T_EN_W::new(self)
        }
        #[doc = "Bits 8:15 - Endpoint (8-15) receive enable."]
        #[inline(always)]
        #[must_use]
        pub fn r_en(&mut self) -> R_EN_W<EPX_CTRL_SPEC, 8> {
            R_EN_W::new(self)
        }
        #[doc = "Bits 17:23 - Transmit endpoint alternate enable."]
        #[inline(always)]
        #[must_use]
        pub fn t_af(&mut self) -> T_AF_W<EPX_CTRL_SPEC, 17> {
            T_AF_W::new(self)
        }
        #[doc = r" Writes raw bits to the register."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
    }
    #[doc = "Endpoint X control register.\n\nYou can [`read`](crate::pac::generic::generic::Reg::read) this register and get [`epx_ctrl::R`](R).  You can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero) this register using [`epx_ctrl::W`](W). You can also [`modify`](crate::pac::generic::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
    pub struct EPX_CTRL_SPEC;
    impl crate::pac::generic::RegisterSpec for EPX_CTRL_SPEC {
        type Ux = u32;
    }
    #[doc = "`read()` method returns [`epx_ctrl::R`](R) reader structure"]
    impl crate::pac::generic::Readable for EPX_CTRL_SPEC {}
    #[doc = "`write(|w| ..)` method takes [`epx_ctrl::W`](W) writer structure"]
    impl crate::pac::generic::Writable for EPX_CTRL_SPEC {
        const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
        const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    }
    #[doc = "`reset()` method sets EPX_CTRL to value 0"]
    impl crate::pac::generic::Resettable for EPX_CTRL_SPEC {
        const RESET_VALUE: Self::Ux = 0;
    }
}
