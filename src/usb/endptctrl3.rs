#[doc = "Register `ENDPTCTRL3` reader"]
pub struct R(crate::R<ENDPTCTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPTCTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPTCTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPTCTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPTCTRL3` writer"]
pub struct W(crate::W<ENDPTCTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDPTCTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ENDPTCTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDPTCTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXS` reader - RX Endpoint Stall - Read/Write 0 End Point OK"]
pub type RXS_R = crate::BitReader<bool>;
#[doc = "Field `RXS` writer - RX Endpoint Stall - Read/Write 0 End Point OK"]
pub type RXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL3_SPEC, bool, O>;
#[doc = "Field `RXD` reader - RX Endpoint Data Sink - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[Default\\]
Should always be written as zero"]
pub type RXD_R = crate::BitReader<bool>;
#[doc = "Field `RXD` writer - RX Endpoint Data Sink - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[Default\\]
Should always be written as zero"]
pub type RXD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL3_SPEC, bool, O>;
#[doc = "Field `RXT` reader - RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
pub type RXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXT` writer - RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
pub type RXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTCTRL3_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXI` reader - RX Data Toggle Inhibit 0 Disabled \\[Default\\]
1 Enabled This bit is only used for test and should always be written as zero"]
pub type RXI_R = crate::BitReader<bool>;
#[doc = "Field `RXI` writer - RX Data Toggle Inhibit 0 Disabled \\[Default\\]
1 Enabled This bit is only used for test and should always be written as zero"]
pub type RXI_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL3_SPEC, bool, O>;
#[doc = "Field `RXR` reader - RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device"]
pub type RXR_R = crate::BitReader<bool>;
#[doc = "Field `RXR` writer - RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device"]
pub type RXR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL3_SPEC, bool, O>;
#[doc = "Field `RXE` reader - RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured"]
pub type RXE_R = crate::BitReader<bool>;
#[doc = "Field `RXE` writer - RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured"]
pub type RXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL3_SPEC, bool, O>;
#[doc = "Field `TXS` reader - TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared"]
pub type TXS_R = crate::BitReader<bool>;
#[doc = "Field `TXS` writer - TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared"]
pub type TXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL3_SPEC, bool, O>;
#[doc = "Field `TXD` reader - TX Endpoint Data Source - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[DEFAULT\\]
Should always be written as 0"]
pub type TXD_R = crate::BitReader<bool>;
#[doc = "Field `TXD` writer - TX Endpoint Data Source - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[DEFAULT\\]
Should always be written as 0"]
pub type TXD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL3_SPEC, bool, O>;
#[doc = "Field `TXT` reader - TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
pub type TXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXT` writer - TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
pub type TXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTCTRL3_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXI` reader - TX Data Toggle Inhibit 0 PID Sequencing Enabled"]
pub type TXI_R = crate::BitReader<bool>;
#[doc = "Field `TXI` writer - TX Data Toggle Inhibit 0 PID Sequencing Enabled"]
pub type TXI_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL3_SPEC, bool, O>;
#[doc = "Field `TXR` reader - TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device"]
pub type TXR_R = crate::BitReader<bool>;
#[doc = "Field `TXR` writer - TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device"]
pub type TXR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL3_SPEC, bool, O>;
#[doc = "Field `TXE` reader - TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured"]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `TXE` writer - TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured"]
pub type TXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RX Endpoint Stall - Read/Write 0 End Point OK"]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX Endpoint Data Sink - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[Default\\]
Should always be written as zero"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    pub fn rxt(&self) -> RXT_R {
        RXT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - RX Data Toggle Inhibit 0 Disabled \\[Default\\]
1 Enabled This bit is only used for test and should always be written as zero"]
    #[inline(always)]
    pub fn rxi(&self) -> RXI_R {
        RXI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device"]
    #[inline(always)]
    pub fn rxr(&self) -> RXR_R {
        RXR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured"]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared"]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX Endpoint Data Source - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[DEFAULT\\]
Should always be written as 0"]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    pub fn txt(&self) -> TXT_R {
        TXT_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - TX Data Toggle Inhibit 0 PID Sequencing Enabled"]
    #[inline(always)]
    pub fn txi(&self) -> TXI_R {
        TXI_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device"]
    #[inline(always)]
    pub fn txr(&self) -> TXR_R {
        TXR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX Endpoint Stall - Read/Write 0 End Point OK"]
    #[inline(always)]
    #[must_use]
    pub fn rxs(&mut self) -> RXS_W<0> {
        RXS_W::new(self)
    }
    #[doc = "Bit 1 - RX Endpoint Data Sink - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[Default\\]
Should always be written as zero"]
    #[inline(always)]
    #[must_use]
    pub fn rxd(&mut self) -> RXD_W<1> {
        RXD_W::new(self)
    }
    #[doc = "Bits 2:3 - RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxt(&mut self) -> RXT_W<2> {
        RXT_W::new(self)
    }
    #[doc = "Bit 5 - RX Data Toggle Inhibit 0 Disabled \\[Default\\]
1 Enabled This bit is only used for test and should always be written as zero"]
    #[inline(always)]
    #[must_use]
    pub fn rxi(&mut self) -> RXI_W<5> {
        RXI_W::new(self)
    }
    #[doc = "Bit 6 - RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device"]
    #[inline(always)]
    #[must_use]
    pub fn rxr(&mut self) -> RXR_W<6> {
        RXR_W::new(self)
    }
    #[doc = "Bit 7 - RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured"]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RXE_W<7> {
        RXE_W::new(self)
    }
    #[doc = "Bit 16 - TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared"]
    #[inline(always)]
    #[must_use]
    pub fn txs(&mut self) -> TXS_W<16> {
        TXS_W::new(self)
    }
    #[doc = "Bit 17 - TX Endpoint Data Source - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[DEFAULT\\]
Should always be written as 0"]
    #[inline(always)]
    #[must_use]
    pub fn txd(&mut self) -> TXD_W<17> {
        TXD_W::new(self)
    }
    #[doc = "Bits 18:19 - TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txt(&mut self) -> TXT_W<18> {
        TXT_W::new(self)
    }
    #[doc = "Bit 21 - TX Data Toggle Inhibit 0 PID Sequencing Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn txi(&mut self) -> TXI_W<21> {
        TXI_W::new(self)
    }
    #[doc = "Bit 22 - TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device"]
    #[inline(always)]
    #[must_use]
    pub fn txr(&mut self) -> TXR_W<22> {
        TXR_W::new(self)
    }
    #[doc = "Bit 23 - TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<23> {
        TXE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptctrl3](index.html) module"]
pub struct ENDPTCTRL3_SPEC;
impl crate::RegisterSpec for ENDPTCTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endptctrl3::R](R) reader structure"]
impl crate::Readable for ENDPTCTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endptctrl3::W](W) writer structure"]
impl crate::Writable for ENDPTCTRL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENDPTCTRL3 to value 0"]
impl crate::Resettable for ENDPTCTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
