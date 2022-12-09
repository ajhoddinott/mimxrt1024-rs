#[doc = "Register `SMCAPTCTRLB` reader"]
pub struct R(crate::R<SMCAPTCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCAPTCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCAPTCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCAPTCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCAPTCTRLB` writer"]
pub struct W(crate::W<SMCAPTCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCAPTCTRLB_SPEC>;
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
impl From<crate::W<SMCAPTCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCAPTCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARMB` reader - Arm B"]
pub type ARMB_R = crate::BitReader<ARMB_A>;
#[doc = "Arm B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARMB_A {
    #[doc = "0: Input capture operation is disabled."]
    DISABLED = 0,
    #[doc = "1: Input capture operation as specified by CAPTCTRLB\\[EDGBx\\]
is enabled."]
    ENABLED = 1,
}
impl From<ARMB_A> for bool {
    #[inline(always)]
    fn from(variant: ARMB_A) -> Self {
        variant as u8 != 0
    }
}
impl ARMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARMB_A {
        match self.bits {
            false => ARMB_A::DISABLED,
            true => ARMB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARMB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARMB_A::ENABLED
    }
}
#[doc = "Field `ARMB` writer - Arm B"]
pub type ARMB_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCAPTCTRLB_SPEC, ARMB_A, O>;
impl<'a, const O: u8> ARMB_W<'a, O> {
    #[doc = "Input capture operation is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARMB_A::DISABLED)
    }
    #[doc = "Input capture operation as specified by CAPTCTRLB\\[EDGBx\\]
