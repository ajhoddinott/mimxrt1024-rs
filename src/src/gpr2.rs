#[doc = "Register `GPR2` reader"]
pub struct R(crate::R<GPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR2` writer"]
pub struct W(crate::W<GPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR2_SPEC>;
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
impl From<crate::W<GPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSISTENT_ARG0` reader - Holds argument of entry function for core0 for waking-up from low power mode"]
pub type PERSISTENT_ARG0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERSISTENT_ARG0` writer - Holds argument of entry function for core0 for waking-up from low power mode"]
pub type PERSISTENT_ARG0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Holds argument of entry function for core0 for waking-up from low power mode"]
    #[inline(always)]
    pub fn persistent_arg0(&self) -> PERSISTENT_ARG0_R {
        PERSISTENT_ARG0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Holds argument of entry function for core0 for waking-up from low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn persistent_arg0(&mut self) -> PERSISTENT_ARG0_W<0> {
        PERSISTENT_ARG0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRC General Purpose Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr2](index.html) module"]
pub struct GPR2_SPEC;
impl crate::RegisterSpec for GPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr2::R](R) reader structure"]
impl crate::Readable for GPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr2::W](W) writer structure"]
impl crate::Writable for GPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR2 to value 0"]
impl crate::Resettable for GPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
