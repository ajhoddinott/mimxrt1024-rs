#[doc = "Register `MATCH` reader"]
pub struct R(crate::R<MATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATCH` writer"]
pub struct W(crate::W<MATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATCH_SPEC>;
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
impl From<crate::W<MATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MA1` reader - Match Address 1"]
pub type MA1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MA1` writer - Match Address 1"]
pub type MA1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MATCH_SPEC, u16, u16, 10, O>;
#[doc = "Field `MA2` reader - Match Address 2"]
pub type MA2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MA2` writer - Match Address 2"]
pub type MA2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MATCH_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Match Address 1"]
    #[inline(always)]
    pub fn ma1(&self) -> MA1_R {
        MA1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Match Address 2"]
    #[inline(always)]
    pub fn ma2(&self) -> MA2_R {
        MA2_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Match Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn ma1(&mut self) -> MA1_W<0> {
        MA1_W::new(self)
    }
    #[doc = "Bits 16:25 - Match Address 2"]
    #[inline(always)]
    #[must_use]
    pub fn ma2(&mut self) -> MA2_W<16> {
        MA2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Match Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match_](index.html) module"]
pub struct MATCH_SPEC;
impl crate::RegisterSpec for MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [match_::R](R) reader structure"]
impl crate::Readable for MATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [match_::W](W) writer structure"]
impl crate::Writable for MATCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MATCH to value 0"]
impl crate::Resettable for MATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
