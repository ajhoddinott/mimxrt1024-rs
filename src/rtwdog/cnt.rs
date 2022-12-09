#[doc = "Register `CNT` reader"]
pub struct R(crate::R<CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT` writer"]
pub struct W(crate::W<CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_SPEC>;
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
impl From<crate::W<CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTLOW` reader - Low byte of the Watchdog Counter"]
pub type CNTLOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNTLOW` writer - Low byte of the Watchdog Counter"]
pub type CNTLOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT_SPEC, u8, u8, 8, O>;
#[doc = "Field `CNTHIGH` reader - High byte of the Watchdog Counter"]
pub type CNTHIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNTHIGH` writer - High byte of the Watchdog Counter"]
pub type CNTHIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Low byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cntlow(&self) -> CNTLOW_R {
        CNTLOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cnthigh(&self) -> CNTHIGH_R {
        CNTHIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of the Watchdog Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cntlow(&mut self) -> CNTLOW_W<0> {
        CNTLOW_W::new(self)
    }
    #[doc = "Bits 8:15 - High byte of the Watchdog Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cnthigh(&mut self) -> CNTHIGH_W<8> {
        CNTHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](index.html) module"]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt::R](R) reader structure"]
impl crate::Readable for CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt::W](W) writer structure"]
impl crate::Writable for CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
