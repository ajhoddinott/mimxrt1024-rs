#[doc = "Register `LPMKCR` reader"]
pub struct R(crate::R<LPMKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMKCR` writer"]
pub struct W(crate::W<LPMKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMKCR_SPEC>;
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
impl From<crate::W<LPMKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTER_KEY_SEL` reader - Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
pub type MASTER_KEY_SEL_R = crate::FieldReader<u8, MASTER_KEY_SEL_A>;
#[doc = "Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASTER_KEY_SEL_A {
    #[doc = "0: Select one time programmable master key."]
    SELECT_OTPMK = 0,
    #[doc = "2: Select zeroizable master key when MKS_EN bit is set ."]
    SELECT_ZMK = 2,
    #[doc = "3: Select combined master key when MKS_EN bit is set ."]
    SELECT_COMBO = 3,
}
impl From<MASTER_KEY_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MASTER_KEY_SEL_A) -> Self {
        variant as _
    }
}
impl MASTER_KEY_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MASTER_KEY_SEL_A> {
        match self.bits {
            0 => Some(MASTER_KEY_SEL_A::SELECT_OTPMK),
            2 => Some(MASTER_KEY_SEL_A::SELECT_ZMK),
            3 => Some(MASTER_KEY_SEL_A::SELECT_COMBO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_OTPMK`"]
    #[inline(always)]
    pub fn is_select_otpmk(&self) -> bool {
        *self == MASTER_KEY_SEL_A::SELECT_OTPMK
    }
    #[doc = "Checks if the value of the field is `SELECT_ZMK`"]
    #[inline(always)]
    pub fn is_select_zmk(&self) -> bool {
        *self == MASTER_KEY_SEL_A::SELECT_ZMK
    }
    #[doc = "Checks if the value of the field is `SELECT_COMBO`"]
    #[inline(always)]
    pub fn is_select_combo(&self) -> bool {
        *self == MASTER_KEY_SEL_A::SELECT_COMBO
    }
}
#[doc = "Field `MASTER_KEY_SEL` writer - Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
pub type MASTER_KEY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPMKCR_SPEC, u8, MASTER_KEY_SEL_A, 2, O>;
impl<'a, const O: u8> MASTER_KEY_SEL_W<'a, O> {
    #[doc = "Select one time programmable master key."]
    #[inline(always)]
    pub fn select_otpmk(self) -> &'a mut W {
        self.variant(MASTER_KEY_SEL_A::SELECT_OTPMK)
    }
    #[doc = "Select zeroizable master key when MKS_EN bit is set ."]
    #[inline(always)]
    pub fn select_zmk(self) -> &'a mut W {
        self.variant(MASTER_KEY_SEL_A::SELECT_ZMK)
    }
    #[doc = "Select combined master key when MKS_EN bit is set ."]
    #[inline(always)]
    pub fn select_combo(self) -> &'a mut W {
        self.variant(MASTER_KEY_SEL_A::SELECT_COMBO)
    }
}
#[doc = "Field `ZMK_HWP` reader - Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
pub type ZMK_HWP_R = crate::BitReader<ZMK_HWP_A>;
#[doc = "Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZMK_HWP_A {
    #[doc = "0: ZMK is in the software programming mode."]
    SW_PROG_MODE = 0,
    #[doc = "1: ZMK is in the hardware programming mode."]
    HW_PROG_MODE = 1,
}
impl From<ZMK_HWP_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_HWP_A) -> Self {
        variant as u8 != 0
    }
}
impl ZMK_HWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_HWP_A {
        match self.bits {
            false => ZMK_HWP_A::SW_PROG_MODE,
            true => ZMK_HWP_A::HW_PROG_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `SW_PROG_MODE`"]
    #[inline(always)]
    pub fn is_sw_prog_mode(&self) -> bool {
        *self == ZMK_HWP_A::SW_PROG_MODE
    }
    #[doc = "Checks if the value of the field is `HW_PROG_MODE`"]
    #[inline(always)]
    pub fn is_hw_prog_mode(&self) -> bool {
        *self == ZMK_HWP_A::HW_PROG_MODE
    }
}
#[doc = "Field `ZMK_HWP` writer - Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
pub type ZMK_HWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMKCR_SPEC, ZMK_HWP_A, O>;
impl<'a, const O: u8> ZMK_HWP_W<'a, O> {
    #[doc = "ZMK is in the software programming mode."]
    #[inline(always)]
    pub fn sw_prog_mode(self) -> &'a mut W {
        self.variant(ZMK_HWP_A::SW_PROG_MODE)
    }
    #[doc = "ZMK is in the hardware programming mode."]
    #[inline(always)]
    pub fn hw_prog_mode(self) -> &'a mut W {
        self.variant(ZMK_HWP_A::HW_PROG_MODE)
    }
}
#[doc = "Field `ZMK_VAL` reader - Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
pub type ZMK_VAL_R = crate::BitReader<ZMK_VAL_A>;
#[doc = "Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZMK_VAL_A {
    #[doc = "0: ZMK is not valid."]
    INVALID = 0,
    #[doc = "1: ZMK is valid."]
    VALID = 1,
}
impl From<ZMK_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_VAL_A) -> Self {
        variant as u8 != 0
    }
}
impl ZMK_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_VAL_A {
        match self.bits {
            false => ZMK_VAL_A::INVALID,
            true => ZMK_VAL_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == ZMK_VAL_A::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == ZMK_VAL_A::VALID
    }
}
#[doc = "Field `ZMK_VAL` writer - Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
pub type ZMK_VAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMKCR_SPEC, ZMK_VAL_A, O>;
impl<'a, const O: u8> ZMK_VAL_W<'a, O> {
    #[doc = "ZMK is not valid."]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(ZMK_VAL_A::INVALID)
    }
    #[doc = "ZMK is valid."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(ZMK_VAL_A::VALID)
    }
}
#[doc = "Field `ZMK_ECC_EN` reader - Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
pub type ZMK_ECC_EN_R = crate::BitReader<ZMK_ECC_EN_A>;
#[doc = "Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZMK_ECC_EN_A {
    #[doc = "0: ZMK ECC check is disabled."]
    DISABLED = 0,
    #[doc = "1: ZMK ECC check is enabled."]
    ENABLED = 1,
}
impl From<ZMK_ECC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_ECC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ZMK_ECC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_ECC_EN_A {
        match self.bits {
            false => ZMK_ECC_EN_A::DISABLED,
            true => ZMK_ECC_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ZMK_ECC_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ZMK_ECC_EN_A::ENABLED
    }
}
#[doc = "Field `ZMK_ECC_EN` writer - Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
pub type ZMK_ECC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMKCR_SPEC, ZMK_ECC_EN_A, O>;
impl<'a, const O: u8> ZMK_ECC_EN_W<'a, O> {
    #[doc = "ZMK ECC check is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ZMK_ECC_EN_A::DISABLED)
    }
    #[doc = "ZMK ECC check is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ZMK_ECC_EN_A::ENABLED)
    }
}
#[doc = "Field `ZMK_ECC_VALUE` reader - Zeroizable Master Key Error Correcting Code Value This field is automatically calculated and set when one is written into ZMK_ECC_EN bit of this register"]
pub type ZMK_ECC_VALUE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1 - Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
    #[inline(always)]
    pub fn master_key_sel(&self) -> MASTER_KEY_SEL_R {
        MASTER_KEY_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
    #[inline(always)]
    pub fn zmk_hwp(&self) -> ZMK_HWP_R {
        ZMK_HWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
    #[inline(always)]
    pub fn zmk_val(&self) -> ZMK_VAL_R {
        ZMK_VAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
    #[inline(always)]
    pub fn zmk_ecc_en(&self) -> ZMK_ECC_EN_R {
        ZMK_ECC_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 7:15 - Zeroizable Master Key Error Correcting Code Value This field is automatically calculated and set when one is written into ZMK_ECC_EN bit of this register"]
    #[inline(always)]
    pub fn zmk_ecc_value(&self) -> ZMK_ECC_VALUE_R {
        ZMK_ECC_VALUE_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
    #[inline(always)]
    #[must_use]
    pub fn master_key_sel(&mut self) -> MASTER_KEY_SEL_W<0> {
        MASTER_KEY_SEL_W::new(self)
    }
    #[doc = "Bit 2 - Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
    #[inline(always)]
    #[must_use]
    pub fn zmk_hwp(&mut self) -> ZMK_HWP_W<2> {
        ZMK_HWP_W::new(self)
    }
    #[doc = "Bit 3 - Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
    #[inline(always)]
    #[must_use]
    pub fn zmk_val(&mut self) -> ZMK_VAL_W<3> {
        ZMK_VAL_W::new(self)
    }
    #[doc = "Bit 4 - Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
    #[inline(always)]
    #[must_use]
    pub fn zmk_ecc_en(&mut self) -> ZMK_ECC_EN_W<4> {
        ZMK_ECC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_LP Master Key Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmkcr](index.html) module"]
pub struct LPMKCR_SPEC;
impl crate::RegisterSpec for LPMKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmkcr::R](R) reader structure"]
impl crate::Readable for LPMKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmkcr::W](W) writer structure"]
impl crate::Writable for LPMKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMKCR to value 0"]
impl crate::Resettable for LPMKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
