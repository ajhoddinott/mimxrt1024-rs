#[doc = "Register `CV` reader"]
pub struct R(crate::R<CV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CV` writer"]
pub struct W(crate::W<CV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CV_SPEC>;
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
impl From<crate::W<CV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CV1` reader - Compare Value 1"]
pub type CV1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CV1` writer - Compare Value 1"]
pub type CV1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CV_SPEC, u16, u16, 12, O>;
#[doc = "Field `CV2` reader - Compare Value 2"]
pub type CV2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CV2` writer - Compare Value 2"]
pub type CV2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CV_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Compare Value 1"]
    #[inline(always)]
    pub fn cv1(&self) -> CV1_R {
        CV1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Compare Value 2"]
    #[inline(always)]
    pub fn cv2(&self) -> CV2_R {
        CV2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Compare Value 1"]
    #[inline(always)]
    #[must_use]
    pub fn cv1(&mut self) -> CV1_W<0> {
        CV1_W::new(self)
    }
    #[doc = "Bits 16:27 - Compare Value 2"]
    #[inline(always)]
    #[must_use]
    pub fn cv2(&mut self) -> CV2_W<16> {
        CV2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](index.html) module"]
pub struct CV_SPEC;
impl crate::RegisterSpec for CV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cv::R](R) reader structure"]
impl crate::Readable for CV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cv::W](W) writer structure"]
impl crate::Writable for CV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CV to value 0"]
impl crate::Resettable for CV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
