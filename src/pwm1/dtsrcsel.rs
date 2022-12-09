#[doc = "Register `DTSRCSEL` reader"]
pub struct R(crate::R<DTSRCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTSRCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTSRCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTSRCSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTSRCSEL` writer"]
pub struct W(crate::W<DTSRCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTSRCSEL_SPEC>;
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
impl From<crate::W<DTSRCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTSRCSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SM0SEL45` reader - Submodule 0 PWM45 Control Select"]
pub type SM0SEL45_R = crate::FieldReader<u8, SM0SEL45_A>;
#[doc = "Submodule 0 PWM45 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM0SEL45_A {
    #[doc = "0: Generated SM0PWM45 signal is used by the deadtime logic."]
    SM0PWM45 = 0,
    #[doc = "1: Inverted generated SM0PWM45 signal is used by the deadtime logic."]
    INVERTED_SM0PWM45 = 1,
    #[doc = "2: SWCOUT\\[SM0OUT45\\]
is used by the deadtime logic."]
    SM0OUT45 = 2,
    #[doc = "3: PWM0_EXTB signal is used by the deadtime logic."]
    PWM0_EXTB = 3,
}
impl From<SM0SEL45_A> for u8 {
    #[inline(always)]
    fn from(variant: SM0SEL45_A) -> Self {
        variant as _
    }
}
impl SM0SEL45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM0SEL45_A {
        match self.bits {
            0 => SM0SEL45_A::SM0PWM45,
            1 => SM0SEL45_A::INVERTED_SM0PWM45,
            2 => SM0SEL45_A::SM0OUT45,
            3 => SM0SEL45_A::PWM0_EXTB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM0PWM45`"]
    #[inline(always)]
    pub fn is_sm0pwm45(&self) -> bool {
        *self == SM0SEL45_A::SM0PWM45
    }
    #[doc = "Checks if the value of the field is `INVERTED_SM0PWM45`"]
    #[inline(always)]
    pub fn is_inverted_sm0pwm45(&self) -> bool {
        *self == SM0SEL45_A::INVERTED_SM0PWM45
    }
    #[doc = "Checks if the value of the field is `SM0OUT45`"]
    #[inline(always)]
    pub fn is_sm0out45(&self) -> bool {
        *self == SM0SEL45_A::SM0OUT45
    }
    #[doc = "Checks if the value of the field is `PWM0_EXTB`"]
    #[inline(always)]
    pub fn is_pwm0_extb(&self) -> bool {
        *self == SM0SEL45_A::PWM0_EXTB
    }
}
#[doc = "Field `SM0SEL45` writer - Submodule 0 PWM45 Control Select"]
pub type SM0SEL45_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DTSRCSEL_SPEC, u8, SM0SEL45_A, 2, O>;
impl<'a, const O: u8> SM0SEL45_W<'a, O> {
    #[doc = "Generated SM0PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm0pwm45(self) -> &'a mut W {
        self.variant(SM0SEL45_A::SM0PWM45)
    }
    #[doc = "Inverted generated SM0PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn inverted_sm0pwm45(self) -> &'a mut W {
        self.variant(SM0SEL45_A::INVERTED_SM0PWM45)
    }
    #[doc = "SWCOUT\\[SM0OUT45\\]
is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm0out45(self) -> &'a mut W {
        self.variant(SM0SEL45_A::SM0OUT45)
    }
    #[doc = "PWM0_EXTB signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn pwm0_extb(self) -> &'a mut W {
        self.variant(SM0SEL45_A::PWM0_EXTB)
    }
}
#[doc = "Field `SM0SEL23` reader - Submodule 0 PWM23 Control Select"]
pub type SM0SEL23_R = crate::FieldReader<u8, SM0SEL23_A>;
#[doc = "Submodule 0 PWM23 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM0SEL23_A {
    #[doc = "0: Generated SM0PWM23 signal is used by the deadtime logic."]
    SM0PWM23 = 0,
    #[doc = "1: Inverted generated SM0PWM23 signal is used by the deadtime logic."]
    INVERTED_SM0PWM23 = 1,
    #[doc = "2: SWCOUT\\[SM0OUT23\\]
is used by the deadtime logic."]
    SM0OUT23 = 2,
    #[doc = "3: PWM0_EXTA signal is used by the deadtime logic."]
    PWM0_EXTA = 3,
}
impl From<SM0SEL23_A> for u8 {
    #[inline(always)]
    fn from(variant: SM0SEL23_A) -> Self {
        variant as _
    }
}
impl SM0SEL23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM0SEL23_A {
        match self.bits {
            0 => SM0SEL23_A::SM0PWM23,
            1 => SM0SEL23_A::INVERTED_SM0PWM23,
            2 => SM0SEL23_A::SM0OUT23,
            3 => SM0SEL23_A::PWM0_EXTA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM0PWM23`"]
    #[inline(always)]
    pub fn is_sm0pwm23(&self) -> bool {
        *self == SM0SEL23_A::SM0PWM23
    }
    #[doc = "Checks if the value of the field is `INVERTED_SM0PWM23`"]
    #[inline(always)]
    pub fn is_inverted_sm0pwm23(&self) -> bool {
        *self == SM0SEL23_A::INVERTED_SM0PWM23
    }
    #[doc = "Checks if the value of the field is `SM0OUT23`"]
    #[inline(always)]
    pub fn is_sm0out23(&self) -> bool {
        *self == SM0SEL23_A::SM0OUT23
    }
    #[doc = "Checks if the value of the field is `PWM0_EXTA`"]
    #[inline(always)]
    pub fn is_pwm0_exta(&self) -> bool {
        *self == SM0SEL23_A::PWM0_EXTA
    }
}
#[doc = "Field `SM0SEL23` writer - Submodule 0 PWM23 Control Select"]
pub type SM0SEL23_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DTSRCSEL_SPEC, u8, SM0SEL23_A, 2, O>;
impl<'a, const O: u8> SM0SEL23_W<'a, O> {
    #[doc = "Generated SM0PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm0pwm23(self) -> &'a mut W {
        self.variant(SM0SEL23_A::SM0PWM23)
    }
    #[doc = "Inverted generated SM0PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn inverted_sm0pwm23(self) -> &'a mut W {
        self.variant(SM0SEL23_A::INVERTED_SM0PWM23)
    }
    #[doc = "SWCOUT\\[SM0OUT23\\]
