#[doc = "Register `BMCR1` reader"]
pub struct R(crate::R<BMCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMCR1` writer"]
pub struct W(crate::W<BMCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMCR1_SPEC>;
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
impl From<crate::W<BMCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WQOS` reader - Weight of QOS"]
pub type WQOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WQOS` writer - Weight of QOS"]
pub type WQOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `WAGE` reader - Weight of AGE"]
pub type WAGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAGE` writer - Weight of AGE"]
pub type WAGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `WPH` reader - Weight of Page Hit"]
pub type WPH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WPH` writer - Weight of Page Hit"]
pub type WPH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `WRWS` reader - Weight of slave hit without Read/Write Switch"]
pub type WRWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRWS` writer - Weight of slave hit without Read/Write Switch"]
pub type WRWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `WBR` reader - Weight of Bank Rotation"]
pub type WBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WBR` writer - Weight of Bank Rotation"]
pub type WBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - Weight of QOS"]
    #[inline(always)]
    pub fn wqos(&self) -> WQOS_R {
        WQOS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight of AGE"]
    #[inline(always)]
    pub fn wage(&self) -> WAGE_R {
        WAGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Weight of Page Hit"]
    #[inline(always)]
    pub fn wph(&self) -> WPH_R {
        WPH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Weight of slave hit without Read/Write Switch"]
    #[inline(always)]
    pub fn wrws(&self) -> WRWS_R {
        WRWS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Weight of Bank Rotation"]
    #[inline(always)]
    pub fn wbr(&self) -> WBR_R {
        WBR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight of QOS"]
    #[inline(always)]
    #[must_use]
    pub fn wqos(&mut self) -> WQOS_W<0> {
        WQOS_W::new(self)
    }
    #[doc = "Bits 4:7 - Weight of AGE"]
    #[inline(always)]
    #[must_use]
    pub fn wage(&mut self) -> WAGE_W<4> {
        WAGE_W::new(self)
    }
    #[doc = "Bits 8:15 - Weight of Page Hit"]
    #[inline(always)]
    #[must_use]
    pub fn wph(&mut self) -> WPH_W<8> {
        WPH_W::new(self)
    }
    #[doc = "Bits 16:23 - Weight of slave hit without Read/Write Switch"]
    #[inline(always)]
    #[must_use]
    pub fn wrws(&mut self) -> WRWS_W<16> {
        WRWS_W::new(self)
    }
    #[doc = "Bits 24:31 - Weight of Bank Rotation"]
    #[inline(always)]
    #[must_use]
    pub fn wbr(&mut self) -> WBR_W<24> {
        WBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus (AXI) Master Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmcr1](index.html) module"]
pub struct BMCR1_SPEC;
impl crate::RegisterSpec for BMCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmcr1::R](R) reader structure"]
impl crate::Readable for BMCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmcr1::W](W) writer structure"]
impl crate::Writable for BMCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMCR1 to value 0"]
impl crate::Resettable for BMCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
