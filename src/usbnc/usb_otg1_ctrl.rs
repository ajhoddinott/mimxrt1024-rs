#[doc = "Register `USB_OTG1_CTRL` reader"]
pub struct R(crate::R<USB_OTG1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_OTG1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_OTG1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_OTG1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_OTG1_CTRL` writer"]
pub struct W(crate::W<USB_OTG1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_OTG1_CTRL_SPEC>;
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
impl From<crate::W<USB_OTG1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_OTG1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVER_CUR_DIS` reader - Disable OTG1 Overcurrent Detection"]
pub type OVER_CUR_DIS_R = crate::BitReader<OVER_CUR_DIS_A>;
#[doc = "Disable OTG1 Overcurrent Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVER_CUR_DIS_A {
    #[doc = "0: Enables overcurrent detection"]
    OVER_CUR_DIS_0 = 0,
    #[doc = "1: Disables overcurrent detection"]
    OVER_CUR_DIS_1 = 1,
}
impl From<OVER_CUR_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: OVER_CUR_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl OVER_CUR_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVER_CUR_DIS_A {
        match self.bits {
            false => OVER_CUR_DIS_A::OVER_CUR_DIS_0,
            true => OVER_CUR_DIS_A::OVER_CUR_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `OVER_CUR_DIS_0`"]
    #[inline(always)]
    pub fn is_over_cur_dis_0(&self) -> bool {
        *self == OVER_CUR_DIS_A::OVER_CUR_DIS_0
    }
    #[doc = "Checks if the value of the field is `OVER_CUR_DIS_1`"]
    #[inline(always)]
    pub fn is_over_cur_dis_1(&self) -> bool {
        *self == OVER_CUR_DIS_A::OVER_CUR_DIS_1
    }
}
#[doc = "Field `OVER_CUR_DIS` writer - Disable OTG1 Overcurrent Detection"]
pub type OVER_CUR_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_OTG1_CTRL_SPEC, OVER_CUR_DIS_A, O>;
impl<'a, const O: u8> OVER_CUR_DIS_W<'a, O> {
    #[doc = "Enables overcurrent detection"]
    #[inline(always)]
    pub fn over_cur_dis_0(self) -> &'a mut W {
        self.variant(OVER_CUR_DIS_A::OVER_CUR_DIS_0)
    }
    #[doc = "Disables overcurrent detection"]
    #[inline(always)]
    pub fn over_cur_dis_1(self) -> &'a mut W {
        self.variant(OVER_CUR_DIS_A::OVER_CUR_DIS_1)
    }
}
#[doc = "Field `OVER_CUR_POL` reader - OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event"]
pub type OVER_CUR_POL_R = crate::BitReader<OVER_CUR_POL_A>;
#[doc = "OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVER_CUR_POL_A {
    #[doc = "0: High active (high on this signal represents an overcurrent condition)"]
    OVER_CUR_POL_0 = 0,
    #[doc = "1: Low active (low on this signal represents an overcurrent condition)"]
    OVER_CUR_POL_1 = 1,
}
impl From<OVER_CUR_POL_A> for bool {
    #[inline(always)]
    fn from(variant: OVER_CUR_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl OVER_CUR_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVER_CUR_POL_A {
        match self.bits {
            false => OVER_CUR_POL_A::OVER_CUR_POL_0,
            true => OVER_CUR_POL_A::OVER_CUR_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `OVER_CUR_POL_0`"]
    #[inline(always)]
    pub fn is_over_cur_pol_0(&self) -> bool {
        *self == OVER_CUR_POL_A::OVER_CUR_POL_0
    }
    #[doc = "Checks if the value of the field is `OVER_CUR_POL_1`"]
    #[inline(always)]
    pub fn is_over_cur_pol_1(&self) -> bool {
        *self == OVER_CUR_POL_A::OVER_CUR_POL_1
    }
}
#[doc = "Field `OVER_CUR_POL` writer - OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event"]
pub type OVER_CUR_POL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_OTG1_CTRL_SPEC, OVER_CUR_POL_A, O>;
impl<'a, const O: u8> OVER_CUR_POL_W<'a, O> {
    #[doc = "High active (high on this signal represents an overcurrent condition)"]
    #[inline(always)]
    pub fn over_cur_pol_0(self) -> &'a mut W {
        self.variant(OVER_CUR_POL_A::OVER_CUR_POL_0)
    }
    #[doc = "Low active (low on this signal represents an overcurrent condition)"]
    #[inline(always)]
    pub fn over_cur_pol_1(self) -> &'a mut W {
        self.variant(OVER_CUR_POL_A::OVER_CUR_POL_1)
    }
}
#[doc = "Field `PWR_POL` reader - OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
pub type PWR_POL_R = crate::BitReader<PWR_POL_A>;
#[doc = "OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWR_POL_A {
    #[doc = "0: PMIC Power Pin is Low active."]
    PWR_POL_0 = 0,
    #[doc = "1: PMIC Power Pin is High active."]
    PWR_POL_1 = 1,
}
impl From<PWR_POL_A> for bool {
    #[inline(always)]
    fn from(variant: PWR_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl PWR_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWR_POL_A {
        match self.bits {
            false => PWR_POL_A::PWR_POL_0,
            true => PWR_POL_A::PWR_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWR_POL_0`"]
    #[inline(always)]
    pub fn is_pwr_pol_0(&self) -> bool {
        *self == PWR_POL_A::PWR_POL_0
    }
    #[doc = "Checks if the value of the field is `PWR_POL_1`"]
    #[inline(always)]
    pub fn is_pwr_pol_1(&self) -> bool {
        *self == PWR_POL_A::PWR_POL_1
    }
}
#[doc = "Field `PWR_POL` writer - OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
pub type PWR_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_OTG1_CTRL_SPEC, PWR_POL_A, O>;
impl<'a, const O: u8> PWR_POL_W<'a, O> {
    #[doc = "PMIC Power Pin is Low active."]
    #[inline(always)]
    pub fn pwr_pol_0(self) -> &'a mut W {
        self.variant(PWR_POL_A::PWR_POL_0)
    }
    #[doc = "PMIC Power Pin is High active."]
    #[inline(always)]
    pub fn pwr_pol_1(self) -> &'a mut W {
        self.variant(PWR_POL_A::PWR_POL_1)
    }
}
#[doc = "Field `WIE` reader - OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt"]
pub type WIE_R = crate::BitReader<WIE_A>;
#[doc = "OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WIE_A {
    #[doc = "0: Interrupt Disabled"]
    WIE_0 = 0,
    #[doc = "1: Interrupt Enabled"]
    WIE_1 = 1,
}
impl From<WIE_A> for bool {
    #[inline(always)]
    fn from(variant: WIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIE_A {
        match self.bits {
            false => WIE_A::WIE_0,
            true => WIE_A::WIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIE_0`"]
    #[inline(always)]
    pub fn is_wie_0(&self) -> bool {
        *self == WIE_A::WIE_0
    }
    #[doc = "Checks if the value of the field is `WIE_1`"]
    #[inline(always)]
    pub fn is_wie_1(&self) -> bool {
        *self == WIE_A::WIE_1
    }
}
#[doc = "Field `WIE` writer - OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt"]
pub type WIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_OTG1_CTRL_SPEC, WIE_A, O>;
impl<'a, const O: u8> WIE_W<'a, O> {
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn wie_0(self) -> &'a mut W {
        self.variant(WIE_A::WIE_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn wie_1(self) -> &'a mut W {
        self.variant(WIE_A::WIE_1)
    }
}
#[doc = "Field `WKUP_SW_EN` reader - OTG1 Software Wake-up Enable"]
pub type WKUP_SW_EN_R = crate::BitReader<WKUP_SW_EN_A>;
#[doc = "OTG1 Software Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUP_SW_EN_A {
    #[doc = "0: Disable"]
    WKUP_SW_EN_0 = 0,
    #[doc = "1: Enable"]
    WKUP_SW_EN_1 = 1,
}
impl From<WKUP_SW_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_SW_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUP_SW_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUP_SW_EN_A {
        match self.bits {
            false => WKUP_SW_EN_A::WKUP_SW_EN_0,
            true => WKUP_SW_EN_A::WKUP_SW_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_SW_EN_0`"]
    #[inline(always)]
    pub fn is_wkup_sw_en_0(&self) -> bool {
        *self == WKUP_SW_EN_A::WKUP_SW_EN_0
    }
    #[doc = "Checks if the value of the field is `WKUP_SW_EN_1`"]
    #[inline(always)]
    pub fn is_wkup_sw_en_1(&self) -> bool {
        *self == WKUP_SW_EN_A::WKUP_SW_EN_1
    }
}
#[doc = "Field `WKUP_SW_EN` writer - OTG1 Software Wake-up Enable"]
pub type WKUP_SW_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_OTG1_CTRL_SPEC, WKUP_SW_EN_A, O>;
impl<'a, const O: u8> WKUP_SW_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn wkup_sw_en_0(self) -> &'a mut W {
        self.variant(WKUP_SW_EN_A::WKUP_SW_EN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn wkup_sw_en_1(self) -> &'a mut W {
        self.variant(WKUP_SW_EN_A::WKUP_SW_EN_1)
    }
}
#[doc = "Field `WKUP_SW` reader - OTG1 Software Wake-up"]
pub type WKUP_SW_R = crate::BitReader<WKUP_SW_A>;
#[doc = "OTG1 Software Wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUP_SW_A {
    #[doc = "0: Inactive"]
    WKUP_SW_0 = 0,
    #[doc = "1: Force wake-up"]
    WKUP_SW_1 = 1,
}
impl From<WKUP_SW_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_SW_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUP_SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUP_SW_A {
        match self.bits {
            false => WKUP_SW_A::WKUP_SW_0,
            true => WKUP_SW_A::WKUP_SW_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_SW_0`"]
    #[inline(always)]
    pub fn is_wkup_sw_0(&self) -> bool {
        *self == WKUP_SW_A::WKUP_SW_0
    }
    #[doc = "Checks if the value of the field is `WKUP_SW_1`"]
    #[inline(always)]
    pub fn is_wkup_sw_1(&self) -> bool {
        *self == WKUP_SW_A::WKUP_SW_1
    }
}
#[doc = "Field `WKUP_SW` writer - OTG1 Software Wake-up"]
pub type WKUP_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_OTG1_CTRL_SPEC, WKUP_SW_A, O>;
impl<'a, const O: u8> WKUP_SW_W<'a, O> {
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn wkup_sw_0(self) -> &'a mut W {
        self.variant(WKUP_SW_A::WKUP_SW_0)
    }
    #[doc = "Force wake-up"]
    #[inline(always)]
    pub fn wkup_sw_1(self) -> &'a mut W {
        self.variant(WKUP_SW_A::WKUP_SW_1)
    }
}
#[doc = "Field `WKUP_ID_EN` reader - OTG1 Wake-up on ID change enable"]
pub type WKUP_ID_EN_R = crate::BitReader<WKUP_ID_EN_A>;
#[doc = "OTG1 Wake-up on ID change enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUP_ID_EN_A {
    #[doc = "0: Disable"]
    WKUP_ID_EN_0 = 0,
    #[doc = "1: Enable"]
    WKUP_ID_EN_1 = 1,
}
impl From<WKUP_ID_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_ID_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUP_ID_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUP_ID_EN_A {
        match self.bits {
            false => WKUP_ID_EN_A::WKUP_ID_EN_0,
            true => WKUP_ID_EN_A::WKUP_ID_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_ID_EN_0`"]
    #[inline(always)]
    pub fn is_wkup_id_en_0(&self) -> bool {
        *self == WKUP_ID_EN_A::WKUP_ID_EN_0
    }
    #[doc = "Checks if the value of the field is `WKUP_ID_EN_1`"]
    #[inline(always)]
    pub fn is_wkup_id_en_1(&self) -> bool {
        *self == WKUP_ID_EN_A::WKUP_ID_EN_1
    }
}
#[doc = "Field `WKUP_ID_EN` writer - OTG1 Wake-up on ID change enable"]
pub type WKUP_ID_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_OTG1_CTRL_SPEC, WKUP_ID_EN_A, O>;
impl<'a, const O: u8> WKUP_ID_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn wkup_id_en_0(self) -> &'a mut W {
        self.variant(WKUP_ID_EN_A::WKUP_ID_EN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn wkup_id_en_1(self) -> &'a mut W {
        self.variant(WKUP_ID_EN_A::WKUP_ID_EN_1)
    }
}
#[doc = "Field `WKUP_VBUS_EN` reader - OTG1 wake-up on VBUS change enable"]
pub type WKUP_VBUS_EN_R = crate::BitReader<WKUP_VBUS_EN_A>;
#[doc = "OTG1 wake-up on VBUS change enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUP_VBUS_EN_A {
    #[doc = "0: Disable"]
    WKUP_VBUS_EN_0 = 0,
    #[doc = "1: Enable"]
    WKUP_VBUS_EN_1 = 1,
}
impl From<WKUP_VBUS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_VBUS_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUP_VBUS_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUP_VBUS_EN_A {
        match self.bits {
            false => WKUP_VBUS_EN_A::WKUP_VBUS_EN_0,
            true => WKUP_VBUS_EN_A::WKUP_VBUS_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_VBUS_EN_0`"]
    #[inline(always)]
    pub fn is_wkup_vbus_en_0(&self) -> bool {
        *self == WKUP_VBUS_EN_A::WKUP_VBUS_EN_0
    }
    #[doc = "Checks if the value of the field is `WKUP_VBUS_EN_1`"]
    #[inline(always)]
    pub fn is_wkup_vbus_en_1(&self) -> bool {
        *self == WKUP_VBUS_EN_A::WKUP_VBUS_EN_1
    }
}
#[doc = "Field `WKUP_VBUS_EN` writer - OTG1 wake-up on VBUS change enable"]
pub type WKUP_VBUS_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_OTG1_CTRL_SPEC, WKUP_VBUS_EN_A, O>;
impl<'a, const O: u8> WKUP_VBUS_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn wkup_vbus_en_0(self) -> &'a mut W {
        self.variant(WKUP_VBUS_EN_A::WKUP_VBUS_EN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn wkup_vbus_en_1(self) -> &'a mut W {
        self.variant(WKUP_VBUS_EN_A::WKUP_VBUS_EN_1)
    }
}
#[doc = "Field `WKUP_DPDM_EN` reader - Wake-up on DPDM change enable"]
pub type WKUP_DPDM_EN_R = crate::BitReader<WKUP_DPDM_EN_A>;
#[doc = "Wake-up on DPDM change enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUP_DPDM_EN_A {
    #[doc = "0: DPDM changes wake-up to be disabled only when VBUS is 0."]
    WKUP_DPDM_EN_0 = 0,
    #[doc = "1: (Default) DPDM changes wake-up to be enabled, it is for device only."]
    WKUP_DPDM_EN_1 = 1,
}
impl From<WKUP_DPDM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_DPDM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUP_DPDM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUP_DPDM_EN_A {
        match self.bits {
            false => WKUP_DPDM_EN_A::WKUP_DPDM_EN_0,
            true => WKUP_DPDM_EN_A::WKUP_DPDM_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_DPDM_EN_0`"]
    #[inline(always)]
    pub fn is_wkup_dpdm_en_0(&self) -> bool {
        *self == WKUP_DPDM_EN_A::WKUP_DPDM_EN_0
    }
    #[doc = "Checks if the value of the field is `WKUP_DPDM_EN_1`"]
    #[inline(always)]
    pub fn is_wkup_dpdm_en_1(&self) -> bool {
        *self == WKUP_DPDM_EN_A::WKUP_DPDM_EN_1
    }
}
#[doc = "Field `WKUP_DPDM_EN` writer - Wake-up on DPDM change enable"]
pub type WKUP_DPDM_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_OTG1_CTRL_SPEC, WKUP_DPDM_EN_A, O>;
impl<'a, const O: u8> WKUP_DPDM_EN_W<'a, O> {
    #[doc = "DPDM changes wake-up to be disabled only when VBUS is 0."]
    #[inline(always)]
    pub fn wkup_dpdm_en_0(self) -> &'a mut W {
        self.variant(WKUP_DPDM_EN_A::WKUP_DPDM_EN_0)
    }
    #[doc = "(Default) DPDM changes wake-up to be enabled, it is for device only."]
    #[inline(always)]
    pub fn wkup_dpdm_en_1(self) -> &'a mut W {
        self.variant(WKUP_DPDM_EN_A::WKUP_DPDM_EN_1)
    }
}
#[doc = "Field `WIR` reader - OTG1 Wake-up Interrupt Request This bit indicates that a wake-up interrupt request is received on the OTG1 port"]
pub type WIR_R = crate::BitReader<WIR_A>;
#[doc = "OTG1 Wake-up Interrupt Request This bit indicates that a wake-up interrupt request is received on the OTG1 port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WIR_A {
    #[doc = "0: No wake-up interrupt request received"]
    WIR_0 = 0,
    #[doc = "1: Wake-up Interrupt Request received"]
    WIR_1 = 1,
}
impl From<WIR_A> for bool {
    #[inline(always)]
    fn from(variant: WIR_A) -> Self {
        variant as u8 != 0
    }
}
impl WIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIR_A {
        match self.bits {
            false => WIR_A::WIR_0,
            true => WIR_A::WIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIR_0`"]
    #[inline(always)]
    pub fn is_wir_0(&self) -> bool {
        *self == WIR_A::WIR_0
    }
    #[doc = "Checks if the value of the field is `WIR_1`"]
    #[inline(always)]
    pub fn is_wir_1(&self) -> bool {
        *self == WIR_A::WIR_1
    }
}
impl R {
    #[doc = "Bit 7 - Disable OTG1 Overcurrent Detection"]
    #[inline(always)]
    pub fn over_cur_dis(&self) -> OVER_CUR_DIS_R {
        OVER_CUR_DIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event"]
    #[inline(always)]
    pub fn over_cur_pol(&self) -> OVER_CUR_POL_R {
        OVER_CUR_POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
    #[inline(always)]
    pub fn pwr_pol(&self) -> PWR_POL_R {
        PWR_POL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt"]
    #[inline(always)]
    pub fn wie(&self) -> WIE_R {
        WIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - OTG1 Software Wake-up Enable"]
    #[inline(always)]
    pub fn wkup_sw_en(&self) -> WKUP_SW_EN_R {
        WKUP_SW_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OTG1 Software Wake-up"]
    #[inline(always)]
    pub fn wkup_sw(&self) -> WKUP_SW_R {
        WKUP_SW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OTG1 Wake-up on ID change enable"]
    #[inline(always)]
    pub fn wkup_id_en(&self) -> WKUP_ID_EN_R {
        WKUP_ID_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OTG1 wake-up on VBUS change enable"]
    #[inline(always)]
    pub fn wkup_vbus_en(&self) -> WKUP_VBUS_EN_R {
        WKUP_VBUS_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 29 - Wake-up on DPDM change enable"]
    #[inline(always)]
    pub fn wkup_dpdm_en(&self) -> WKUP_DPDM_EN_R {
        WKUP_DPDM_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - OTG1 Wake-up Interrupt Request This bit indicates that a wake-up interrupt request is received on the OTG1 port"]
    #[inline(always)]
    pub fn wir(&self) -> WIR_R {
        WIR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Disable OTG1 Overcurrent Detection"]
    #[inline(always)]
    #[must_use]
    pub fn over_cur_dis(&mut self) -> OVER_CUR_DIS_W<7> {
        OVER_CUR_DIS_W::new(self)
    }
    #[doc = "Bit 8 - OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event"]
    #[inline(always)]
    #[must_use]
    pub fn over_cur_pol(&mut self) -> OVER_CUR_POL_W<8> {
        OVER_CUR_POL_W::new(self)
    }
    #[doc = "Bit 9 - OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_pol(&mut self) -> PWR_POL_W<9> {
        PWR_POL_W::new(self)
    }
    #[doc = "Bit 10 - OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wie(&mut self) -> WIE_W<10> {
        WIE_W::new(self)
    }
    #[doc = "Bit 14 - OTG1 Software Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkup_sw_en(&mut self) -> WKUP_SW_EN_W<14> {
        WKUP_SW_EN_W::new(self)
    }
    #[doc = "Bit 15 - OTG1 Software Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn wkup_sw(&mut self) -> WKUP_SW_W<15> {
        WKUP_SW_W::new(self)
    }
    #[doc = "Bit 16 - OTG1 Wake-up on ID change enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkup_id_en(&mut self) -> WKUP_ID_EN_W<16> {
        WKUP_ID_EN_W::new(self)
    }
    #[doc = "Bit 17 - OTG1 wake-up on VBUS change enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkup_vbus_en(&mut self) -> WKUP_VBUS_EN_W<17> {
        WKUP_VBUS_EN_W::new(self)
    }
    #[doc = "Bit 29 - Wake-up on DPDM change enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkup_dpdm_en(&mut self) -> WKUP_DPDM_EN_W<29> {
        WKUP_DPDM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB OTG1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_otg1_ctrl](index.html) module"]
pub struct USB_OTG1_CTRL_SPEC;
impl crate::RegisterSpec for USB_OTG1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_otg1_ctrl::R](R) reader structure"]
impl crate::Readable for USB_OTG1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_otg1_ctrl::W](W) writer structure"]
impl crate::Writable for USB_OTG1_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_OTG1_CTRL to value 0x3000_1000"]
impl crate::Resettable for USB_OTG1_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3000_1000;
}
