#[doc = "Register `GPR10` reader"]
pub struct R(crate::R<GPR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR10` writer"]
pub struct W(crate::W<GPR10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR10_SPEC>;
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
impl From<crate::W<GPR10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSIST_REDUNDANT_BOOT` reader - This field identifies which image must be used - 0/1/2/3"]
pub type PERSIST_REDUNDANT_BOOT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERSIST_REDUNDANT_BOOT` writer - This field identifies which image must be used - 0/1/2/3"]
pub type PERSIST_REDUNDANT_BOOT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR10_SPEC, u8, u8, 2, O>;
#[doc = "Field `PERSIST_SECONDARY_BOOT` reader - This bit identifies which image must be used - primary and secondary"]
pub type PERSIST_SECONDARY_BOOT_R = crate::BitReader<bool>;
#[doc = "Field `PERSIST_SECONDARY_BOOT` writer - This bit identifies which image must be used - primary and secondary"]
pub type PERSIST_SECONDARY_BOOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR10_SPEC, bool, O>;
impl R {
    #[doc = "Bits 26:27 - This field identifies which image must be used - 0/1/2/3"]
    #[inline(always)]
    pub fn persist_redundant_boot(&self) -> PERSIST_REDUNDANT_BOOT_R {
        PERSIST_REDUNDANT_BOOT_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 30 - This bit identifies which image must be used - primary and secondary"]
    #[inline(always)]
    pub fn persist_secondary_boot(&self) -> PERSIST_SECONDARY_BOOT_R {
        PERSIST_SECONDARY_BOOT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 26:27 - This field identifies which image must be used - 0/1/2/3"]
    #[inline(always)]
    #[must_use]
    pub fn persist_redundant_boot(&mut self) -> PERSIST_REDUNDANT_BOOT_W<26> {
        PERSIST_REDUNDANT_BOOT_W::new(self)
    }
    #[doc = "Bit 30 - This bit identifies which image must be used - primary and secondary"]
    #[inline(always)]
    #[must_use]
    pub fn persist_secondary_boot(&mut self) -> PERSIST_SECONDARY_BOOT_W<30> {
        PERSIST_SECONDARY_BOOT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRC General Purpose Register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr10](index.html) module"]
pub struct GPR10_SPEC;
impl crate::RegisterSpec for GPR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr10::R](R) reader structure"]
impl crate::Readable for GPR10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr10::W](W) writer structure"]
impl crate::Writable for GPR10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR10 to value 0"]
impl crate::Resettable for GPR10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
