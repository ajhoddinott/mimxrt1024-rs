#[doc = "Register `FILT2` reader"]
pub struct R(crate::R<FILT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILT2` writer"]
pub struct W(crate::W<FILT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILT2_SPEC>;
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
impl From<crate::W<FILT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILT_PER` reader - Input Filter Sample Period"]
pub type FILT_PER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILT_PER` writer - Input Filter Sample Period"]
pub type FILT_PER_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FILT2_SPEC, u8, u8, 8, O>;
#[doc = "Field `FILT_CNT` reader - Input Filter Sample Count"]
pub type FILT_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILT_CNT` writer - Input Filter Sample Count"]
pub type FILT_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FILT2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:7 - Input Filter Sample Period"]
    #[inline(always)]
    pub fn filt_per(&self) -> FILT_PER_R {
        FILT_PER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Input Filter Sample Count"]
    #[inline(always)]
    pub fn filt_cnt(&self) -> FILT_CNT_R {
        FILT_CNT_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input Filter Sample Period"]
    #[inline(always)]
    #[must_use]
    pub fn filt_per(&mut self) -> FILT_PER_W<0> {
        FILT_PER_W::new(self)
    }
    #[doc = "Bits 8:10 - Input Filter Sample Count"]
    #[inline(always)]
    #[must_use]
    pub fn filt_cnt(&mut self) -> FILT_CNT_W<8> {
        FILT_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Input Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filt2](index.html) module"]
pub struct FILT2_SPEC;
impl crate::RegisterSpec for FILT2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [filt2::R](R) reader structure"]
impl crate::Readable for FILT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filt2::W](W) writer structure"]
impl crate::Writable for FILT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILT2 to value 0"]
impl crate::Resettable for FILT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