is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm0out23(self) -> &'a mut W {
        self.variant(SM0SEL23_A::SM0OUT23)
    }
    #[doc = "PWM0_EXTA signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn pwm0_exta(self) -> &'a mut W {
        self.variant(SM0SEL23_A::PWM0_EXTA)
    }
}
#[doc = "Field `SM1SEL45` reader - Submodule 1 PWM45 Control Select"]
pub type SM1SEL45_R = crate::FieldReader<u8, SM1SEL45_A>;
#[doc = "Submodule 1 PWM45 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM1SEL45_A {
    #[doc = "0: Generated SM1PWM45 signal is used by the deadtime logic."]
    SM1PWM45 = 0,
    #[doc = "1: Inverted generated SM1PWM45 signal is used by the deadtime logic."]
    INVERTED_SM1PWM45 = 1,
    #[doc = "2: SWCOUT\\[SM1OUT45\\]
is used by the deadtime logic."]
    SM1OUT45 = 2,
    #[doc = "3: PWM1_EXTB signal is used by the deadtime logic."]
    PWM1_EXTB = 3,
}
impl From<SM1SEL45_A> for u8 {
    #[inline(always)]
    fn from(variant: SM1SEL45_A) -> Self {
        variant as _
    }
}
impl SM1SEL45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM1SEL45_A {
        match self.bits {
            0 => SM1SEL45_A::SM1PWM45,
            1 => SM1SEL45_A::INVERTED_SM1PWM45,
            2 => SM1SEL45_A::SM1OUT45,
            3 => SM1SEL45_A::PWM1_EXTB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM1PWM45`"]
    #[inline(always)]
    pub fn is_sm1pwm45(&self) -> bool {
        *self == SM1SEL45_A::SM1PWM45
    }
    #[doc = "Checks if the value of the field is `INVERTED_SM1PWM45`"]
    #[inline(always)]
    pub fn is_inverted_sm1pwm45(&self) -> bool {
        *self == SM1SEL45_A::INVERTED_SM1PWM45
    }
    #[doc = "Checks if the value of the field is `SM1OUT45`"]
    #[inline(always)]
    pub fn is_sm1out45(&self) -> bool {
        *self == SM1SEL45_A::SM1OUT45
    }
    #[doc = "Checks if the value of the field is `PWM1_EXTB`"]
    #[inline(always)]
    pub fn is_pwm1_extb(&self) -> bool {
        *self == SM1SEL45_A::PWM1_EXTB
    }
}
#[doc = "Field `SM1SEL45` writer - Submodule 1 PWM45 Control Select"]
pub type SM1SEL45_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DTSRCSEL_SPEC, u8, SM1SEL45_A, 2, O>;
impl<'a, const O: u8> SM1SEL45_W<'a, O> {
    #[doc = "Generated SM1PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm1pwm45(self) -> &'a mut W {
        self.variant(SM1SEL45_A::SM1PWM45)
    }
    #[doc = "Inverted generated SM1PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn inverted_sm1pwm45(self) -> &'a mut W {
        self.variant(SM1SEL45_A::INVERTED_SM1PWM45)
    }
    #[doc = "SWCOUT\\[SM1OUT45\\]
is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm1out45(self) -> &'a mut W {
        self.variant(SM1SEL45_A::SM1OUT45)
    }
    #[doc = "PWM1_EXTB signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn pwm1_extb(self) -> &'a mut W {
        self.variant(SM1SEL45_A::PWM1_EXTB)
    }
}
#[doc = "Field `SM1SEL23` reader - Submodule 1 PWM23 Control Select"]
pub type SM1SEL23_R = crate::FieldReader<u8, SM1SEL23_A>;
#[doc = "Submodule 1 PWM23 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM1SEL23_A {
    #[doc = "0: Generated SM1PWM23 signal is used by the deadtime logic."]
    SM1PWM23 = 0,
    #[doc = "1: Inverted generated SM1PWM23 signal is used by the deadtime logic."]
    INVERTED_SM1PWM23 = 1,
    #[doc = "2: SWCOUT\\[SM1OUT23\\]
is used by the deadtime logic."]
    SM1OUT23 = 2,
    #[doc = "3: PWM1_EXTA signal is used by the deadtime logic."]
    PWM1_EXTA = 3,
}
impl From<SM1SEL23_A> for u8 {
    #[inline(always)]
    fn from(variant: SM1SEL23_A) -> Self {
        variant as _
    }
}
impl SM1SEL23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM1SEL23_A {
        match self.bits {
            0 => SM1SEL23_A::SM1PWM23,
            1 => SM1SEL23_A::INVERTED_SM1PWM23,
            2 => SM1SEL23_A::SM1OUT23,
            3 => SM1SEL23_A::PWM1_EXTA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM1PWM23`"]
    #[inline(always)]
    pub fn is_sm1pwm23(&self) -> bool {
        *self == SM1SEL23_A::SM1PWM23
    }
    #[doc = "Checks if the value of the field is `INVERTED_SM1PWM23`"]
    #[inline(always)]
    pub fn is_inverted_sm1pwm23(&self) -> bool {
        *self == SM1SEL23_A::INVERTED_SM1PWM23
    }
    #[doc = "Checks if the value of the field is `SM1OUT23`"]
    #[inline(always)]
    pub fn is_sm1out23(&self) -> bool {
        *self == SM1SEL23_A::SM1OUT23
    }
    #[doc = "Checks if the value of the field is `PWM1_EXTA`"]
    #[inline(always)]
    pub fn is_pwm1_exta(&self) -> bool {
        *self == SM1SEL23_A::PWM1_EXTA
    }
}
#[doc = "Field `SM1SEL23` writer - Submodule 1 PWM23 Control Select"]
pub type SM1SEL23_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DTSRCSEL_SPEC, u8, SM1SEL23_A, 2, O>;
impl<'a, const O: u8> SM1SEL23_W<'a, O> {
    #[doc = "Generated SM1PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm1pwm23(self) -> &'a mut W {
        self.variant(SM1SEL23_A::SM1PWM23)
    }
    #[doc = "Inverted generated SM1PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn inverted_sm1pwm23(self) -> &'a mut W {
        self.variant(SM1SEL23_A::INVERTED_SM1PWM23)
    }
    #[doc = "SWCOUT\\[SM1OUT23\\]
