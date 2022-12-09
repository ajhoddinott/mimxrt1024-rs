#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPCMDDONE` reader - IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
pub type IPCMDDONE_R = crate::BitReader<bool>;
#[doc = "Field `IPCMDDONE` writer - IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
pub type IPCMDDONE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `IPCMDGE` reader - IP triggered Command Sequences Grant Timeout interrupt."]
pub type IPCMDGE_R = crate::BitReader<bool>;
#[doc = "Field `IPCMDGE` writer - IP triggered Command Sequences Grant Timeout interrupt."]
pub type IPCMDGE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `AHBCMDGE` reader - AHB triggered Command Sequences Grant Timeout interrupt."]
pub type AHBCMDGE_R = crate::BitReader<bool>;
#[doc = "Field `AHBCMDGE` writer - AHB triggered Command Sequences Grant Timeout interrupt."]
pub type AHBCMDGE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `IPCMDERR` reader - IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
pub type IPCMDERR_R = crate::BitReader<bool>;
#[doc = "Field `IPCMDERR` writer - IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
pub type IPCMDERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `AHBCMDERR` reader - AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
pub type AHBCMDERR_R = crate::BitReader<bool>;
#[doc = "Field `AHBCMDERR` writer - AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
pub type AHBCMDERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `IPRXWA` reader - IP RX FIFO watermark available interrupt."]
pub type IPRXWA_R = crate::BitReader<bool>;
#[doc = "Field `IPRXWA` writer - IP RX FIFO watermark available interrupt."]
pub type IPRXWA_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `IPTXWE` reader - IP TX FIFO watermark empty interrupt."]
pub type IPTXWE_R = crate::BitReader<bool>;
#[doc = "Field `IPTXWE` writer - IP TX FIFO watermark empty interrupt."]
pub type IPTXWE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `SCKSTOPBYRD` reader - SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
pub type SCKSTOPBYRD_R = crate::BitReader<bool>;
#[doc = "Field `SCKSTOPBYRD` writer - SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
pub type SCKSTOPBYRD_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `SCKSTOPBYWR` reader - SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
pub type SCKSTOPBYWR_R = crate::BitReader<bool>;
#[doc = "Field `SCKSTOPBYWR` writer - SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
pub type SCKSTOPBYWR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `AHBBUSTIMEOUT` reader - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
pub type AHBBUSTIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `AHBBUSTIMEOUT` writer - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
pub type AHBBUSTIMEOUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `SEQTIMEOUT` reader - Sequence execution timeout interrupt."]
pub type SEQTIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `SEQTIMEOUT` writer - Sequence execution timeout interrupt."]
pub type SEQTIMEOUT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
    #[inline(always)]
    pub fn ipcmddone(&self) -> IPCMDDONE_R {
        IPCMDDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub fn ipcmdge(&self) -> IPCMDGE_R {
        IPCMDGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub fn ahbcmdge(&self) -> AHBCMDGE_R {
        AHBCMDGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub fn ipcmderr(&self) -> IPCMDERR_R {
        IPCMDERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub fn ahbcmderr(&self) -> AHBCMDERR_R {
        AHBCMDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IP RX FIFO watermark available interrupt."]
    #[inline(always)]
    pub fn iprxwa(&self) -> IPRXWA_R {
        IPRXWA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IP TX FIFO watermark empty interrupt."]
    #[inline(always)]
    pub fn iptxwe(&self) -> IPTXWE_R {
        IPTXWE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
    #[inline(always)]
    pub fn sckstopbyrd(&self) -> SCKSTOPBYRD_R {
        SCKSTOPBYRD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
    #[inline(always)]
    pub fn sckstopbywr(&self) -> SCKSTOPBYWR_R {
        SCKSTOPBYWR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn ahbbustimeout(&self) -> AHBBUSTIMEOUT_R {
        AHBBUSTIMEOUT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt."]
    #[inline(always)]
    pub fn seqtimeout(&self) -> SEQTIMEOUT_R {
        SEQTIMEOUT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
    #[inline(always)]
    #[must_use]
    pub fn ipcmddone(&mut self) -> IPCMDDONE_W<0> {
        IPCMDDONE_W::new(self)
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ipcmdge(&mut self) -> IPCMDGE_W<1> {
        IPCMDGE_W::new(self)
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ahbcmdge(&mut self) -> AHBCMDGE_W<2> {
        AHBCMDGE_W::new(self)
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
    #[inline(always)]
    #[must_use]
    pub fn ipcmderr(&mut self) -> IPCMDERR_W<3> {
        IPCMDERR_W::new(self)
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
    #[inline(always)]
    #[must_use]
    pub fn ahbcmderr(&mut self) -> AHBCMDERR_W<4> {
        AHBCMDERR_W::new(self)
    }
    #[doc = "Bit 5 - IP RX FIFO watermark available interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iprxwa(&mut self) -> IPRXWA_W<5> {
        IPRXWA_W::new(self)
    }
    #[doc = "Bit 6 - IP TX FIFO watermark empty interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iptxwe(&mut self) -> IPTXWE_W<6> {
        IPTXWE_W::new(self)
    }
    #[doc = "Bit 8 - SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sckstopbyrd(&mut self) -> SCKSTOPBYRD_W<8> {
        SCKSTOPBYRD_W::new(self)
    }
    #[doc = "Bit 9 - SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sckstopbywr(&mut self) -> SCKSTOPBYWR_W<9> {
        SCKSTOPBYWR_W::new(self)
    }
    #[doc = "Bit 10 - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    #[must_use]
    pub fn ahbbustimeout(&mut self) -> AHBBUSTIMEOUT_W<10> {
        AHBBUSTIMEOUT_W::new(self)
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn seqtimeout(&mut self) -> SEQTIMEOUT_W<11> {
        SEQTIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f7f;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
