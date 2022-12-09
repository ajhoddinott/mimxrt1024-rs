#[doc = "Register `FLSHA2CR0` reader"]
pub struct R(crate::R<FLSHA2CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLSHA2CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLSHA2CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLSHA2CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLSHA2CR0` writer"]
pub struct W(crate::W<FLSHA2CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLSHA2CR0_SPEC>;
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
impl From<crate::W<FLSHA2CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLSHA2CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLSHSZ` reader - Flash Size in KByte."]
pub type FLSHSZ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLSHSZ` writer - Flash Size in KByte."]
pub type FLSHSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLSHA2CR0_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bits 0:22 - Flash Size in KByte."]
    #[inline(always)]
    pub fn flshsz(&self) -> FLSHSZ_R {
        FLSHSZ_R::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - Flash Size in KByte."]
    #[inline(always)]
    #[must_use]
    pub fn flshsz(&mut self) -> FLSHSZ_W<0> {
        FLSHSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flsha2cr0](index.html) module"]
pub struct FLSHA2CR0_SPEC;
impl crate::RegisterSpec for FLSHA2CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flsha2cr0::R](R) reader structure"]
impl crate::Readable for FLSHA2CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flsha2cr0::W](W) writer structure"]
impl crate::Writable for FLSHA2CR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLSHA2CR0 to value 0x0001_0000"]
impl crate::Resettable for FLSHA2CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
