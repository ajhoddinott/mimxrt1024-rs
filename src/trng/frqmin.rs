#[doc = "Register `FRQMIN` reader"]
pub struct R(crate::R<FRQMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRQMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRQMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRQMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRQMIN` writer"]
pub struct W(crate::W<FRQMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRQMIN_SPEC>;
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
impl From<crate::W<FRQMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRQMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRQ_MIN` reader - Frequency Count Minimum Limit"]
pub type FRQ_MIN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRQ_MIN` writer - Frequency Count Minimum Limit"]
pub type FRQ_MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRQMIN_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - Frequency Count Minimum Limit"]
    #[inline(always)]
    pub fn frq_min(&self) -> FRQ_MIN_R {
        FRQ_MIN_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Frequency Count Minimum Limit"]
    #[inline(always)]
    #[must_use]
    pub fn frq_min(&mut self) -> FRQ_MIN_W<0> {
        FRQ_MIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency Count Minimum Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frqmin](index.html) module"]
pub struct FRQMIN_SPEC;
impl crate::RegisterSpec for FRQMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frqmin::R](R) reader structure"]
impl crate::Readable for FRQMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frqmin::W](W) writer structure"]
impl crate::Writable for FRQMIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRQMIN to value 0x0640"]
impl crate::Resettable for FRQMIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0640;
}
