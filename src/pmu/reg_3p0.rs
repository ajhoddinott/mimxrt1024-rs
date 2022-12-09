#[doc = "Register `REG_3P0` reader"]
pub struct R(crate::R<REG_3P0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_3P0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_3P0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_3P0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_3P0` writer"]
pub struct W(crate::W<REG_3P0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_3P0_SPEC>;
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
impl From<crate::W<REG_3P0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_3P0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_LINREG` reader - Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
pub type ENABLE_LINREG_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_LINREG` writer - Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
pub type ENABLE_LINREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_3P0_SPEC, bool, O>;
#[doc = "Field `ENABLE_BO` reader - Control bit to enable the brownout circuitry in the regulator."]
pub type ENABLE_BO_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_BO` writer - Control bit to enable the brownout circuitry in the regulator."]
pub type ENABLE_BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_3P0_SPEC, bool, O>;
#[doc = "Field `ENABLE_ILIMIT` reader - Control bit to enable the current-limit circuitry in the regulator."]
pub type ENABLE_ILIMIT_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_ILIMIT` writer - Control bit to enable the current-limit circuitry in the regulator."]
pub type ENABLE_ILIMIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_3P0_SPEC, bool, O>;
#[doc = "Field `BO_OFFSET` reader - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
pub type BO_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BO_OFFSET` writer - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
pub type BO_OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_3P0_SPEC, u8, u8, 3, O>;
#[doc = "Field `VBUS_SEL` reader - Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
pub type VBUS_SEL_R = crate::BitReader<VBUS_SEL_A>;
#[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBUS_SEL_A {
    #[doc = "0: Utilize VBUS OTG2 power"]
    USB_OTG2_VBUS = 0,
    #[doc = "1: Utilize VBUS OTG1 power"]
    USB_OTG1_VBUS = 1,
}
impl From<VBUS_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VBUS_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VBUS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_SEL_A {
        match self.bits {
            false => VBUS_SEL_A::USB_OTG2_VBUS,
            true => VBUS_SEL_A::USB_OTG1_VBUS,
        }
    }
    #[doc = "Checks if the value of the field is `USB_OTG2_VBUS`"]
    #[inline(always)]
    pub fn is_usb_otg2_vbus(&self) -> bool {
        *self == VBUS_SEL_A::USB_OTG2_VBUS
    }
    #[doc = "Checks if the value of the field is `USB_OTG1_VBUS`"]
    #[inline(always)]
    pub fn is_usb_otg1_vbus(&self) -> bool {
        *self == VBUS_SEL_A::USB_OTG1_VBUS
    }
}
#[doc = "Field `VBUS_SEL` writer - Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
pub type VBUS_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_3P0_SPEC, VBUS_SEL_A, O>;
impl<'a, const O: u8> VBUS_SEL_W<'a, O> {
    #[doc = "Utilize VBUS OTG2 power"]
    #[inline(always)]
    pub fn usb_otg2_vbus(self) -> &'a mut W {
        self.variant(VBUS_SEL_A::USB_OTG2_VBUS)
    }
    #[doc = "Utilize VBUS OTG1 power"]
    #[inline(always)]
    pub fn usb_otg1_vbus(self) -> &'a mut W {
        self.variant(VBUS_SEL_A::USB_OTG1_VBUS)
    }
}
#[doc = "Field `OUTPUT_TRG` reader - Control bits to adjust the regulator output voltage"]
pub type OUTPUT_TRG_R = crate::FieldReader<u8, OUTPUT_TRG_A>;
#[doc = "Control bits to adjust the regulator output voltage\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTPUT_TRG_A {
    #[doc = "0: 2.625V"]
    OUTPUT_TRG_0 = 0,
    #[doc = "15: 3.000V"]
    OUTPUT_TRG_15 = 15,
    #[doc = "31: 3.400V"]
    OUTPUT_TRG_31 = 31,
}
impl From<OUTPUT_TRG_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTPUT_TRG_A) -> Self {
        variant as _
    }
}
impl OUTPUT_TRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUTPUT_TRG_A> {
        match self.bits {
            0 => Some(OUTPUT_TRG_A::OUTPUT_TRG_0),
            15 => Some(OUTPUT_TRG_A::OUTPUT_TRG_15),
            31 => Some(OUTPUT_TRG_A::OUTPUT_TRG_31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_0`"]
    #[inline(always)]
    pub fn is_output_trg_0(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_0
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_15`"]
    #[inline(always)]
    pub fn is_output_trg_15(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_15
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_31`"]
    #[inline(always)]
    pub fn is_output_trg_31(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_31
    }
}
#[doc = "Field `OUTPUT_TRG` writer - Control bits to adjust the regulator output voltage"]
pub type OUTPUT_TRG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REG_3P0_SPEC, u8, OUTPUT_TRG_A, 5, O>;
impl<'a, const O: u8> OUTPUT_TRG_W<'a, O> {
    #[doc = "2.625V"]
    #[inline(always)]
    pub fn output_trg_0(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_0)
    }
    #[doc = "3.000V"]
    #[inline(always)]
    pub fn output_trg_15(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_15)
    }
    #[doc = "3.400V"]
    #[inline(always)]
    pub fn output_trg_31(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_31)
    }
}
#[doc = "Field `BO_VDD3P0` reader - Status bit that signals when a brownout is detected on the regulator output."]
pub type BO_VDD3P0_R = crate::BitReader<bool>;
#[doc = "Field `OK_VDD3P0` reader - Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
pub type OK_VDD3P0_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub fn enable_linreg(&self) -> ENABLE_LINREG_R {
        ENABLE_LINREG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub fn enable_bo(&self) -> ENABLE_BO_R {
        ENABLE_BO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub fn enable_ilimit(&self) -> ENABLE_ILIMIT_R {
        ENABLE_ILIMIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub fn bo_offset(&self) -> BO_OFFSET_R {
        BO_OFFSET_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub fn vbus_sel(&self) -> VBUS_SEL_R {
        VBUS_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub fn output_trg(&self) -> OUTPUT_TRG_R {
        OUTPUT_TRG_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub fn bo_vdd3p0(&self) -> BO_VDD3P0_R {
        BO_VDD3P0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub fn ok_vdd3p0(&self) -> OK_VDD3P0_R {
        OK_VDD3P0_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    #[must_use]
    pub fn enable_linreg(&mut self) -> ENABLE_LINREG_W<0> {
        ENABLE_LINREG_W::new(self)
    }
    #[doc = "Bit 1 - Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    #[must_use]
    pub fn enable_bo(&mut self) -> ENABLE_BO_W<1> {
        ENABLE_BO_W::new(self)
    }
    #[doc = "Bit 2 - Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    #[must_use]
    pub fn enable_ilimit(&mut self) -> ENABLE_ILIMIT_W<2> {
        ENABLE_ILIMIT_W::new(self)
    }
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    #[must_use]
    pub fn bo_offset(&mut self) -> BO_OFFSET_W<4> {
        BO_OFFSET_W::new(self)
    }
    #[doc = "Bit 7 - Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_sel(&mut self) -> VBUS_SEL_W<7> {
        VBUS_SEL_W::new(self)
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    #[must_use]
    pub fn output_trg(&mut self) -> OUTPUT_TRG_W<8> {
        OUTPUT_TRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Regulator 3P0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_3p0](index.html) module"]
pub struct REG_3P0_SPEC;
impl crate::RegisterSpec for REG_3P0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_3p0::R](R) reader structure"]
impl crate::Readable for REG_3P0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_3p0::W](W) writer structure"]
impl crate::Writable for REG_3P0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_3P0 to value 0x0f74"]
impl crate::Resettable for REG_3P0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f74;
}
