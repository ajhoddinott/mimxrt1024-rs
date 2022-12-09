#[doc = "Register `PERIODICLISTBASE` reader"]
pub struct R(crate::R<DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIODICLISTBASE` writer"]
pub struct W(crate::W<DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC>;
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
impl From<crate::W<DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEADR` reader - Base Address (Low)"]
pub type BASEADR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASEADR` writer - Base Address (Low)"]
pub type BASEADR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 12:31 - Base Address (Low)"]
    #[inline(always)]
    pub fn baseadr(&self) -> BASEADR_R {
        BASEADR_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - Base Address (Low)"]
    #[inline(always)]
    #[must_use]
    pub fn baseadr(&mut self) -> BASEADR_W<12> {
        BASEADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame List Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceaddr_periodiclistbase_periodiclistbase](index.html) module"]
pub struct DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC;
impl crate::RegisterSpec for DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deviceaddr_periodiclistbase_periodiclistbase::R](R) reader structure"]
impl crate::Readable for DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [deviceaddr_periodiclistbase_periodiclistbase::W](W) writer structure"]
impl crate::Writable for DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIODICLISTBASE to value 0"]
impl crate::Resettable for DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
