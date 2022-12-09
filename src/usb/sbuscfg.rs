#[doc = "Register `SBUSCFG` reader"]
pub struct R(crate::R<SBUSCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBUSCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBUSCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBUSCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBUSCFG` writer"]
pub struct W(crate::W<SBUSCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBUSCFG_SPEC>;
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
impl From<crate::W<SBUSCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBUSCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHBBRST` reader - AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)"]
pub type AHBBRST_R = crate::FieldReader<u8, AHBBRST_A>;
#[doc = "AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AHBBRST_A {
    #[doc = "0: Incremental burst of unspecified length only"]
    AHBBRST_0 = 0,
    #[doc = "1: INCR4 burst, then single transfer"]
    AHBBRST_1 = 1,
    #[doc = "2: INCR8 burst, INCR4 burst, then single transfer"]
    AHBBRST_2 = 2,
    #[doc = "3: INCR16 burst, INCR8 burst, INCR4 burst, then single transfer"]
    AHBBRST_3 = 3,
    #[doc = "5: INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_5 = 5,
    #[doc = "6: INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_6 = 6,
    #[doc = "7: INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_7 = 7,
}
impl From<AHBBRST_A> for u8 {
    #[inline(always)]
    fn from(variant: AHBBRST_A) -> Self {
        variant as _
    }
}
impl AHBBRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AHBBRST_A> {
        match self.bits {
            0 => Some(AHBBRST_A::AHBBRST_0),
            1 => Some(AHBBRST_A::AHBBRST_1),
            2 => Some(AHBBRST_A::AHBBRST_2),
            3 => Some(AHBBRST_A::AHBBRST_3),
            5 => Some(AHBBRST_A::AHBBRST_5),
            6 => Some(AHBBRST_A::AHBBRST_6),
            7 => Some(AHBBRST_A::AHBBRST_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AHBBRST_0`"]
    #[inline(always)]
    pub fn is_ahbbrst_0(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_0
    }
    #[doc = "Checks if the value of the field is `AHBBRST_1`"]
    #[inline(always)]
    pub fn is_ahbbrst_1(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_1
    }
    #[doc = "Checks if the value of the field is `AHBBRST_2`"]
    #[inline(always)]
    pub fn is_ahbbrst_2(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_2
    }
    #[doc = "Checks if the value of the field is `AHBBRST_3`"]
    #[inline(always)]
    pub fn is_ahbbrst_3(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_3
    }
    #[doc = "Checks if the value of the field is `AHBBRST_5`"]
    #[inline(always)]
    pub fn is_ahbbrst_5(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_5
    }
    #[doc = "Checks if the value of the field is `AHBBRST_6`"]
    #[inline(always)]
    pub fn is_ahbbrst_6(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_6
    }
    #[doc = "Checks if the value of the field is `AHBBRST_7`"]
    #[inline(always)]
    pub fn is_ahbbrst_7(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_7
    }
}
#[doc = "Field `AHBBRST` writer - AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)"]
pub type AHBBRST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SBUSCFG_SPEC, u8, AHBBRST_A, 3, O>;
impl<'a, const O: u8> AHBBRST_W<'a, O> {
    #[doc = "Incremental burst of unspecified length only"]
    #[inline(always)]
    pub fn ahbbrst_0(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_0)
    }
    #[doc = "INCR4 burst, then single transfer"]
    #[inline(always)]
    pub fn ahbbrst_1(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_1)
    }
    #[doc = "INCR8 burst, INCR4 burst, then single transfer"]
    #[inline(always)]
    pub fn ahbbrst_2(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_2)
    }
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then single transfer"]
    #[inline(always)]
    pub fn ahbbrst_3(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_3)
    }
    #[doc = "INCR4 burst, then incremental burst of unspecified length"]
    #[inline(always)]
    pub fn ahbbrst_5(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_5)
    }
    #[doc = "INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    #[inline(always)]
    pub fn ahbbrst_6(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_6)
    }
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    #[inline(always)]
    pub fn ahbbrst_7(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_7)
    }
}
impl R {
    #[doc = "Bits 0:2 - AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)"]
    #[inline(always)]
    pub fn ahbbrst(&self) -> AHBBRST_R {
        AHBBRST_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)"]
    #[inline(always)]
    #[must_use]
    pub fn ahbbrst(&mut self) -> AHBBRST_W<0> {
        AHBBRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Bus Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbuscfg](index.html) module"]
pub struct SBUSCFG_SPEC;
impl crate::RegisterSpec for SBUSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbuscfg::R](R) reader structure"]
impl crate::Readable for SBUSCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbuscfg::W](W) writer structure"]
impl crate::Writable for SBUSCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBUSCFG to value 0x02"]
impl crate::Resettable for SBUSCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
