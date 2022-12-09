#[doc = "Register `SMCAPTCTRLA` reader"]
pub struct R(crate::R<SMCAPTCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCAPTCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCAPTCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCAPTCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCAPTCTRLA` writer"]
pub struct W(crate::W<SMCAPTCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCAPTCTRLA_SPEC>;
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
impl From<crate::W<SMCAPTCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCAPTCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARMA` reader - Arm A"]
pub type ARMA_R = crate::BitReader<ARMA_A>;
#[doc = "Arm A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARMA_A {
    #[doc = "0: Input capture operation is disabled."]
    DISABLED = 0,
    #[doc = "1: Input capture operation as specified by CAPTCTRLA\\[EDGAx\\]
is enabled."]
    ENABLED = 1,
}
impl From<ARMA_A> for bool {
    #[inline(always)]
    fn from(variant: ARMA_A) -> Self {
        variant as u8 != 0
    }
}
impl ARMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARMA_A {
        match self.bits {
            false => ARMA_A::DISABLED,
            true => ARMA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARMA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARMA_A::ENABLED
    }
}
#[doc = "Field `ARMA` writer - Arm A"]
pub type ARMA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCAPTCTRLA_SPEC, ARMA_A, O>;
impl<'a, const O: u8> ARMA_W<'a, O> {
    #[doc = "Input capture operation is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARMA_A::DISABLED)
    }
    #[doc = "Input capture operation as specified by CAPTCTRLA\\[EDGAx\\]
