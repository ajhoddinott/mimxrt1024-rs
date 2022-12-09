#[doc = "Register `SMCAPTCTRLX` reader"]
pub struct R(crate::R<SMCAPTCTRLX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCAPTCTRLX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCAPTCTRLX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCAPTCTRLX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCAPTCTRLX` writer"]
pub struct W(crate::W<SMCAPTCTRLX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCAPTCTRLX_SPEC>;
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
impl From<crate::W<SMCAPTCTRLX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCAPTCTRLX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARMX` reader - Arm X"]
pub type ARMX_R = crate::BitReader<ARMX_A>;
#[doc = "Arm X\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARMX_A {
    #[doc = "0: Input capture operation is disabled."]
    DISABLED = 0,
    #[doc = "1: Input capture operation as specified by CAPTCTRLX\\[EDGXx\\]
is enabled."]
    ENABLED = 1,
}
impl From<ARMX_A> for bool {
    #[inline(always)]
    fn from(variant: ARMX_A) -> Self {
        variant as u8 != 0
    }
}
impl ARMX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARMX_A {
        match self.bits {
            false => ARMX_A::DISABLED,
            true => ARMX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARMX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARMX_A::ENABLED
    }
}
#[doc = "Field `ARMX` writer - Arm X"]
pub type ARMX_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCAPTCTRLX_SPEC, ARMX_A, O>;
impl<'a, const O: u8> ARMX_W<'a, O> {
    #[doc = "Input capture operation is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARMX_A::DISABLED)
    }
    #[doc = "Input capture operation as specified by CAPTCTRLX\\[EDGXx\\]
