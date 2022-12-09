#[doc = "Register `GFWR` reader"]
pub struct R(crate::R<GFWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GFWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GFWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GFWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GFWR` writer"]
pub struct W(crate::W<GFWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GFWR_SPEC>;
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
impl From<crate::W<GFWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GFWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GFWR` reader - It determines the Glitch Filter Width"]
pub type GFWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GFWR` writer - It determines the Glitch Filter Width"]
pub type GFWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GFWR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - It determines the Glitch Filter Width"]
    #[inline(always)]
    pub fn gfwr(&self) -> GFWR_R {
        GFWR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - It determines the Glitch Filter Width"]
    #[inline(always)]
    #[must_use]
    pub fn gfwr(&mut self) -> GFWR_W<0> {
        GFWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Glitch Filter Width Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfwr](index.html) module"]
pub struct GFWR_SPEC;
impl crate::RegisterSpec for GFWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gfwr::R](R) reader structure"]
impl crate::Readable for GFWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gfwr::W](W) writer structure"]
impl crate::Writable for GFWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GFWR to value 0x7f"]
impl crate::Resettable for GFWR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
