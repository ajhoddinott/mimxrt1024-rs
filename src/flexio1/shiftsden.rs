#[doc = "Register `SHIFTSDEN` reader"]
pub struct R(crate::R<SHIFTSDEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTSDEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTSDEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTSDEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTSDEN` writer"]
pub struct W(crate::W<SHIFTSDEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTSDEN_SPEC>;
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
impl From<crate::W<SHIFTSDEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTSDEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSDE` reader - Shifter Status DMA Enable"]
pub type SSDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSDE` writer - Shifter Status DMA Enable"]
pub type SSDE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHIFTSDEN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Shifter Status DMA Enable"]
    #[inline(always)]
    pub fn ssde(&self) -> SSDE_R {
        SSDE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shifter Status DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssde(&mut self) -> SSDE_W<0> {
        SSDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Status DMA Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftsden](index.html) module"]
pub struct SHIFTSDEN_SPEC;
impl crate::RegisterSpec for SHIFTSDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftsden::R](R) reader structure"]
impl crate::Readable for SHIFTSDEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftsden::W](W) writer structure"]
impl crate::Writable for SHIFTSDEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTSDEN to value 0"]
impl crate::Resettable for SHIFTSDEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
