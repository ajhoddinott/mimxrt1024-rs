#[doc = "Register `RX` reader"]
pub struct R(crate::R<RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX` writer"]
pub struct W(crate::W<RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_SPEC>;
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
impl From<crate::W<RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENVADJ` reader - The ENVADJ field adjusts the trip point for the envelope detector"]
pub type ENVADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENVADJ` writer - The ENVADJ field adjusts the trip point for the envelope detector"]
pub type ENVADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSVD0` reader - Reserved."]
pub type RSVD0_R = crate::BitReader<bool>;
#[doc = "Field `DISCONADJ` reader - The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
pub type DISCONADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISCONADJ` writer - The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
pub type DISCONADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSVD1` reader - Reserved."]
pub type RSVD1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXDBYPASS` reader - 0 = Normal operation"]
pub type RXDBYPASS_R = crate::BitReader<bool>;
#[doc = "Field `RXDBYPASS` writer - 0 = Normal operation"]
pub type RXDBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_SPEC, bool, O>;
#[doc = "Field `RSVD2` reader - Reserved."]
pub type RSVD2_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&self) -> ENVADJ_R {
        ENVADJ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub fn disconadj(&self) -> DISCONADJ_R {
        DISCONADJ_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:21 - Reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 7) & 0x7fff) as u16)
    }
    #[doc = "Bit 22 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxdbypass(&self) -> RXDBYPASS_R {
        RXDBYPASS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - Reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    #[must_use]
    pub fn envadj(&mut self) -> ENVADJ_W<0> {
        ENVADJ_W::new(self)
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    #[must_use]
    pub fn disconadj(&mut self) -> DISCONADJ_W<4> {
        DISCONADJ_W::new(self)
    }
    #[doc = "Bit 22 - 0 = Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn rxdbypass(&mut self) -> RXDBYPASS_W<22> {
        RXDBYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Receiver Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx](index.html) module"]
pub struct RX_SPEC;
impl crate::RegisterSpec for RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx::R](R) reader structure"]
impl crate::Readable for RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx::W](W) writer structure"]
impl crate::Writable for RX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX to value 0"]
impl crate::Resettable for RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