is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm1out23(self) -> &'a mut W {
        self.variant(SM1SEL23_A::SM1OUT23)
    }
    #[doc = "PWM1_EXTA signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn pwm1_exta(self) -> &'a mut W {
        self.variant(SM1SEL23_A::PWM1_EXTA)
    }
}
#[doc = "Field `SM2SEL45` reader - Submodule 2 PWM45 Control Select"]
pub type SM2SEL45_R = crate::FieldReader<u8, SM2SEL45_A>;
#[doc = "Submodule 2 PWM45 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM2SEL45_A {
    #[doc = "0: Generated SM2PWM45 signal is used by the deadtime logic."]
    SM2PWM45 = 0,
    #[doc = "1: Inverted generated SM2PWM45 signal is used by the deadtime logic."]
    INVERTED_SM2PWM45 = 1,
    #[doc = "2: SWCOUT\\[SM2OUT45\\]
is used by the deadtime logic."]
    SM2OUT45 = 2,
    #[doc = "3: PWM2_EXTB signal is used by the deadtime logic."]
    PWM2_EXTB = 3,
}
impl From<SM2SEL45_A> for u8 {
    #[inline(always)]
    fn from(variant: SM2SEL45_A) -> Self {
        variant as _
    }
}
impl SM2SEL45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM2SEL45_A {
        match self.bits {
            0 => SM2SEL45_A::SM2PWM45,
            1 => SM2SEL45_A::INVERTED_SM2PWM45,
            2 => SM2SEL45_A::SM2OUT45,
            3 => SM2SEL45_A::PWM2_EXTB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM2PWM45`"]
    #[inline(always)]
    pub fn is_sm2pwm45(&self) -> bool {
        *self == SM2SEL45_A::SM2PWM45
    }
    #[doc = "Checks if the value of the field is `INVERTED_SM2PWM45`"]
    #[inline(always)]
    pub fn is_inverted_sm2pwm45(&self) -> bool {
        *self == SM2SEL45_A::INVERTED_SM2PWM45
    }
    #[doc = "Checks if the value of the field is `SM2OUT45`"]
    #[inline(always)]
    pub fn is_sm2out45(&self) -> bool {
        *self == SM2SEL45_A::SM2OUT45
    }
    #[doc = "Checks if the value of the field is `PWM2_EXTB`"]
    #[inline(always)]
    pub fn is_pwm2_extb(&self) -> bool {
        *self == SM2SEL45_A::PWM2_EXTB
    }
}
#[doc = "Field `SM2SEL45` writer - Submodule 2 PWM45 Control Select"]
pub type SM2SEL45_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DTSRCSEL_SPEC, u8, SM2SEL45_A, 2, O>;
impl<'a, const O: u8> SM2SEL45_W<'a, O> {
    #[doc = "Generated SM2PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm2pwm45(self) -> &'a mut W {
        self.variant(SM2SEL45_A::SM2PWM45)
    }
    #[doc = "Inverted generated SM2PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn inverted_sm2pwm45(self) -> &'a mut W {
        self.variant(SM2SEL45_A::INVERTED_SM2PWM45)
    }
    #[doc = "SWCOUT\\[SM2OUT45\\]
is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm2out45(self) -> &'a mut W {
        self.variant(SM2SEL45_A::SM2OUT45)
    }
    #[doc = "PWM2_EXTB signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn pwm2_extb(self) -> &'a mut W {
        self.variant(SM2SEL45_A::PWM2_EXTB)
    }
}
#[doc = "Field `SM2SEL23` reader - Submodule 2 PWM23 Control Select"]
pub type SM2SEL23_R = crate::FieldReader<u8, SM2SEL23_A>;
#[doc = "Submodule 2 PWM23 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM2SEL23_A {
    #[doc = "0: Generated SM2PWM23 signal is used by the deadtime logic."]
    SM2PWM23 = 0,
    #[doc = "1: Inverted generated SM2PWM23 signal is used by the deadtime logic."]
    INVERTED_SM2PWM23 = 1,
    #[doc = "2: SWCOUT\\[SM2OUT23\\]
is used by the deadtime logic."]
    SM2OUT23 = 2,
    #[doc = "3: PWM2_EXTA signal is used by the deadtime logic."]
    PWM2_EXTA = 3,
}
impl From<SM2SEL23_A> for u8 {
    #[inline(always)]
    fn from(variant: SM2SEL23_A) -> Self {
        variant as _
    }
}
impl SM2SEL23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM2SEL23_A {
        match self.bits {
            0 => SM2SEL23_A::SM2PWM23,
            1 => SM2SEL23_A::INVERTED_SM2PWM23,
            2 => SM2SEL23_A::SM2OUT23,
            3 => SM2SEL23_A::PWM2_EXTA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM2PWM23`"]
    #[inline(always)]
    pub fn is_sm2pwm23(&self) -> bool {
        *self == SM2SEL23_A::SM2PWM23
    }
    #[doc = "Checks if the value of the field is `INVERTED_SM2PWM23`"]
    #[inline(always)]
    pub fn is_inverted_sm2pwm23(&self) -> bool {
        *self == SM2SEL23_A::INVERTED_SM2PWM23
    }
    #[doc = "Checks if the value of the field is `SM2OUT23`"]
    #[inline(always)]
    pub fn is_sm2out23(&self) -> bool {
        *self == SM2SEL23_A::SM2OUT23
    }
    #[doc = "Checks if the value of the field is `PWM2_EXTA`"]
    #[inline(always)]
    pub fn is_pwm2_exta(&self) -> bool {
        *self == SM2SEL23_A::PWM2_EXTA
    }
}
#[doc = "Field `SM2SEL23` writer - Submodule 2 PWM23 Control Select"]
pub type SM2SEL23_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DTSRCSEL_SPEC, u8, SM2SEL23_A, 2, O>;
impl<'a, const O: u8> SM2SEL23_W<'a, O> {
    #[doc = "Generated SM2PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm2pwm23(self) -> &'a mut W {
        self.variant(SM2SEL23_A::SM2PWM23)
    }
    #[doc = "Inverted generated SM2PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn inverted_sm2pwm23(self) -> &'a mut W {
        self.variant(SM2SEL23_A::INVERTED_SM2PWM23)
    }
    #[doc = "SWCOUT\\[SM2OUT23\\]
