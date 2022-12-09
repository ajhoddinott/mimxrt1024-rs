#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWMEN` reader - EWM enable."]
pub type EWMEN_R = crate::BitReader<EWMEN_A>;
#[doc = "EWM enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWMEN_A {
    #[doc = "0: EWM module is disabled."]
    DISABLE = 0,
    #[doc = "1: EWM module is enabled."]
    ENABLE = 1,
}
impl From<EWMEN_A> for bool {
    #[inline(always)]
    fn from(variant: EWMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl EWMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWMEN_A {
        match self.bits {
            false => EWMEN_A::DISABLE,
            true => EWMEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EWMEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EWMEN_A::ENABLE
    }
}
#[doc = "Field `EWMEN` writer - EWM enable."]
pub type EWMEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, EWMEN_A, O>;
impl<'a, const O: u8> EWMEN_W<'a, O> {
    #[doc = "EWM module is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EWMEN_A::DISABLE)
    }
    #[doc = "EWM module is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EWMEN_A::ENABLE)
    }
}
#[doc = "Field `ASSIN` reader - EWM_in's Assertion State Select."]
pub type ASSIN_R = crate::BitReader<ASSIN_A>;
#[doc = "EWM_in's Assertion State Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSIN_A {
    #[doc = "0: Default assert state of the EWM_in signal."]
    DISABLE = 0,
    #[doc = "1: Inverts the assert state of EWM_in signal."]
    ENABLE = 1,
}
impl From<ASSIN_A> for bool {
    #[inline(always)]
    fn from(variant: ASSIN_A) -> Self {
        variant as u8 != 0
    }
}
impl ASSIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASSIN_A {
        match self.bits {
            false => ASSIN_A::DISABLE,
            true => ASSIN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ASSIN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ASSIN_A::ENABLE
    }
}
#[doc = "Field `ASSIN` writer - EWM_in's Assertion State Select."]
pub type ASSIN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, ASSIN_A, O>;
impl<'a, const O: u8> ASSIN_W<'a, O> {
    #[doc = "Default assert state of the EWM_in signal."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ASSIN_A::DISABLE)
    }
    #[doc = "Inverts the assert state of EWM_in signal."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ASSIN_A::ENABLE)
    }
}
#[doc = "Field `INEN` reader - Input Enable."]
pub type INEN_R = crate::BitReader<INEN_A>;
#[doc = "Input Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INEN_A {
    #[doc = "0: EWM_in port is disabled."]
    DISABLE = 0,
    #[doc = "1: EWM_in port is enabled."]
    ENABLE = 1,
}
impl From<INEN_A> for bool {
    #[inline(always)]
    fn from(variant: INEN_A) -> Self {
        variant as u8 != 0
    }
}
impl INEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INEN_A {
        match self.bits {
            false => INEN_A::DISABLE,
            true => INEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INEN_A::ENABLE
    }
}
#[doc = "Field `INEN` writer - Input Enable."]
pub type INEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, INEN_A, O>;
impl<'a, const O: u8> INEN_W<'a, O> {
    #[doc = "EWM_in port is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INEN_A::DISABLE)
    }
    #[doc = "EWM_in port is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INEN_A::ENABLE)
    }
}
#[doc = "Field `INTEN` reader - Interrupt Enable."]
pub type INTEN_R = crate::BitReader<INTEN_A>;
#[doc = "Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN_A {
    #[doc = "0: Deasserts the interrupt request."]
    ZERO = 0,
    #[doc = "1: Generates an interrupt request, when EWM_OUT_b is asserted."]
    INT_REQ = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::ZERO,
            true => INTEN_A::INT_REQ,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == INTEN_A::ZERO
    }
    #[doc = "Checks if the value of the field is `INT_REQ`"]
    #[inline(always)]
    pub fn is_int_req(&self) -> bool {
        *self == INTEN_A::INT_REQ
    }
}
#[doc = "Field `INTEN` writer - Interrupt Enable."]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, INTEN_A, O>;
impl<'a, const O: u8> INTEN_W<'a, O> {
    #[doc = "Deasserts the interrupt request."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(INTEN_A::ZERO)
    }
    #[doc = "Generates an interrupt request, when EWM_OUT_b is asserted."]
    #[inline(always)]
    pub fn int_req(self) -> &'a mut W {
        self.variant(INTEN_A::INT_REQ)
    }
}
impl R {
    #[doc = "Bit 0 - EWM enable."]
    #[inline(always)]
    pub fn ewmen(&self) -> EWMEN_R {
        EWMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EWM_in's Assertion State Select."]
    #[inline(always)]
    pub fn assin(&self) -> ASSIN_R {
        ASSIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Enable."]
    #[inline(always)]
    pub fn inen(&self) -> INEN_R {
        INEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EWM enable."]
    #[inline(always)]
    #[must_use]
    pub fn ewmen(&mut self) -> EWMEN_W<0> {
        EWMEN_W::new(self)
    }
    #[doc = "Bit 1 - EWM_in's Assertion State Select."]
    #[inline(always)]
    #[must_use]
    pub fn assin(&mut self) -> ASSIN_W<1> {
        ASSIN_W::new(self)
    }
    #[doc = "Bit 2 - Input Enable."]
    #[inline(always)]
    #[must_use]
    pub fn inen(&mut self) -> INEN_W<2> {
        INEN_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<3> {
        INTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
