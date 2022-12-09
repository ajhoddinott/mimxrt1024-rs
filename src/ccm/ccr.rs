#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSCNT` reader - Oscillator ready counter value. These bits define value of 32KHz counter, that serve as counter for oscillator lock time (count to n+1 ckil's). This is used for oscillator lock time. Current estimation is ~5ms. This counter will be used in ignition sequence and in wake from stop sequence if sbyos bit was defined, to notify that on chip oscillator output is ready for the dpll_ip to use and only then the gate in dpll_ip can be opened."]
pub type OSCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSCNT` writer - Oscillator ready counter value. These bits define value of 32KHz counter, that serve as counter for oscillator lock time (count to n+1 ckil's). This is used for oscillator lock time. Current estimation is ~5ms. This counter will be used in ignition sequence and in wake from stop sequence if sbyos bit was defined, to notify that on chip oscillator output is ready for the dpll_ip to use and only then the gate in dpll_ip can be opened."]
pub type OSCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `COSC_EN` reader - On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
pub type COSC_EN_R = crate::BitReader<COSC_EN_A>;
#[doc = "On chip oscillator enable bit - this bit value is reflected on the output cosc_en\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COSC_EN_A {
    #[doc = "0: disable on chip oscillator"]
    COSC_EN_0 = 0,
    #[doc = "1: enable on chip oscillator"]
    COSC_EN_1 = 1,
}
impl From<COSC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COSC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl COSC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COSC_EN_A {
        match self.bits {
            false => COSC_EN_A::COSC_EN_0,
            true => COSC_EN_A::COSC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_EN_0`"]
    #[inline(always)]
    pub fn is_cosc_en_0(&self) -> bool {
        *self == COSC_EN_A::COSC_EN_0
    }
    #[doc = "Checks if the value of the field is `COSC_EN_1`"]
    #[inline(always)]
    pub fn is_cosc_en_1(&self) -> bool {
        *self == COSC_EN_A::COSC_EN_1
    }
}
#[doc = "Field `COSC_EN` writer - On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
pub type COSC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, COSC_EN_A, O>;
impl<'a, const O: u8> COSC_EN_W<'a, O> {
    #[doc = "disable on chip oscillator"]
    #[inline(always)]
    pub fn cosc_en_0(self) -> &'a mut W {
        self.variant(COSC_EN_A::COSC_EN_0)
    }
    #[doc = "enable on chip oscillator"]
    #[inline(always)]
    pub fn cosc_en_1(self) -> &'a mut W {
        self.variant(COSC_EN_A::COSC_EN_1)
    }
}
#[doc = "Field `REG_BYPASS_COUNT` reader - Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
pub type REG_BYPASS_COUNT_R = crate::FieldReader<u8, REG_BYPASS_COUNT_A>;
#[doc = "Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ\n\nValue on reset: 32"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG_BYPASS_COUNT_A {
    #[doc = "0: no delay"]
    REG_BYPASS_COUNT_0 = 0,
    #[doc = "1: 1 CKIL clock period delay"]
    REG_BYPASS_COUNT_1 = 1,
    #[doc = "63: 63 CKIL clock periods delay"]
    REG_BYPASS_COUNT_63 = 63,
}
impl From<REG_BYPASS_COUNT_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_BYPASS_COUNT_A) -> Self {
        variant as _
    }
}
impl REG_BYPASS_COUNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_BYPASS_COUNT_A> {
        match self.bits {
            0 => Some(REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_0),
            1 => Some(REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_1),
            63 => Some(REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_63),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REG_BYPASS_COUNT_0`"]
    #[inline(always)]
    pub fn is_reg_bypass_count_0(&self) -> bool {
        *self == REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_0
    }
    #[doc = "Checks if the value of the field is `REG_BYPASS_COUNT_1`"]
    #[inline(always)]
    pub fn is_reg_bypass_count_1(&self) -> bool {
        *self == REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_1
    }
    #[doc = "Checks if the value of the field is `REG_BYPASS_COUNT_63`"]
    #[inline(always)]
    pub fn is_reg_bypass_count_63(&self) -> bool {
        *self == REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_63
    }
}
#[doc = "Field `REG_BYPASS_COUNT` writer - Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
pub type REG_BYPASS_COUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCR_SPEC, u8, REG_BYPASS_COUNT_A, 6, O>;
impl<'a, const O: u8> REG_BYPASS_COUNT_W<'a, O> {
    #[doc = "no delay"]
    #[inline(always)]
    pub fn reg_bypass_count_0(self) -> &'a mut W {
        self.variant(REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_0)
    }
    #[doc = "1 CKIL clock period delay"]
    #[inline(always)]
    pub fn reg_bypass_count_1(self) -> &'a mut W {
        self.variant(REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_1)
    }
    #[doc = "63 CKIL clock periods delay"]
    #[inline(always)]
    pub fn reg_bypass_count_63(self) -> &'a mut W {
        self.variant(REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_63)
    }
}
#[doc = "Field `RBC_EN` reader - Enable for REG_BYPASS_COUNTER"]
pub type RBC_EN_R = crate::BitReader<RBC_EN_A>;
#[doc = "Enable for REG_BYPASS_COUNTER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBC_EN_A {
    #[doc = "0: REG_BYPASS_COUNTER disabled"]
    RBC_EN_0 = 0,
    #[doc = "1: REG_BYPASS_COUNTER enabled."]
    RBC_EN_1 = 1,
}
impl From<RBC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RBC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RBC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBC_EN_A {
        match self.bits {
            false => RBC_EN_A::RBC_EN_0,
            true => RBC_EN_A::RBC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RBC_EN_0`"]
    #[inline(always)]
    pub fn is_rbc_en_0(&self) -> bool {
        *self == RBC_EN_A::RBC_EN_0
    }
    #[doc = "Checks if the value of the field is `RBC_EN_1`"]
    #[inline(always)]
    pub fn is_rbc_en_1(&self) -> bool {
        *self == RBC_EN_A::RBC_EN_1
    }
}
#[doc = "Field `RBC_EN` writer - Enable for REG_BYPASS_COUNTER"]
pub type RBC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, RBC_EN_A, O>;
impl<'a, const O: u8> RBC_EN_W<'a, O> {
    #[doc = "REG_BYPASS_COUNTER disabled"]
    #[inline(always)]
    pub fn rbc_en_0(self) -> &'a mut W {
        self.variant(RBC_EN_A::RBC_EN_0)
    }
    #[doc = "REG_BYPASS_COUNTER enabled."]
    #[inline(always)]
    pub fn rbc_en_1(self) -> &'a mut W {
        self.variant(RBC_EN_A::RBC_EN_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Oscillator ready counter value. These bits define value of 32KHz counter, that serve as counter for oscillator lock time (count to n+1 ckil's). This is used for oscillator lock time. Current estimation is ~5ms. This counter will be used in ignition sequence and in wake from stop sequence if sbyos bit was defined, to notify that on chip oscillator output is ready for the dpll_ip to use and only then the gate in dpll_ip can be opened."]
    #[inline(always)]
    pub fn oscnt(&self) -> OSCNT_R {
        OSCNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    #[inline(always)]
    pub fn cosc_en(&self) -> COSC_EN_R {
        COSC_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 21:26 - Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
    #[inline(always)]
    pub fn reg_bypass_count(&self) -> REG_BYPASS_COUNT_R {
        REG_BYPASS_COUNT_R::new(((self.bits >> 21) & 0x3f) as u8)
    }
    #[doc = "Bit 27 - Enable for REG_BYPASS_COUNTER"]
    #[inline(always)]
    pub fn rbc_en(&self) -> RBC_EN_R {
        RBC_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Oscillator ready counter value. These bits define value of 32KHz counter, that serve as counter for oscillator lock time (count to n+1 ckil's). This is used for oscillator lock time. Current estimation is ~5ms. This counter will be used in ignition sequence and in wake from stop sequence if sbyos bit was defined, to notify that on chip oscillator output is ready for the dpll_ip to use and only then the gate in dpll_ip can be opened."]
    #[inline(always)]
    #[must_use]
    pub fn oscnt(&mut self) -> OSCNT_W<0> {
        OSCNT_W::new(self)
    }
    #[doc = "Bit 12 - On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    #[inline(always)]
    #[must_use]
    pub fn cosc_en(&mut self) -> COSC_EN_W<12> {
        COSC_EN_W::new(self)
    }
    #[doc = "Bits 21:26 - Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bypass_count(&mut self) -> REG_BYPASS_COUNT_W<21> {
        REG_BYPASS_COUNT_W::new(self)
    }
    #[doc = "Bit 27 - Enable for REG_BYPASS_COUNTER"]
    #[inline(always)]
    #[must_use]
    pub fn rbc_en(&mut self) -> RBC_EN_W<27> {
        RBC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0x0401_107f"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0401_107f;
}
