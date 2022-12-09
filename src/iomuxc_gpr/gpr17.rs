#[doc = "Register `GPR17` reader"]
pub struct R(crate::R<GPR17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR17` writer"]
pub struct W(crate::W<GPR17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR17_SPEC>;
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
impl From<crate::W<GPR17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLEXRAM_BANK_CFG` reader - FlexRAM bank config value"]
pub type FLEXRAM_BANK_CFG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLEXRAM_BANK_CFG` writer - FlexRAM bank config value"]
pub type FLEXRAM_BANK_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR17_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - FlexRAM bank config value"]
    #[inline(always)]
    pub fn flexram_bank_cfg(&self) -> FLEXRAM_BANK_CFG_R {
        FLEXRAM_BANK_CFG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FlexRAM bank config value"]
    #[inline(always)]
    #[must_use]
    pub fn flexram_bank_cfg(&mut self) -> FLEXRAM_BANK_CFG_W<0> {
        FLEXRAM_BANK_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR17 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr17](index.html) module"]
pub struct GPR17_SPEC;
impl crate::RegisterSpec for GPR17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr17::R](R) reader structure"]
impl crate::Readable for GPR17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr17::W](W) writer structure"]
impl crate::Writable for GPR17_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR17 to value 0"]
impl crate::Resettable for GPR17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
