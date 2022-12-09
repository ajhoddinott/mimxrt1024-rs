#[doc = "Register `KPSR` reader"]
pub struct R(crate::R<KPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KPSR` writer"]
pub struct W(crate::W<KPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KPSR_SPEC>;
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
impl From<crate::W<KPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KPKD` reader - Keypad Key Depress"]
pub type KPKD_R = crate::BitReader<KPKD_A>;
#[doc = "Keypad Key Depress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KPKD_A {
    #[doc = "0: No key presses detected"]
    KPKD_0 = 0,
    #[doc = "1: A key has been depressed"]
    KPKD_1 = 1,
}
impl From<KPKD_A> for bool {
    #[inline(always)]
    fn from(variant: KPKD_A) -> Self {
        variant as u8 != 0
    }
}
impl KPKD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KPKD_A {
        match self.bits {
            false => KPKD_A::KPKD_0,
            true => KPKD_A::KPKD_1,
        }
    }
    #[doc = "Checks if the value of the field is `KPKD_0`"]
    #[inline(always)]
    pub fn is_kpkd_0(&self) -> bool {
        *self == KPKD_A::KPKD_0
    }
    #[doc = "Checks if the value of the field is `KPKD_1`"]
    #[inline(always)]
    pub fn is_kpkd_1(&self) -> bool {
        *self == KPKD_A::KPKD_1
    }
}
#[doc = "Field `KPKD` writer - Keypad Key Depress"]
pub type KPKD_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, KPSR_SPEC, KPKD_A, O>;
impl<'a, const O: u8> KPKD_W<'a, O> {
    #[doc = "No key presses detected"]
    #[inline(always)]
    pub fn kpkd_0(self) -> &'a mut W {
        self.variant(KPKD_A::KPKD_0)
    }
    #[doc = "A key has been depressed"]
    #[inline(always)]
    pub fn kpkd_1(self) -> &'a mut W {
        self.variant(KPKD_A::KPKD_1)
    }
}
#[doc = "Field `KPKR` reader - Keypad Key Release"]
pub type KPKR_R = crate::BitReader<KPKR_A>;
#[doc = "Keypad Key Release\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KPKR_A {
    #[doc = "0: No key release detected"]
    KPKR_0 = 0,
    #[doc = "1: All keys have been released"]
    KPKR_1 = 1,
}
impl From<KPKR_A> for bool {
    #[inline(always)]
    fn from(variant: KPKR_A) -> Self {
        variant as u8 != 0
    }
}
impl KPKR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KPKR_A {
        match self.bits {
            false => KPKR_A::KPKR_0,
            true => KPKR_A::KPKR_1,
        }
    }
    #[doc = "Checks if the value of the field is `KPKR_0`"]
    #[inline(always)]
    pub fn is_kpkr_0(&self) -> bool {
        *self == KPKR_A::KPKR_0
    }
    #[doc = "Checks if the value of the field is `KPKR_1`"]
    #[inline(always)]
    pub fn is_kpkr_1(&self) -> bool {
        *self == KPKR_A::KPKR_1
    }
}
#[doc = "Field `KPKR` writer - Keypad Key Release"]
pub type KPKR_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, KPSR_SPEC, KPKR_A, O>;
impl<'a, const O: u8> KPKR_W<'a, O> {
    #[doc = "No key release detected"]
    #[inline(always)]
    pub fn kpkr_0(self) -> &'a mut W {
        self.variant(KPKR_A::KPKR_0)
    }
    #[doc = "All keys have been released"]
    #[inline(always)]
    pub fn kpkr_1(self) -> &'a mut W {
        self.variant(KPKR_A::KPKR_1)
    }
}
#[doc = "Field `KDSC` reader - Key Depress Synchronizer Clear"]
pub type KDSC_R = crate::BitReader<KDSC_A>;
#[doc = "Key Depress Synchronizer Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KDSC_A {
    #[doc = "0: No effect"]
    KDSC_0 = 0,
    #[doc = "1: Set bits that clear the keypad depress synchronizer chain"]
    KDSC_1 = 1,
}
impl From<KDSC_A> for bool {
    #[inline(always)]
    fn from(variant: KDSC_A) -> Self {
        variant as u8 != 0
    }
}
impl KDSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KDSC_A {
        match self.bits {
            false => KDSC_A::KDSC_0,
            true => KDSC_A::KDSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `KDSC_0`"]
    #[inline(always)]
    pub fn is_kdsc_0(&self) -> bool {
        *self == KDSC_A::KDSC_0
    }
    #[doc = "Checks if the value of the field is `KDSC_1`"]
    #[inline(always)]
    pub fn is_kdsc_1(&self) -> bool {
        *self == KDSC_A::KDSC_1
    }
}
#[doc = "Field `KDSC` writer - Key Depress Synchronizer Clear"]
pub type KDSC_W<'a, const O: u8> = crate::BitWriter<'a, u16, KPSR_SPEC, KDSC_A, O>;
impl<'a, const O: u8> KDSC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn kdsc_0(self) -> &'a mut W {
        self.variant(KDSC_A::KDSC_0)
    }
    #[doc = "Set bits that clear the keypad depress synchronizer chain"]
    #[inline(always)]
    pub fn kdsc_1(self) -> &'a mut W {
        self.variant(KDSC_A::KDSC_1)
    }
}
#[doc = "Field `KRSS` reader - Key Release Synchronizer Set"]
pub type KRSS_R = crate::BitReader<KRSS_A>;
#[doc = "Key Release Synchronizer Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRSS_A {
    #[doc = "0: No effect"]
    KRSS_0 = 0,
    #[doc = "1: Set bits which sets keypad release synchronizer chain"]
    KRSS_1 = 1,
}
impl From<KRSS_A> for bool {
    #[inline(always)]
    fn from(variant: KRSS_A) -> Self {
        variant as u8 != 0
    }
}
impl KRSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRSS_A {
        match self.bits {
            false => KRSS_A::KRSS_0,
            true => KRSS_A::KRSS_1,
        }
    }
    #[doc = "Checks if the value of the field is `KRSS_0`"]
    #[inline(always)]
    pub fn is_krss_0(&self) -> bool {
        *self == KRSS_A::KRSS_0
    }
    #[doc = "Checks if the value of the field is `KRSS_1`"]
    #[inline(always)]
    pub fn is_krss_1(&self) -> bool {
        *self == KRSS_A::KRSS_1
    }
}
#[doc = "Field `KRSS` writer - Key Release Synchronizer Set"]
pub type KRSS_W<'a, const O: u8> = crate::BitWriter<'a, u16, KPSR_SPEC, KRSS_A, O>;
impl<'a, const O: u8> KRSS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn krss_0(self) -> &'a mut W {
        self.variant(KRSS_A::KRSS_0)
    }
    #[doc = "Set bits which sets keypad release synchronizer chain"]
    #[inline(always)]
    pub fn krss_1(self) -> &'a mut W {
        self.variant(KRSS_A::KRSS_1)
    }
}
#[doc = "Field `KDIE` reader - Keypad Key Depress Interrupt Enable"]
pub type KDIE_R = crate::BitReader<KDIE_A>;
#[doc = "Keypad Key Depress Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KDIE_A {
    #[doc = "0: No interrupt request is generated when KPKD is set."]
    KDIE_0 = 0,
    #[doc = "1: An interrupt request is generated when KPKD is set."]
    KDIE_1 = 1,
}
impl From<KDIE_A> for bool {
    #[inline(always)]
    fn from(variant: KDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl KDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KDIE_A {
        match self.bits {
            false => KDIE_A::KDIE_0,
            true => KDIE_A::KDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `KDIE_0`"]
    #[inline(always)]
    pub fn is_kdie_0(&self) -> bool {
        *self == KDIE_A::KDIE_0
    }
    #[doc = "Checks if the value of the field is `KDIE_1`"]
    #[inline(always)]
    pub fn is_kdie_1(&self) -> bool {
        *self == KDIE_A::KDIE_1
    }
}
#[doc = "Field `KDIE` writer - Keypad Key Depress Interrupt Enable"]
pub type KDIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, KPSR_SPEC, KDIE_A, O>;
impl<'a, const O: u8> KDIE_W<'a, O> {
    #[doc = "No interrupt request is generated when KPKD is set."]
    #[inline(always)]
    pub fn kdie_0(self) -> &'a mut W {
        self.variant(KDIE_A::KDIE_0)
    }
    #[doc = "An interrupt request is generated when KPKD is set."]
    #[inline(always)]
    pub fn kdie_1(self) -> &'a mut W {
        self.variant(KDIE_A::KDIE_1)
    }
}
#[doc = "Field `KRIE` reader - Keypad Release Interrupt Enable"]
pub type KRIE_R = crate::BitReader<KRIE_A>;
#[doc = "Keypad Release Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRIE_A {
    #[doc = "0: No interrupt request is generated when KPKR is set."]
    KRIE_0 = 0,
    #[doc = "1: An interrupt request is generated when KPKR is set."]
    KRIE_1 = 1,
}
impl From<KRIE_A> for bool {
    #[inline(always)]
    fn from(variant: KRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl KRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRIE_A {
        match self.bits {
            false => KRIE_A::KRIE_0,
            true => KRIE_A::KRIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `KRIE_0`"]
    #[inline(always)]
    pub fn is_krie_0(&self) -> bool {
        *self == KRIE_A::KRIE_0
    }
    #[doc = "Checks if the value of the field is `KRIE_1`"]
    #[inline(always)]
    pub fn is_krie_1(&self) -> bool {
        *self == KRIE_A::KRIE_1
    }
}
#[doc = "Field `KRIE` writer - Keypad Release Interrupt Enable"]
pub type KRIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, KPSR_SPEC, KRIE_A, O>;
impl<'a, const O: u8> KRIE_W<'a, O> {
    #[doc = "No interrupt request is generated when KPKR is set."]
    #[inline(always)]
    pub fn krie_0(self) -> &'a mut W {
        self.variant(KRIE_A::KRIE_0)
    }
    #[doc = "An interrupt request is generated when KPKR is set."]
    #[inline(always)]
    pub fn krie_1(self) -> &'a mut W {
        self.variant(KRIE_A::KRIE_1)
    }
}
impl R {
    #[doc = "Bit 0 - Keypad Key Depress"]
    #[inline(always)]
    pub fn kpkd(&self) -> KPKD_R {
        KPKD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Keypad Key Release"]
    #[inline(always)]
    pub fn kpkr(&self) -> KPKR_R {
        KPKR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key Depress Synchronizer Clear"]
    #[inline(always)]
    pub fn kdsc(&self) -> KDSC_R {
        KDSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Key Release Synchronizer Set"]
    #[inline(always)]
    pub fn krss(&self) -> KRSS_R {
        KRSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Keypad Key Depress Interrupt Enable"]
    #[inline(always)]
    pub fn kdie(&self) -> KDIE_R {
        KDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Keypad Release Interrupt Enable"]
    #[inline(always)]
    pub fn krie(&self) -> KRIE_R {
        KRIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keypad Key Depress"]
    #[inline(always)]
    #[must_use]
    pub fn kpkd(&mut self) -> KPKD_W<0> {
        KPKD_W::new(self)
    }
    #[doc = "Bit 1 - Keypad Key Release"]
    #[inline(always)]
    #[must_use]
    pub fn kpkr(&mut self) -> KPKR_W<1> {
        KPKR_W::new(self)
    }
    #[doc = "Bit 2 - Key Depress Synchronizer Clear"]
    #[inline(always)]
    #[must_use]
    pub fn kdsc(&mut self) -> KDSC_W<2> {
        KDSC_W::new(self)
    }
    #[doc = "Bit 3 - Key Release Synchronizer Set"]
    #[inline(always)]
    #[must_use]
    pub fn krss(&mut self) -> KRSS_W<3> {
        KRSS_W::new(self)
    }
    #[doc = "Bit 8 - Keypad Key Depress Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn kdie(&mut self) -> KDIE_W<8> {
        KDIE_W::new(self)
    }
    #[doc = "Bit 9 - Keypad Release Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn krie(&mut self) -> KRIE_W<9> {
        KRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Keypad Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kpsr](index.html) module"]
pub struct KPSR_SPEC;
impl crate::RegisterSpec for KPSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [kpsr::R](R) reader structure"]
impl crate::Readable for KPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kpsr::W](W) writer structure"]
impl crate::Writable for KPSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
}
#[doc = "`reset()` method sets KPSR to value 0x0400"]
impl crate::Resettable for KPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
