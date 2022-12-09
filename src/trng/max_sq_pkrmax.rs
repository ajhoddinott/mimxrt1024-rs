#[doc = "Register `PKRMAX` reader"]
pub struct R(crate::R<MAX_SQ_PKRMAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAX_SQ_PKRMAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAX_SQ_PKRMAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAX_SQ_PKRMAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKRMAX` writer"]
pub struct W(crate::W<MAX_SQ_PKRMAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAX_SQ_PKRMAX_SPEC>;
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
impl From<crate::W<MAX_SQ_PKRMAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAX_SQ_PKRMAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKR_MAX` reader - Poker Maximum Limit."]
pub type PKR_MAX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PKR_MAX` writer - Poker Maximum Limit."]
pub type PKR_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAX_SQ_PKRMAX_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Poker Maximum Limit."]
    #[inline(always)]
    pub fn pkr_max(&self) -> PKR_MAX_R {
        PKR_MAX_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Poker Maximum Limit."]
    #[inline(always)]
    #[must_use]
    pub fn pkr_max(&mut self) -> PKR_MAX_W<0> {
        PKR_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Poker Maximum Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [max_sq_pkrmax](index.html) module"]
pub struct MAX_SQ_PKRMAX_SPEC;
impl crate::RegisterSpec for MAX_SQ_PKRMAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [max_sq_pkrmax::R](R) reader structure"]
impl crate::Readable for MAX_SQ_PKRMAX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [max_sq_pkrmax::W](W) writer structure"]
impl crate::Writable for MAX_SQ_PKRMAX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PKRMAX to value 0x6920"]
impl crate::Resettable for MAX_SQ_PKRMAX_SPEC {
    const RESET_VALUE: Self::Ux = 0x6920;
}
