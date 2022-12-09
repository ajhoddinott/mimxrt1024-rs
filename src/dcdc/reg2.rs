#[doc = "Register `REG2` reader"]
pub struct R(crate::R<REG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG2` writer"]
pub struct W(crate::W<REG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG2_SPEC>;
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
impl From<crate::W<REG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOOPCTRL_DC_FF` reader - Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
pub type LOOPCTRL_DC_FF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOOPCTRL_DC_FF` writer - Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
pub type LOOPCTRL_DC_FF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG2_SPEC, u8, u8, 3, O>;
#[doc = "Field `LOOPCTRL_EN_RCSCALE` reader - Enable RC Scale"]
pub type LOOPCTRL_EN_RCSCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOOPCTRL_EN_RCSCALE` writer - Enable RC Scale"]
pub type LOOPCTRL_EN_RCSCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REG2_SPEC, u8, u8, 3, O>;
#[doc = "Field `LOOPCTRL_RCSCALE_THRSH` reader - Increase the threshold detection for RC scale circuit."]
pub type LOOPCTRL_RCSCALE_THRSH_R = crate::BitReader<LOOPCTRL_RCSCALE_THRSH_A>;
#[doc = "Increase the threshold detection for RC scale circuit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPCTRL_RCSCALE_THRSH_A {
    #[doc = "0: Do not increase the threshold detection for RC scale circuit."]
    NO_INCREASE_THRSH = 0,
    #[doc = "1: Increase the threshold detection for RC scale circuit."]
    INCREASE_THRSH = 1,
}
impl From<LOOPCTRL_RCSCALE_THRSH_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPCTRL_RCSCALE_THRSH_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPCTRL_RCSCALE_THRSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPCTRL_RCSCALE_THRSH_A {
        match self.bits {
            false => LOOPCTRL_RCSCALE_THRSH_A::NO_INCREASE_THRSH,
            true => LOOPCTRL_RCSCALE_THRSH_A::INCREASE_THRSH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INCREASE_THRSH`"]
    #[inline(always)]
    pub fn is_no_increase_thrsh(&self) -> bool {
        *self == LOOPCTRL_RCSCALE_THRSH_A::NO_INCREASE_THRSH
    }
    #[doc = "Checks if the value of the field is `INCREASE_THRSH`"]
    #[inline(always)]
    pub fn is_increase_thrsh(&self) -> bool {
        *self == LOOPCTRL_RCSCALE_THRSH_A::INCREASE_THRSH
    }
}
#[doc = "Field `LOOPCTRL_RCSCALE_THRSH` writer - Increase the threshold detection for RC scale circuit."]
pub type LOOPCTRL_RCSCALE_THRSH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG2_SPEC, LOOPCTRL_RCSCALE_THRSH_A, O>;
impl<'a, const O: u8> LOOPCTRL_RCSCALE_THRSH_W<'a, O> {
    #[doc = "Do not increase the threshold detection for RC scale circuit."]
    #[inline(always)]
    pub fn no_increase_thrsh(self) -> &'a mut W {
        self.variant(LOOPCTRL_RCSCALE_THRSH_A::NO_INCREASE_THRSH)
    }
    #[doc = "Increase the threshold detection for RC scale circuit."]
    #[inline(always)]
    pub fn increase_thrsh(self) -> &'a mut W {
        self.variant(LOOPCTRL_RCSCALE_THRSH_A::INCREASE_THRSH)
    }
}
#[doc = "Field `LOOPCTRL_HYST_SIGN` reader - Invert the sign of the hysteresis in DC-DC analog comparators."]
pub type LOOPCTRL_HYST_SIGN_R = crate::BitReader<LOOPCTRL_HYST_SIGN_A>;
#[doc = "Invert the sign of the hysteresis in DC-DC analog comparators.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPCTRL_HYST_SIGN_A {
    #[doc = "0: Do not invert sign of the hysteresis"]
    NOINVERT = 0,
    #[doc = "1: Invert sign of the hysteresis"]
    INVERT = 1,
}
impl From<LOOPCTRL_HYST_SIGN_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPCTRL_HYST_SIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPCTRL_HYST_SIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPCTRL_HYST_SIGN_A {
        match self.bits {
            false => LOOPCTRL_HYST_SIGN_A::NOINVERT,
            true => LOOPCTRL_HYST_SIGN_A::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NOINVERT`"]
    #[inline(always)]
    pub fn is_noinvert(&self) -> bool {
        *self == LOOPCTRL_HYST_SIGN_A::NOINVERT
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == LOOPCTRL_HYST_SIGN_A::INVERT
    }
}
#[doc = "Field `LOOPCTRL_HYST_SIGN` writer - Invert the sign of the hysteresis in DC-DC analog comparators."]
pub type LOOPCTRL_HYST_SIGN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG2_SPEC, LOOPCTRL_HYST_SIGN_A, O>;
impl<'a, const O: u8> LOOPCTRL_HYST_SIGN_W<'a, O> {
    #[doc = "Do not invert sign of the hysteresis"]
    #[inline(always)]
    pub fn noinvert(self) -> &'a mut W {
        self.variant(LOOPCTRL_HYST_SIGN_A::NOINVERT)
    }
    #[doc = "Invert sign of the hysteresis"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(LOOPCTRL_HYST_SIGN_A::INVERT)
    }
}
#[doc = "Field `DISABLE_PULSE_SKIP` reader - Disable Pulse Skip"]
pub type DISABLE_PULSE_SKIP_R = crate::BitReader<DISABLE_PULSE_SKIP_A>;
#[doc = "Disable Pulse Skip\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISABLE_PULSE_SKIP_A {
    #[doc = "0: DCDC will be idle to save current dissipation when the duty cycle get to the low limit which is set by NEGLIMIT_IN."]
    IDLE = 0,
    #[doc = "1: DCDC will keep working with the low limited duty cycle NEGLIMIT_IN."]
    NOT_IDLE = 1,
}
impl From<DISABLE_PULSE_SKIP_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLE_PULSE_SKIP_A) -> Self {
        variant as u8 != 0
    }
}
impl DISABLE_PULSE_SKIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLE_PULSE_SKIP_A {
        match self.bits {
            false => DISABLE_PULSE_SKIP_A::IDLE,
            true => DISABLE_PULSE_SKIP_A::NOT_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == DISABLE_PULSE_SKIP_A::IDLE
    }
    #[doc = "Checks if the value of the field is `NOT_IDLE`"]
    #[inline(always)]
    pub fn is_not_idle(&self) -> bool {
        *self == DISABLE_PULSE_SKIP_A::NOT_IDLE
    }
}
#[doc = "Field `DISABLE_PULSE_SKIP` writer - Disable Pulse Skip"]
pub type DISABLE_PULSE_SKIP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG2_SPEC, DISABLE_PULSE_SKIP_A, O>;
impl<'a, const O: u8> DISABLE_PULSE_SKIP_W<'a, O> {
    #[doc = "DCDC will be idle to save current dissipation when the duty cycle get to the low limit which is set by NEGLIMIT_IN."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(DISABLE_PULSE_SKIP_A::IDLE)
    }
    #[doc = "DCDC will keep working with the low limited duty cycle NEGLIMIT_IN."]
    #[inline(always)]
    pub fn not_idle(self) -> &'a mut W {
        self.variant(DISABLE_PULSE_SKIP_A::NOT_IDLE)
    }
}
#[doc = "Field `DCM_SET_CTRL` reader - DCM Set Control"]
pub type DCM_SET_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `DCM_SET_CTRL` writer - DCM Set Control"]
pub type DCM_SET_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 6:8 - Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
    #[inline(always)]
    pub fn loopctrl_dc_ff(&self) -> LOOPCTRL_DC_FF_R {
        LOOPCTRL_DC_FF_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Enable RC Scale"]
    #[inline(always)]
    pub fn loopctrl_en_rcscale(&self) -> LOOPCTRL_EN_RCSCALE_R {
        LOOPCTRL_EN_RCSCALE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Increase the threshold detection for RC scale circuit."]
    #[inline(always)]
    pub fn loopctrl_rcscale_thrsh(&self) -> LOOPCTRL_RCSCALE_THRSH_R {
        LOOPCTRL_RCSCALE_THRSH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Invert the sign of the hysteresis in DC-DC analog comparators."]
    #[inline(always)]
    pub fn loopctrl_hyst_sign(&self) -> LOOPCTRL_HYST_SIGN_R {
        LOOPCTRL_HYST_SIGN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 27 - Disable Pulse Skip"]
    #[inline(always)]
    pub fn disable_pulse_skip(&self) -> DISABLE_PULSE_SKIP_R {
        DISABLE_PULSE_SKIP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DCM Set Control"]
    #[inline(always)]
    pub fn dcm_set_ctrl(&self) -> DCM_SET_CTRL_R {
        DCM_SET_CTRL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:8 - Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
    #[inline(always)]
    #[must_use]
    pub fn loopctrl_dc_ff(&mut self) -> LOOPCTRL_DC_FF_W<6> {
        LOOPCTRL_DC_FF_W::new(self)
    }
    #[doc = "Bits 9:11 - Enable RC Scale"]
    #[inline(always)]
    #[must_use]
    pub fn loopctrl_en_rcscale(&mut self) -> LOOPCTRL_EN_RCSCALE_W<9> {
        LOOPCTRL_EN_RCSCALE_W::new(self)
    }
    #[doc = "Bit 12 - Increase the threshold detection for RC scale circuit."]
    #[inline(always)]
    #[must_use]
    pub fn loopctrl_rcscale_thrsh(&mut self) -> LOOPCTRL_RCSCALE_THRSH_W<12> {
        LOOPCTRL_RCSCALE_THRSH_W::new(self)
    }
    #[doc = "Bit 13 - Invert the sign of the hysteresis in DC-DC analog comparators."]
    #[inline(always)]
    #[must_use]
    pub fn loopctrl_hyst_sign(&mut self) -> LOOPCTRL_HYST_SIGN_W<13> {
        LOOPCTRL_HYST_SIGN_W::new(self)
    }
    #[doc = "Bit 27 - Disable Pulse Skip"]
    #[inline(always)]
    #[must_use]
    pub fn disable_pulse_skip(&mut self) -> DISABLE_PULSE_SKIP_W<27> {
        DISABLE_PULSE_SKIP_W::new(self)
    }
    #[doc = "Bit 28 - DCM Set Control"]
    #[inline(always)]
    #[must_use]
    pub fn dcm_set_ctrl(&mut self) -> DCM_SET_CTRL_W<28> {
        DCM_SET_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg2](index.html) module"]
pub struct REG2_SPEC;
impl crate::RegisterSpec for REG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg2::R](R) reader structure"]
impl crate::Readable for REG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg2::W](W) writer structure"]
impl crate::Writable for REG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG2 to value 0x02"]
impl crate::Resettable for REG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
