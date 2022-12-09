#[doc = "Register `TIMCFG[%s]` reader"]
pub struct R(crate::R<TIMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMCFG[%s]` writer"]
pub struct W(crate::W<TIMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMCFG_SPEC>;
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
impl From<crate::W<TIMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSTART` reader - Timer Start Bit"]
pub type TSTART_R = crate::BitReader<TSTART_A>;
#[doc = "Timer Start Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTART_A {
    #[doc = "0: Start bit disabled"]
    DISABLE = 0,
    #[doc = "1: Start bit enabled"]
    ENABLE = 1,
}
impl From<TSTART_A> for bool {
    #[inline(always)]
    fn from(variant: TSTART_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTART_A {
        match self.bits {
            false => TSTART_A::DISABLE,
            true => TSTART_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTART_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TSTART_A::ENABLE
    }
}
#[doc = "Field `TSTART` writer - Timer Start Bit"]
pub type TSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCFG_SPEC, TSTART_A, O>;
impl<'a, const O: u8> TSTART_W<'a, O> {
    #[doc = "Start bit disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTART_A::DISABLE)
    }
    #[doc = "Start bit enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TSTART_A::ENABLE)
    }
}
#[doc = "Field `TSTOP` reader - Timer Stop Bit"]
pub type TSTOP_R = crate::FieldReader<u8, TSTOP_A>;
#[doc = "Timer Stop Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTOP_A {
    #[doc = "0: Stop bit disabled"]
    STOP_DISABLE = 0,
    #[doc = "1: Stop bit is enabled on timer compare"]
    ENABLE_TMRCMP = 1,
    #[doc = "2: Stop bit is enabled on timer disable"]
    ENABLE_TMRDISABLE = 2,
    #[doc = "3: Stop bit is enabled on timer compare and timer disable"]
    ENABLE_TMR_CMP_DIS = 3,
}
impl From<TSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTOP_A) -> Self {
        variant as _
    }
}
impl TSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTOP_A {
        match self.bits {
            0 => TSTOP_A::STOP_DISABLE,
            1 => TSTOP_A::ENABLE_TMRCMP,
            2 => TSTOP_A::ENABLE_TMRDISABLE,
            3 => TSTOP_A::ENABLE_TMR_CMP_DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP_DISABLE`"]
    #[inline(always)]
    pub fn is_stop_disable(&self) -> bool {
        *self == TSTOP_A::STOP_DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE_TMRCMP`"]
    #[inline(always)]
    pub fn is_enable_tmrcmp(&self) -> bool {
        *self == TSTOP_A::ENABLE_TMRCMP
    }
    #[doc = "Checks if the value of the field is `ENABLE_TMRDISABLE`"]
    #[inline(always)]
    pub fn is_enable_tmrdisable(&self) -> bool {
        *self == TSTOP_A::ENABLE_TMRDISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE_TMR_CMP_DIS`"]
    #[inline(always)]
    pub fn is_enable_tmr_cmp_dis(&self) -> bool {
        *self == TSTOP_A::ENABLE_TMR_CMP_DIS
    }
}
#[doc = "Field `TSTOP` writer - Timer Stop Bit"]
pub type TSTOP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TIMCFG_SPEC, u8, TSTOP_A, 2, O>;
impl<'a, const O: u8> TSTOP_W<'a, O> {
    #[doc = "Stop bit disabled"]
    #[inline(always)]
    pub fn stop_disable(self) -> &'a mut W {
        self.variant(TSTOP_A::STOP_DISABLE)
    }
    #[doc = "Stop bit is enabled on timer compare"]
    #[inline(always)]
    pub fn enable_tmrcmp(self) -> &'a mut W {
        self.variant(TSTOP_A::ENABLE_TMRCMP)
    }
    #[doc = "Stop bit is enabled on timer disable"]
    #[inline(always)]
    pub fn enable_tmrdisable(self) -> &'a mut W {
        self.variant(TSTOP_A::ENABLE_TMRDISABLE)
    }
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    #[inline(always)]
    pub fn enable_tmr_cmp_dis(self) -> &'a mut W {
        self.variant(TSTOP_A::ENABLE_TMR_CMP_DIS)
    }
}
#[doc = "Field `TIMENA` reader - Timer Enable"]
pub type TIMENA_R = crate::FieldReader<u8, TIMENA_A>;
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMENA_A {
    #[doc = "0: Timer always enabled"]
    ENABLE = 0,
    #[doc = "1: Timer enabled on Timer N-1 enable"]
    TMR_NMINUS1_EN = 1,
    #[doc = "2: Timer enabled on Trigger high"]
    TMR_TRIGHI_EN = 2,
    #[doc = "3: Timer enabled on Trigger high and Pin high"]
    TMR_TRIG_PIN_HI_EN = 3,
    #[doc = "4: Timer enabled on Pin rising edge"]
    TMR_PINRISE_EN = 4,
    #[doc = "5: Timer enabled on Pin rising edge and Trigger high"]
    TMR_PINRISE_TRIGHI_EN = 5,
    #[doc = "6: Timer enabled on Trigger rising edge"]
    TMR_TRIGRISE_EN = 6,
    #[doc = "7: Timer enabled on Trigger rising or falling edge"]
    TMR_TRIGEDGE_EN = 7,
}
impl From<TIMENA_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMENA_A) -> Self {
        variant as _
    }
}
impl TIMENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMENA_A {
        match self.bits {
            0 => TIMENA_A::ENABLE,
            1 => TIMENA_A::TMR_NMINUS1_EN,
            2 => TIMENA_A::TMR_TRIGHI_EN,
            3 => TIMENA_A::TMR_TRIG_PIN_HI_EN,
            4 => TIMENA_A::TMR_PINRISE_EN,
            5 => TIMENA_A::TMR_PINRISE_TRIGHI_EN,
            6 => TIMENA_A::TMR_TRIGRISE_EN,
            7 => TIMENA_A::TMR_TRIGEDGE_EN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMENA_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `TMR_NMINUS1_EN`"]
    #[inline(always)]
    pub fn is_tmr_nminus1_en(&self) -> bool {
        *self == TIMENA_A::TMR_NMINUS1_EN
    }
    #[doc = "Checks if the value of the field is `TMR_TRIGHI_EN`"]
    #[inline(always)]
    pub fn is_tmr_trighi_en(&self) -> bool {
        *self == TIMENA_A::TMR_TRIGHI_EN
    }
    #[doc = "Checks if the value of the field is `TMR_TRIG_PIN_HI_EN`"]
    #[inline(always)]
    pub fn is_tmr_trig_pin_hi_en(&self) -> bool {
        *self == TIMENA_A::TMR_TRIG_PIN_HI_EN
    }
    #[doc = "Checks if the value of the field is `TMR_PINRISE_EN`"]
    #[inline(always)]
    pub fn is_tmr_pinrise_en(&self) -> bool {
        *self == TIMENA_A::TMR_PINRISE_EN
    }
    #[doc = "Checks if the value of the field is `TMR_PINRISE_TRIGHI_EN`"]
    #[inline(always)]
    pub fn is_tmr_pinrise_trighi_en(&self) -> bool {
        *self == TIMENA_A::TMR_PINRISE_TRIGHI_EN
    }
    #[doc = "Checks if the value of the field is `TMR_TRIGRISE_EN`"]
    #[inline(always)]
    pub fn is_tmr_trigrise_en(&self) -> bool {
        *self == TIMENA_A::TMR_TRIGRISE_EN
    }
    #[doc = "Checks if the value of the field is `TMR_TRIGEDGE_EN`"]
    #[inline(always)]
    pub fn is_tmr_trigedge_en(&self) -> bool {
        *self == TIMENA_A::TMR_TRIGEDGE_EN
    }
}
#[doc = "Field `TIMENA` writer - Timer Enable"]
pub type TIMENA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMCFG_SPEC, u8, TIMENA_A, 3, O>;
impl<'a, const O: u8> TIMENA_W<'a, O> {
    #[doc = "Timer always enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMENA_A::ENABLE)
    }
    #[doc = "Timer enabled on Timer N-1 enable"]
    #[inline(always)]
    pub fn tmr_nminus1_en(self) -> &'a mut W {
        self.variant(TIMENA_A::TMR_NMINUS1_EN)
    }
    #[doc = "Timer enabled on Trigger high"]
    #[inline(always)]
    pub fn tmr_trighi_en(self) -> &'a mut W {
        self.variant(TIMENA_A::TMR_TRIGHI_EN)
    }
    #[doc = "Timer enabled on Trigger high and Pin high"]
    #[inline(always)]
    pub fn tmr_trig_pin_hi_en(self) -> &'a mut W {
        self.variant(TIMENA_A::TMR_TRIG_PIN_HI_EN)
    }
    #[doc = "Timer enabled on Pin rising edge"]
    #[inline(always)]
    pub fn tmr_pinrise_en(self) -> &'a mut W {
        self.variant(TIMENA_A::TMR_PINRISE_EN)
    }
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    #[inline(always)]
    pub fn tmr_pinrise_trighi_en(self) -> &'a mut W {
        self.variant(TIMENA_A::TMR_PINRISE_TRIGHI_EN)
    }
    #[doc = "Timer enabled on Trigger rising edge"]
    #[inline(always)]
    pub fn tmr_trigrise_en(self) -> &'a mut W {
        self.variant(TIMENA_A::TMR_TRIGRISE_EN)
    }
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    #[inline(always)]
    pub fn tmr_trigedge_en(self) -> &'a mut W {
        self.variant(TIMENA_A::TMR_TRIGEDGE_EN)
    }
}
#[doc = "Field `TIMDIS` reader - Timer Disable"]
pub type TIMDIS_R = crate::FieldReader<u8, TIMDIS_A>;
#[doc = "Timer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMDIS_A {
    #[doc = "0: Timer never disabled"]
    NEVER = 0,
    #[doc = "1: Timer disabled on Timer N-1 disable"]
    TMR_NMINUS1 = 1,
    #[doc = "2: Timer disabled on Timer compare (upper 8-bits match and decrement)"]
    TMR_CMP = 2,
    #[doc = "3: Timer disabled on Timer compare (upper 8-bits match and decrement) and Trigger Low"]
    TMR_CMP_TRIGLOW = 3,
    #[doc = "4: Timer disabled on Pin rising or falling edge"]
    PIN_EDGE = 4,
    #[doc = "5: Timer disabled on Pin rising or falling edge provided Trigger is high"]
    PIN_EDGE_TRIGHI = 5,
    #[doc = "6: Timer disabled on Trigger falling edge"]
    TRIG_FALLEDGE = 6,
}
impl From<TIMDIS_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMDIS_A) -> Self {
        variant as _
    }
}
impl TIMDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMDIS_A> {
        match self.bits {
            0 => Some(TIMDIS_A::NEVER),
            1 => Some(TIMDIS_A::TMR_NMINUS1),
            2 => Some(TIMDIS_A::TMR_CMP),
            3 => Some(TIMDIS_A::TMR_CMP_TRIGLOW),
            4 => Some(TIMDIS_A::PIN_EDGE),
            5 => Some(TIMDIS_A::PIN_EDGE_TRIGHI),
            6 => Some(TIMDIS_A::TRIG_FALLEDGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NEVER`"]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        *self == TIMDIS_A::NEVER
    }
    #[doc = "Checks if the value of the field is `TMR_NMINUS1`"]
    #[inline(always)]
    pub fn is_tmr_nminus1(&self) -> bool {
        *self == TIMDIS_A::TMR_NMINUS1
    }
    #[doc = "Checks if the value of the field is `TMR_CMP`"]
    #[inline(always)]
    pub fn is_tmr_cmp(&self) -> bool {
        *self == TIMDIS_A::TMR_CMP
    }
    #[doc = "Checks if the value of the field is `TMR_CMP_TRIGLOW`"]
    #[inline(always)]
    pub fn is_tmr_cmp_triglow(&self) -> bool {
        *self == TIMDIS_A::TMR_CMP_TRIGLOW
    }
    #[doc = "Checks if the value of the field is `PIN_EDGE`"]
    #[inline(always)]
    pub fn is_pin_edge(&self) -> bool {
        *self == TIMDIS_A::PIN_EDGE
    }
    #[doc = "Checks if the value of the field is `PIN_EDGE_TRIGHI`"]
    #[inline(always)]
    pub fn is_pin_edge_trighi(&self) -> bool {
        *self == TIMDIS_A::PIN_EDGE_TRIGHI
    }
    #[doc = "Checks if the value of the field is `TRIG_FALLEDGE`"]
    #[inline(always)]
    pub fn is_trig_falledge(&self) -> bool {
        *self == TIMDIS_A::TRIG_FALLEDGE
    }
}
#[doc = "Field `TIMDIS` writer - Timer Disable"]
pub type TIMDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMCFG_SPEC, u8, TIMDIS_A, 3, O>;
impl<'a, const O: u8> TIMDIS_W<'a, O> {
    #[doc = "Timer never disabled"]
    #[inline(always)]
    pub fn never(self) -> &'a mut W {
        self.variant(TIMDIS_A::NEVER)
    }
    #[doc = "Timer disabled on Timer N-1 disable"]
    #[inline(always)]
    pub fn tmr_nminus1(self) -> &'a mut W {
        self.variant(TIMDIS_A::TMR_NMINUS1)
    }
    #[doc = "Timer disabled on Timer compare (upper 8-bits match and decrement)"]
    #[inline(always)]
    pub fn tmr_cmp(self) -> &'a mut W {
        self.variant(TIMDIS_A::TMR_CMP)
    }
    #[doc = "Timer disabled on Timer compare (upper 8-bits match and decrement) and Trigger Low"]
    #[inline(always)]
    pub fn tmr_cmp_triglow(self) -> &'a mut W {
        self.variant(TIMDIS_A::TMR_CMP_TRIGLOW)
    }
    #[doc = "Timer disabled on Pin rising or falling edge"]
    #[inline(always)]
    pub fn pin_edge(self) -> &'a mut W {
        self.variant(TIMDIS_A::PIN_EDGE)
    }
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    #[inline(always)]
    pub fn pin_edge_trighi(self) -> &'a mut W {
        self.variant(TIMDIS_A::PIN_EDGE_TRIGHI)
    }
    #[doc = "Timer disabled on Trigger falling edge"]
    #[inline(always)]
    pub fn trig_falledge(self) -> &'a mut W {
        self.variant(TIMDIS_A::TRIG_FALLEDGE)
    }
}
#[doc = "Field `TIMRST` reader - Timer Reset"]
pub type TIMRST_R = crate::FieldReader<u8, TIMRST_A>;
#[doc = "Timer Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMRST_A {
    #[doc = "0: Timer never reset"]
    NEVER = 0,
    #[doc = "2: Timer reset on Timer Pin equal to Timer Output"]
    PIN_EQ_TMR_OUT = 2,
    #[doc = "3: Timer reset on Timer Trigger equal to Timer Output"]
    TRIG_EQ_TMR_OUT = 3,
    #[doc = "4: Timer reset on Timer Pin rising edge"]
    PIN_RISE_EDGE = 4,
    #[doc = "6: Timer reset on Trigger rising edge"]
    TRIG_RISE_EDGE = 6,
    #[doc = "7: Timer reset on Trigger rising or falling edge"]
    TRIG_EDGE = 7,
}
impl From<TIMRST_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMRST_A) -> Self {
        variant as _
    }
}
impl TIMRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMRST_A> {
        match self.bits {
            0 => Some(TIMRST_A::NEVER),
            2 => Some(TIMRST_A::PIN_EQ_TMR_OUT),
            3 => Some(TIMRST_A::TRIG_EQ_TMR_OUT),
            4 => Some(TIMRST_A::PIN_RISE_EDGE),
            6 => Some(TIMRST_A::TRIG_RISE_EDGE),
            7 => Some(TIMRST_A::TRIG_EDGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NEVER`"]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        *self == TIMRST_A::NEVER
    }
    #[doc = "Checks if the value of the field is `PIN_EQ_TMR_OUT`"]
    #[inline(always)]
    pub fn is_pin_eq_tmr_out(&self) -> bool {
        *self == TIMRST_A::PIN_EQ_TMR_OUT
    }
    #[doc = "Checks if the value of the field is `TRIG_EQ_TMR_OUT`"]
    #[inline(always)]
    pub fn is_trig_eq_tmr_out(&self) -> bool {
        *self == TIMRST_A::TRIG_EQ_TMR_OUT
    }
    #[doc = "Checks if the value of the field is `PIN_RISE_EDGE`"]
    #[inline(always)]
    pub fn is_pin_rise_edge(&self) -> bool {
        *self == TIMRST_A::PIN_RISE_EDGE
    }
    #[doc = "Checks if the value of the field is `TRIG_RISE_EDGE`"]
    #[inline(always)]
    pub fn is_trig_rise_edge(&self) -> bool {
        *self == TIMRST_A::TRIG_RISE_EDGE
    }
    #[doc = "Checks if the value of the field is `TRIG_EDGE`"]
    #[inline(always)]
    pub fn is_trig_edge(&self) -> bool {
        *self == TIMRST_A::TRIG_EDGE
    }
}
#[doc = "Field `TIMRST` writer - Timer Reset"]
pub type TIMRST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMCFG_SPEC, u8, TIMRST_A, 3, O>;
impl<'a, const O: u8> TIMRST_W<'a, O> {
    #[doc = "Timer never reset"]
    #[inline(always)]
    pub fn never(self) -> &'a mut W {
        self.variant(TIMRST_A::NEVER)
    }
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    #[inline(always)]
    pub fn pin_eq_tmr_out(self) -> &'a mut W {
        self.variant(TIMRST_A::PIN_EQ_TMR_OUT)
    }
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    #[inline(always)]
    pub fn trig_eq_tmr_out(self) -> &'a mut W {
        self.variant(TIMRST_A::TRIG_EQ_TMR_OUT)
    }
    #[doc = "Timer reset on Timer Pin rising edge"]
    #[inline(always)]
    pub fn pin_rise_edge(self) -> &'a mut W {
        self.variant(TIMRST_A::PIN_RISE_EDGE)
    }
    #[doc = "Timer reset on Trigger rising edge"]
    #[inline(always)]
    pub fn trig_rise_edge(self) -> &'a mut W {
        self.variant(TIMRST_A::TRIG_RISE_EDGE)
    }
    #[doc = "Timer reset on Trigger rising or falling edge"]
    #[inline(always)]
    pub fn trig_edge(self) -> &'a mut W {
        self.variant(TIMRST_A::TRIG_EDGE)
    }
}
#[doc = "Field `TIMDEC` reader - Timer Decrement"]
pub type TIMDEC_R = crate::FieldReader<u8, TIMDEC_A>;
#[doc = "Timer Decrement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMDEC_A {
    #[doc = "0: Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    FLEXIO_CLK_SHIFTCLK_TMR_OUT = 0,
    #[doc = "1: Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    TRIG_EDGE_SHIFTCLK_TMR_OUT = 1,
    #[doc = "2: Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    PIN_EDGE_SHIFTCLK_TMR_OUT = 2,
    #[doc = "3: Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    TRIG_EDGE_SHIFTCLK_TRIG_IN = 3,
}
impl From<TIMDEC_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMDEC_A) -> Self {
        variant as _
    }
}
impl TIMDEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMDEC_A {
        match self.bits {
            0 => TIMDEC_A::FLEXIO_CLK_SHIFTCLK_TMR_OUT,
            1 => TIMDEC_A::TRIG_EDGE_SHIFTCLK_TMR_OUT,
            2 => TIMDEC_A::PIN_EDGE_SHIFTCLK_TMR_OUT,
            3 => TIMDEC_A::TRIG_EDGE_SHIFTCLK_TRIG_IN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO_CLK_SHIFTCLK_TMR_OUT`"]
    #[inline(always)]
    pub fn is_flexio_clk_shiftclk_tmr_out(&self) -> bool {
        *self == TIMDEC_A::FLEXIO_CLK_SHIFTCLK_TMR_OUT
    }
    #[doc = "Checks if the value of the field is `TRIG_EDGE_SHIFTCLK_TMR_OUT`"]
    #[inline(always)]
    pub fn is_trig_edge_shiftclk_tmr_out(&self) -> bool {
        *self == TIMDEC_A::TRIG_EDGE_SHIFTCLK_TMR_OUT
    }
    #[doc = "Checks if the value of the field is `PIN_EDGE_SHIFTCLK_TMR_OUT`"]
    #[inline(always)]
    pub fn is_pin_edge_shiftclk_tmr_out(&self) -> bool {
        *self == TIMDEC_A::PIN_EDGE_SHIFTCLK_TMR_OUT
    }
    #[doc = "Checks if the value of the field is `TRIG_EDGE_SHIFTCLK_TRIG_IN`"]
    #[inline(always)]
    pub fn is_trig_edge_shiftclk_trig_in(&self) -> bool {
        *self == TIMDEC_A::TRIG_EDGE_SHIFTCLK_TRIG_IN
    }
}
#[doc = "Field `TIMDEC` writer - Timer Decrement"]
pub type TIMDEC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMCFG_SPEC, u8, TIMDEC_A, 2, O>;
impl<'a, const O: u8> TIMDEC_W<'a, O> {
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    #[inline(always)]
    pub fn flexio_clk_shiftclk_tmr_out(self) -> &'a mut W {
        self.variant(TIMDEC_A::FLEXIO_CLK_SHIFTCLK_TMR_OUT)
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    #[inline(always)]
    pub fn trig_edge_shiftclk_tmr_out(self) -> &'a mut W {
        self.variant(TIMDEC_A::TRIG_EDGE_SHIFTCLK_TMR_OUT)
    }
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    #[inline(always)]
    pub fn pin_edge_shiftclk_tmr_out(self) -> &'a mut W {
        self.variant(TIMDEC_A::PIN_EDGE_SHIFTCLK_TMR_OUT)
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    #[inline(always)]
    pub fn trig_edge_shiftclk_trig_in(self) -> &'a mut W {
        self.variant(TIMDEC_A::TRIG_EDGE_SHIFTCLK_TRIG_IN)
    }
}
#[doc = "Field `TIMOUT` reader - Timer Output"]
pub type TIMOUT_R = crate::FieldReader<u8, TIMOUT_A>;
#[doc = "Timer Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMOUT_A {
    #[doc = "0: Timer output is logic one when enabled and is not affected by timer reset"]
    ONE = 0,
    #[doc = "1: Timer output is logic zero when enabled and is not affected by timer reset"]
    ZERO = 1,
    #[doc = "2: Timer output is logic one when enabled and on timer reset"]
    ONE_TMRRESET = 2,
    #[doc = "3: Timer output is logic zero when enabled and on timer reset"]
    ZERO_TMRRESET = 3,
}
impl From<TIMOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMOUT_A) -> Self {
        variant as _
    }
}
impl TIMOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMOUT_A {
        match self.bits {
            0 => TIMOUT_A::ONE,
            1 => TIMOUT_A::ZERO,
            2 => TIMOUT_A::ONE_TMRRESET,
            3 => TIMOUT_A::ZERO_TMRRESET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == TIMOUT_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TIMOUT_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE_TMRRESET`"]
    #[inline(always)]
    pub fn is_one_tmrreset(&self) -> bool {
        *self == TIMOUT_A::ONE_TMRRESET
    }
    #[doc = "Checks if the value of the field is `ZERO_TMRRESET`"]
    #[inline(always)]
    pub fn is_zero_tmrreset(&self) -> bool {
        *self == TIMOUT_A::ZERO_TMRRESET
    }
}
#[doc = "Field `TIMOUT` writer - Timer Output"]
pub type TIMOUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMCFG_SPEC, u8, TIMOUT_A, 2, O>;
impl<'a, const O: u8> TIMOUT_W<'a, O> {
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(TIMOUT_A::ONE)
    }
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TIMOUT_A::ZERO)
    }
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    #[inline(always)]
    pub fn one_tmrreset(self) -> &'a mut W {
        self.variant(TIMOUT_A::ONE_TMRRESET)
    }
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    #[inline(always)]
    pub fn zero_tmrreset(self) -> &'a mut W {
        self.variant(TIMOUT_A::ZERO_TMRRESET)
    }
}
impl R {
    #[doc = "Bit 1 - Timer Start Bit"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Timer Stop Bit"]
    #[inline(always)]
    pub fn tstop(&self) -> TSTOP_R {
        TSTOP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Timer Enable"]
    #[inline(always)]
    pub fn timena(&self) -> TIMENA_R {
        TIMENA_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Timer Disable"]
    #[inline(always)]
    pub fn timdis(&self) -> TIMDIS_R {
        TIMDIS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Timer Reset"]
    #[inline(always)]
    pub fn timrst(&self) -> TIMRST_R {
        TIMRST_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Timer Decrement"]
    #[inline(always)]
    pub fn timdec(&self) -> TIMDEC_R {
        TIMDEC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Timer Output"]
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Timer Start Bit"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TSTART_W<1> {
        TSTART_W::new(self)
    }
    #[doc = "Bits 4:5 - Timer Stop Bit"]
    #[inline(always)]
    #[must_use]
    pub fn tstop(&mut self) -> TSTOP_W<4> {
        TSTOP_W::new(self)
    }
    #[doc = "Bits 8:10 - Timer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timena(&mut self) -> TIMENA_W<8> {
        TIMENA_W::new(self)
    }
    #[doc = "Bits 12:14 - Timer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn timdis(&mut self) -> TIMDIS_W<12> {
        TIMDIS_W::new(self)
    }
    #[doc = "Bits 16:18 - Timer Reset"]
    #[inline(always)]
    #[must_use]
    pub fn timrst(&mut self) -> TIMRST_W<16> {
        TIMRST_W::new(self)
    }
    #[doc = "Bits 20:21 - Timer Decrement"]
    #[inline(always)]
    #[must_use]
    pub fn timdec(&mut self) -> TIMDEC_W<20> {
        TIMDEC_W::new(self)
    }
    #[doc = "Bits 24:25 - Timer Output"]
    #[inline(always)]
    #[must_use]
    pub fn timout(&mut self) -> TIMOUT_W<24> {
        TIMOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcfg](index.html) module"]
pub struct TIMCFG_SPEC;
impl crate::RegisterSpec for TIMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timcfg::R](R) reader structure"]
impl crate::Readable for TIMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timcfg::W](W) writer structure"]
impl crate::Writable for TIMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMCFG[%s]
to value 0"]
impl crate::Resettable for TIMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
