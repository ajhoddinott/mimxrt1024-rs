#[doc = "Register `SMOCTRL` reader"]
pub struct R(crate::R<SMOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMOCTRL` writer"]
pub struct W(crate::W<SMOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMOCTRL_SPEC>;
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
impl From<crate::W<SMOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWMXFS` reader - PWM_X Fault State"]
pub type PWMXFS_R = crate::FieldReader<u8, PWMXFS_A>;
#[doc = "PWM_X Fault State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWMXFS_A {
    #[doc = "0: Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0,
    #[doc = "1: Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 1,
    #[doc = "2: Output is tristated."]
    TRISTATED = 2,
    #[doc = "3: Output is tristated."]
    TRISTATED_ALT = 3,
}
impl From<PWMXFS_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMXFS_A) -> Self {
        variant as _
    }
}
impl PWMXFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMXFS_A {
        match self.bits {
            0 => PWMXFS_A::LOGIC_0,
            1 => PWMXFS_A::LOGIC_1,
            2 => PWMXFS_A::TRISTATED,
            3 => PWMXFS_A::TRISTATED_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOGIC_0`"]
    #[inline(always)]
    pub fn is_logic_0(&self) -> bool {
        *self == PWMXFS_A::LOGIC_0
    }
    #[doc = "Checks if the value of the field is `LOGIC_1`"]
    #[inline(always)]
    pub fn is_logic_1(&self) -> bool {
        *self == PWMXFS_A::LOGIC_1
    }
    #[doc = "Checks if the value of the field is `TRISTATED`"]
    #[inline(always)]
    pub fn is_tristated(&self) -> bool {
        *self == PWMXFS_A::TRISTATED
    }
    #[doc = "Checks if the value of the field is `TRISTATED_ALT`"]
    #[inline(always)]
    pub fn is_tristated_alt(&self) -> bool {
        *self == PWMXFS_A::TRISTATED_ALT
    }
}
#[doc = "Field `PWMXFS` writer - PWM_X Fault State"]
pub type PWMXFS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SMOCTRL_SPEC, u8, PWMXFS_A, 2, O>;
impl<'a, const O: u8> PWMXFS_W<'a, O> {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    #[inline(always)]
    pub fn logic_0(self) -> &'a mut W {
        self.variant(PWMXFS_A::LOGIC_0)
    }
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    #[inline(always)]
    pub fn logic_1(self) -> &'a mut W {
        self.variant(PWMXFS_A::LOGIC_1)
    }
    #[doc = "Output is tristated."]
    #[inline(always)]
    pub fn tristated(self) -> &'a mut W {
        self.variant(PWMXFS_A::TRISTATED)
    }
    #[doc = "Output is tristated."]
    #[inline(always)]
    pub fn tristated_alt(self) -> &'a mut W {
        self.variant(PWMXFS_A::TRISTATED_ALT)
    }
}
#[doc = "Field `PWMBFS` reader - PWM_B Fault State"]
pub type PWMBFS_R = crate::FieldReader<u8, PWMBFS_A>;
#[doc = "PWM_B Fault State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWMBFS_A {
    #[doc = "0: Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0,
    #[doc = "1: Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 1,
    #[doc = "2: Output is tristated."]
    TRISTATED = 2,
    #[doc = "3: Output is tristated."]
    TRISTATED_ALT = 3,
}
impl From<PWMBFS_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMBFS_A) -> Self {
        variant as _
    }
}
impl PWMBFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMBFS_A {
        match self.bits {
            0 => PWMBFS_A::LOGIC_0,
            1 => PWMBFS_A::LOGIC_1,
            2 => PWMBFS_A::TRISTATED,
            3 => PWMBFS_A::TRISTATED_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOGIC_0`"]
    #[inline(always)]
    pub fn is_logic_0(&self) -> bool {
        *self == PWMBFS_A::LOGIC_0
    }
    #[doc = "Checks if the value of the field is `LOGIC_1`"]
    #[inline(always)]
    pub fn is_logic_1(&self) -> bool {
        *self == PWMBFS_A::LOGIC_1
    }
    #[doc = "Checks if the value of the field is `TRISTATED`"]
    #[inline(always)]
    pub fn is_tristated(&self) -> bool {
        *self == PWMBFS_A::TRISTATED
    }
    #[doc = "Checks if the value of the field is `TRISTATED_ALT`"]
    #[inline(always)]
    pub fn is_tristated_alt(&self) -> bool {
        *self == PWMBFS_A::TRISTATED_ALT
    }
}
#[doc = "Field `PWMBFS` writer - PWM_B Fault State"]
pub type PWMBFS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SMOCTRL_SPEC, u8, PWMBFS_A, 2, O>;
impl<'a, const O: u8> PWMBFS_W<'a, O> {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    #[inline(always)]
    pub fn logic_0(self) -> &'a mut W {
        self.variant(PWMBFS_A::LOGIC_0)
    }
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    #[inline(always)]
    pub fn logic_1(self) -> &'a mut W {
        self.variant(PWMBFS_A::LOGIC_1)
    }
    #[doc = "Output is tristated."]
    #[inline(always)]
    pub fn tristated(self) -> &'a mut W {
        self.variant(PWMBFS_A::TRISTATED)
    }
    #[doc = "Output is tristated."]
    #[inline(always)]
    pub fn tristated_alt(self) -> &'a mut W {
        self.variant(PWMBFS_A::TRISTATED_ALT)
    }
}
#[doc = "Field `PWMAFS` reader - PWM_A Fault State"]
pub type PWMAFS_R = crate::FieldReader<u8, PWMAFS_A>;
#[doc = "PWM_A Fault State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWMAFS_A {
    #[doc = "0: Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0,
    #[doc = "1: Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 1,
    #[doc = "2: Output is tristated."]
    TRISTATED = 2,
    #[doc = "3: Output is tristated."]
    TRISTATED_ALT = 3,
}
impl From<PWMAFS_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMAFS_A) -> Self {
        variant as _
    }
}
impl PWMAFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMAFS_A {
        match self.bits {
            0 => PWMAFS_A::LOGIC_0,
            1 => PWMAFS_A::LOGIC_1,
            2 => PWMAFS_A::TRISTATED,
            3 => PWMAFS_A::TRISTATED_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOGIC_0`"]
    #[inline(always)]
    pub fn is_logic_0(&self) -> bool {
        *self == PWMAFS_A::LOGIC_0
    }
    #[doc = "Checks if the value of the field is `LOGIC_1`"]
    #[inline(always)]
    pub fn is_logic_1(&self) -> bool {
        *self == PWMAFS_A::LOGIC_1
    }
    #[doc = "Checks if the value of the field is `TRISTATED`"]
    #[inline(always)]
    pub fn is_tristated(&self) -> bool {
        *self == PWMAFS_A::TRISTATED
    }
    #[doc = "Checks if the value of the field is `TRISTATED_ALT`"]
    #[inline(always)]
    pub fn is_tristated_alt(&self) -> bool {
        *self == PWMAFS_A::TRISTATED_ALT
    }
}
#[doc = "Field `PWMAFS` writer - PWM_A Fault State"]
pub type PWMAFS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SMOCTRL_SPEC, u8, PWMAFS_A, 2, O>;
impl<'a, const O: u8> PWMAFS_W<'a, O> {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    #[inline(always)]
    pub fn logic_0(self) -> &'a mut W {
        self.variant(PWMAFS_A::LOGIC_0)
    }
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    #[inline(always)]
    pub fn logic_1(self) -> &'a mut W {
        self.variant(PWMAFS_A::LOGIC_1)
    }
    #[doc = "Output is tristated."]
    #[inline(always)]
    pub fn tristated(self) -> &'a mut W {
        self.variant(PWMAFS_A::TRISTATED)
    }
    #[doc = "Output is tristated."]
    #[inline(always)]
    pub fn tristated_alt(self) -> &'a mut W {
        self.variant(PWMAFS_A::TRISTATED_ALT)
    }
}
#[doc = "Field `POLX` reader - PWM_X Output Polarity"]
pub type POLX_R = crate::BitReader<POLX_A>;
#[doc = "PWM_X Output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLX_A {
    #[doc = "0: PWM_X output not inverted. A high level on the PWM_X pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0,
    #[doc = "1: PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
    INVERTED = 1,
}
impl From<POLX_A> for bool {
    #[inline(always)]
    fn from(variant: POLX_A) -> Self {
        variant as u8 != 0
    }
}
impl POLX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLX_A {
        match self.bits {
            false => POLX_A::NOT_INVERTED,
            true => POLX_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == POLX_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == POLX_A::INVERTED
    }
}
#[doc = "Field `POLX` writer - PWM_X Output Polarity"]
pub type POLX_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMOCTRL_SPEC, POLX_A, O>;
impl<'a, const O: u8> POLX_W<'a, O> {
    #[doc = "PWM_X output not inverted. A high level on the PWM_X pin represents the \"on\" or \"active\" state."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(POLX_A::NOT_INVERTED)
    }
    #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(POLX_A::INVERTED)
    }
}
#[doc = "Field `POLB` reader - PWM_B Output Polarity"]
pub type POLB_R = crate::BitReader<POLB_A>;
#[doc = "PWM_B Output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLB_A {
    #[doc = "0: PWM_B output not inverted. A high level on the PWM_B pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0,
    #[doc = "1: PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
    INVERTED = 1,
}
impl From<POLB_A> for bool {
    #[inline(always)]
    fn from(variant: POLB_A) -> Self {
        variant as u8 != 0
    }
}
impl POLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLB_A {
        match self.bits {
            false => POLB_A::NOT_INVERTED,
            true => POLB_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == POLB_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == POLB_A::INVERTED
    }
}
#[doc = "Field `POLB` writer - PWM_B Output Polarity"]
pub type POLB_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMOCTRL_SPEC, POLB_A, O>;
impl<'a, const O: u8> POLB_W<'a, O> {
    #[doc = "PWM_B output not inverted. A high level on the PWM_B pin represents the \"on\" or \"active\" state."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(POLB_A::NOT_INVERTED)
    }
    #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(POLB_A::INVERTED)
    }
}
#[doc = "Field `POLA` reader - PWM_A Output Polarity"]
pub type POLA_R = crate::BitReader<POLA_A>;
#[doc = "PWM_A Output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLA_A {
    #[doc = "0: PWM_A output not inverted. A high level on the PWM_A pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0,
    #[doc = "1: PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
    INVERTED = 1,
}
impl From<POLA_A> for bool {
    #[inline(always)]
    fn from(variant: POLA_A) -> Self {
        variant as u8 != 0
    }
}
impl POLA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLA_A {
        match self.bits {
            false => POLA_A::NOT_INVERTED,
            true => POLA_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == POLA_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == POLA_A::INVERTED
    }
}
#[doc = "Field `POLA` writer - PWM_A Output Polarity"]
pub type POLA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMOCTRL_SPEC, POLA_A, O>;
impl<'a, const O: u8> POLA_W<'a, O> {
    #[doc = "PWM_A output not inverted. A high level on the PWM_A pin represents the \"on\" or \"active\" state."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(POLA_A::NOT_INVERTED)
    }
    #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(POLA_A::INVERTED)
    }
}
#[doc = "Field `PWMX_IN` reader - PWM_X Input"]
pub type PWMX_IN_R = crate::BitReader<bool>;
#[doc = "Field `PWMB_IN` reader - PWM_B Input"]
pub type PWMB_IN_R = crate::BitReader<bool>;
#[doc = "Field `PWMA_IN` reader - PWM_A Input"]
pub type PWMA_IN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - PWM_X Fault State"]
    #[inline(always)]
    pub fn pwmxfs(&self) -> PWMXFS_R {
        PWMXFS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PWM_B Fault State"]
    #[inline(always)]
    pub fn pwmbfs(&self) -> PWMBFS_R {
        PWMBFS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PWM_A Fault State"]
    #[inline(always)]
    pub fn pwmafs(&self) -> PWMAFS_R {
        PWMAFS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - PWM_X Output Polarity"]
    #[inline(always)]
    pub fn polx(&self) -> POLX_R {
        POLX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM_B Output Polarity"]
    #[inline(always)]
    pub fn polb(&self) -> POLB_R {
        POLB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM_A Output Polarity"]
    #[inline(always)]
    pub fn pola(&self) -> POLA_R {
        POLA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - PWM_X Input"]
    #[inline(always)]
    pub fn pwmx_in(&self) -> PWMX_IN_R {
        PWMX_IN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PWM_B Input"]
    #[inline(always)]
    pub fn pwmb_in(&self) -> PWMB_IN_R {
        PWMB_IN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PWM_A Input"]
    #[inline(always)]
    pub fn pwma_in(&self) -> PWMA_IN_R {
        PWMA_IN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWM_X Fault State"]
    #[inline(always)]
    #[must_use]
    pub fn pwmxfs(&mut self) -> PWMXFS_W<0> {
        PWMXFS_W::new(self)
    }
    #[doc = "Bits 2:3 - PWM_B Fault State"]
    #[inline(always)]
    #[must_use]
    pub fn pwmbfs(&mut self) -> PWMBFS_W<2> {
        PWMBFS_W::new(self)
    }
    #[doc = "Bits 4:5 - PWM_A Fault State"]
    #[inline(always)]
    #[must_use]
    pub fn pwmafs(&mut self) -> PWMAFS_W<4> {
        PWMAFS_W::new(self)
    }
    #[doc = "Bit 8 - PWM_X Output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn polx(&mut self) -> POLX_W<8> {
        POLX_W::new(self)
    }
    #[doc = "Bit 9 - PWM_B Output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn polb(&mut self) -> POLB_W<9> {
        POLB_W::new(self)
    }
    #[doc = "Bit 10 - PWM_A Output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pola(&mut self) -> POLA_W<10> {
        POLA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smoctrl](index.html) module"]
pub struct SMOCTRL_SPEC;
impl crate::RegisterSpec for SMOCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smoctrl::R](R) reader structure"]
impl crate::Readable for SMOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smoctrl::W](W) writer structure"]
impl crate::Writable for SMOCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMOCTRL to value 0"]
impl crate::Resettable for SMOCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
