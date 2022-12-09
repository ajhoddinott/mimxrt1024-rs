#[doc = "Register `CSSELR` reader"]
pub struct R(crate::R<CSSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSSELR` writer"]
pub struct W(crate::W<CSSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSSELR_SPEC>;
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
impl From<crate::W<CSSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IND` reader - Instruction not data bit"]
pub type IND_R = crate::BitReader<IND_A>;
#[doc = "Instruction not data bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IND_A {
    #[doc = "0: Data or unified cache."]
    IND_0 = 0,
    #[doc = "1: Instruction cache."]
    IND_1 = 1,
}
impl From<IND_A> for bool {
    #[inline(always)]
    fn from(variant: IND_A) -> Self {
        variant as u8 != 0
    }
}
impl IND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IND_A {
        match self.bits {
            false => IND_A::IND_0,
            true => IND_A::IND_1,
        }
    }
    #[doc = "Checks if the value of the field is `IND_0`"]
    #[inline(always)]
    pub fn is_ind_0(&self) -> bool {
        *self == IND_A::IND_0
    }
    #[doc = "Checks if the value of the field is `IND_1`"]
    #[inline(always)]
    pub fn is_ind_1(&self) -> bool {
        *self == IND_A::IND_1
    }
}
#[doc = "Field `IND` writer - Instruction not data bit"]
pub type IND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSSELR_SPEC, IND_A, O>;
impl<'a, const O: u8> IND_W<'a, O> {
    #[doc = "Data or unified cache."]
    #[inline(always)]
    pub fn ind_0(self) -> &'a mut W {
        self.variant(IND_A::IND_0)
    }
    #[doc = "Instruction cache."]
    #[inline(always)]
    pub fn ind_1(self) -> &'a mut W {
        self.variant(IND_A::IND_1)
    }
}
#[doc = "Field `LEVEL` reader - Cache level of required cache"]
pub type LEVEL_R = crate::FieldReader<u8, LEVEL_A>;
#[doc = "Cache level of required cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEVEL_A {
    #[doc = "0: Level 1 cache."]
    LEVEL_0 = 0,
    #[doc = "1: Level 2 cache."]
    LEVEL_1 = 1,
    #[doc = "2: Level 3 cache."]
    LEVEL_2 = 2,
    #[doc = "3: Level 4 cache."]
    LEVEL_3 = 3,
    #[doc = "4: Level 5 cache."]
    LEVEL_4 = 4,
    #[doc = "5: Level 6 cache."]
    LEVEL_5 = 5,
    #[doc = "6: Level 7 cache."]
    LEVEL_6 = 6,
}
impl From<LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LEVEL_A) -> Self {
        variant as _
    }
}
impl LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LEVEL_A> {
        match self.bits {
            0 => Some(LEVEL_A::LEVEL_0),
            1 => Some(LEVEL_A::LEVEL_1),
            2 => Some(LEVEL_A::LEVEL_2),
            3 => Some(LEVEL_A::LEVEL_3),
            4 => Some(LEVEL_A::LEVEL_4),
            5 => Some(LEVEL_A::LEVEL_5),
            6 => Some(LEVEL_A::LEVEL_6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_0`"]
    #[inline(always)]
    pub fn is_level_0(&self) -> bool {
        *self == LEVEL_A::LEVEL_0
    }
    #[doc = "Checks if the value of the field is `LEVEL_1`"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == LEVEL_A::LEVEL_1
    }
    #[doc = "Checks if the value of the field is `LEVEL_2`"]
    #[inline(always)]
    pub fn is_level_2(&self) -> bool {
        *self == LEVEL_A::LEVEL_2
    }
    #[doc = "Checks if the value of the field is `LEVEL_3`"]
    #[inline(always)]
    pub fn is_level_3(&self) -> bool {
        *self == LEVEL_A::LEVEL_3
    }
    #[doc = "Checks if the value of the field is `LEVEL_4`"]
    #[inline(always)]
    pub fn is_level_4(&self) -> bool {
        *self == LEVEL_A::LEVEL_4
    }
    #[doc = "Checks if the value of the field is `LEVEL_5`"]
    #[inline(always)]
    pub fn is_level_5(&self) -> bool {
        *self == LEVEL_A::LEVEL_5
    }
    #[doc = "Checks if the value of the field is `LEVEL_6`"]
    #[inline(always)]
    pub fn is_level_6(&self) -> bool {
        *self == LEVEL_A::LEVEL_6
    }
}
#[doc = "Field `LEVEL` writer - Cache level of required cache"]
pub type LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSSELR_SPEC, u8, LEVEL_A, 3, O>;
impl<'a, const O: u8> LEVEL_W<'a, O> {
    #[doc = "Level 1 cache."]
    #[inline(always)]
    pub fn level_0(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_0)
    }
    #[doc = "Level 2 cache."]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_1)
    }
    #[doc = "Level 3 cache."]
    #[inline(always)]
    pub fn level_2(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_2)
    }
    #[doc = "Level 4 cache."]
    #[inline(always)]
    pub fn level_3(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_3)
    }
    #[doc = "Level 5 cache."]
    #[inline(always)]
    pub fn level_4(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_4)
    }
    #[doc = "Level 6 cache."]
    #[inline(always)]
    pub fn level_5(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_5)
    }
    #[doc = "Level 7 cache."]
    #[inline(always)]
    pub fn level_6(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_6)
    }
}
impl R {
    #[doc = "Bit 0 - Instruction not data bit"]
    #[inline(always)]
    pub fn ind(&self) -> IND_R {
        IND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Cache level of required cache"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction not data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ind(&mut self) -> IND_W<0> {
        IND_W::new(self)
    }
    #[doc = "Bits 1:3 - Cache level of required cache"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<1> {
        LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Size Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csselr](index.html) module"]
pub struct CSSELR_SPEC;
impl crate::RegisterSpec for CSSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csselr::R](R) reader structure"]
impl crate::Readable for CSSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csselr::W](W) writer structure"]
impl crate::Writable for CSSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSSELR to value 0"]
impl crate::Resettable for CSSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
