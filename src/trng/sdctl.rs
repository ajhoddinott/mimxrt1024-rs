#[doc = "Register `SDCTL` reader"]
pub struct R(crate::R<SDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDCTL` writer"]
pub struct W(crate::W<SDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDCTL_SPEC>;
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
impl From<crate::W<SDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMP_SIZE` reader - Sample Size"]
pub type SAMP_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAMP_SIZE` writer - Sample Size"]
pub type SAMP_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCTL_SPEC, u16, u16, 16, O>;
#[doc = "Field `ENT_DLY` reader - Entropy Delay"]
pub type ENT_DLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ENT_DLY` writer - Entropy Delay"]
pub type ENT_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCTL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Sample Size"]
    #[inline(always)]
    pub fn samp_size(&self) -> SAMP_SIZE_R {
        SAMP_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Entropy Delay"]
    #[inline(always)]
    pub fn ent_dly(&self) -> ENT_DLY_R {
        ENT_DLY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sample Size"]
    #[inline(always)]
    #[must_use]
    pub fn samp_size(&mut self) -> SAMP_SIZE_W<0> {
        SAMP_SIZE_W::new(self)
    }
    #[doc = "Bits 16:31 - Entropy Delay"]
    #[inline(always)]
    #[must_use]
    pub fn ent_dly(&mut self) -> ENT_DLY_W<16> {
        ENT_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Seed Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdctl](index.html) module"]
pub struct SDCTL_SPEC;
impl crate::RegisterSpec for SDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdctl::R](R) reader structure"]
impl crate::Readable for SDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdctl::W](W) writer structure"]
impl crate::Writable for SDCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDCTL to value 0x0c80_09c4"]
impl crate::Resettable for SDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c80_09c4;
}
