#[doc = "Register `TCSR1` reader"]
pub struct R(crate::R<TCSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCSR1` writer"]
pub struct W(crate::W<TCSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCSR1_SPEC>;
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
impl From<crate::W<TCSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDRE` reader - Timer DMA Request Enable"]
pub type TDRE_R = crate::BitReader<TDRE_A>;
#[doc = "Timer DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRE_A {
    #[doc = "0: DMA request is disabled"]
    ZERO = 0,
    #[doc = "1: DMA request is enabled"]
    ONE = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::ZERO,
            true => TDRE_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TDRE_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == TDRE_A::ONE
    }
}
#[doc = "Field `TDRE` writer - Timer DMA Request Enable"]
pub type TDRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCSR1_SPEC, TDRE_A, O>;
impl<'a, const O: u8> TDRE_W<'a, O> {
    #[doc = "DMA request is disabled"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TDRE_A::ZERO)
    }
    #[doc = "DMA request is enabled"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(TDRE_A::ONE)
    }
}
#[doc = "Field `TMODE` reader - Timer Mode"]
pub type TMODE_R = crate::FieldReader<u8, TMODE_A>;
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMODE_A {
    #[doc = "0: Timer Channel is disabled."]
    TMR_DIS = 0,
    #[doc = "1: Timer Channel is configured for Input Capture on rising edge."]
    TMR_RE = 1,
    #[doc = "2: Timer Channel is configured for Input Capture on falling edge."]
    TMR_FE = 2,
    #[doc = "3: Timer Channel is configured for Input Capture on both edges."]
    TMR_BE = 3,
    #[doc = "4: Timer Channel is configured for Output Compare - software only."]
    TMR_OUT = 4,
    #[doc = "5: Timer Channel is configured for Output Compare - toggle output on compare."]
    TMR_TOGGLE = 5,
    #[doc = "6: Timer Channel is configured for Output Compare - clear output on compare."]
    TMR_CLR = 6,
    #[doc = "7: Timer Channel is configured for Output Compare - set output on compare."]
    TMR_SET_OUT = 7,
    #[doc = "9: Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    TMR_CLR_SET1 = 9,
    #[doc = "10: Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    TMR_CLR_SET = 10,
    #[doc = "14: Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_LOW = 14,
    #[doc = "15: Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_HIGH = 15,
}
impl From<TMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TMODE_A) -> Self {
        variant as _
    }
}
impl TMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMODE_A> {
        match self.bits {
            0 => Some(TMODE_A::TMR_DIS),
            1 => Some(TMODE_A::TMR_RE),
            2 => Some(TMODE_A::TMR_FE),
            3 => Some(TMODE_A::TMR_BE),
            4 => Some(TMODE_A::TMR_OUT),
            5 => Some(TMODE_A::TMR_TOGGLE),
            6 => Some(TMODE_A::TMR_CLR),
            7 => Some(TMODE_A::TMR_SET_OUT),
            9 => Some(TMODE_A::TMR_CLR_SET1),
            10 => Some(TMODE_A::TMR_CLR_SET),
            14 => Some(TMODE_A::TMR_OUT_CMP_LOW),
            15 => Some(TMODE_A::TMR_OUT_CMP_HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TMR_DIS`"]
    #[inline(always)]
    pub fn is_tmr_dis(&self) -> bool {
        *self == TMODE_A::TMR_DIS
    }
    #[doc = "Checks if the value of the field is `TMR_RE`"]
    #[inline(always)]
    pub fn is_tmr_re(&self) -> bool {
        *self == TMODE_A::TMR_RE
    }
    #[doc = "Checks if the value of the field is `TMR_FE`"]
    #[inline(always)]
    pub fn is_tmr_fe(&self) -> bool {
        *self == TMODE_A::TMR_FE
    }
    #[doc = "Checks if the value of the field is `TMR_BE`"]
    #[inline(always)]
    pub fn is_tmr_be(&self) -> bool {
        *self == TMODE_A::TMR_BE
    }
    #[doc = "Checks if the value of the field is `TMR_OUT`"]
    #[inline(always)]
    pub fn is_tmr_out(&self) -> bool {
        *self == TMODE_A::TMR_OUT
    }
    #[doc = "Checks if the value of the field is `TMR_TOGGLE`"]
    #[inline(always)]
    pub fn is_tmr_toggle(&self) -> bool {
        *self == TMODE_A::TMR_TOGGLE
    }
    #[doc = "Checks if the value of the field is `TMR_CLR`"]
    #[inline(always)]
    pub fn is_tmr_clr(&self) -> bool {
        *self == TMODE_A::TMR_CLR
    }
    #[doc = "Checks if the value of the field is `TMR_SET_OUT`"]
    #[inline(always)]
    pub fn is_tmr_set_out(&self) -> bool {
        *self == TMODE_A::TMR_SET_OUT
    }
    #[doc = "Checks if the value of the field is `TMR_CLR_SET1`"]
    #[inline(always)]
    pub fn is_tmr_clr_set1(&self) -> bool {
        *self == TMODE_A::TMR_CLR_SET1
    }
    #[doc = "Checks if the value of the field is `TMR_CLR_SET`"]
    #[inline(always)]
    pub fn is_tmr_clr_set(&self) -> bool {
        *self == TMODE_A::TMR_CLR_SET
    }
    #[doc = "Checks if the value of the field is `TMR_OUT_CMP_LOW`"]
    #[inline(always)]
    pub fn is_tmr_out_cmp_low(&self) -> bool {
        *self == TMODE_A::TMR_OUT_CMP_LOW
    }
    #[doc = "Checks if the value of the field is `TMR_OUT_CMP_HIGH`"]
    #[inline(always)]
    pub fn is_tmr_out_cmp_high(&self) -> bool {
        *self == TMODE_A::TMR_OUT_CMP_HIGH
    }
}
#[doc = "Field `TMODE` writer - Timer Mode"]
pub type TMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCSR1_SPEC, u8, TMODE_A, 4, O>;
impl<'a, const O: u8> TMODE_W<'a, O> {
    #[doc = "Timer Channel is disabled."]
    #[inline(always)]
    pub fn tmr_dis(self) -> &'a mut W {
        self.variant(TMODE_A::TMR_DIS)
    }
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    #[inline(always)]
    pub fn tmr_re(self) -> &'a mut W {
        self.variant(TMODE_A::TMR_RE)
    }
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    #[inline(always)]
    pub fn tmr_fe(self) -> &'a mut W {
        self.variant(TMODE_A::TMR_FE)
    }
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    #[inline(always)]
    pub fn tmr_be(self) -> &'a mut W {
        self.variant(TMODE_A::TMR_BE)
    }
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    #[inline(always)]
    pub fn tmr_out(self) -> &'a mut W {
        self.variant(TMODE_A::TMR_OUT)
    }
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    #[inline(always)]
    pub fn tmr_toggle(self) -> &'a mut W {
        self.variant(TMODE_A::TMR_TOGGLE)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    #[inline(always)]
    pub fn tmr_clr(self) -> &'a mut W {
        self.variant(TMODE_A::TMR_CLR)
    }
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    #[inline(always)]
    pub fn tmr_set_out(self) -> &'a mut W {
        self.variant(TMODE_A::TMR_SET_OUT)
    }
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    #[inline(always)]
    pub fn tmr_clr_set1(self) -> &'a mut W {
        self.variant(TMODE_A::TMR_CLR_SET1)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    #[inline(always)]
    pub fn tmr_clr_set(self) -> &'a mut W {
        self.variant(TMODE_A::TMR_CLR_SET)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    #[inline(always)]
    pub fn tmr_out_cmp_low(self) -> &'a mut W {
        self.variant(TMODE_A::TMR_OUT_CMP_LOW)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    #[inline(always)]
    pub fn tmr_out_cmp_high(self) -> &'a mut W {
        self.variant(TMODE_A::TMR_OUT_CMP_HIGH)
    }
}
#[doc = "Field `TIE` reader - Timer Interrupt Enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: Interrupt is disabled"]
    ZERO = 0,
    #[doc = "1: Interrupt is enabled"]
    ONE = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::ZERO,
            true => TIE_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TIE_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == TIE_A::ONE
    }
}
#[doc = "Field `TIE` writer - Timer Interrupt Enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCSR1_SPEC, TIE_A, O>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TIE_A::ZERO)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(TIE_A::ONE)
    }
}
#[doc = "Field `TF` reader - Timer Flag"]
pub type TF_R = crate::BitReader<TF_A>;
#[doc = "Timer Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF_A {
    #[doc = "0: Input Capture or Output Compare has not occurred."]
    ZERO = 0,
    #[doc = "1: Input Capture or Output Compare has occurred."]
    ONE = 1,
}
impl From<TF_A> for bool {
    #[inline(always)]
    fn from(variant: TF_A) -> Self {
        variant as u8 != 0
    }
}
impl TF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_A {
        match self.bits {
            false => TF_A::ZERO,
            true => TF_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TF_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == TF_A::ONE
    }
}
#[doc = "Field `TF` writer - Timer Flag"]
pub type TF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, TCSR1_SPEC, TF_A, O>;
impl<'a, const O: u8> TF_W<'a, O> {
    #[doc = "Input Capture or Output Compare has not occurred."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TF_A::ZERO)
    }
    #[doc = "Input Capture or Output Compare has occurred."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(TF_A::ONE)
    }
}
#[doc = "Field `TPWC` reader - Timer PulseWidth Control"]
pub type TPWC_R = crate::FieldReader<u8, TPWC_A>;
#[doc = "Timer PulseWidth Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPWC_A {
    #[doc = "0: Pulse width is one 1588-clock cycle."]
    VALW1 = 0,
    #[doc = "1: Pulse width is two 1588-clock cycles."]
    VALW2 = 1,
    #[doc = "2: Pulse width is three 1588-clock cycles."]
    VALW3 = 2,
    #[doc = "3: Pulse width is four 1588-clock cycles."]
    VALW4 = 3,
    #[doc = "31: Pulse width is 32 1588-clock cycles."]
    VALW32 = 31,
}
impl From<TPWC_A> for u8 {
    #[inline(always)]
    fn from(variant: TPWC_A) -> Self {
        variant as _
    }
}
impl TPWC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPWC_A> {
        match self.bits {
            0 => Some(TPWC_A::VALW1),
            1 => Some(TPWC_A::VALW2),
            2 => Some(TPWC_A::VALW3),
            3 => Some(TPWC_A::VALW4),
            31 => Some(TPWC_A::VALW32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALW1`"]
    #[inline(always)]
    pub fn is_valw1(&self) -> bool {
        *self == TPWC_A::VALW1
    }
    #[doc = "Checks if the value of the field is `VALW2`"]
    #[inline(always)]
    pub fn is_valw2(&self) -> bool {
        *self == TPWC_A::VALW2
    }
    #[doc = "Checks if the value of the field is `VALW3`"]
    #[inline(always)]
    pub fn is_valw3(&self) -> bool {
        *self == TPWC_A::VALW3
    }
    #[doc = "Checks if the value of the field is `VALW4`"]
    #[inline(always)]
    pub fn is_valw4(&self) -> bool {
        *self == TPWC_A::VALW4
    }
    #[doc = "Checks if the value of the field is `VALW32`"]
    #[inline(always)]
    pub fn is_valw32(&self) -> bool {
        *self == TPWC_A::VALW32
    }
}
#[doc = "Field `TPWC` writer - Timer PulseWidth Control"]
pub type TPWC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCSR1_SPEC, u8, TPWC_A, 5, O>;
impl<'a, const O: u8> TPWC_W<'a, O> {
    #[doc = "Pulse width is one 1588-clock cycle."]
    #[inline(always)]
    pub fn valw1(self) -> &'a mut W {
        self.variant(TPWC_A::VALW1)
    }
    #[doc = "Pulse width is two 1588-clock cycles."]
    #[inline(always)]
    pub fn valw2(self) -> &'a mut W {
        self.variant(TPWC_A::VALW2)
    }
    #[doc = "Pulse width is three 1588-clock cycles."]
    #[inline(always)]
    pub fn valw3(self) -> &'a mut W {
        self.variant(TPWC_A::VALW3)
    }
    #[doc = "Pulse width is four 1588-clock cycles."]
    #[inline(always)]
    pub fn valw4(self) -> &'a mut W {
        self.variant(TPWC_A::VALW4)
    }
    #[doc = "Pulse width is 32 1588-clock cycles."]
    #[inline(always)]
    pub fn valw32(self) -> &'a mut W {
        self.variant(TPWC_A::VALW32)
    }
}
impl R {
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 11:15 - Timer PulseWidth Control"]
    #[inline(always)]
    pub fn tpwc(&self) -> TPWC_R {
        TPWC_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdre(&mut self) -> TDRE_W<0> {
        TDRE_W::new(self)
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmode(&mut self) -> TMODE_W<2> {
        TMODE_W::new(self)
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<6> {
        TIE_W::new(self)
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tf(&mut self) -> TF_W<7> {
        TF_W::new(self)
    }
    #[doc = "Bits 11:15 - Timer PulseWidth Control"]
    #[inline(always)]
    #[must_use]
    pub fn tpwc(&mut self) -> TPWC_W<11> {
        TPWC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcsr1](index.html) module"]
pub struct TCSR1_SPEC;
impl crate::RegisterSpec for TCSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcsr1::R](R) reader structure"]
impl crate::Readable for TCSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcsr1::W](W) writer structure"]
impl crate::Writable for TCSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x80;
}
#[doc = "`reset()` method sets TCSR1 to value 0"]
impl crate::Resettable for TCSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