is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARMX_A::ENABLED)
    }
}
#[doc = "Field `ONESHOTX` reader - One Shot Mode Aux"]
pub type ONESHOTX_R = crate::BitReader<ONESHOTX_A>;
#[doc = "One Shot Mode Aux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONESHOTX_A {
    #[doc = "0: Free Running"]
    FREE_RUNNING = 0,
    #[doc = "1: One Shot"]
    ONE_SHOT = 1,
}
impl From<ONESHOTX_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOTX_A) -> Self {
        variant as u8 != 0
    }
}
impl ONESHOTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOTX_A {
        match self.bits {
            false => ONESHOTX_A::FREE_RUNNING,
            true => ONESHOTX_A::ONE_SHOT,
        }
    }
    #[doc = "Checks if the value of the field is `FREE_RUNNING`"]
    #[inline(always)]
    pub fn is_free_running(&self) -> bool {
        *self == ONESHOTX_A::FREE_RUNNING
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == ONESHOTX_A::ONE_SHOT
    }
}
#[doc = "Field `ONESHOTX` writer - One Shot Mode Aux"]
pub type ONESHOTX_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCAPTCTRLX_SPEC, ONESHOTX_A, O>;
impl<'a, const O: u8> ONESHOTX_W<'a, O> {
    #[doc = "Free Running"]
    #[inline(always)]
    pub fn free_running(self) -> &'a mut W {
        self.variant(ONESHOTX_A::FREE_RUNNING)
    }
    #[doc = "One Shot"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(ONESHOTX_A::ONE_SHOT)
    }
}
#[doc = "Field `EDGX0` reader - Edge X 0"]
pub type EDGX0_R = crate::FieldReader<u8, EDGX0_A>;
#[doc = "Edge X 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGX0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Capture falling edges"]
    FALLING_EDGE = 1,
    #[doc = "2: Capture rising edges"]
    RISING_EDGE = 2,
    #[doc = "3: Capture any edge"]
    ANY_EDGE = 3,
}
impl From<EDGX0_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGX0_A) -> Self {
        variant as _
    }
}
impl EDGX0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGX0_A {
        match self.bits {
            0 => EDGX0_A::DISABLED,
            1 => EDGX0_A::FALLING_EDGE,
            2 => EDGX0_A::RISING_EDGE,
            3 => EDGX0_A::ANY_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDGX0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EDGX0_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EDGX0_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `ANY_EDGE`"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == EDGX0_A::ANY_EDGE
    }
}
#[doc = "Field `EDGX0` writer - Edge X 0"]
pub type EDGX0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SMCAPTCTRLX_SPEC, u8, EDGX0_A, 2, O>;
impl<'a, const O: u8> EDGX0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDGX0_A::DISABLED)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EDGX0_A::FALLING_EDGE)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EDGX0_A::RISING_EDGE)
    }
    #[doc = "Capture any edge"]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut W {
        self.variant(EDGX0_A::ANY_EDGE)
    }
}
#[doc = "Field `EDGX1` reader - Edge X 1"]
pub type EDGX1_R = crate::FieldReader<u8, EDGX1_A>;
#[doc = "Edge X 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGX1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Capture falling edges"]
    FALLING_EDGE = 1,
    #[doc = "2: Capture rising edges"]
    RISING_EDGE = 2,
    #[doc = "3: Capture any edge"]
    ANY_EDGE = 3,
}
impl From<EDGX1_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGX1_A) -> Self {
        variant as _
    }
}
impl EDGX1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGX1_A {
        match self.bits {
            0 => EDGX1_A::DISABLED,
            1 => EDGX1_A::FALLING_EDGE,
            2 => EDGX1_A::RISING_EDGE,
            3 => EDGX1_A::ANY_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDGX1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EDGX1_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EDGX1_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `ANY_EDGE`"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == EDGX1_A::ANY_EDGE
    }
}
#[doc = "Field `EDGX1` writer - Edge X 1"]
pub type EDGX1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SMCAPTCTRLX_SPEC, u8, EDGX1_A, 2, O>;
impl<'a, const O: u8> EDGX1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDGX1_A::DISABLED)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EDGX1_A::FALLING_EDGE)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EDGX1_A::RISING_EDGE)
    }
    #[doc = "Capture any edge"]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut W {
        self.variant(EDGX1_A::ANY_EDGE)
    }
}
#[doc = "Field `INP_SELX` reader - Input Select X"]
pub type INP_SELX_R = crate::BitReader<INP_SELX_A>;
#[doc = "Input Select X\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INP_SELX_A {
    #[doc = "0: Raw PWM_X input signal selected as source."]
    PWM_X = 0,
    #[doc = "1: Edge Counter"]
    EDGE_COUNTER = 1,
}
impl From<INP_SELX_A> for bool {
    #[inline(always)]
    fn from(variant: INP_SELX_A) -> Self {
        variant as u8 != 0
    }
}
impl INP_SELX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INP_SELX_A {
        match self.bits {
            false => INP_SELX_A::PWM_X,
            true => INP_SELX_A::EDGE_COUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_X`"]
    #[inline(always)]
    pub fn is_pwm_x(&self) -> bool {
        *self == INP_SELX_A::PWM_X
    }
    #[doc = "Checks if the value of the field is `EDGE_COUNTER`"]
    #[inline(always)]
    pub fn is_edge_counter(&self) -> bool {
        *self == INP_SELX_A::EDGE_COUNTER
    }
}
#[doc = "Field `INP_SELX` writer - Input Select X"]
pub type INP_SELX_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCAPTCTRLX_SPEC, INP_SELX_A, O>;
impl<'a, const O: u8> INP_SELX_W<'a, O> {
    #[doc = "Raw PWM_X input signal selected as source."]
    #[inline(always)]
    pub fn pwm_x(self) -> &'a mut W {
        self.variant(INP_SELX_A::PWM_X)
    }
    #[doc = "Edge Counter"]
    #[inline(always)]
    pub fn edge_counter(self) -> &'a mut W {
        self.variant(INP_SELX_A::EDGE_COUNTER)
    }
}
#[doc = "Field `EDGCNTX_EN` reader - Edge Counter X Enable"]
pub type EDGCNTX_EN_R = crate::BitReader<EDGCNTX_EN_A>;
#[doc = "Edge Counter X Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGCNTX_EN_A {
    #[doc = "0: Edge counter disabled and held in reset"]
    DISABLED = 0,
    #[doc = "1: Edge counter enabled"]
    ENABLED = 1,
}
impl From<EDGCNTX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EDGCNTX_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGCNTX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGCNTX_EN_A {
        match self.bits {
            false => EDGCNTX_EN_A::DISABLED,
            true => EDGCNTX_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDGCNTX_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDGCNTX_EN_A::ENABLED
    }
}
#[doc = "Field `EDGCNTX_EN` writer - Edge Counter X Enable"]
pub type EDGCNTX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SMCAPTCTRLX_SPEC, EDGCNTX_EN_A, O>;
impl<'a, const O: u8> EDGCNTX_EN_W<'a, O> {
    #[doc = "Edge counter disabled and held in reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDGCNTX_EN_A::DISABLED)
    }
    #[doc = "Edge counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EDGCNTX_EN_A::ENABLED)
    }
}
#[doc = "Field `CFXWM` reader - Capture X FIFOs Water Mark"]
pub type CFXWM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFXWM` writer - Capture X FIFOs Water Mark"]
pub type CFXWM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMCAPTCTRLX_SPEC, u8, u8, 2, O>;
#[doc = "Field `CX0CNT` reader - Capture X0 FIFO Word Count"]
pub type CX0CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CX1CNT` reader - Capture X1 FIFO Word Count"]
pub type CX1CNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Arm X"]
    #[inline(always)]
    pub fn armx(&self) -> ARMX_R {
        ARMX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - One Shot Mode Aux"]
    #[inline(always)]
    pub fn oneshotx(&self) -> ONESHOTX_R {
        ONESHOTX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Edge X 0"]
    #[inline(always)]
    pub fn edgx0(&self) -> EDGX0_R {
        EDGX0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Edge X 1"]
    #[inline(always)]
    pub fn edgx1(&self) -> EDGX1_R {
        EDGX1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Input Select X"]
    #[inline(always)]
    pub fn inp_selx(&self) -> INP_SELX_R {
        INP_SELX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Edge Counter X Enable"]
    #[inline(always)]
    pub fn edgcntx_en(&self) -> EDGCNTX_EN_R {
        EDGCNTX_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub fn cfxwm(&self) -> CFXWM_R {
        CFXWM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Capture X0 FIFO Word Count"]
    #[inline(always)]
    pub fn cx0cnt(&self) -> CX0CNT_R {
        CX0CNT_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Capture X1 FIFO Word Count"]
    #[inline(always)]
    pub fn cx1cnt(&self) -> CX1CNT_R {
        CX1CNT_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Arm X"]
    #[inline(always)]
    #[must_use]
    pub fn armx(&mut self) -> ARMX_W<0> {
        ARMX_W::new(self)
    }
    #[doc = "Bit 1 - One Shot Mode Aux"]
    #[inline(always)]
    #[must_use]
    pub fn oneshotx(&mut self) -> ONESHOTX_W<1> {
        ONESHOTX_W::new(self)
    }
    #[doc = "Bits 2:3 - Edge X 0"]
    #[inline(always)]
    #[must_use]
    pub fn edgx0(&mut self) -> EDGX0_W<2> {
        EDGX0_W::new(self)
    }
    #[doc = "Bits 4:5 - Edge X 1"]
    #[inline(always)]
    #[must_use]
    pub fn edgx1(&mut self) -> EDGX1_W<4> {
        EDGX1_W::new(self)
    }
    #[doc = "Bit 6 - Input Select X"]
    #[inline(always)]
    #[must_use]
    pub fn inp_selx(&mut self) -> INP_SELX_W<6> {
        INP_SELX_W::new(self)
    }
    #[doc = "Bit 7 - Edge Counter X Enable"]
    #[inline(always)]
    #[must_use]
    pub fn edgcntx_en(&mut self) -> EDGCNTX_EN_W<7> {
        EDGCNTX_EN_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture X FIFOs Water Mark"]
    #[inline(always)]
    #[must_use]
    pub fn cfxwm(&mut self) -> CFXWM_W<8> {
        CFXWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Control X Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcaptctrlx](index.html) module"]
pub struct SMCAPTCTRLX_SPEC;
impl crate::RegisterSpec for SMCAPTCTRLX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcaptctrlx::R](R) reader structure"]
impl crate::Readable for SMCAPTCTRLX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcaptctrlx::W](W) writer structure"]
impl crate::Writable for SMCAPTCTRLX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCAPTCTRLX to value 0"]
impl crate::Resettable for SMCAPTCTRLX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