is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARMB_A::ENABLED)
    }
}
#[doc = "Field `ONESHOTB` reader - One Shot Mode B"]
pub type ONESHOTB_R = crate::BitReader<ONESHOTB_A>;
#[doc = "One Shot Mode B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONESHOTB_A {
    #[doc = "0: Free Running"]
    FREE_RUNNING = 0,
    #[doc = "1: One Shot"]
    ONE_SHOT = 1,
}
impl From<ONESHOTB_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOTB_A) -> Self {
        variant as u8 != 0
    }
}
impl ONESHOTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOTB_A {
        match self.bits {
            false => ONESHOTB_A::FREE_RUNNING,
            true => ONESHOTB_A::ONE_SHOT,
        }
    }
    #[doc = "Checks if the value of the field is `FREE_RUNNING`"]
    #[inline(always)]
    pub fn is_free_running(&self) -> bool {
        *self == ONESHOTB_A::FREE_RUNNING
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == ONESHOTB_A::ONE_SHOT
    }
}
#[doc = "Field `ONESHOTB` writer - One Shot Mode B"]
pub type ONESHOTB_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCAPTCTRLB_SPEC, ONESHOTB_A, O>;
impl<'a, const O: u8> ONESHOTB_W<'a, O> {
    #[doc = "Free Running"]
    #[inline(always)]
    pub fn free_running(self) -> &'a mut W {
        self.variant(ONESHOTB_A::FREE_RUNNING)
    }
    #[doc = "One Shot"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(ONESHOTB_A::ONE_SHOT)
    }
}
#[doc = "Field `EDGB0` reader - Edge B 0"]
pub type EDGB0_R = crate::FieldReader<u8, EDGB0_A>;
#[doc = "Edge B 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGB0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Capture falling edges"]
    FALLING_EDGE = 1,
    #[doc = "2: Capture rising edges"]
    RISING_EDGE = 2,
    #[doc = "3: Capture any edge"]
    ANY_EDGE = 3,
}
impl From<EDGB0_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGB0_A) -> Self {
        variant as _
    }
}
impl EDGB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGB0_A {
        match self.bits {
            0 => EDGB0_A::DISABLED,
            1 => EDGB0_A::FALLING_EDGE,
            2 => EDGB0_A::RISING_EDGE,
            3 => EDGB0_A::ANY_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDGB0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EDGB0_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EDGB0_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `ANY_EDGE`"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == EDGB0_A::ANY_EDGE
    }
}
#[doc = "Field `EDGB0` writer - Edge B 0"]
pub type EDGB0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SMCAPTCTRLB_SPEC, u8, EDGB0_A, 2, O>;
impl<'a, const O: u8> EDGB0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDGB0_A::DISABLED)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EDGB0_A::FALLING_EDGE)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EDGB0_A::RISING_EDGE)
    }
    #[doc = "Capture any edge"]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut W {
        self.variant(EDGB0_A::ANY_EDGE)
    }
}
#[doc = "Field `EDGB1` reader - Edge B 1"]
pub type EDGB1_R = crate::FieldReader<u8, EDGB1_A>;
#[doc = "Edge B 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGB1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Capture falling edges"]
    FALLING_EDGE = 1,
    #[doc = "2: Capture rising edges"]
    RISING_EDGE = 2,
    #[doc = "3: Capture any edge"]
    ANY_EDGE = 3,
}
impl From<EDGB1_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGB1_A) -> Self {
        variant as _
    }
}
impl EDGB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGB1_A {
        match self.bits {
            0 => EDGB1_A::DISABLED,
            1 => EDGB1_A::FALLING_EDGE,
            2 => EDGB1_A::RISING_EDGE,
            3 => EDGB1_A::ANY_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDGB1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EDGB1_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EDGB1_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `ANY_EDGE`"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == EDGB1_A::ANY_EDGE
    }
}
#[doc = "Field `EDGB1` writer - Edge B 1"]
pub type EDGB1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SMCAPTCTRLB_SPEC, u8, EDGB1_A, 2, O>;
impl<'a, const O: u8> EDGB1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDGB1_A::DISABLED)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EDGB1_A::FALLING_EDGE)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EDGB1_A::RISING_EDGE)
    }
    #[doc = "Capture any edge"]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut W {
        self.variant(EDGB1_A::ANY_EDGE)
    }
}
#[doc = "Field `INP_SELB` reader - Input Select B"]
pub type INP_SELB_R = crate::BitReader<INP_SELB_A>;
#[doc = "Input Select B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INP_SELB_A {
    #[doc = "0: Raw PWM_B input signal selected as source."]
    PWM_B = 0,
    #[doc = "1: Edge Counter"]
    EDGE_COUNTER = 1,
}
impl From<INP_SELB_A> for bool {
    #[inline(always)]
    fn from(variant: INP_SELB_A) -> Self {
        variant as u8 != 0
    }
}
impl INP_SELB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INP_SELB_A {
        match self.bits {
            false => INP_SELB_A::PWM_B,
            true => INP_SELB_A::EDGE_COUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_B`"]
    #[inline(always)]
    pub fn is_pwm_b(&self) -> bool {
        *self == INP_SELB_A::PWM_B
    }
    #[doc = "Checks if the value of the field is `EDGE_COUNTER`"]
    #[inline(always)]
    pub fn is_edge_counter(&self) -> bool {
        *self == INP_SELB_A::EDGE_COUNTER
    }
}
#[doc = "Field `INP_SELB` writer - Input Select B"]
pub type INP_SELB_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCAPTCTRLB_SPEC, INP_SELB_A, O>;
impl<'a, const O: u8> INP_SELB_W<'a, O> {
    #[doc = "Raw PWM_B input signal selected as source."]
    #[inline(always)]
    pub fn pwm_b(self) -> &'a mut W {
        self.variant(INP_SELB_A::PWM_B)
    }
    #[doc = "Edge Counter"]
    #[inline(always)]
    pub fn edge_counter(self) -> &'a mut W {
        self.variant(INP_SELB_A::EDGE_COUNTER)
    }
}
#[doc = "Field `EDGCNTB_EN` reader - Edge Counter B Enable"]
pub type EDGCNTB_EN_R = crate::BitReader<EDGCNTB_EN_A>;
#[doc = "Edge Counter B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGCNTB_EN_A {
    #[doc = "0: Edge counter disabled and held in reset"]
    DISABLED = 0,
    #[doc = "1: Edge counter enabled"]
    ENABLED = 1,
}
impl From<EDGCNTB_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EDGCNTB_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGCNTB_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGCNTB_EN_A {
        match self.bits {
            false => EDGCNTB_EN_A::DISABLED,
            true => EDGCNTB_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDGCNTB_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDGCNTB_EN_A::ENABLED
    }
}
#[doc = "Field `EDGCNTB_EN` writer - Edge Counter B Enable"]
pub type EDGCNTB_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SMCAPTCTRLB_SPEC, EDGCNTB_EN_A, O>;
impl<'a, const O: u8> EDGCNTB_EN_W<'a, O> {
    #[doc = "Edge counter disabled and held in reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDGCNTB_EN_A::DISABLED)
    }
    #[doc = "Edge counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EDGCNTB_EN_A::ENABLED)
    }
}
#[doc = "Field `CFBWM` reader - Capture B FIFOs Water Mark"]
pub type CFBWM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFBWM` writer - Capture B FIFOs Water Mark"]
pub type CFBWM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMCAPTCTRLB_SPEC, u8, u8, 2, O>;
#[doc = "Field `CB0CNT` reader - Capture B0 FIFO Word Count"]
pub type CB0CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CB1CNT` reader - Capture B1 FIFO Word Count"]
pub type CB1CNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Arm B"]
    #[inline(always)]
    pub fn armb(&self) -> ARMB_R {
        ARMB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - One Shot Mode B"]
    #[inline(always)]
    pub fn oneshotb(&self) -> ONESHOTB_R {
        ONESHOTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Edge B 0"]
    #[inline(always)]
    pub fn edgb0(&self) -> EDGB0_R {
        EDGB0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Edge B 1"]
    #[inline(always)]
    pub fn edgb1(&self) -> EDGB1_R {
        EDGB1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Input Select B"]
    #[inline(always)]
    pub fn inp_selb(&self) -> INP_SELB_R {
        INP_SELB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Edge Counter B Enable"]
    #[inline(always)]
    pub fn edgcntb_en(&self) -> EDGCNTB_EN_R {
        EDGCNTB_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub fn cfbwm(&self) -> CFBWM_R {
        CFBWM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Capture B0 FIFO Word Count"]
    #[inline(always)]
    pub fn cb0cnt(&self) -> CB0CNT_R {
        CB0CNT_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Capture B1 FIFO Word Count"]
    #[inline(always)]
    pub fn cb1cnt(&self) -> CB1CNT_R {
        CB1CNT_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Arm B"]
    #[inline(always)]
    #[must_use]
    pub fn armb(&mut self) -> ARMB_W<0> {
        ARMB_W::new(self)
    }
    #[doc = "Bit 1 - One Shot Mode B"]
    #[inline(always)]
    #[must_use]
    pub fn oneshotb(&mut self) -> ONESHOTB_W<1> {
        ONESHOTB_W::new(self)
    }
    #[doc = "Bits 2:3 - Edge B 0"]
    #[inline(always)]
    #[must_use]
    pub fn edgb0(&mut self) -> EDGB0_W<2> {
        EDGB0_W::new(self)
    }
    #[doc = "Bits 4:5 - Edge B 1"]
    #[inline(always)]
    #[must_use]
    pub fn edgb1(&mut self) -> EDGB1_W<4> {
        EDGB1_W::new(self)
    }
    #[doc = "Bit 6 - Input Select B"]
    #[inline(always)]
    #[must_use]
    pub fn inp_selb(&mut self) -> INP_SELB_W<6> {
        INP_SELB_W::new(self)
    }
    #[doc = "Bit 7 - Edge Counter B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn edgcntb_en(&mut self) -> EDGCNTB_EN_W<7> {
        EDGCNTB_EN_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture B FIFOs Water Mark"]
    #[inline(always)]
    #[must_use]
    pub fn cfbwm(&mut self) -> CFBWM_W<8> {
        CFBWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Control B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcaptctrlb](index.html) module"]
pub struct SMCAPTCTRLB_SPEC;
impl crate::RegisterSpec for SMCAPTCTRLB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcaptctrlb::R](R) reader structure"]
impl crate::Readable for SMCAPTCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcaptctrlb::W](W) writer structure"]
impl crate::Writable for SMCAPTCTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCAPTCTRLB to value 0"]
impl crate::Resettable for SMCAPTCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
