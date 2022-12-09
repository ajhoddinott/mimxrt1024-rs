#[doc = "Register `DEBUG1_SET` reader"]
pub struct R(crate::R<DEBUG1_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG1_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG1_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG1_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG1_SET` writer"]
pub struct W(crate::W<DEBUG1_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG1_SET_SPEC>;
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
impl From<crate::W<DEBUG1_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG1_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSVD0` reader - Reserved. Note: This bit should remain clear."]
pub type RSVD0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RSVD0` writer - Reserved. Note: This bit should remain clear."]
pub type RSVD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEBUG1_SET_SPEC, u16, u16, 13, O>;
#[doc = "Field `ENTAILADJVD` reader - Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
pub type ENTAILADJVD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENTAILADJVD` writer - Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
pub type ENTAILADJVD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG1_SET_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSVD1` reader - Reserved."]
pub type RSVD1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:12 - Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub fn entailadjvd(&self) -> ENTAILADJVD_R {
        ENTAILADJVD_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:31 - Reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:12 - Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd0(&mut self) -> RSVD0_W<0> {
        RSVD0_W::new(self)
    }
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    #[must_use]
    pub fn entailadjvd(&mut self) -> ENTAILADJVD_W<13> {
        ENTAILADJVD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UTMI Debug Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug1_set](index.html) module"]
pub struct DEBUG1_SET_SPEC;
impl crate::RegisterSpec for DEBUG1_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug1_set::R](R) reader structure"]
impl crate::Readable for DEBUG1_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug1_set::W](W) writer structure"]
impl crate::Writable for DEBUG1_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG1_SET to value 0x1000"]
impl crate::Resettable for DEBUG1_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
