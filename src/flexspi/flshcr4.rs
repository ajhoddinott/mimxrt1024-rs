#[doc = "Register `FLSHCR4` reader"]
pub struct R(crate::R<FLSHCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLSHCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLSHCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLSHCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLSHCR4` writer"]
pub struct W(crate::W<FLSHCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLSHCR4_SPEC>;
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
impl From<crate::W<FLSHCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLSHCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WMOPT1` reader - Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
pub type WMOPT1_R = crate::BitReader<WMOPT1_A>;
#[doc = "Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WMOPT1_A {
    #[doc = "0: DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    DISABLE = 0,
    #[doc = "1: DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    ENABLE = 1,
}
impl From<WMOPT1_A> for bool {
    #[inline(always)]
    fn from(variant: WMOPT1_A) -> Self {
        variant as u8 != 0
    }
}
impl WMOPT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMOPT1_A {
        match self.bits {
            false => WMOPT1_A::DISABLE,
            true => WMOPT1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WMOPT1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WMOPT1_A::ENABLE
    }
}
#[doc = "Field `WMOPT1` writer - Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
pub type WMOPT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLSHCR4_SPEC, WMOPT1_A, O>;
impl<'a, const O: u8> WMOPT1_W<'a, O> {
    #[doc = "DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WMOPT1_A::DISABLE)
    }
    #[doc = "DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WMOPT1_A::ENABLE)
    }
}
#[doc = "Field `WMENA` reader - Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
pub type WMENA_R = crate::BitReader<WMENA_A>;
#[doc = "Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WMENA_A {
    #[doc = "0: Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    WMENA_0 = 0,
    #[doc = "1: Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    WMENA_1 = 1,
}
impl From<WMENA_A> for bool {
    #[inline(always)]
    fn from(variant: WMENA_A) -> Self {
        variant as u8 != 0
    }
}
impl WMENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMENA_A {
        match self.bits {
            false => WMENA_A::WMENA_0,
            true => WMENA_A::WMENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `WMENA_0`"]
    #[inline(always)]
    pub fn is_wmena_0(&self) -> bool {
        *self == WMENA_A::WMENA_0
    }
    #[doc = "Checks if the value of the field is `WMENA_1`"]
    #[inline(always)]
    pub fn is_wmena_1(&self) -> bool {
        *self == WMENA_A::WMENA_1
    }
}
#[doc = "Field `WMENA` writer - Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
pub type WMENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLSHCR4_SPEC, WMENA_A, O>;
impl<'a, const O: u8> WMENA_W<'a, O> {
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    #[inline(always)]
    pub fn wmena_0(self) -> &'a mut W {
        self.variant(WMENA_A::WMENA_0)
    }
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    #[inline(always)]
    pub fn wmena_1(self) -> &'a mut W {
        self.variant(WMENA_A::WMENA_1)
    }
}
#[doc = "Field `WMENB` reader - Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
pub type WMENB_R = crate::BitReader<WMENB_A>;
#[doc = "Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WMENB_A {
    #[doc = "0: Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    WMENB_0 = 0,
    #[doc = "1: Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    WMENB_1 = 1,
}
impl From<WMENB_A> for bool {
    #[inline(always)]
    fn from(variant: WMENB_A) -> Self {
        variant as u8 != 0
    }
}
impl WMENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMENB_A {
        match self.bits {
            false => WMENB_A::WMENB_0,
            true => WMENB_A::WMENB_1,
        }
    }
    #[doc = "Checks if the value of the field is `WMENB_0`"]
    #[inline(always)]
    pub fn is_wmenb_0(&self) -> bool {
        *self == WMENB_A::WMENB_0
    }
    #[doc = "Checks if the value of the field is `WMENB_1`"]
    #[inline(always)]
    pub fn is_wmenb_1(&self) -> bool {
        *self == WMENB_A::WMENB_1
    }
}
#[doc = "Field `WMENB` writer - Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
pub type WMENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLSHCR4_SPEC, WMENB_A, O>;
impl<'a, const O: u8> WMENB_W<'a, O> {
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    #[inline(always)]
    pub fn wmenb_0(self) -> &'a mut W {
        self.variant(WMENB_A::WMENB_0)
    }
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    #[inline(always)]
    pub fn wmenb_1(self) -> &'a mut W {
        self.variant(WMENB_A::WMENB_1)
    }
}
impl R {
    #[doc = "Bit 0 - Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
    #[inline(always)]
    pub fn wmopt1(&self) -> WMOPT1_R {
        WMOPT1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
    #[inline(always)]
    pub fn wmena(&self) -> WMENA_R {
        WMENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
    #[inline(always)]
    pub fn wmenb(&self) -> WMENB_R {
        WMENB_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
    #[inline(always)]
    #[must_use]
    pub fn wmopt1(&mut self) -> WMOPT1_W<0> {
        WMOPT1_W::new(self)
    }
    #[doc = "Bit 2 - Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
    #[inline(always)]
    #[must_use]
    pub fn wmena(&mut self) -> WMENA_W<2> {
        WMENA_W::new(self)
    }
    #[doc = "Bit 3 - Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
    #[inline(always)]
    #[must_use]
    pub fn wmenb(&mut self) -> WMENB_W<3> {
        WMENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flshcr4](index.html) module"]
pub struct FLSHCR4_SPEC;
impl crate::RegisterSpec for FLSHCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flshcr4::R](R) reader structure"]
impl crate::Readable for FLSHCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flshcr4::W](W) writer structure"]
impl crate::Writable for FLSHCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLSHCR4 to value 0"]
impl crate::Resettable for FLSHCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
