#[doc = "Register `SWCOUT` reader"]
pub struct R(crate::R<SWCOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWCOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWCOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWCOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWCOUT` writer"]
pub struct W(crate::W<SWCOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWCOUT_SPEC>;
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
impl From<crate::W<SWCOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWCOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SM0OUT45` reader - Submodule 0 Software Controlled Output 45"]
pub type SM0OUT45_R = crate::BitReader<SM0OUT45_A>;
#[doc = "Submodule 0 Software Controlled Output 45\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM0OUT45_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    LOGIC_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    LOGIC_1 = 1,
}
impl From<SM0OUT45_A> for bool {
    #[inline(always)]
    fn from(variant: SM0OUT45_A) -> Self {
        variant as u8 != 0
    }
}
impl SM0OUT45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM0OUT45_A {
        match self.bits {
            false => SM0OUT45_A::LOGIC_0,
            true => SM0OUT45_A::LOGIC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOGIC_0`"]
    #[inline(always)]
    pub fn is_logic_0(&self) -> bool {
        *self == SM0OUT45_A::LOGIC_0
    }
    #[doc = "Checks if the value of the field is `LOGIC_1`"]
    #[inline(always)]
    pub fn is_logic_1(&self) -> bool {
        *self == SM0OUT45_A::LOGIC_1
    }
}
#[doc = "Field `SM0OUT45` writer - Submodule 0 Software Controlled Output 45"]
pub type SM0OUT45_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWCOUT_SPEC, SM0OUT45_A, O>;
impl<'a, const O: u8> SM0OUT45_W<'a, O> {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    #[inline(always)]
    pub fn logic_0(self) -> &'a mut W {
        self.variant(SM0OUT45_A::LOGIC_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    #[inline(always)]
    pub fn logic_1(self) -> &'a mut W {
        self.variant(SM0OUT45_A::LOGIC_1)
    }
}
#[doc = "Field `SM0OUT23` reader - Submodule 0 Software Controlled Output 23"]
pub type SM0OUT23_R = crate::BitReader<SM0OUT23_A>;
#[doc = "Submodule 0 Software Controlled Output 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM0OUT23_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    LOGIC_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    LOGIC_1 = 1,
}
impl From<SM0OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: SM0OUT23_A) -> Self {
        variant as u8 != 0
    }
}
impl SM0OUT23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM0OUT23_A {
        match self.bits {
            false => SM0OUT23_A::LOGIC_0,
            true => SM0OUT23_A::LOGIC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOGIC_0`"]
    #[inline(always)]
    pub fn is_logic_0(&self) -> bool {
        *self == SM0OUT23_A::LOGIC_0
    }
    #[doc = "Checks if the value of the field is `LOGIC_1`"]
    #[inline(always)]
    pub fn is_logic_1(&self) -> bool {
        *self == SM0OUT23_A::LOGIC_1
    }
}
#[doc = "Field `SM0OUT23` writer - Submodule 0 Software Controlled Output 23"]
pub type SM0OUT23_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWCOUT_SPEC, SM0OUT23_A, O>;
impl<'a, const O: u8> SM0OUT23_W<'a, O> {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    #[inline(always)]
    pub fn logic_0(self) -> &'a mut W {
        self.variant(SM0OUT23_A::LOGIC_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    #[inline(always)]
    pub fn logic_1(self) -> &'a mut W {
        self.variant(SM0OUT23_A::LOGIC_1)
    }
}
#[doc = "Field `SM1OUT45` reader - Submodule 1 Software Controlled Output 45"]
pub type SM1OUT45_R = crate::BitReader<SM1OUT45_A>;
#[doc = "Submodule 1 Software Controlled Output 45\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM1OUT45_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    LOGIC_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    LOGIC_1 = 1,
}
impl From<SM1OUT45_A> for bool {
    #[inline(always)]
    fn from(variant: SM1OUT45_A) -> Self {
        variant as u8 != 0
    }
}
impl SM1OUT45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM1OUT45_A {
        match self.bits {
            false => SM1OUT45_A::LOGIC_0,
            true => SM1OUT45_A::LOGIC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOGIC_0`"]
    #[inline(always)]
    pub fn is_logic_0(&self) -> bool {
        *self == SM1OUT45_A::LOGIC_0
    }
    #[doc = "Checks if the value of the field is `LOGIC_1`"]
    #[inline(always)]
    pub fn is_logic_1(&self) -> bool {
        *self == SM1OUT45_A::LOGIC_1
    }
}
#[doc = "Field `SM1OUT45` writer - Submodule 1 Software Controlled Output 45"]
pub type SM1OUT45_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWCOUT_SPEC, SM1OUT45_A, O>;
impl<'a, const O: u8> SM1OUT45_W<'a, O> {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    #[inline(always)]
    pub fn logic_0(self) -> &'a mut W {
        self.variant(SM1OUT45_A::LOGIC_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    #[inline(always)]
    pub fn logic_1(self) -> &'a mut W {
        self.variant(SM1OUT45_A::LOGIC_1)
    }
}
#[doc = "Field `SM1OUT23` reader - Submodule 1 Software Controlled Output 23"]
pub type SM1OUT23_R = crate::BitReader<SM1OUT23_A>;
#[doc = "Submodule 1 Software Controlled Output 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM1OUT23_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    LOGIC_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    LOGIC_1 = 1,
}
impl From<SM1OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: SM1OUT23_A) -> Self {
        variant as u8 != 0
    }
}
impl SM1OUT23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM1OUT23_A {
        match self.bits {
            false => SM1OUT23_A::LOGIC_0,
            true => SM1OUT23_A::LOGIC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOGIC_0`"]
    #[inline(always)]
    pub fn is_logic_0(&self) -> bool {
        *self == SM1OUT23_A::LOGIC_0
    }
    #[doc = "Checks if the value of the field is `LOGIC_1`"]
    #[inline(always)]
    pub fn is_logic_1(&self) -> bool {
        *self == SM1OUT23_A::LOGIC_1
    }
}
#[doc = "Field `SM1OUT23` writer - Submodule 1 Software Controlled Output 23"]
pub type SM1OUT23_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWCOUT_SPEC, SM1OUT23_A, O>;
impl<'a, const O: u8> SM1OUT23_W<'a, O> {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    #[inline(always)]
    pub fn logic_0(self) -> &'a mut W {
        self.variant(SM1OUT23_A::LOGIC_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    #[inline(always)]
    pub fn logic_1(self) -> &'a mut W {
        self.variant(SM1OUT23_A::LOGIC_1)
    }
}
#[doc = "Field `SM2OUT45` reader - Submodule 2 Software Controlled Output 45"]
pub type SM2OUT45_R = crate::BitReader<SM2OUT45_A>;
#[doc = "Submodule 2 Software Controlled Output 45\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM2OUT45_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    LOGIC_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    LOGIC_1 = 1,
}
impl From<SM2OUT45_A> for bool {
    #[inline(always)]
    fn from(variant: SM2OUT45_A) -> Self {
        variant as u8 != 0
    }
}
impl SM2OUT45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM2OUT45_A {
        match self.bits {
            false => SM2OUT45_A::LOGIC_0,
            true => SM2OUT45_A::LOGIC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOGIC_0`"]
    #[inline(always)]
    pub fn is_logic_0(&self) -> bool {
        *self == SM2OUT45_A::LOGIC_0
    }
    #[doc = "Checks if the value of the field is `LOGIC_1`"]
    #[inline(always)]
    pub fn is_logic_1(&self) -> bool {
        *self == SM2OUT45_A::LOGIC_1
    }
}
#[doc = "Field `SM2OUT45` writer - Submodule 2 Software Controlled Output 45"]
pub type SM2OUT45_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWCOUT_SPEC, SM2OUT45_A, O>;
impl<'a, const O: u8> SM2OUT45_W<'a, O> {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    #[inline(always)]
    pub fn logic_0(self) -> &'a mut W {
        self.variant(SM2OUT45_A::LOGIC_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    #[inline(always)]
    pub fn logic_1(self) -> &'a mut W {
        self.variant(SM2OUT45_A::LOGIC_1)
    }
}
#[doc = "Field `SM2OUT23` reader - Submodule 2 Software Controlled Output 23"]
pub type SM2OUT23_R = crate::BitReader<SM2OUT23_A>;
#[doc = "Submodule 2 Software Controlled Output 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM2OUT23_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    LOGIC_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    LOGIC_1 = 1,
}
impl From<SM2OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: SM2OUT23_A) -> Self {
        variant as u8 != 0
    }
}
impl SM2OUT23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM2OUT23_A {
        match self.bits {
            false => SM2OUT23_A::LOGIC_0,
            true => SM2OUT23_A::LOGIC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOGIC_0`"]
    #[inline(always)]
    pub fn is_logic_0(&self) -> bool {
        *self == SM2OUT23_A::LOGIC_0
    }
    #[doc = "Checks if the value of the field is `LOGIC_1`"]
    #[inline(always)]
    pub fn is_logic_1(&self) -> bool {
        *self == SM2OUT23_A::LOGIC_1
    }
}
#[doc = "Field `SM2OUT23` writer - Submodule 2 Software Controlled Output 23"]
pub type SM2OUT23_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWCOUT_SPEC, SM2OUT23_A, O>;
impl<'a, const O: u8> SM2OUT23_W<'a, O> {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    #[inline(always)]
    pub fn logic_0(self) -> &'a mut W {
        self.variant(SM2OUT23_A::LOGIC_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    #[inline(always)]
    pub fn logic_1(self) -> &'a mut W {
        self.variant(SM2OUT23_A::LOGIC_1)
    }
}
#[doc = "Field `SM3OUT45` reader - Submodule 3 Software Controlled Output 45"]
pub type SM3OUT45_R = crate::BitReader<SM3OUT45_A>;
#[doc = "Submodule 3 Software Controlled Output 45\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM3OUT45_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    LOGIC_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    LOGIC_1 = 1,
}
impl From<SM3OUT45_A> for bool {
    #[inline(always)]
    fn from(variant: SM3OUT45_A) -> Self {
        variant as u8 != 0
    }
}
impl SM3OUT45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM3OUT45_A {
        match self.bits {
            false => SM3OUT45_A::LOGIC_0,
            true => SM3OUT45_A::LOGIC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOGIC_0`"]
    #[inline(always)]
    pub fn is_logic_0(&self) -> bool {
        *self == SM3OUT45_A::LOGIC_0
    }
    #[doc = "Checks if the value of the field is `LOGIC_1`"]
    #[inline(always)]
    pub fn is_logic_1(&self) -> bool {
        *self == SM3OUT45_A::LOGIC_1
    }
}
#[doc = "Field `SM3OUT45` writer - Submodule 3 Software Controlled Output 45"]
pub type SM3OUT45_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWCOUT_SPEC, SM3OUT45_A, O>;
impl<'a, const O: u8> SM3OUT45_W<'a, O> {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    #[inline(always)]
    pub fn logic_0(self) -> &'a mut W {
        self.variant(SM3OUT45_A::LOGIC_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    #[inline(always)]
    pub fn logic_1(self) -> &'a mut W {
        self.variant(SM3OUT45_A::LOGIC_1)
    }
}
#[doc = "Field `SM3OUT23` reader - Submodule 3 Software Controlled Output 23"]
pub type SM3OUT23_R = crate::BitReader<SM3OUT23_A>;
#[doc = "Submodule 3 Software Controlled Output 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM3OUT23_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    LOGIC_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    LOGIC_1 = 1,
}
impl From<SM3OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: SM3OUT23_A) -> Self {
        variant as u8 != 0
    }
}
impl SM3OUT23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM3OUT23_A {
        match self.bits {
            false => SM3OUT23_A::LOGIC_0,
            true => SM3OUT23_A::LOGIC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOGIC_0`"]
    #[inline(always)]
    pub fn is_logic_0(&self) -> bool {
        *self == SM3OUT23_A::LOGIC_0
    }
    #[doc = "Checks if the value of the field is `LOGIC_1`"]
    #[inline(always)]
    pub fn is_logic_1(&self) -> bool {
        *self == SM3OUT23_A::LOGIC_1
    }
}
#[doc = "Field `SM3OUT23` writer - Submodule 3 Software Controlled Output 23"]
pub type SM3OUT23_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWCOUT_SPEC, SM3OUT23_A, O>;
impl<'a, const O: u8> SM3OUT23_W<'a, O> {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    #[inline(always)]
    pub fn logic_0(self) -> &'a mut W {
        self.variant(SM3OUT23_A::LOGIC_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    #[inline(always)]
    pub fn logic_1(self) -> &'a mut W {
        self.variant(SM3OUT23_A::LOGIC_1)
    }
}
impl R {
    #[doc = "Bit 0 - Submodule 0 Software Controlled Output 45"]
    #[inline(always)]
    pub fn sm0out45(&self) -> SM0OUT45_R {
        SM0OUT45_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Submodule 0 Software Controlled Output 23"]
    #[inline(always)]
    pub fn sm0out23(&self) -> SM0OUT23_R {
        SM0OUT23_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Submodule 1 Software Controlled Output 45"]
    #[inline(always)]
    pub fn sm1out45(&self) -> SM1OUT45_R {
        SM1OUT45_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Submodule 1 Software Controlled Output 23"]
    #[inline(always)]
    pub fn sm1out23(&self) -> SM1OUT23_R {
        SM1OUT23_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Submodule 2 Software Controlled Output 45"]
    #[inline(always)]
    pub fn sm2out45(&self) -> SM2OUT45_R {
        SM2OUT45_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Submodule 2 Software Controlled Output 23"]
    #[inline(always)]
    pub fn sm2out23(&self) -> SM2OUT23_R {
        SM2OUT23_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Submodule 3 Software Controlled Output 45"]
    #[inline(always)]
    pub fn sm3out45(&self) -> SM3OUT45_R {
        SM3OUT45_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Submodule 3 Software Controlled Output 23"]
    #[inline(always)]
    pub fn sm3out23(&self) -> SM3OUT23_R {
        SM3OUT23_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Submodule 0 Software Controlled Output 45"]
    #[inline(always)]
    #[must_use]
    pub fn sm0out45(&mut self) -> SM0OUT45_W<0> {
        SM0OUT45_W::new(self)
    }
    #[doc = "Bit 1 - Submodule 0 Software Controlled Output 23"]
    #[inline(always)]
    #[must_use]
    pub fn sm0out23(&mut self) -> SM0OUT23_W<1> {
        SM0OUT23_W::new(self)
    }
    #[doc = "Bit 2 - Submodule 1 Software Controlled Output 45"]
    #[inline(always)]
    #[must_use]
    pub fn sm1out45(&mut self) -> SM1OUT45_W<2> {
        SM1OUT45_W::new(self)
    }
    #[doc = "Bit 3 - Submodule 1 Software Controlled Output 23"]
    #[inline(always)]
    #[must_use]
    pub fn sm1out23(&mut self) -> SM1OUT23_W<3> {
        SM1OUT23_W::new(self)
    }
    #[doc = "Bit 4 - Submodule 2 Software Controlled Output 45"]
    #[inline(always)]
    #[must_use]
    pub fn sm2out45(&mut self) -> SM2OUT45_W<4> {
        SM2OUT45_W::new(self)
    }
    #[doc = "Bit 5 - Submodule 2 Software Controlled Output 23"]
    #[inline(always)]
    #[must_use]
    pub fn sm2out23(&mut self) -> SM2OUT23_W<5> {
        SM2OUT23_W::new(self)
    }
    #[doc = "Bit 6 - Submodule 3 Software Controlled Output 45"]
    #[inline(always)]
    #[must_use]
    pub fn sm3out45(&mut self) -> SM3OUT45_W<6> {
        SM3OUT45_W::new(self)
    }
    #[doc = "Bit 7 - Submodule 3 Software Controlled Output 23"]
    #[inline(always)]
    #[must_use]
    pub fn sm3out23(&mut self) -> SM3OUT23_W<7> {
        SM3OUT23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Controlled Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swcout](index.html) module"]
pub struct SWCOUT_SPEC;
impl crate::RegisterSpec for SWCOUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [swcout::R](R) reader structure"]
impl crate::Readable for SWCOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swcout::W](W) writer structure"]
impl crate::Writable for SWCOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWCOUT to value 0"]
impl crate::Resettable for SWCOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
