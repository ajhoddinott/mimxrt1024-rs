#[doc = "Register `CM7_DTCMCR` reader"]
pub struct R(crate::R<CM7_DTCMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM7_DTCMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM7_DTCMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM7_DTCMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM7_DTCMCR` writer"]
pub struct W(crate::W<CM7_DTCMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM7_DTCMCR_SPEC>;
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
impl From<crate::W<CM7_DTCMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM7_DTCMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "TCM enable. When a TCM is disabled all accesses are made to the AXIM interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: TCM disabled."]
    EN_0 = 0,
    #[doc = "1: TCM enabled."]
    EN_1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::EN_0,
            true => EN_A::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline(always)]
    pub fn is_en_0(&self) -> bool {
        *self == EN_A::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline(always)]
    pub fn is_en_1(&self) -> bool {
        *self == EN_A::EN_1
    }
}
#[doc = "Field `EN` writer - TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_DTCMCR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "TCM disabled."]
    #[inline(always)]
    pub fn en_0(self) -> &'a mut W {
        self.variant(EN_A::EN_0)
    }
    #[doc = "TCM enabled."]
    #[inline(always)]
    pub fn en_1(self) -> &'a mut W {
        self.variant(EN_A::EN_1)
    }
}
#[doc = "Field `RMW` reader - Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
pub type RMW_R = crate::BitReader<RMW_A>;
#[doc = "Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMW_A {
    #[doc = "0: RMW disabled."]
    RMW_0 = 0,
    #[doc = "1: RMW enabled."]
    RMW_1 = 1,
}
impl From<RMW_A> for bool {
    #[inline(always)]
    fn from(variant: RMW_A) -> Self {
        variant as u8 != 0
    }
}
impl RMW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMW_A {
        match self.bits {
            false => RMW_A::RMW_0,
            true => RMW_A::RMW_1,
        }
    }
    #[doc = "Checks if the value of the field is `RMW_0`"]
    #[inline(always)]
    pub fn is_rmw_0(&self) -> bool {
        *self == RMW_A::RMW_0
    }
    #[doc = "Checks if the value of the field is `RMW_1`"]
    #[inline(always)]
    pub fn is_rmw_1(&self) -> bool {
        *self == RMW_A::RMW_1
    }
}
#[doc = "Field `RMW` writer - Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
pub type RMW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_DTCMCR_SPEC, RMW_A, O>;
impl<'a, const O: u8> RMW_W<'a, O> {
    #[doc = "RMW disabled."]
    #[inline(always)]
    pub fn rmw_0(self) -> &'a mut W {
        self.variant(RMW_A::RMW_0)
    }
    #[doc = "RMW enabled."]
    #[inline(always)]
    pub fn rmw_1(self) -> &'a mut W {
        self.variant(RMW_A::RMW_1)
    }
}
#[doc = "Field `RETEN` reader - Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
pub type RETEN_R = crate::BitReader<RETEN_A>;
#[doc = "Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RETEN_A {
    #[doc = "0: Retry phase disabled."]
    RETEN_0 = 0,
    #[doc = "1: Retry phase enabled."]
    RETEN_1 = 1,
}
impl From<RETEN_A> for bool {
    #[inline(always)]
    fn from(variant: RETEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RETEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETEN_A {
        match self.bits {
            false => RETEN_A::RETEN_0,
            true => RETEN_A::RETEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RETEN_0`"]
    #[inline(always)]
    pub fn is_reten_0(&self) -> bool {
        *self == RETEN_A::RETEN_0
    }
    #[doc = "Checks if the value of the field is `RETEN_1`"]
    #[inline(always)]
    pub fn is_reten_1(&self) -> bool {
        *self == RETEN_A::RETEN_1
    }
}
#[doc = "Field `RETEN` writer - Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
pub type RETEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_DTCMCR_SPEC, RETEN_A, O>;
impl<'a, const O: u8> RETEN_W<'a, O> {
    #[doc = "Retry phase disabled."]
    #[inline(always)]
    pub fn reten_0(self) -> &'a mut W {
        self.variant(RETEN_A::RETEN_0)
    }
    #[doc = "Retry phase enabled."]
    #[inline(always)]
    pub fn reten_1(self) -> &'a mut W {
        self.variant(RETEN_A::RETEN_1)
    }
}
#[doc = "Field `SZ` reader - TCM size. Indicates the size of the relevant TCM."]
pub type SZ_R = crate::FieldReader<u8, SZ_A>;
#[doc = "TCM size. Indicates the size of the relevant TCM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SZ_A {
    #[doc = "0: No TCM implemented."]
    SZ_0 = 0,
    #[doc = "3: 4KB."]
    SZ_3 = 3,
    #[doc = "4: 8KB."]
    SZ_4 = 4,
    #[doc = "5: 16KB."]
    SZ_5 = 5,
    #[doc = "6: 32KB."]
    SZ_6 = 6,
    #[doc = "7: 64KB."]
    SZ_7 = 7,
    #[doc = "8: 128KB."]
    SZ_8 = 8,
    #[doc = "9: 256KB."]
    SZ_9 = 9,
    #[doc = "10: 512KB."]
    SZ_10 = 10,
    #[doc = "11: 1MB."]
    SZ_11 = 11,
    #[doc = "12: 2MB."]
    SZ_12 = 12,
    #[doc = "13: 4MB."]
    SZ_13 = 13,
    #[doc = "14: 8MB."]
    SZ_14 = 14,
    #[doc = "15: 16MB."]
    SZ_15 = 15,
}
impl From<SZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SZ_A) -> Self {
        variant as _
    }
}
impl SZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SZ_A> {
        match self.bits {
            0 => Some(SZ_A::SZ_0),
            3 => Some(SZ_A::SZ_3),
            4 => Some(SZ_A::SZ_4),
            5 => Some(SZ_A::SZ_5),
            6 => Some(SZ_A::SZ_6),
            7 => Some(SZ_A::SZ_7),
            8 => Some(SZ_A::SZ_8),
            9 => Some(SZ_A::SZ_9),
            10 => Some(SZ_A::SZ_10),
            11 => Some(SZ_A::SZ_11),
            12 => Some(SZ_A::SZ_12),
            13 => Some(SZ_A::SZ_13),
            14 => Some(SZ_A::SZ_14),
            15 => Some(SZ_A::SZ_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SZ_0`"]
    #[inline(always)]
    pub fn is_sz_0(&self) -> bool {
        *self == SZ_A::SZ_0
    }
    #[doc = "Checks if the value of the field is `SZ_3`"]
    #[inline(always)]
    pub fn is_sz_3(&self) -> bool {
        *self == SZ_A::SZ_3
    }
    #[doc = "Checks if the value of the field is `SZ_4`"]
    #[inline(always)]
    pub fn is_sz_4(&self) -> bool {
        *self == SZ_A::SZ_4
    }
    #[doc = "Checks if the value of the field is `SZ_5`"]
    #[inline(always)]
    pub fn is_sz_5(&self) -> bool {
        *self == SZ_A::SZ_5
    }
    #[doc = "Checks if the value of the field is `SZ_6`"]
    #[inline(always)]
    pub fn is_sz_6(&self) -> bool {
        *self == SZ_A::SZ_6
    }
    #[doc = "Checks if the value of the field is `SZ_7`"]
    #[inline(always)]
    pub fn is_sz_7(&self) -> bool {
        *self == SZ_A::SZ_7
    }
    #[doc = "Checks if the value of the field is `SZ_8`"]
    #[inline(always)]
    pub fn is_sz_8(&self) -> bool {
        *self == SZ_A::SZ_8
    }
    #[doc = "Checks if the value of the field is `SZ_9`"]
    #[inline(always)]
    pub fn is_sz_9(&self) -> bool {
        *self == SZ_A::SZ_9
    }
    #[doc = "Checks if the value of the field is `SZ_10`"]
    #[inline(always)]
    pub fn is_sz_10(&self) -> bool {
        *self == SZ_A::SZ_10
    }
    #[doc = "Checks if the value of the field is `SZ_11`"]
    #[inline(always)]
    pub fn is_sz_11(&self) -> bool {
        *self == SZ_A::SZ_11
    }
    #[doc = "Checks if the value of the field is `SZ_12`"]
    #[inline(always)]
    pub fn is_sz_12(&self) -> bool {
        *self == SZ_A::SZ_12
    }
    #[doc = "Checks if the value of the field is `SZ_13`"]
    #[inline(always)]
    pub fn is_sz_13(&self) -> bool {
        *self == SZ_A::SZ_13
    }
    #[doc = "Checks if the value of the field is `SZ_14`"]
    #[inline(always)]
    pub fn is_sz_14(&self) -> bool {
        *self == SZ_A::SZ_14
    }
    #[doc = "Checks if the value of the field is `SZ_15`"]
    #[inline(always)]
    pub fn is_sz_15(&self) -> bool {
        *self == SZ_A::SZ_15
    }
}
impl R {
    #[doc = "Bit 0 - TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
    #[inline(always)]
    pub fn rmw(&self) -> RMW_R {
        RMW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
    #[inline(always)]
    pub fn reten(&self) -> RETEN_R {
        RETEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - TCM size. Indicates the size of the relevant TCM."]
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
    #[inline(always)]
    #[must_use]
    pub fn rmw(&mut self) -> RMW_W<1> {
        RMW_W::new(self)
    }
    #[doc = "Bit 2 - Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
    #[inline(always)]
    #[must_use]
    pub fn reten(&mut self) -> RETEN_W<2> {
        RETEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Tightly-Coupled Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_dtcmcr](index.html) module"]
pub struct CM7_DTCMCR_SPEC;
impl crate::RegisterSpec for CM7_DTCMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm7_dtcmcr::R](R) reader structure"]
impl crate::Readable for CM7_DTCMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm7_dtcmcr::W](W) writer structure"]
impl crate::Writable for CM7_DTCMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM7_DTCMCR to value 0"]
impl crate::Resettable for CM7_DTCMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
