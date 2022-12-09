#[doc = "Register `GPR12` reader"]
pub struct R(crate::R<GPR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR12` writer"]
pub struct W(crate::W<GPR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR12_SPEC>;
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
impl From<crate::W<GPR12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLEXIO1_IPG_STOP_MODE` reader - FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
pub type FLEXIO1_IPG_STOP_MODE_R = crate::BitReader<FLEXIO1_IPG_STOP_MODE_A>;
#[doc = "FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXIO1_IPG_STOP_MODE_A {
    #[doc = "0: FlexIO1 is functional in Stop mode"]
    FLEXIO1_IPG_STOP_MODE_0 = 0,
    #[doc = "1: When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO1 is not functional in Stop mode"]
    FLEXIO1_IPG_STOP_MODE_1 = 1,
}
impl From<FLEXIO1_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO1_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXIO1_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO1_IPG_STOP_MODE_A {
        match self.bits {
            false => FLEXIO1_IPG_STOP_MODE_A::FLEXIO1_IPG_STOP_MODE_0,
            true => FLEXIO1_IPG_STOP_MODE_A::FLEXIO1_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_flexio1_ipg_stop_mode_0(&self) -> bool {
        *self == FLEXIO1_IPG_STOP_MODE_A::FLEXIO1_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_flexio1_ipg_stop_mode_1(&self) -> bool {
        *self == FLEXIO1_IPG_STOP_MODE_A::FLEXIO1_IPG_STOP_MODE_1
    }
}
#[doc = "Field `FLEXIO1_IPG_STOP_MODE` writer - FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
pub type FLEXIO1_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR12_SPEC, FLEXIO1_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> FLEXIO1_IPG_STOP_MODE_W<'a, O> {
    #[doc = "FlexIO1 is functional in Stop mode"]
    #[inline(always)]
    pub fn flexio1_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(FLEXIO1_IPG_STOP_MODE_A::FLEXIO1_IPG_STOP_MODE_0)
    }
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO1 is not functional in Stop mode"]
    #[inline(always)]
    pub fn flexio1_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(FLEXIO1_IPG_STOP_MODE_A::FLEXIO1_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `FLEXIO1_IPG_DOZE` reader - FLEXIO1 ipg_doze mode"]
pub type FLEXIO1_IPG_DOZE_R = crate::BitReader<FLEXIO1_IPG_DOZE_A>;
#[doc = "FLEXIO1 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXIO1_IPG_DOZE_A {
    #[doc = "0: FLEXIO1 is not in doze mode"]
    FLEXIO1_IPG_DOZE_0 = 0,
    #[doc = "1: FLEXIO1 is in doze mode"]
    FLEXIO1_IPG_DOZE_1 = 1,
}
impl From<FLEXIO1_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO1_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXIO1_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO1_IPG_DOZE_A {
        match self.bits {
            false => FLEXIO1_IPG_DOZE_A::FLEXIO1_IPG_DOZE_0,
            true => FLEXIO1_IPG_DOZE_A::FLEXIO1_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_flexio1_ipg_doze_0(&self) -> bool {
        *self == FLEXIO1_IPG_DOZE_A::FLEXIO1_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_flexio1_ipg_doze_1(&self) -> bool {
        *self == FLEXIO1_IPG_DOZE_A::FLEXIO1_IPG_DOZE_1
    }
}
#[doc = "Field `FLEXIO1_IPG_DOZE` writer - FLEXIO1 ipg_doze mode"]
pub type FLEXIO1_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR12_SPEC, FLEXIO1_IPG_DOZE_A, O>;
impl<'a, const O: u8> FLEXIO1_IPG_DOZE_W<'a, O> {
    #[doc = "FLEXIO1 is not in doze mode"]
    #[inline(always)]
    pub fn flexio1_ipg_doze_0(self) -> &'a mut W {
        self.variant(FLEXIO1_IPG_DOZE_A::FLEXIO1_IPG_DOZE_0)
    }
    #[doc = "FLEXIO1 is in doze mode"]
    #[inline(always)]
    pub fn flexio1_ipg_doze_1(self) -> &'a mut W {
        self.variant(FLEXIO1_IPG_DOZE_A::FLEXIO1_IPG_DOZE_1)
    }
}
#[doc = "Field `ACMP_IPG_STOP_MODE` reader - ACMP stop mode selection. Cannot change when ipg_stop is asserted."]
pub type ACMP_IPG_STOP_MODE_R = crate::BitReader<ACMP_IPG_STOP_MODE_A>;
#[doc = "ACMP stop mode selection. Cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP_IPG_STOP_MODE_A {
    #[doc = "0: ACMP is functional in Stop mode"]
    ACMP_IPG_STOP_MODE_0 = 0,
    #[doc = "1: When this bit is equal to 1'b1 and ipg_stop is asserted, ACMP is not functional in Stop mode"]
    ACMP_IPG_STOP_MODE_1 = 1,
}
impl From<ACMP_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_IPG_STOP_MODE_A {
        match self.bits {
            false => ACMP_IPG_STOP_MODE_A::ACMP_IPG_STOP_MODE_0,
            true => ACMP_IPG_STOP_MODE_A::ACMP_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_acmp_ipg_stop_mode_0(&self) -> bool {
        *self == ACMP_IPG_STOP_MODE_A::ACMP_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `ACMP_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_acmp_ipg_stop_mode_1(&self) -> bool {
        *self == ACMP_IPG_STOP_MODE_A::ACMP_IPG_STOP_MODE_1
    }
}
#[doc = "Field `ACMP_IPG_STOP_MODE` writer - ACMP stop mode selection. Cannot change when ipg_stop is asserted."]
pub type ACMP_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR12_SPEC, ACMP_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> ACMP_IPG_STOP_MODE_W<'a, O> {
    #[doc = "ACMP is functional in Stop mode"]
    #[inline(always)]
    pub fn acmp_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(ACMP_IPG_STOP_MODE_A::ACMP_IPG_STOP_MODE_0)
    }
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, ACMP is not functional in Stop mode"]
    #[inline(always)]
    pub fn acmp_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(ACMP_IPG_STOP_MODE_A::ACMP_IPG_STOP_MODE_1)
    }
}
impl R {
    #[doc = "Bit 0 - FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn flexio1_ipg_stop_mode(&self) -> FLEXIO1_IPG_STOP_MODE_R {
        FLEXIO1_IPG_STOP_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLEXIO1 ipg_doze mode"]
    #[inline(always)]
    pub fn flexio1_ipg_doze(&self) -> FLEXIO1_IPG_DOZE_R {
        FLEXIO1_IPG_DOZE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - ACMP stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn acmp_ipg_stop_mode(&self) -> ACMP_IPG_STOP_MODE_R {
        ACMP_IPG_STOP_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn flexio1_ipg_stop_mode(&mut self) -> FLEXIO1_IPG_STOP_MODE_W<0> {
        FLEXIO1_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 1 - FLEXIO1 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn flexio1_ipg_doze(&mut self) -> FLEXIO1_IPG_DOZE_W<1> {
        FLEXIO1_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 4 - ACMP stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn acmp_ipg_stop_mode(&mut self) -> ACMP_IPG_STOP_MODE_W<4> {
        ACMP_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR12 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr12](index.html) module"]
pub struct GPR12_SPEC;
impl crate::RegisterSpec for GPR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr12::R](R) reader structure"]
impl crate::Readable for GPR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr12::W](W) writer structure"]
impl crate::Writable for GPR12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR12 to value 0"]
impl crate::Resettable for GPR12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
