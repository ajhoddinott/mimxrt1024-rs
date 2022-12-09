#[doc = "Register `HC%s` reader"]
pub struct R(crate::R<HC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HC%s` writer"]
pub struct W(crate::W<HC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_SPEC>;
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
impl From<crate::W<HC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCH` reader - Input Channel Select"]
pub type ADCH_R = crate::FieldReader<u8, ADCH_A>;
#[doc = "Input Channel Select\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCH_A {
    #[doc = "0: External channels 0 to 15 See External Signals for more information"]
    ADCH_0 = 0,
    #[doc = "1: External channels 0 to 15 See External Signals for more information"]
    ADCH_1 = 1,
    #[doc = "2: External channels 0 to 15 See External Signals for more information"]
    ADCH_2 = 2,
    #[doc = "3: External channels 0 to 15 See External Signals for more information"]
    ADCH_3 = 3,
    #[doc = "4: External channels 0 to 15 See External Signals for more information"]
    ADCH_4 = 4,
    #[doc = "5: External channels 0 to 15 See External Signals for more information"]
    ADCH_5 = 5,
    #[doc = "6: External channels 0 to 15 See External Signals for more information"]
    ADCH_6 = 6,
    #[doc = "7: External channels 0 to 15 See External Signals for more information"]
    ADCH_7 = 7,
    #[doc = "8: External channels 0 to 15 See External Signals for more information"]
    ADCH_8 = 8,
    #[doc = "9: External channels 0 to 15 See External Signals for more information"]
    ADCH_9 = 9,
    #[doc = "16: External channel selection from ADC_ETC"]
    ADCH_16 = 16,
    #[doc = "25: VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
    ADCH_25 = 25,
    #[doc = "31: Conversion Disabled. Hardware Triggers will not initiate any conversion."]
    ADCH_31 = 31,
}
impl From<ADCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCH_A) -> Self {
        variant as _
    }
}
impl ADCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCH_A> {
        match self.bits {
            0 => Some(ADCH_A::ADCH_0),
            1 => Some(ADCH_A::ADCH_1),
            2 => Some(ADCH_A::ADCH_2),
            3 => Some(ADCH_A::ADCH_3),
            4 => Some(ADCH_A::ADCH_4),
            5 => Some(ADCH_A::ADCH_5),
            6 => Some(ADCH_A::ADCH_6),
            7 => Some(ADCH_A::ADCH_7),
            8 => Some(ADCH_A::ADCH_8),
            9 => Some(ADCH_A::ADCH_9),
            16 => Some(ADCH_A::ADCH_16),
            25 => Some(ADCH_A::ADCH_25),
            31 => Some(ADCH_A::ADCH_31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADCH_0`"]
    #[inline(always)]
    pub fn is_adch_0(&self) -> bool {
        *self == ADCH_A::ADCH_0
    }
    #[doc = "Checks if the value of the field is `ADCH_1`"]
    #[inline(always)]
    pub fn is_adch_1(&self) -> bool {
        *self == ADCH_A::ADCH_1
    }
    #[doc = "Checks if the value of the field is `ADCH_2`"]
    #[inline(always)]
    pub fn is_adch_2(&self) -> bool {
        *self == ADCH_A::ADCH_2
    }
    #[doc = "Checks if the value of the field is `ADCH_3`"]
    #[inline(always)]
    pub fn is_adch_3(&self) -> bool {
        *self == ADCH_A::ADCH_3
    }
    #[doc = "Checks if the value of the field is `ADCH_4`"]
    #[inline(always)]
    pub fn is_adch_4(&self) -> bool {
        *self == ADCH_A::ADCH_4
    }
    #[doc = "Checks if the value of the field is `ADCH_5`"]
    #[inline(always)]
    pub fn is_adch_5(&self) -> bool {
        *self == ADCH_A::ADCH_5
    }
    #[doc = "Checks if the value of the field is `ADCH_6`"]
    #[inline(always)]
    pub fn is_adch_6(&self) -> bool {
        *self == ADCH_A::ADCH_6
    }
    #[doc = "Checks if the value of the field is `ADCH_7`"]
    #[inline(always)]
    pub fn is_adch_7(&self) -> bool {
        *self == ADCH_A::ADCH_7
    }
    #[doc = "Checks if the value of the field is `ADCH_8`"]
    #[inline(always)]
    pub fn is_adch_8(&self) -> bool {
        *self == ADCH_A::ADCH_8
    }
    #[doc = "Checks if the value of the field is `ADCH_9`"]
    #[inline(always)]
    pub fn is_adch_9(&self) -> bool {
        *self == ADCH_A::ADCH_9
    }
    #[doc = "Checks if the value of the field is `ADCH_16`"]
    #[inline(always)]
    pub fn is_adch_16(&self) -> bool {
        *self == ADCH_A::ADCH_16
    }
    #[doc = "Checks if the value of the field is `ADCH_25`"]
    #[inline(always)]
    pub fn is_adch_25(&self) -> bool {
        *self == ADCH_A::ADCH_25
    }
    #[doc = "Checks if the value of the field is `ADCH_31`"]
    #[inline(always)]
    pub fn is_adch_31(&self) -> bool {
        *self == ADCH_A::ADCH_31
    }
}
#[doc = "Field `ADCH` writer - Input Channel Select"]
pub type ADCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HC_SPEC, u8, ADCH_A, 5, O>;
impl<'a, const O: u8> ADCH_W<'a, O> {
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    #[inline(always)]
    pub fn adch_0(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_0)
    }
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    #[inline(always)]
    pub fn adch_1(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_1)
    }
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    #[inline(always)]
    pub fn adch_2(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_2)
    }
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    #[inline(always)]
    pub fn adch_3(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_3)
    }
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    #[inline(always)]
    pub fn adch_4(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_4)
    }
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    #[inline(always)]
    pub fn adch_5(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_5)
    }
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    #[inline(always)]
    pub fn adch_6(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_6)
    }
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    #[inline(always)]
    pub fn adch_7(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_7)
    }
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    #[inline(always)]
    pub fn adch_8(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_8)
    }
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    #[inline(always)]
    pub fn adch_9(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_9)
    }
    #[doc = "External channel selection from ADC_ETC"]
    #[inline(always)]
    pub fn adch_16(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_16)
    }
    #[doc = "VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
    #[inline(always)]
    pub fn adch_25(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_25)
    }
    #[doc = "Conversion Disabled. Hardware Triggers will not initiate any conversion."]
    #[inline(always)]
    pub fn adch_31(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_31)
    }
}
#[doc = "Field `AIEN` reader - Conversion Complete Interrupt Enable/Disable Control"]
pub type AIEN_R = crate::BitReader<AIEN_A>;
#[doc = "Conversion Complete Interrupt Enable/Disable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AIEN_A {
    #[doc = "0: Conversion complete interrupt disabled"]
    AIEN_0 = 0,
    #[doc = "1: Conversion complete interrupt enabled"]
    AIEN_1 = 1,
}
impl From<AIEN_A> for bool {
    #[inline(always)]
    fn from(variant: AIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIEN_A {
        match self.bits {
            false => AIEN_A::AIEN_0,
            true => AIEN_A::AIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AIEN_0`"]
    #[inline(always)]
    pub fn is_aien_0(&self) -> bool {
        *self == AIEN_A::AIEN_0
    }
    #[doc = "Checks if the value of the field is `AIEN_1`"]
    #[inline(always)]
    pub fn is_aien_1(&self) -> bool {
        *self == AIEN_A::AIEN_1
    }
}
#[doc = "Field `AIEN` writer - Conversion Complete Interrupt Enable/Disable Control"]
pub type AIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HC_SPEC, AIEN_A, O>;
impl<'a, const O: u8> AIEN_W<'a, O> {
    #[doc = "Conversion complete interrupt disabled"]
    #[inline(always)]
    pub fn aien_0(self) -> &'a mut W {
        self.variant(AIEN_A::AIEN_0)
    }
    #[doc = "Conversion complete interrupt enabled"]
    #[inline(always)]
    pub fn aien_1(self) -> &'a mut W {
        self.variant(AIEN_A::AIEN_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input Channel Select"]
    #[inline(always)]
    pub fn adch(&self) -> ADCH_R {
        ADCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Conversion Complete Interrupt Enable/Disable Control"]
    #[inline(always)]
    pub fn aien(&self) -> AIEN_R {
        AIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn adch(&mut self) -> ADCH_W<0> {
        ADCH_W::new(self)
    }
    #[doc = "Bit 7 - Conversion Complete Interrupt Enable/Disable Control"]
    #[inline(always)]
    #[must_use]
    pub fn aien(&mut self) -> AIEN_W<7> {
        AIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register for hardware triggers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc](index.html) module"]
pub struct HC_SPEC;
impl crate::RegisterSpec for HC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc::R](R) reader structure"]
impl crate::Readable for HC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc::W](W) writer structure"]
impl crate::Writable for HC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HC%s to value 0x1f"]
impl crate::Resettable for HC_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
