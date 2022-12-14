#[doc = "Register `LPGPR[%s]` reader"]
pub struct R(crate::R<LPGPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPGPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPGPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPGPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPGPR[%s]` writer"]
pub struct W(crate::W<LPGPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPGPR_SPEC>;
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
impl From<crate::W<LPGPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPGPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPR` reader - General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
pub type GPR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPR` writer - General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
pub type GPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPGPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    #[inline(always)]
    pub fn gpr(&self) -> GPR_R {
        GPR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn gpr(&mut self) -> GPR_W<0> {
        GPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_LP General Purpose Registers 0 .. 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpgpr](index.html) module"]
pub struct LPGPR_SPEC;
impl crate::RegisterSpec for LPGPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpgpr::R](R) reader structure"]
impl crate::Readable for LPGPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpgpr::W](W) writer structure"]
impl crate::Writable for LPGPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPGPR[%s]
to value 0"]
impl crate::Resettable for LPGPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
