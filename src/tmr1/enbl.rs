#[doc = "Register `ENBL` reader"]
pub struct R(crate::R<ENBL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENBL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENBL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENBL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENBL` writer"]
pub struct W(crate::W<ENBL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENBL_SPEC>;
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
impl From<crate::W<ENBL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENBL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENBL` reader - Timer Channel Enable"]
pub type ENBL_R = crate::FieldReader<u8, ENBL_A>;
#[doc = "Timer Channel Enable\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENBL_A {
    #[doc = "0: Timer channel is disabled."]
    DISABLE = 0,
    #[doc = "1: Timer channel is enabled. (default)"]
    ENABLE = 1,
}
impl From<ENBL_A> for u8 {
    #[inline(always)]
    fn from(variant: ENBL_A) -> Self {
        variant as _
    }
}
impl ENBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENBL_A> {
        match self.bits {
            0 => Some(ENBL_A::DISABLE),
            1 => Some(ENBL_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENBL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENBL_A::ENABLE
    }
}
#[doc = "Field `ENBL` writer - Timer Channel Enable"]
pub type ENBL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ENBL_SPEC, u8, ENBL_A, 4, O>;
impl<'a, const O: u8> ENBL_W<'a, O> {
    #[doc = "Timer channel is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENBL_A::DISABLE)
    }
    #[doc = "Timer channel is enabled. (default)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENBL_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - Timer Channel Enable"]
    #[inline(always)]
    pub fn enbl(&self) -> ENBL_R {
        ENBL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbl(&mut self) -> ENBL_W<0> {
        ENBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enbl](index.html) module"]
pub struct ENBL_SPEC;
impl crate::RegisterSpec for ENBL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [enbl::R](R) reader structure"]
impl crate::Readable for ENBL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enbl::W](W) writer structure"]
impl crate::Writable for ENBL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENBL to value 0x0f"]
impl crate::Resettable for ENBL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
