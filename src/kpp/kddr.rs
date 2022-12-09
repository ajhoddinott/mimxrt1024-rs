#[doc = "Register `KDDR` reader"]
pub struct R(crate::R<KDDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KDDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KDDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KDDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KDDR` writer"]
pub struct W(crate::W<KDDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KDDR_SPEC>;
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
impl From<crate::W<KDDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KDDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KRDD` reader - Keypad Row Data Direction"]
pub type KRDD_R = crate::FieldReader<u8, KRDD_A>;
#[doc = "Keypad Row Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KRDD_A {
    #[doc = "0: ROWn pin configured as an input."]
    INPUT = 0,
    #[doc = "1: ROWn pin configured as an output."]
    OUTPUT = 1,
}
impl From<KRDD_A> for u8 {
    #[inline(always)]
    fn from(variant: KRDD_A) -> Self {
        variant as _
    }
}
impl KRDD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KRDD_A> {
        match self.bits {
            0 => Some(KRDD_A::INPUT),
            1 => Some(KRDD_A::OUTPUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == KRDD_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == KRDD_A::OUTPUT
    }
}
#[doc = "Field `KRDD` writer - Keypad Row Data Direction"]
pub type KRDD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, KDDR_SPEC, u8, KRDD_A, 8, O>;
impl<'a, const O: u8> KRDD_W<'a, O> {
    #[doc = "ROWn pin configured as an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(KRDD_A::INPUT)
    }
    #[doc = "ROWn pin configured as an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(KRDD_A::OUTPUT)
    }
}
#[doc = "Field `KCDD` reader - Keypad Column Data Direction Register"]
pub type KCDD_R = crate::FieldReader<u8, KCDD_A>;
#[doc = "Keypad Column Data Direction Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KCDD_A {
    #[doc = "0: COLn pin is configured as an input."]
    INPUT = 0,
    #[doc = "1: COLn pin is configured as an output."]
    OUTPUT = 1,
}
impl From<KCDD_A> for u8 {
    #[inline(always)]
    fn from(variant: KCDD_A) -> Self {
        variant as _
    }
}
impl KCDD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KCDD_A> {
        match self.bits {
            0 => Some(KCDD_A::INPUT),
            1 => Some(KCDD_A::OUTPUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == KCDD_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == KCDD_A::OUTPUT
    }
}
#[doc = "Field `KCDD` writer - Keypad Column Data Direction Register"]
pub type KCDD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, KDDR_SPEC, u8, KCDD_A, 8, O>;
impl<'a, const O: u8> KCDD_W<'a, O> {
    #[doc = "COLn pin is configured as an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(KCDD_A::INPUT)
    }
    #[doc = "COLn pin is configured as an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(KCDD_A::OUTPUT)
    }
}
impl R {
    #[doc = "Bits 0:7 - Keypad Row Data Direction"]
    #[inline(always)]
    pub fn krdd(&self) -> KRDD_R {
        KRDD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Keypad Column Data Direction Register"]
    #[inline(always)]
    pub fn kcdd(&self) -> KCDD_R {
        KCDD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Keypad Row Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn krdd(&mut self) -> KRDD_W<0> {
        KRDD_W::new(self)
    }
    #[doc = "Bits 8:15 - Keypad Column Data Direction Register"]
    #[inline(always)]
    #[must_use]
    pub fn kcdd(&mut self) -> KCDD_W<8> {
        KCDD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Keypad Data Direction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kddr](index.html) module"]
pub struct KDDR_SPEC;
impl crate::RegisterSpec for KDDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [kddr::R](R) reader structure"]
impl crate::Readable for KDDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kddr::W](W) writer structure"]
impl crate::Writable for KDDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KDDR to value 0"]
impl crate::Resettable for KDDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
