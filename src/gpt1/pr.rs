#[doc = "Register `PR` reader"]
pub struct R(crate::R<PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR` writer"]
pub struct W(crate::W<PR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_SPEC>;
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
impl From<crate::W<PR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER` reader - Prescaler bits"]
pub type PRESCALER_R = crate::FieldReader<u16, PRESCALER_A>;
#[doc = "Prescaler bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PRESCALER_A {
    #[doc = "0: Divide by 1"]
    PRESCALER_0 = 0,
    #[doc = "1: Divide by 2"]
    PRESCALER_1 = 1,
    #[doc = "4095: Divide by 4096"]
    PRESCALER_4095 = 4095,
}
impl From<PRESCALER_A> for u16 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
impl PRESCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCALER_A> {
        match self.bits {
            0 => Some(PRESCALER_A::PRESCALER_0),
            1 => Some(PRESCALER_A::PRESCALER_1),
            4095 => Some(PRESCALER_A::PRESCALER_4095),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALER_0`"]
    #[inline(always)]
    pub fn is_prescaler_0(&self) -> bool {
        *self == PRESCALER_A::PRESCALER_0
    }
    #[doc = "Checks if the value of the field is `PRESCALER_1`"]
    #[inline(always)]
    pub fn is_prescaler_1(&self) -> bool {
        *self == PRESCALER_A::PRESCALER_1
    }
    #[doc = "Checks if the value of the field is `PRESCALER_4095`"]
    #[inline(always)]
    pub fn is_prescaler_4095(&self) -> bool {
        *self == PRESCALER_A::PRESCALER_4095
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler bits"]
pub type PRESCALER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PR_SPEC, u16, PRESCALER_A, 12, O>;
impl<'a, const O: u8> PRESCALER_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn prescaler_0(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn prescaler_1(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_1)
    }
    #[doc = "Divide by 4096"]
    #[inline(always)]
    pub fn prescaler_4095(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_4095)
    }
}
#[doc = "Field `PRESCALER24M` reader - Prescaler bits"]
pub type PRESCALER24M_R = crate::FieldReader<u8, PRESCALER24M_A>;
#[doc = "Prescaler bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALER24M_A {
    #[doc = "0: Divide by 1"]
    PRESCALER24M_0 = 0,
    #[doc = "1: Divide by 2"]
    PRESCALER24M_1 = 1,
    #[doc = "15: Divide by 16"]
    PRESCALER24M_15 = 15,
}
impl From<PRESCALER24M_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER24M_A) -> Self {
        variant as _
    }
}
impl PRESCALER24M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCALER24M_A> {
        match self.bits {
            0 => Some(PRESCALER24M_A::PRESCALER24M_0),
            1 => Some(PRESCALER24M_A::PRESCALER24M_1),
            15 => Some(PRESCALER24M_A::PRESCALER24M_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALER24M_0`"]
    #[inline(always)]
    pub fn is_prescaler24m_0(&self) -> bool {
        *self == PRESCALER24M_A::PRESCALER24M_0
    }
    #[doc = "Checks if the value of the field is `PRESCALER24M_1`"]
    #[inline(always)]
    pub fn is_prescaler24m_1(&self) -> bool {
        *self == PRESCALER24M_A::PRESCALER24M_1
    }
    #[doc = "Checks if the value of the field is `PRESCALER24M_15`"]
    #[inline(always)]
    pub fn is_prescaler24m_15(&self) -> bool {
        *self == PRESCALER24M_A::PRESCALER24M_15
    }
}
#[doc = "Field `PRESCALER24M` writer - Prescaler bits"]
pub type PRESCALER24M_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PR_SPEC, u8, PRESCALER24M_A, 4, O>;
impl<'a, const O: u8> PRESCALER24M_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn prescaler24m_0(self) -> &'a mut W {
        self.variant(PRESCALER24M_A::PRESCALER24M_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn prescaler24m_1(self) -> &'a mut W {
        self.variant(PRESCALER24M_A::PRESCALER24M_1)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn prescaler24m_15(self) -> &'a mut W {
        self.variant(PRESCALER24M_A::PRESCALER24M_15)
    }
}
impl R {
    #[doc = "Bits 0:11 - Prescaler bits"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Prescaler bits"]
    #[inline(always)]
    pub fn prescaler24m(&self) -> PRESCALER24M_R {
        PRESCALER24M_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Prescaler bits"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<0> {
        PRESCALER_W::new(self)
    }
    #[doc = "Bits 12:15 - Prescaler bits"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler24m(&mut self) -> PRESCALER24M_W<12> {
        PRESCALER24M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPT Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](index.html) module"]
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr::R](R) reader structure"]
impl crate::Readable for PR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr::W](W) writer structure"]
impl crate::Writable for PR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
