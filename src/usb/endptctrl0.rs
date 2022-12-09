#[doc = "Register `ENDPTCTRL0` reader"]
pub struct R(crate::R<ENDPTCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPTCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPTCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPTCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPTCTRL0` writer"]
pub struct W(crate::W<ENDPTCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDPTCTRL0_SPEC>;
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
impl From<crate::W<ENDPTCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDPTCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXS` reader - RX Endpoint Stall - Read/Write 0 End Point OK"]
pub type RXS_R = crate::BitReader<bool>;
#[doc = "Field `RXS` writer - RX Endpoint Stall - Read/Write 0 End Point OK"]
pub type RXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL0_SPEC, bool, O>;
#[doc = "Field `RXT` reader - RX Endpoint Type - Read/Write 00 Control Endpoint0 is fixed as a Control End Point."]
pub type RXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXT` writer - RX Endpoint Type - Read/Write 00 Control Endpoint0 is fixed as a Control End Point."]
pub type RXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTCTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXE` reader - RX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
pub type RXE_R = crate::BitReader<bool>;
#[doc = "Field `RXE` writer - RX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
pub type RXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL0_SPEC, bool, O>;
#[doc = "Field `TXS` reader - TX Endpoint Stall - Read/Write 0 End Point OK \\[Default\\]
1 End Point Stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host"]
pub type TXS_R = crate::BitReader<bool>;
#[doc = "Field `TXS` writer - TX Endpoint Stall - Read/Write 0 End Point OK \\[Default\\]
1 End Point Stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host"]
pub type TXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL0_SPEC, bool, O>;
#[doc = "Field `TXT` reader - TX Endpoint Type - Read/Write 00 - Control Endpoint0 is fixed as a Control End Point."]
pub type TXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXT` writer - TX Endpoint Type - Read/Write 00 - Control Endpoint0 is fixed as a Control End Point."]
pub type TXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTCTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXE` reader - TX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `TXE` writer - TX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
pub type TXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENDPTCTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RX Endpoint Stall - Read/Write 0 End Point OK"]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - RX Endpoint Type - Read/Write 00 Control Endpoint0 is fixed as a Control End Point."]
    #[inline(always)]
    pub fn rxt(&self) -> RXT_R {
        RXT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - RX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - TX Endpoint Stall - Read/Write 0 End Point OK \\[Default\\]
1 End Point Stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host"]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - TX Endpoint Type - Read/Write 00 - Control Endpoint0 is fixed as a Control End Point."]
    #[inline(always)]
    pub fn txt(&self) -> TXT_R {
        TXT_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 23 - TX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
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
    #[doc = "Bits 2:3 - RX Endpoint Type - Read/Write 00 Control Endpoint0 is fixed as a Control End Point."]
    #[inline(always)]
    #[must_use]
    pub fn rxt(&mut self) -> RXT_W<2> {
        RXT_W::new(self)
    }
    #[doc = "Bit 7 - RX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RXE_W<7> {
        RXE_W::new(self)
    }
    #[doc = "Bit 16 - TX Endpoint Stall - Read/Write 0 End Point OK \\[Default\\]
1 End Point Stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host"]
    #[inline(always)]
    #[must_use]
    pub fn txs(&mut self) -> TXS_W<16> {
        TXS_W::new(self)
    }
    #[doc = "Bits 18:19 - TX Endpoint Type - Read/Write 00 - Control Endpoint0 is fixed as a Control End Point."]
    #[inline(always)]
    #[must_use]
    pub fn txt(&mut self) -> TXT_W<18> {
        TXT_W::new(self)
    }
    #[doc = "Bit 23 - TX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
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
#[doc = "Endpoint Control0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptctrl0](index.html) module"]
pub struct ENDPTCTRL0_SPEC;
impl crate::RegisterSpec for ENDPTCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endptctrl0::R](R) reader structure"]
impl crate::Readable for ENDPTCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endptctrl0::W](W) writer structure"]
impl crate::Writable for ENDPTCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENDPTCTRL0 to value 0x0080_0080"]
impl crate::Resettable for ENDPTCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0080;
}