is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARMA_A::ENABLED)
    }
}
#[doc = "Field `ONESHOTA` reader - One Shot Mode A"]
pub type ONESHOTA_R = crate::BitReader<ONESHOTA_A>;
#[doc = "One Shot Mode A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONESHOTA_A {
    #[doc = "0: Free Running"]
    FREE_RUNNING = 0,
    #[doc = "1: One Shot"]
    ONE_SHOT = 1,
}
impl From<ONESHOTA_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOTA_A) -> Self {
        variant as u8 != 0
    }
}
impl ONESHOTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOTA_A {
        match self.bits {
            false => ONESHOTA_A::FREE_RUNNING,
            true => ONESHOTA_A::ONE_SHOT,
        }
    }
    #[doc = "Checks if the value of the field is `FREE_RUNNING`"]
    #[inline(always)]
    pub fn is_free_running(&self) -> bool {
        *self == ONESHOTA_A::FREE_RUNNING
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == ONESHOTA_A::ONE_SHOT
    }
}
#[doc = "Field `ONESHOTA` writer - One Shot Mode A"]
pub type ONESHOTA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCAPTCTRLA_SPEC, ONESHOTA_A, O>;
impl<'a, const O: u8> ONESHOTA_W<'a, O> {
    #[doc = "Free Running"]
    #[inline(always)]
    pub fn free_running(self) -> &'a mut W {
        self.variant(ONESHOTA_A::FREE_RUNNING)
    }
    #[doc = "One Shot"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(ONESHOTA_A::ONE_SHOT)
    }
}
#[doc = "Field `EDGA0` reader - Edge A 0"]
pub type EDGA0_R = crate::FieldReader<u8, EDGA0_A>;
#[doc = "Edge A 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGA0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Capture falling edges"]
    FALLING_EDGE = 1,
    #[doc = "2: Capture rising edges"]
    RISING_EDGE = 2,
    #[doc = "3: Capture any edge"]
    ANY_EDGE = 3,
}
impl From<EDGA0_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGA0_A) -> Self {
        variant as _
    }
}
impl EDGA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGA0_A {
        match self.bits {
            0 => EDGA0_A::DISABLED,
            1 => EDGA0_A::FALLING_EDGE,
            2 => EDGA0_A::RISING_EDGE,
            3 => EDGA0_A::ANY_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDGA0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EDGA0_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EDGA0_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `ANY_EDGE`"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == EDGA0_A::ANY_EDGE
    }
}
#[doc = "Field `EDGA0` writer - Edge A 0"]
pub type EDGA0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SMCAPTCTRLA_SPEC, u8, EDGA0_A, 2, O>;
impl<'a, const O: u8> EDGA0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDGA0_A::DISABLED)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EDGA0_A::FALLING_EDGE)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EDGA0_A::RISING_EDGE)
    }
    #[doc = "Capture any edge"]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut W {
        self.variant(EDGA0_A::ANY_EDGE)
    }
}
#[doc = "Field `EDGA1` reader - Edge A 1"]
pub type EDGA1_R = crate::FieldReader<u8, EDGA1_A>;
#[doc = "Edge A 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGA1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Capture falling edges"]
    FALLING_EDGE = 1,
    #[doc = "2: Capture rising edges"]
    RISING_EDGE = 2,
    #[doc = "3: Capture any edge"]
    ANY_EDGE = 3,
}
impl From<EDGA1_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGA1_A) -> Self {
        variant as _
    }
}
impl EDGA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGA1_A {
        match self.bits {
            0 => EDGA1_A::DISABLED,
            1 => EDGA1_A::FALLING_EDGE,
            2 => EDGA1_A::RISING_EDGE,
            3 => EDGA1_A::ANY_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDGA1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EDGA1_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EDGA1_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `ANY_EDGE`"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == EDGA1_A::ANY_EDGE
    }
}
#[doc = "Field `EDGA1` writer - Edge A 1"]
pub type EDGA1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SMCAPTCTRLA_SPEC, u8, EDGA1_A, 2, O>;
impl<'a, const O: u8> EDGA1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDGA1_A::DISABLED)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EDGA1_A::FALLING_EDGE)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EDGA1_A::RISING_EDGE)
    }
    #[doc = "Capture any edge"]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut W {
        self.variant(EDGA1_A::ANY_EDGE)
    }
}
#[doc = "Field `INP_SELA` reader - Input Select A"]
pub type INP_SELA_R = crate::BitReader<INP_SELA_A>;
#[doc = "Input Select A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INP_SELA_A {
    #[doc = "0: Raw PWM_A input signal selected as source."]
    PWM_A = 0,
    #[doc = "1: Edge Counter"]
    EDGE_COUNTER = 1,
}
impl From<INP_SELA_A> for bool {
    #[inline(always)]
    fn from(variant: INP_SELA_A) -> Self {
        variant as u8 != 0
    }
}
impl INP_SELA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INP_SELA_A {
        match self.bits {
            false => INP_SELA_A::PWM_A,
            true => INP_SELA_A::EDGE_COUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_A`"]
    #[inline(always)]
    pub fn is_pwm_a(&self) -> bool {
        *self == INP_SELA_A::PWM_A
    }
    #[doc = "Checks if the value of the field is `EDGE_COUNTER`"]
    #[inline(always)]
    pub fn is_edge_counter(&self) -> bool {
        *self == INP_SELA_A::EDGE_COUNTER
    }
}
#[doc = "Field `INP_SELA` writer - Input Select A"]
pub type INP_SELA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCAPTCTRLA_SPEC, INP_SELA_A, O>;
impl<'a, const O: u8> INP_SELA_W<'a, O> {
    #[doc = "Raw PWM_A input signal selected as source."]
    #[inline(always)]
    pub fn pwm_a(self) -> &'a mut W {
        self.variant(INP_SELA_A::PWM_A)
    }
    #[doc = "Edge Counter"]
    #[inline(always)]
    pub fn edge_counter(self) -> &'a mut W {
        self.variant(INP_SELA_A::EDGE_COUNTER)
    }
}
#[doc = "Field `EDGCNTA_EN` reader - Edge Counter A Enable"]
pub type EDGCNTA_EN_R = crate::BitReader<EDGCNTA_EN_A>;
#[doc = "Edge Counter A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGCNTA_EN_A {
    #[doc = "0: Edge counter disabled and held in reset"]
    DISABLED = 0,
    #[doc = "1: Edge counter enabled"]
    ENABLED = 1,
}
impl From<EDGCNTA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EDGCNTA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGCNTA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGCNTA_EN_A {
        match self.bits {
            false => EDGCNTA_EN_A::DISABLED,
            true => EDGCNTA_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDGCNTA_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDGCNTA_EN_A::ENABLED
    }
}
#[doc = "Field `EDGCNTA_EN` writer - Edge Counter A Enable"]
pub type EDGCNTA_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SMCAPTCTRLA_SPEC, EDGCNTA_EN_A, O>;
impl<'a, const O: u8> EDGCNTA_EN_W<'a, O> {
    #[doc = "Edge counter disabled and held in reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDGCNTA_EN_A::DISABLED)
    }
    #[doc = "Edge counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EDGCNTA_EN_A::ENABLED)
    }
}
#[doc = "Field `CFAWM` reader - Capture A FIFOs Water Mark"]
pub type CFAWM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFAWM` writer - Capture A FIFOs Water Mark"]
pub type CFAWM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMCAPTCTRLA_SPEC, u8, u8, 2, O>;
#[doc = "Field `CA0CNT` reader - Capture A0 FIFO Word Count"]
pub type CA0CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CA1CNT` reader - Capture A1 FIFO Word Count"]
pub type CA1CNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Arm A"]
    #[inline(always)]
    pub fn arma(&self) -> ARMA_R {
        ARMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - One Shot Mode A"]
    #[inline(always)]
    pub fn oneshota(&self) -> ONESHOTA_R {
        ONESHOTA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Edge A 0"]
    #[inline(always)]
    pub fn edga0(&self) -> EDGA0_R {
        EDGA0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Edge A 1"]
    #[inline(always)]
    pub fn edga1(&self) -> EDGA1_R {
        EDGA1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Input Select A"]
    #[inline(always)]
    pub fn inp_sela(&self) -> INP_SELA_R {
        INP_SELA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Edge Counter A Enable"]
    #[inline(always)]
    pub fn edgcnta_en(&self) -> EDGCNTA_EN_R {
        EDGCNTA_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub fn cfawm(&self) -> CFAWM_R {
        CFAWM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Capture A0 FIFO Word Count"]
    #[inline(always)]
    pub fn ca0cnt(&self) -> CA0CNT_R {
        CA0CNT_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Capture A1 FIFO Word Count"]
    #[inline(always)]
    pub fn ca1cnt(&self) -> CA1CNT_R {
        CA1CNT_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Arm A"]
    #[inline(always)]
    #[must_use]
    pub fn arma(&mut self) -> ARMA_W<0> {
        ARMA_W::new(self)
    }
    #[doc = "Bit 1 - One Shot Mode A"]
    #[inline(always)]
    #[must_use]
    pub fn oneshota(&mut self) -> ONESHOTA_W<1> {
        ONESHOTA_W::new(self)
    }
    #[doc = "Bits 2:3 - Edge A 0"]
    #[inline(always)]
    #[must_use]
    pub fn edga0(&mut self) -> EDGA0_W<2> {
        EDGA0_W::new(self)
    }
    #[doc = "Bits 4:5 - Edge A 1"]
    #[inline(always)]
    #[must_use]
    pub fn edga1(&mut self) -> EDGA1_W<4> {
        EDGA1_W::new(self)
    }
    #[doc = "Bit 6 - Input Select A"]
    #[inline(always)]
    #[must_use]
    pub fn inp_sela(&mut self) -> INP_SELA_W<6> {
        INP_SELA_W::new(self)
    }
    #[doc = "Bit 7 - Edge Counter A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn edgcnta_en(&mut self) -> EDGCNTA_EN_W<7> {
        EDGCNTA_EN_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture A FIFOs Water Mark"]
    #[inline(always)]
    #[must_use]
    pub fn cfawm(&mut self) -> CFAWM_W<8> {
        CFAWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Control A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcaptctrla](index.html) module"]
pub struct SMCAPTCTRLA_SPEC;
impl crate::RegisterSpec for SMCAPTCTRLA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcaptctrla::R](R) reader structure"]
impl crate::Readable for SMCAPTCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcaptctrla::W](W) writer structure"]
impl crate::Writable for SMCAPTCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCAPTCTRLA to value 0"]
impl crate::Resettable for SMCAPTCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