is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm2out23(self) -> &'a mut W {
        self.variant(SM2SEL23_A::SM2OUT23)
    }
    #[doc = "PWM2_EXTA signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn pwm2_exta(self) -> &'a mut W {
        self.variant(SM2SEL23_A::PWM2_EXTA)
    }
}
#[doc = "Field `SM3SEL45` reader - Submodule 3 PWM45 Control Select"]
pub type SM3SEL45_R = crate::FieldReader<u8, SM3SEL45_A>;
#[doc = "Submodule 3 PWM45 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM3SEL45_A {
    #[doc = "0: Generated SM3PWM45 signal is used by the deadtime logic."]
    SM3PWM45 = 0,
    #[doc = "1: Inverted generated SM3PWM45 signal is used by the deadtime logic."]
    INVERTED_SM3PWM45 = 1,
    #[doc = "2: SWCOUT\\[SM3OUT45\\]
is used by the deadtime logic."]
    SM3OUT45 = 2,
    #[doc = "3: PWM3_EXTB signal is used by the deadtime logic."]
    PWM3_EXTB = 3,
}
impl From<SM3SEL45_A> for u8 {
    #[inline(always)]
    fn from(variant: SM3SEL45_A) -> Self {
        variant as _
    }
}
impl SM3SEL45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM3SEL45_A {
        match self.bits {
            0 => SM3SEL45_A::SM3PWM45,
            1 => SM3SEL45_A::INVERTED_SM3PWM45,
            2 => SM3SEL45_A::SM3OUT45,
            3 => SM3SEL45_A::PWM3_EXTB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM3PWM45`"]
    #[inline(always)]
    pub fn is_sm3pwm45(&self) -> bool {
        *self == SM3SEL45_A::SM3PWM45
    }
    #[doc = "Checks if the value of the field is `INVERTED_SM3PWM45`"]
    #[inline(always)]
    pub fn is_inverted_sm3pwm45(&self) -> bool {
        *self == SM3SEL45_A::INVERTED_SM3PWM45
    }
    #[doc = "Checks if the value of the field is `SM3OUT45`"]
    #[inline(always)]
    pub fn is_sm3out45(&self) -> bool {
        *self == SM3SEL45_A::SM3OUT45
    }
    #[doc = "Checks if the value of the field is `PWM3_EXTB`"]
    #[inline(always)]
    pub fn is_pwm3_extb(&self) -> bool {
        *self == SM3SEL45_A::PWM3_EXTB
    }
}
#[doc = "Field `SM3SEL45` writer - Submodule 3 PWM45 Control Select"]
pub type SM3SEL45_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DTSRCSEL_SPEC, u8, SM3SEL45_A, 2, O>;
impl<'a, const O: u8> SM3SEL45_W<'a, O> {
    #[doc = "Generated SM3PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm3pwm45(self) -> &'a mut W {
        self.variant(SM3SEL45_A::SM3PWM45)
    }
    #[doc = "Inverted generated SM3PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn inverted_sm3pwm45(self) -> &'a mut W {
        self.variant(SM3SEL45_A::INVERTED_SM3PWM45)
    }
    #[doc = "SWCOUT\\[SM3OUT45\\]
is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm3out45(self) -> &'a mut W {
        self.variant(SM3SEL45_A::SM3OUT45)
    }
    #[doc = "PWM3_EXTB signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn pwm3_extb(self) -> &'a mut W {
        self.variant(SM3SEL45_A::PWM3_EXTB)
    }
}
#[doc = "Field `SM3SEL23` reader - Submodule 3 PWM23 Control Select"]
pub type SM3SEL23_R = crate::FieldReader<u8, SM3SEL23_A>;
#[doc = "Submodule 3 PWM23 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM3SEL23_A {
    #[doc = "0: Generated SM3PWM23 signal is used by the deadtime logic."]
    SM3PWM23 = 0,
    #[doc = "1: Inverted generated SM3PWM23 signal is used by the deadtime logic."]
    INVERTED_SM3PWM23 = 1,
    #[doc = "2: SWCOUT\\[SM3OUT23\\]
is used by the deadtime logic."]
    SM3OUT23 = 2,
    #[doc = "3: PWM3_EXTA signal is used by the deadtime logic."]
    PWM3_EXTA = 3,
}
impl From<SM3SEL23_A> for u8 {
    #[inline(always)]
    fn from(variant: SM3SEL23_A) -> Self {
        variant as _
    }
}
impl SM3SEL23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM3SEL23_A {
        match self.bits {
            0 => SM3SEL23_A::SM3PWM23,
            1 => SM3SEL23_A::INVERTED_SM3PWM23,
            2 => SM3SEL23_A::SM3OUT23,
            3 => SM3SEL23_A::PWM3_EXTA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM3PWM23`"]
    #[inline(always)]
    pub fn is_sm3pwm23(&self) -> bool {
        *self == SM3SEL23_A::SM3PWM23
    }
    #[doc = "Checks if the value of the field is `INVERTED_SM3PWM23`"]
    #[inline(always)]
    pub fn is_inverted_sm3pwm23(&self) -> bool {
        *self == SM3SEL23_A::INVERTED_SM3PWM23
    }
    #[doc = "Checks if the value of the field is `SM3OUT23`"]
    #[inline(always)]
    pub fn is_sm3out23(&self) -> bool {
        *self == SM3SEL23_A::SM3OUT23
    }
    #[doc = "Checks if the value of the field is `PWM3_EXTA`"]
    #[inline(always)]
    pub fn is_pwm3_exta(&self) -> bool {
        *self == SM3SEL23_A::PWM3_EXTA
    }
}
#[doc = "Field `SM3SEL23` writer - Submodule 3 PWM23 Control Select"]
pub type SM3SEL23_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DTSRCSEL_SPEC, u8, SM3SEL23_A, 2, O>;
impl<'a, const O: u8> SM3SEL23_W<'a, O> {
    #[doc = "Generated SM3PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm3pwm23(self) -> &'a mut W {
        self.variant(SM3SEL23_A::SM3PWM23)
    }
    #[doc = "Inverted generated SM3PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn inverted_sm3pwm23(self) -> &'a mut W {
        self.variant(SM3SEL23_A::INVERTED_SM3PWM23)
    }
    #[doc = "SWCOUT\\[SM3OUT23\\]
