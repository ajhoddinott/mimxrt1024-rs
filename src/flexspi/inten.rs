#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPCMDDONEEN` reader - IP triggered Command Sequences Execution finished interrupt enable."]
pub type IPCMDDONEEN_R = crate::BitReader<bool>;
#[doc = "Field `IPCMDDONEEN` writer - IP triggered Command Sequences Execution finished interrupt enable."]
pub type IPCMDDONEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `IPCMDGEEN` reader - IP triggered Command Sequences Grant Timeout interrupt enable."]
pub type IPCMDGEEN_R = crate::BitReader<bool>;
#[doc = "Field `IPCMDGEEN` writer - IP triggered Command Sequences Grant Timeout interrupt enable."]
pub type IPCMDGEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `AHBCMDGEEN` reader - AHB triggered Command Sequences Grant Timeout interrupt enable."]
pub type AHBCMDGEEN_R = crate::BitReader<bool>;
#[doc = "Field `AHBCMDGEEN` writer - AHB triggered Command Sequences Grant Timeout interrupt enable."]
pub type AHBCMDGEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `IPCMDERREN` reader - IP triggered Command Sequences Error Detected interrupt enable."]
pub type IPCMDERREN_R = crate::BitReader<bool>;
#[doc = "Field `IPCMDERREN` writer - IP triggered Command Sequences Error Detected interrupt enable."]
pub type IPCMDERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `AHBCMDERREN` reader - AHB triggered Command Sequences Error Detected interrupt enable."]
pub type AHBCMDERREN_R = crate::BitReader<bool>;
#[doc = "Field `AHBCMDERREN` writer - AHB triggered Command Sequences Error Detected interrupt enable."]
pub type AHBCMDERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `IPRXWAEN` reader - IP RX FIFO WaterMark available interrupt enable."]
pub type IPRXWAEN_R = crate::BitReader<bool>;
#[doc = "Field `IPRXWAEN` writer - IP RX FIFO WaterMark available interrupt enable."]
pub type IPRXWAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `IPTXWEEN` reader - IP TX FIFO WaterMark empty interrupt enable."]
pub type IPTXWEEN_R = crate::BitReader<bool>;
#[doc = "Field `IPTXWEEN` writer - IP TX FIFO WaterMark empty interrupt enable."]
pub type IPTXWEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `SCKSTOPBYRDEN` reader - SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
pub type SCKSTOPBYRDEN_R = crate::BitReader<bool>;
#[doc = "Field `SCKSTOPBYRDEN` writer - SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
pub type SCKSTOPBYRDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `SCKSTOPBYWREN` reader - SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
pub type SCKSTOPBYWREN_R = crate::BitReader<bool>;
#[doc = "Field `SCKSTOPBYWREN` writer - SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
pub type SCKSTOPBYWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `AHBBUSTIMEOUTEN` reader - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
pub type AHBBUSTIMEOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `AHBBUSTIMEOUTEN` writer - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
pub type AHBBUSTIMEOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `SEQTIMEOUTEN` reader - Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
pub type SEQTIMEOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `SEQTIMEOUTEN` writer - Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
pub type SEQTIMEOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt enable."]
    #[inline(always)]
    pub fn ipcmddoneen(&self) -> IPCMDDONEEN_R {
        IPCMDDONEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub fn ipcmdgeen(&self) -> IPCMDGEEN_R {
        IPCMDGEEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub fn ahbcmdgeen(&self) -> AHBCMDGEEN_R {
        AHBCMDGEEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub fn ipcmderren(&self) -> IPCMDERREN_R {
        IPCMDERREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub fn ahbcmderren(&self) -> AHBCMDERREN_R {
        AHBCMDERREN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IP RX FIFO WaterMark available interrupt enable."]
    #[inline(always)]
    pub fn iprxwaen(&self) -> IPRXWAEN_R {
        IPRXWAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IP TX FIFO WaterMark empty interrupt enable."]
    #[inline(always)]
    pub fn iptxween(&self) -> IPTXWEEN_R {
        IPTXWEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    #[inline(always)]
    pub fn sckstopbyrden(&self) -> SCKSTOPBYRDEN_R {
        SCKSTOPBYRDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    #[inline(always)]
    pub fn sckstopbywren(&self) -> SCKSTOPBYWREN_R {
        SCKSTOPBYWREN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn ahbbustimeouten(&self) -> AHBBUSTIMEOUTEN_R {
        AHBBUSTIMEOUTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn seqtimeouten(&self) -> SEQTIMEOUTEN_R {
        SEQTIMEOUTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ipcmddoneen(&mut self) -> IPCMDDONEEN_W<0> {
        IPCMDDONEEN_W::new(self)
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ipcmdgeen(&mut self) -> IPCMDGEEN_W<1> {
        IPCMDGEEN_W::new(self)
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ahbcmdgeen(&mut self) -> AHBCMDGEEN_W<2> {
        AHBCMDGEEN_W::new(self)
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ipcmderren(&mut self) -> IPCMDERREN_W<3> {
        IPCMDERREN_W::new(self)
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ahbcmderren(&mut self) -> AHBCMDERREN_W<4> {
        AHBCMDERREN_W::new(self)
    }
    #[doc = "Bit 5 - IP RX FIFO WaterMark available interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn iprxwaen(&mut self) -> IPRXWAEN_W<5> {
        IPRXWAEN_W::new(self)
    }
    #[doc = "Bit 6 - IP TX FIFO WaterMark empty interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn iptxween(&mut self) -> IPTXWEEN_W<6> {
        IPTXWEEN_W::new(self)
    }
    #[doc = "Bit 8 - SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn sckstopbyrden(&mut self) -> SCKSTOPBYRDEN_W<8> {
        SCKSTOPBYRDEN_W::new(self)
    }
    #[doc = "Bit 9 - SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn sckstopbywren(&mut self) -> SCKSTOPBYWREN_W<9> {
        SCKSTOPBYWREN_W::new(self)
    }
    #[doc = "Bit 10 - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    #[must_use]
    pub fn ahbbustimeouten(&mut self) -> AHBBUSTIMEOUTEN_W<10> {
        AHBBUSTIMEOUTEN_W::new(self)
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
    #[inline(always)]
    #[must_use]
    pub fn seqtimeouten(&mut self) -> SEQTIMEOUTEN_W<11> {
        SEQTIMEOUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
