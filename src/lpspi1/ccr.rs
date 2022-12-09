#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCKDIV` reader - SCK Divider"]
pub type SCKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCKDIV` writer - SCK Divider"]
pub type SCKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DBT` reader - Delay Between Transfers"]
pub type DBT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBT` writer - Delay Between Transfers"]
pub type DBT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PCSSCK` reader - PCS-to-SCK Delay"]
pub type PCSSCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCSSCK` writer - PCS-to-SCK Delay"]
pub type PCSSCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCKPCS` reader - SCK-to-PCS Delay"]
pub type SCKPCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCKPCS` writer - SCK-to-PCS Delay"]
pub type SCKPCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SCK Divider"]
    #[inline(always)]
    pub fn sckdiv(&self) -> SCKDIV_R {
        SCKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay Between Transfers"]
    #[inline(always)]
    pub fn dbt(&self) -> DBT_R {
        DBT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PCS-to-SCK Delay"]
    #[inline(always)]
    pub fn pcssck(&self) -> PCSSCK_R {
        PCSSCK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCK-to-PCS Delay"]
    #[inline(always)]
    pub fn sckpcs(&self) -> SCKPCS_R {
        SCKPCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCK Divider"]
    #[inline(always)]
    #[must_use]
    pub fn sckdiv(&mut self) -> SCKDIV_W<0> {
        SCKDIV_W::new(self)
    }
    #[doc = "Bits 8:15 - Delay Between Transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dbt(&mut self) -> DBT_W<8> {
        DBT_W::new(self)
    }
    #[doc = "Bits 16:23 - PCS-to-SCK Delay"]
    #[inline(always)]
    #[must_use]
    pub fn pcssck(&mut self) -> PCSSCK_W<16> {
        PCSSCK_W::new(self)
    }
    #[doc = "Bits 24:31 - SCK-to-PCS Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sckpcs(&mut self) -> SCKPCS_W<24> {
        SCKPCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