is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm3out23(self) -> &'a mut W {
        self.variant(SM3SEL23_A::SM3OUT23)
    }
    #[doc = "PWM3_EXTA signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn pwm3_exta(self) -> &'a mut W {
        self.variant(SM3SEL23_A::PWM3_EXTA)
    }
}
impl R {
    #[doc = "Bits 0:1 - Submodule 0 PWM45 Control Select"]
    #[inline(always)]
    pub fn sm0sel45(&self) -> SM0SEL45_R {
        SM0SEL45_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Submodule 0 PWM23 Control Select"]
    #[inline(always)]
    pub fn sm0sel23(&self) -> SM0SEL23_R {
        SM0SEL23_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Submodule 1 PWM45 Control Select"]
    #[inline(always)]
    pub fn sm1sel45(&self) -> SM1SEL45_R {
        SM1SEL45_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Submodule 1 PWM23 Control Select"]
    #[inline(always)]
    pub fn sm1sel23(&self) -> SM1SEL23_R {
        SM1SEL23_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Submodule 2 PWM45 Control Select"]
    #[inline(always)]
    pub fn sm2sel45(&self) -> SM2SEL45_R {
        SM2SEL45_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Submodule 2 PWM23 Control Select"]
    #[inline(always)]
    pub fn sm2sel23(&self) -> SM2SEL23_R {
        SM2SEL23_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Submodule 3 PWM45 Control Select"]
    #[inline(always)]
    pub fn sm3sel45(&self) -> SM3SEL45_R {
        SM3SEL45_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Submodule 3 PWM23 Control Select"]
    #[inline(always)]
    pub fn sm3sel23(&self) -> SM3SEL23_R {
        SM3SEL23_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Submodule 0 PWM45 Control Select"]
    #[inline(always)]
    #[must_use]
    pub fn sm0sel45(&mut self) -> SM0SEL45_W<0> {
        SM0SEL45_W::new(self)
    }
    #[doc = "Bits 2:3 - Submodule 0 PWM23 Control Select"]
    #[inline(always)]
    #[must_use]
    pub fn sm0sel23(&mut self) -> SM0SEL23_W<2> {
        SM0SEL23_W::new(self)
    }
    #[doc = "Bits 4:5 - Submodule 1 PWM45 Control Select"]
    #[inline(always)]
    #[must_use]
    pub fn sm1sel45(&mut self) -> SM1SEL45_W<4> {
        SM1SEL45_W::new(self)
    }
    #[doc = "Bits 6:7 - Submodule 1 PWM23 Control Select"]
    #[inline(always)]
    #[must_use]
    pub fn sm1sel23(&mut self) -> SM1SEL23_W<6> {
        SM1SEL23_W::new(self)
    }
    #[doc = "Bits 8:9 - Submodule 2 PWM45 Control Select"]
    #[inline(always)]
    #[must_use]
    pub fn sm2sel45(&mut self) -> SM2SEL45_W<8> {
        SM2SEL45_W::new(self)
    }
    #[doc = "Bits 10:11 - Submodule 2 PWM23 Control Select"]
    #[inline(always)]
    #[must_use]
    pub fn sm2sel23(&mut self) -> SM2SEL23_W<10> {
        SM2SEL23_W::new(self)
    }
    #[doc = "Bits 12:13 - Submodule 3 PWM45 Control Select"]
    #[inline(always)]
    #[must_use]
    pub fn sm3sel45(&mut self) -> SM3SEL45_W<12> {
        SM3SEL45_W::new(self)
    }
    #[doc = "Bits 14:15 - Submodule 3 PWM23 Control Select"]
    #[inline(always)]
    #[must_use]
    pub fn sm3sel23(&mut self) -> SM3SEL23_W<14> {
        SM3SEL23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Source Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtsrcsel](index.html) module"]
pub struct DTSRCSEL_SPEC;
impl crate::RegisterSpec for DTSRCSEL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dtsrcsel::R](R) reader structure"]
impl crate::Readable for DTSRCSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtsrcsel::W](W) writer structure"]
impl crate::Writable for DTSRCSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTSRCSEL to value 0"]
impl crate::Resettable for DTSRCSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
