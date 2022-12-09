#[doc = "Register `GLOBAL` reader"]
pub struct R(crate::R<GLOBAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBAL` writer"]
pub struct W(crate::W<GLOBAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBAL_SPEC>;
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
impl From<crate::W<GLOBAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST` reader - Software Reset"]
pub type RST_R = crate::BitReader<RST_A>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_A {
    #[doc = "0: Module is not reset."]
    NO_EFFECT = 0,
    #[doc = "1: Module is reset."]
    RESET = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::NO_EFFECT,
            true => RST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RST_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RST_A::RESET
    }
}
#[doc = "Field `RST` writer - Software Reset"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLOBAL_SPEC, RST_A, O>;
impl<'a, const O: u8> RST_W<'a, O> {
    #[doc = "Module is not reset."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RST_A::NO_EFFECT)
    }
    #[doc = "Module is reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RST_A::RESET)
    }
}
impl R {
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<1> {
        RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Global Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [global](index.html) module"]
pub struct GLOBAL_SPEC;
impl crate::RegisterSpec for GLOBAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [global::R](R) reader structure"]
impl crate::Readable for GLOBAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [global::W](W) writer structure"]
impl crate::Writable for GLOBAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLOBAL to value 0"]
impl crate::Resettable for GLOBAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
