#[doc = "Register `ADMA_SYS_ADDR` reader"]
pub struct R(crate::R<ADMA_SYS_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADMA_SYS_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADMA_SYS_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADMA_SYS_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADMA_SYS_ADDR` writer"]
pub struct W(crate::W<ADMA_SYS_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADMA_SYS_ADDR_SPEC>;
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
impl From<crate::W<ADMA_SYS_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADMA_SYS_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADS_ADDR` reader - ADMA system address"]
pub type ADS_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADS_ADDR` writer - ADMA system address"]
pub type ADS_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADMA_SYS_ADDR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - ADMA system address"]
    #[inline(always)]
    pub fn ads_addr(&self) -> ADS_ADDR_R {
        ADS_ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - ADMA system address"]
    #[inline(always)]
    #[must_use]
    pub fn ads_addr(&mut self) -> ADS_ADDR_W<2> {
        ADS_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADMA System Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adma_sys_addr](index.html) module"]
pub struct ADMA_SYS_ADDR_SPEC;
impl crate::RegisterSpec for ADMA_SYS_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adma_sys_addr::R](R) reader structure"]
impl crate::Readable for ADMA_SYS_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adma_sys_addr::W](W) writer structure"]
impl crate::Writable for ADMA_SYS_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADMA_SYS_ADDR to value 0"]
impl crate::Resettable for ADMA_SYS_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
