#[doc = "Register `FLSHCR1%s` reader"]
pub struct R(crate::R<FLSHCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLSHCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLSHCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLSHCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLSHCR1%s` writer"]
pub struct W(crate::W<FLSHCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLSHCR1_SPEC>;
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
impl From<crate::W<FLSHCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLSHCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCSS` reader - Serial Flash CS setup time."]
pub type TCSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCSS` writer - Serial Flash CS setup time."]
pub type TCSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLSHCR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `TCSH` reader - Serial Flash CS Hold time."]
pub type TCSH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCSH` writer - Serial Flash CS Hold time."]
pub type TCSH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLSHCR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `WA` reader - Word Addressable."]
pub type WA_R = crate::BitReader<bool>;
#[doc = "Field `WA` writer - Word Addressable."]
pub type WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLSHCR1_SPEC, bool, O>;
#[doc = "Field `CAS` reader - Column Address Size."]
pub type CAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAS` writer - Column Address Size."]
pub type CAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLSHCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CSINTERVALUNIT` reader - CS interval unit"]
pub type CSINTERVALUNIT_R = crate::BitReader<CSINTERVALUNIT_A>;
#[doc = "CS interval unit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSINTERVALUNIT_A {
    #[doc = "0: The CS interval unit is 1 serial clock cycle"]
    CSINTERVALUNIT_0 = 0,
    #[doc = "1: The CS interval unit is 256 serial clock cycle"]
    CSINTERVALUNIT_1 = 1,
}
impl From<CSINTERVALUNIT_A> for bool {
    #[inline(always)]
    fn from(variant: CSINTERVALUNIT_A) -> Self {
        variant as u8 != 0
    }
}
impl CSINTERVALUNIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSINTERVALUNIT_A {
        match self.bits {
            false => CSINTERVALUNIT_A::CSINTERVALUNIT_0,
            true => CSINTERVALUNIT_A::CSINTERVALUNIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CSINTERVALUNIT_0`"]
    #[inline(always)]
    pub fn is_csintervalunit_0(&self) -> bool {
        *self == CSINTERVALUNIT_A::CSINTERVALUNIT_0
    }
    #[doc = "Checks if the value of the field is `CSINTERVALUNIT_1`"]
    #[inline(always)]
    pub fn is_csintervalunit_1(&self) -> bool {
        *self == CSINTERVALUNIT_A::CSINTERVALUNIT_1
    }
}
#[doc = "Field `CSINTERVALUNIT` writer - CS interval unit"]
pub type CSINTERVALUNIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLSHCR1_SPEC, CSINTERVALUNIT_A, O>;
impl<'a, const O: u8> CSINTERVALUNIT_W<'a, O> {
    #[doc = "The CS interval unit is 1 serial clock cycle"]
    #[inline(always)]
    pub fn csintervalunit_0(self) -> &'a mut W {
        self.variant(CSINTERVALUNIT_A::CSINTERVALUNIT_0)
    }
    #[doc = "The CS interval unit is 256 serial clock cycle"]
    #[inline(always)]
    pub fn csintervalunit_1(self) -> &'a mut W {
        self.variant(CSINTERVALUNIT_A::CSINTERVALUNIT_1)
    }
}
#[doc = "Field `CSINTERVAL` reader - This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
pub type CSINTERVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CSINTERVAL` writer - This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
pub type CSINTERVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLSHCR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:4 - Serial Flash CS setup time."]
    #[inline(always)]
    pub fn tcss(&self) -> TCSS_R {
        TCSS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Serial Flash CS Hold time."]
    #[inline(always)]
    pub fn tcsh(&self) -> TCSH_R {
        TCSH_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - Word Addressable."]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - Column Address Size."]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - CS interval unit"]
    #[inline(always)]
    pub fn csintervalunit(&self) -> CSINTERVALUNIT_R {
        CSINTERVALUNIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    #[inline(always)]
    pub fn csinterval(&self) -> CSINTERVAL_R {
        CSINTERVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Serial Flash CS setup time."]
    #[inline(always)]
    #[must_use]
    pub fn tcss(&mut self) -> TCSS_W<0> {
        TCSS_W::new(self)
    }
    #[doc = "Bits 5:9 - Serial Flash CS Hold time."]
    #[inline(always)]
    #[must_use]
    pub fn tcsh(&mut self) -> TCSH_W<5> {
        TCSH_W::new(self)
    }
    #[doc = "Bit 10 - Word Addressable."]
    #[inline(always)]
    #[must_use]
    pub fn wa(&mut self) -> WA_W<10> {
        WA_W::new(self)
    }
    #[doc = "Bits 11:14 - Column Address Size."]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CAS_W<11> {
        CAS_W::new(self)
    }
    #[doc = "Bit 15 - CS interval unit"]
    #[inline(always)]
    #[must_use]
    pub fn csintervalunit(&mut self) -> CSINTERVALUNIT_W<15> {
        CSINTERVALUNIT_W::new(self)
    }
    #[doc = "Bits 16:31 - This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn csinterval(&mut self) -> CSINTERVAL_W<16> {
        CSINTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flshcr1](index.html) module"]
pub struct FLSHCR1_SPEC;
impl crate::RegisterSpec for FLSHCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flshcr1::R](R) reader structure"]
impl crate::Readable for FLSHCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flshcr1::W](W) writer structure"]
impl crate::Writable for FLSHCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLSHCR1%s to value 0x63"]
impl crate::Resettable for FLSHCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x63;
}
