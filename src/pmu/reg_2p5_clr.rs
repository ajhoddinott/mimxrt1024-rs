#[doc = "Register `REG_2P5_CLR` reader"]
pub struct R(crate::R<REG_2P5_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_2P5_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_2P5_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_2P5_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_2P5_CLR` writer"]
pub struct W(crate::W<REG_2P5_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_2P5_CLR_SPEC>;
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
impl From<crate::W<REG_2P5_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_2P5_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_LINREG` reader - Control bit to enable the regulator output."]
pub type ENABLE_LINREG_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_LINREG` writer - Control bit to enable the regulator output."]
pub type ENABLE_LINREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_2P5_CLR_SPEC, bool, O>;
#[doc = "Field `ENABLE_BO` reader - Control bit to enable the brownout circuitry in the regulator."]
pub type ENABLE_BO_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_BO` writer - Control bit to enable the brownout circuitry in the regulator."]
pub type ENABLE_BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_2P5_CLR_SPEC, bool, O>;
#[doc = "Field `ENABLE_ILIMIT` reader - Control bit to enable the current-limit circuitry in the regulator."]
pub type ENABLE_ILIMIT_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_ILIMIT` writer - Control bit to enable the current-limit circuitry in the regulator."]
pub type ENABLE_ILIMIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_2P5_CLR_SPEC, bool, O>;
#[doc = "Field `ENABLE_PULLDOWN` reader - Control bit to enable the pull-down circuitry in the regulator"]
pub type ENABLE_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_PULLDOWN` writer - Control bit to enable the pull-down circuitry in the regulator"]
pub type ENABLE_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_2P5_CLR_SPEC, bool, O>;
#[doc = "Field `BO_OFFSET` reader - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
pub type BO_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BO_OFFSET` writer - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
pub type BO_OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_2P5_CLR_SPEC, u8, u8, 3, O>;
#[doc = "Field `OUTPUT_TRG` reader - Control bits to adjust the regulator output voltage"]
pub type OUTPUT_TRG_R = crate::FieldReader<u8, OUTPUT_TRG_A>;
#[doc = "Control bits to adjust the regulator output voltage\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTPUT_TRG_A {
    #[doc = "0: 2.10V"]
    OUTPUT_TRG_0 = 0,
    #[doc = "16: 2.50V"]
    OUTPUT_TRG_16 = 16,
    #[doc = "31: 2.875V"]
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
            16 => Some(OUTPUT_TRG_A::OUTPUT_TRG_16),
            31 => Some(OUTPUT_TRG_A::OUTPUT_TRG_31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_0`"]
    #[inline(always)]
    pub fn is_output_trg_0(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_0
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_16`"]
    #[inline(always)]
    pub fn is_output_trg_16(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_16
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_31`"]
    #[inline(always)]
    pub fn is_output_trg_31(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_31
    }
}
#[doc = "Field `OUTPUT_TRG` writer - Control bits to adjust the regulator output voltage"]
pub type OUTPUT_TRG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REG_2P5_CLR_SPEC, u8, OUTPUT_TRG_A, 5, O>;
impl<'a, const O: u8> OUTPUT_TRG_W<'a, O> {
    #[doc = "2.10V"]
    #[inline(always)]
    pub fn output_trg_0(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_0)
    }
    #[doc = "2.50V"]
    #[inline(always)]
    pub fn output_trg_16(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_16)
    }
    #[doc = "2.875V"]
    #[inline(always)]
    pub fn output_trg_31(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_31)
    }
}
#[doc = "Field `BO_VDD2P5` reader - Status bit that signals when a brownout is detected on the regulator output."]
pub type BO_VDD2P5_R = crate::BitReader<bool>;
#[doc = "Field `OK_VDD2P5` reader - Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
pub type OK_VDD2P5_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_WEAK_LINREG` reader - Enables the weak 2p5 regulator"]
pub type ENABLE_WEAK_LINREG_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_WEAK_LINREG` writer - Enables the weak 2p5 regulator"]
pub type ENABLE_WEAK_LINREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG_2P5_CLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Control bit to enable the regulator output."]
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
    #[doc = "Bit 3 - Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub fn enable_pulldown(&self) -> ENABLE_PULLDOWN_R {
        ENABLE_PULLDOWN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub fn bo_offset(&self) -> BO_OFFSET_R {
        BO_OFFSET_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub fn output_trg(&self) -> OUTPUT_TRG_R {
        OUTPUT_TRG_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub fn bo_vdd2p5(&self) -> BO_VDD2P5_R {
        BO_VDD2P5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub fn ok_vdd2p5(&self) -> OK_VDD2P5_R {
        OK_VDD2P5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub fn enable_weak_linreg(&self) -> ENABLE_WEAK_LINREG_R {
        ENABLE_WEAK_LINREG_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control bit to enable the regulator output."]
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
    #[doc = "Bit 3 - Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    #[must_use]
    pub fn enable_pulldown(&mut self) -> ENABLE_PULLDOWN_W<3> {
        ENABLE_PULLDOWN_W::new(self)
    }
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    #[must_use]
    pub fn bo_offset(&mut self) -> BO_OFFSET_W<4> {
        BO_OFFSET_W::new(self)
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    #[must_use]
    pub fn output_trg(&mut self) -> OUTPUT_TRG_W<8> {
        OUTPUT_TRG_W::new(self)
    }
    #[doc = "Bit 18 - Enables the weak 2p5 regulator"]
    #[inline(always)]
    #[must_use]
    pub fn enable_weak_linreg(&mut self) -> ENABLE_WEAK_LINREG_W<18> {
        ENABLE_WEAK_LINREG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Regulator 2P5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_2p5_clr](index.html) module"]
pub struct REG_2P5_CLR_SPEC;
impl crate::RegisterSpec for REG_2P5_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_2p5_clr::R](R) reader structure"]
impl crate::Readable for REG_2P5_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_2p5_clr::W](W) writer structure"]
impl crate::Writable for REG_2P5_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_2P5_CLR to value 0x1073"]
impl crate::Resettable for REG_2P5_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1073;
}
