#[doc = "Register `SDRAMCR1` reader"]
pub struct R(crate::R<SDRAMCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRAMCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRAMCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRAMCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRAMCR1` writer"]
pub struct W(crate::W<SDRAMCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRAMCR1_SPEC>;
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
impl From<crate::W<SDRAMCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRAMCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE2ACT` reader - PRECHARGE to ACTIVE/REFRESH command wait time"]
pub type PRE2ACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE2ACT` writer - PRECHARGE to ACTIVE/REFRESH command wait time"]
pub type PRE2ACT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRAMCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `ACT2RW` reader - ACTIVE to READ/WRITE delay"]
pub type ACT2RW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT2RW` writer - ACTIVE to READ/WRITE delay"]
pub type ACT2RW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRAMCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RFRC` reader - REFRESH recovery time"]
pub type RFRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFRC` writer - REFRESH recovery time"]
pub type RFRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRAMCR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `WRC` reader - WRITE recovery time"]
pub type WRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRC` writer - WRITE recovery time"]
pub type WRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRAMCR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `CKEOFF` reader - CKE off minimum time"]
pub type CKEOFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKEOFF` writer - CKE off minimum time"]
pub type CKEOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRAMCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `ACT2PRE` reader - ACTIVE to PRECHARGE minimum time"]
pub type ACT2PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT2PRE` writer - ACTIVE to PRECHARGE minimum time"]
pub type ACT2PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRAMCR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PRECHARGE to ACTIVE/REFRESH command wait time"]
    #[inline(always)]
    pub fn pre2act(&self) -> PRE2ACT_R {
        PRE2ACT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ACTIVE to READ/WRITE delay"]
    #[inline(always)]
    pub fn act2rw(&self) -> ACT2RW_R {
        ACT2RW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - REFRESH recovery time"]
    #[inline(always)]
    pub fn rfrc(&self) -> RFRC_R {
        RFRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - WRITE recovery time"]
    #[inline(always)]
    pub fn wrc(&self) -> WRC_R {
        WRC_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - CKE off minimum time"]
    #[inline(always)]
    pub fn ckeoff(&self) -> CKEOFF_R {
        CKEOFF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - ACTIVE to PRECHARGE minimum time"]
    #[inline(always)]
    pub fn act2pre(&self) -> ACT2PRE_R {
        ACT2PRE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PRECHARGE to ACTIVE/REFRESH command wait time"]
    #[inline(always)]
    #[must_use]
    pub fn pre2act(&mut self) -> PRE2ACT_W<0> {
        PRE2ACT_W::new(self)
    }
    #[doc = "Bits 4:7 - ACTIVE to READ/WRITE delay"]
    #[inline(always)]
    #[must_use]
    pub fn act2rw(&mut self) -> ACT2RW_W<4> {
        ACT2RW_W::new(self)
    }
    #[doc = "Bits 8:12 - REFRESH recovery time"]
    #[inline(always)]
    #[must_use]
    pub fn rfrc(&mut self) -> RFRC_W<8> {
        RFRC_W::new(self)
    }
    #[doc = "Bits 13:15 - WRITE recovery time"]
    #[inline(always)]
    #[must_use]
    pub fn wrc(&mut self) -> WRC_W<13> {
        WRC_W::new(self)
    }
    #[doc = "Bits 16:19 - CKE off minimum time"]
    #[inline(always)]
    #[must_use]
    pub fn ckeoff(&mut self) -> CKEOFF_W<16> {
        CKEOFF_W::new(self)
    }
    #[doc = "Bits 20:23 - ACTIVE to PRECHARGE minimum time"]
    #[inline(always)]
    #[must_use]
    pub fn act2pre(&mut self) -> ACT2PRE_W<20> {
        ACT2PRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramcr1](index.html) module"]
pub struct SDRAMCR1_SPEC;
impl crate::RegisterSpec for SDRAMCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdramcr1::R](R) reader structure"]
impl crate::Readable for SDRAMCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdramcr1::W](W) writer structure"]
impl crate::Writable for SDRAMCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDRAMCR1 to value 0x0099_4934"]
impl crate::Resettable for SDRAMCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0099_4934;
}
