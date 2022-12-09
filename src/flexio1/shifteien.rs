#[doc = "Register `SHIFTEIEN` reader"]
pub struct R(crate::R<SHIFTEIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTEIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTEIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTEIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTEIEN` writer"]
pub struct W(crate::W<SHIFTEIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTEIEN_SPEC>;
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
impl From<crate::W<SHIFTEIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTEIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEIE` reader - Shifter Error Interrupt Enable"]
pub type SEIE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEIE` writer - Shifter Error Interrupt Enable"]
pub type SEIE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHIFTEIEN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shifter Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SEIE_W<0> {
        SEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Error Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shifteien](index.html) module"]
pub struct SHIFTEIEN_SPEC;
impl crate::RegisterSpec for SHIFTEIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shifteien::R](R) reader structure"]
impl crate::Readable for SHIFTEIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shifteien::W](W) writer structure"]
impl crate::Writable for SHIFTEIEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTEIEN to value 0"]
impl crate::Resettable for SHIFTEIEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
