#[doc = "Register `RCR3` reader"]
pub struct R(crate::R<RCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR3` writer"]
pub struct W(crate::W<RCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR3_SPEC>;
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
impl From<crate::W<RCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDFL` reader - Word Flag Configuration"]
pub type WDFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDFL` writer - Word Flag Configuration"]
pub type WDFL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `RCE` reader - Receive Channel Enable"]
pub type RCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCE` writer - Receive Channel Enable"]
pub type RCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFR` reader - Channel FIFO Reset"]
pub type CFR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFR` writer - Channel FIFO Reset"]
pub type CFR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR3_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline(always)]
    pub fn wdfl(&self) -> WDFL_R {
        WDFL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Receive Channel Enable"]
    #[inline(always)]
    pub fn rce(&self) -> RCE_R {
        RCE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Channel FIFO Reset"]
    #[inline(always)]
    pub fn cfr(&self) -> CFR_R {
        CFR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wdfl(&mut self) -> WDFL_W<0> {
        WDFL_W::new(self)
    }
    #[doc = "Bits 16:19 - Receive Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rce(&mut self) -> RCE_W<16> {
        RCE_W::new(self)
    }
    #[doc = "Bits 24:27 - Channel FIFO Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cfr(&mut self) -> CFR_W<24> {
        CFR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Configuration 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr3](index.html) module"]
pub struct RCR3_SPEC;
impl crate::RegisterSpec for RCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr3::R](R) reader structure"]
impl crate::Readable for RCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr3::W](W) writer structure"]
impl crate::Writable for RCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR3 to value 0"]
impl crate::Resettable for RCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
