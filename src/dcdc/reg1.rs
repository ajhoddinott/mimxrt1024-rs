#[doc = "Register `REG1` reader"]
pub struct R(crate::R<REG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG1` writer"]
pub struct W(crate::W<REG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG1_SPEC>;
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
impl From<crate::W<REG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_FBK_SEL` reader - Select the feedback point of the internal regulator"]
pub type REG_FBK_SEL_R = crate::FieldReader<u8, REG_FBK_SEL_A>;
#[doc = "Select the feedback point of the internal regulator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG_FBK_SEL_A {
    #[doc = "0: The regulator outputs 1.0V with 1.2V reference voltage"]
    REG_FBK_SEL0 = 0,
    #[doc = "1: The regulator outputs 1.1V with 1.2V reference voltage"]
    REG_FBK_SEL1 = 1,
    #[doc = "2: The regulator outputs 1.0V with 1.3V reference voltage"]
    REG_FBK_SEL2 = 2,
    #[doc = "3: The regulator outputs 1.1V with 1.3V reference voltage"]
    REG_FBK_SEL3 = 3,
}
impl From<REG_FBK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_FBK_SEL_A) -> Self {
        variant as _
    }
}
impl REG_FBK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_FBK_SEL_A {
        match self.bits {
            0 => REG_FBK_SEL_A::REG_FBK_SEL0,
            1 => REG_FBK_SEL_A::REG_FBK_SEL1,
            2 => REG_FBK_SEL_A::REG_FBK_SEL2,
            3 => REG_FBK_SEL_A::REG_FBK_SEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REG_FBK_SEL0`"]
    #[inline(always)]
    pub fn is_reg_fbk_sel0(&self) -> bool {
        *self == REG_FBK_SEL_A::REG_FBK_SEL0
    }
    #[doc = "Checks if the value of the field is `REG_FBK_SEL1`"]
    #[inline(always)]
    pub fn is_reg_fbk_sel1(&self) -> bool {
        *self == REG_FBK_SEL_A::REG_FBK_SEL1
    }
    #[doc = "Checks if the value of the field is `REG_FBK_SEL2`"]
    #[inline(always)]
    pub fn is_reg_fbk_sel2(&self) -> bool {
        *self == REG_FBK_SEL_A::REG_FBK_SEL2
    }
    #[doc = "Checks if the value of the field is `REG_FBK_SEL3`"]
    #[inline(always)]
    pub fn is_reg_fbk_sel3(&self) -> bool {
        *self == REG_FBK_SEL_A::REG_FBK_SEL3
    }
}
#[doc = "Field `REG_FBK_SEL` writer - Select the feedback point of the internal regulator"]
pub type REG_FBK_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REG1_SPEC, u8, REG_FBK_SEL_A, 2, O>;
impl<'a, const O: u8> REG_FBK_SEL_W<'a, O> {
    #[doc = "The regulator outputs 1.0V with 1.2V reference voltage"]
    #[inline(always)]
    pub fn reg_fbk_sel0(self) -> &'a mut W {
        self.variant(REG_FBK_SEL_A::REG_FBK_SEL0)
    }
    #[doc = "The regulator outputs 1.1V with 1.2V reference voltage"]
    #[inline(always)]
    pub fn reg_fbk_sel1(self) -> &'a mut W {
        self.variant(REG_FBK_SEL_A::REG_FBK_SEL1)
    }
    #[doc = "The regulator outputs 1.0V with 1.3V reference voltage"]
    #[inline(always)]
    pub fn reg_fbk_sel2(self) -> &'a mut W {
        self.variant(REG_FBK_SEL_A::REG_FBK_SEL2)
    }
    #[doc = "The regulator outputs 1.1V with 1.3V reference voltage"]
    #[inline(always)]
    pub fn reg_fbk_sel3(self) -> &'a mut W {
        self.variant(REG_FBK_SEL_A::REG_FBK_SEL3)
    }
}
#[doc = "Field `REG_RLOAD_SW` reader - This controls the load resistor of the internal regulator of DCDC"]
pub type REG_RLOAD_SW_R = crate::BitReader<REG_RLOAD_SW_A>;
#[doc = "This controls the load resistor of the internal regulator of DCDC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REG_RLOAD_SW_A {
    #[doc = "0: Load resistor disconnected"]
    LOAD_R_DISCONNECT = 0,
    #[doc = "1: Load resistor connected"]
    LOAD_R_CONNECT = 1,
}
impl From<REG_RLOAD_SW_A> for bool {
    #[inline(always)]
    fn from(variant: REG_RLOAD_SW_A) -> Self {
        variant as u8 != 0
    }
}
impl REG_RLOAD_SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_RLOAD_SW_A {
        match self.bits {
            false => REG_RLOAD_SW_A::LOAD_R_DISCONNECT,
            true => REG_RLOAD_SW_A::LOAD_R_CONNECT,
        }
    }
    #[doc = "Checks if the value of the field is `LOAD_R_DISCONNECT`"]
    #[inline(always)]
    pub fn is_load_r_disconnect(&self) -> bool {
        *self == REG_RLOAD_SW_A::LOAD_R_DISCONNECT
    }
    #[doc = "Checks if the value of the field is `LOAD_R_CONNECT`"]
    #[inline(always)]
    pub fn is_load_r_connect(&self) -> bool {
        *self == REG_RLOAD_SW_A::LOAD_R_CONNECT
    }
}
#[doc = "Field `REG_RLOAD_SW` writer - This controls the load resistor of the internal regulator of DCDC"]
pub type REG_RLOAD_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG1_SPEC, REG_RLOAD_SW_A, O>;
impl<'a, const O: u8> REG_RLOAD_SW_W<'a, O> {
    #[doc = "Load resistor disconnected"]
    #[inline(always)]
    pub fn load_r_disconnect(self) -> &'a mut W {
        self.variant(REG_RLOAD_SW_A::LOAD_R_DISCONNECT)
    }
    #[doc = "Load resistor connected"]
    #[inline(always)]
    pub fn load_r_connect(self) -> &'a mut W {
        self.variant(REG_RLOAD_SW_A::LOAD_R_CONNECT)
    }
}
#[doc = "Field `LP_CMP_ISRC_SEL` reader - Low Power Comparator Current Bias"]
pub type LP_CMP_ISRC_SEL_R = crate::FieldReader<u8, LP_CMP_ISRC_SEL_A>;
#[doc = "Low Power Comparator Current Bias\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LP_CMP_ISRC_SEL_A {
    #[doc = "0: 50 nA"]
    SEL0 = 0,
    #[doc = "1: 100 nA"]
    SEL1 = 1,
    #[doc = "2: 200 nA"]
    SEL2 = 2,
    #[doc = "3: 400 nA"]
    SEL3 = 3,
}
impl From<LP_CMP_ISRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LP_CMP_ISRC_SEL_A) -> Self {
        variant as _
    }
}
impl LP_CMP_ISRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_CMP_ISRC_SEL_A {
        match self.bits {
            0 => LP_CMP_ISRC_SEL_A::SEL0,
            1 => LP_CMP_ISRC_SEL_A::SEL1,
            2 => LP_CMP_ISRC_SEL_A::SEL2,
            3 => LP_CMP_ISRC_SEL_A::SEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEL0`"]
    #[inline(always)]
    pub fn is_sel0(&self) -> bool {
        *self == LP_CMP_ISRC_SEL_A::SEL0
    }
    #[doc = "Checks if the value of the field is `SEL1`"]
    #[inline(always)]
    pub fn is_sel1(&self) -> bool {
        *self == LP_CMP_ISRC_SEL_A::SEL1
    }
    #[doc = "Checks if the value of the field is `SEL2`"]
    #[inline(always)]
    pub fn is_sel2(&self) -> bool {
        *self == LP_CMP_ISRC_SEL_A::SEL2
    }
    #[doc = "Checks if the value of the field is `SEL3`"]
    #[inline(always)]
    pub fn is_sel3(&self) -> bool {
        *self == LP_CMP_ISRC_SEL_A::SEL3
    }
}
#[doc = "Field `LP_CMP_ISRC_SEL` writer - Low Power Comparator Current Bias"]
pub type LP_CMP_ISRC_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REG1_SPEC, u8, LP_CMP_ISRC_SEL_A, 2, O>;
impl<'a, const O: u8> LP_CMP_ISRC_SEL_W<'a, O> {
    #[doc = "50 nA"]
    #[inline(always)]
    pub fn sel0(self) -> &'a mut W {
        self.variant(LP_CMP_ISRC_SEL_A::SEL0)
    }
    #[doc = "100 nA"]
    #[inline(always)]
    pub fn sel1(self) -> &'a mut W {
        self.variant(LP_CMP_ISRC_SEL_A::SEL1)
    }
    #[doc = "200 nA"]
    #[inline(always)]
    pub fn sel2(self) -> &'a mut W {
        self.variant(LP_CMP_ISRC_SEL_A::SEL2)
    }
    #[doc = "400 nA"]
    #[inline(always)]
    pub fn sel3(self) -> &'a mut W {
        self.variant(LP_CMP_ISRC_SEL_A::SEL3)
    }
}
#[doc = "Field `LOOPCTRL_HST_THRESH` reader - Increase Threshold Detection"]
pub type LOOPCTRL_HST_THRESH_R = crate::BitReader<LOOPCTRL_HST_THRESH_A>;
#[doc = "Increase Threshold Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPCTRL_HST_THRESH_A {
    #[doc = "0: Lower hysteresis threshold (about 2.5mV in typical, but this value can vary with PVT corners"]
    LOW_HYST_THRESH = 0,
    #[doc = "1: Higher hysteresis threshold (about 5mV in typical)"]
    HIGH_HYST_THRESH = 1,
}
impl From<LOOPCTRL_HST_THRESH_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPCTRL_HST_THRESH_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPCTRL_HST_THRESH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPCTRL_HST_THRESH_A {
        match self.bits {
            false => LOOPCTRL_HST_THRESH_A::LOW_HYST_THRESH,
            true => LOOPCTRL_HST_THRESH_A::HIGH_HYST_THRESH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_HYST_THRESH`"]
    #[inline(always)]
    pub fn is_low_hyst_thresh(&self) -> bool {
        *self == LOOPCTRL_HST_THRESH_A::LOW_HYST_THRESH
    }
    #[doc = "Checks if the value of the field is `HIGH_HYST_THRESH`"]
    #[inline(always)]
    pub fn is_high_hyst_thresh(&self) -> bool {
        *self == LOOPCTRL_HST_THRESH_A::HIGH_HYST_THRESH
    }
}
#[doc = "Field `LOOPCTRL_HST_THRESH` writer - Increase Threshold Detection"]
pub type LOOPCTRL_HST_THRESH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG1_SPEC, LOOPCTRL_HST_THRESH_A, O>;
impl<'a, const O: u8> LOOPCTRL_HST_THRESH_W<'a, O> {
    #[doc = "Lower hysteresis threshold (about 2.5mV in typical, but this value can vary with PVT corners"]
    #[inline(always)]
    pub fn low_hyst_thresh(self) -> &'a mut W {
        self.variant(LOOPCTRL_HST_THRESH_A::LOW_HYST_THRESH)
    }
    #[doc = "Higher hysteresis threshold (about 5mV in typical)"]
    #[inline(always)]
    pub fn high_hyst_thresh(self) -> &'a mut W {
        self.variant(LOOPCTRL_HST_THRESH_A::HIGH_HYST_THRESH)
    }
}
#[doc = "Field `LOOPCTRL_EN_HYST` reader - Enable Hysteresis"]
pub type LOOPCTRL_EN_HYST_R = crate::BitReader<LOOPCTRL_EN_HYST_A>;
#[doc = "Enable Hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPCTRL_EN_HYST_A {
    #[doc = "0: Disable hysteresis in switching converter common mode analog comparators"]
    DISABLE = 0,
    #[doc = "1: Enable hysteresis in switching converter common mode analog comparators"]
    ENABLE = 1,
}
impl From<LOOPCTRL_EN_HYST_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPCTRL_EN_HYST_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPCTRL_EN_HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPCTRL_EN_HYST_A {
        match self.bits {
            false => LOOPCTRL_EN_HYST_A::DISABLE,
            true => LOOPCTRL_EN_HYST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOOPCTRL_EN_HYST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LOOPCTRL_EN_HYST_A::ENABLE
    }
}
#[doc = "Field `LOOPCTRL_EN_HYST` writer - Enable Hysteresis"]
pub type LOOPCTRL_EN_HYST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG1_SPEC, LOOPCTRL_EN_HYST_A, O>;
impl<'a, const O: u8> LOOPCTRL_EN_HYST_W<'a, O> {
    #[doc = "Disable hysteresis in switching converter common mode analog comparators"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOOPCTRL_EN_HYST_A::DISABLE)
    }
    #[doc = "Enable hysteresis in switching converter common mode analog comparators"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOOPCTRL_EN_HYST_A::ENABLE)
    }
}
#[doc = "Field `VBG_TRIM` reader - Trim Bandgap Voltage"]
pub type VBG_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBG_TRIM` writer - Trim Bandgap Voltage"]
pub type VBG_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 7:8 - Select the feedback point of the internal regulator"]
    #[inline(always)]
    pub fn reg_fbk_sel(&self) -> REG_FBK_SEL_R {
        REG_FBK_SEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - This controls the load resistor of the internal regulator of DCDC"]
    #[inline(always)]
    pub fn reg_rload_sw(&self) -> REG_RLOAD_SW_R {
        REG_RLOAD_SW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Low Power Comparator Current Bias"]
    #[inline(always)]
    pub fn lp_cmp_isrc_sel(&self) -> LP_CMP_ISRC_SEL_R {
        LP_CMP_ISRC_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 21 - Increase Threshold Detection"]
    #[inline(always)]
    pub fn loopctrl_hst_thresh(&self) -> LOOPCTRL_HST_THRESH_R {
        LOOPCTRL_HST_THRESH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Hysteresis"]
    #[inline(always)]
    pub fn loopctrl_en_hyst(&self) -> LOOPCTRL_EN_HYST_R {
        LOOPCTRL_EN_HYST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Trim Bandgap Voltage"]
    #[inline(always)]
    pub fn vbg_trim(&self) -> VBG_TRIM_R {
        VBG_TRIM_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:8 - Select the feedback point of the internal regulator"]
    #[inline(always)]
    #[must_use]
    pub fn reg_fbk_sel(&mut self) -> REG_FBK_SEL_W<7> {
        REG_FBK_SEL_W::new(self)
    }
    #[doc = "Bit 9 - This controls the load resistor of the internal regulator of DCDC"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rload_sw(&mut self) -> REG_RLOAD_SW_W<9> {
        REG_RLOAD_SW_W::new(self)
    }
    #[doc = "Bits 12:13 - Low Power Comparator Current Bias"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cmp_isrc_sel(&mut self) -> LP_CMP_ISRC_SEL_W<12> {
        LP_CMP_ISRC_SEL_W::new(self)
    }
    #[doc = "Bit 21 - Increase Threshold Detection"]
    #[inline(always)]
    #[must_use]
    pub fn loopctrl_hst_thresh(&mut self) -> LOOPCTRL_HST_THRESH_W<21> {
        LOOPCTRL_HST_THRESH_W::new(self)
    }
    #[doc = "Bit 23 - Enable Hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn loopctrl_en_hyst(&mut self) -> LOOPCTRL_EN_HYST_W<23> {
        LOOPCTRL_EN_HYST_W::new(self)
    }
    #[doc = "Bits 24:28 - Trim Bandgap Voltage"]
    #[inline(always)]
    #[must_use]
    pub fn vbg_trim(&mut self) -> VBG_TRIM_W<24> {
        VBG_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg1](index.html) module"]
pub struct REG1_SPEC;
impl crate::RegisterSpec for REG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg1::R](R) reader structure"]
impl crate::Readable for REG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg1::W](W) writer structure"]
impl crate::Writable for REG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG1 to value 0x111b_a29c"]
impl crate::Resettable for REG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x111b_a29c;
}
