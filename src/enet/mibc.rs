#[doc = "Register `MIBC` reader"]
pub struct R(crate::R<MIBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIBC` writer"]
pub struct W(crate::W<MIBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIBC_SPEC>;
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
impl From<crate::W<MIBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIB_CLEAR` reader - MIB Clear"]
pub type MIB_CLEAR_R = crate::BitReader<MIB_CLEAR_A>;
#[doc = "MIB Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIB_CLEAR_A {
    #[doc = "0: See note above."]
    ZERO = 0,
    #[doc = "1: All statistics counters are reset to 0."]
    ONE = 1,
}
impl From<MIB_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: MIB_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl MIB_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIB_CLEAR_A {
        match self.bits {
            false => MIB_CLEAR_A::ZERO,
            true => MIB_CLEAR_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == MIB_CLEAR_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == MIB_CLEAR_A::ONE
    }
}
#[doc = "Field `MIB_CLEAR` writer - MIB Clear"]
pub type MIB_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIBC_SPEC, MIB_CLEAR_A, O>;
impl<'a, const O: u8> MIB_CLEAR_W<'a, O> {
    #[doc = "See note above."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(MIB_CLEAR_A::ZERO)
    }
    #[doc = "All statistics counters are reset to 0."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(MIB_CLEAR_A::ONE)
    }
}
#[doc = "Field `MIB_IDLE` reader - MIB Idle"]
pub type MIB_IDLE_R = crate::BitReader<MIB_IDLE_A>;
#[doc = "MIB Idle\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIB_IDLE_A {
    #[doc = "0: The MIB block is updating MIB counters."]
    ZERO = 0,
    #[doc = "1: The MIB block is not currently updating any MIB counters."]
    ONE = 1,
}
impl From<MIB_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: MIB_IDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl MIB_IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIB_IDLE_A {
        match self.bits {
            false => MIB_IDLE_A::ZERO,
            true => MIB_IDLE_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == MIB_IDLE_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == MIB_IDLE_A::ONE
    }
}
#[doc = "Field `MIB_DIS` reader - Disable MIB Logic"]
pub type MIB_DIS_R = crate::BitReader<MIB_DIS_A>;
#[doc = "Disable MIB Logic\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIB_DIS_A {
    #[doc = "0: MIB logic is enabled."]
    ZERO = 0,
    #[doc = "1: MIB logic is disabled. The MIB logic halts and does not update any MIB counters."]
    ONE = 1,
}
impl From<MIB_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: MIB_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl MIB_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIB_DIS_A {
        match self.bits {
            false => MIB_DIS_A::ZERO,
            true => MIB_DIS_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == MIB_DIS_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == MIB_DIS_A::ONE
    }
}
#[doc = "Field `MIB_DIS` writer - Disable MIB Logic"]
pub type MIB_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIBC_SPEC, MIB_DIS_A, O>;
impl<'a, const O: u8> MIB_DIS_W<'a, O> {
    #[doc = "MIB logic is enabled."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(MIB_DIS_A::ZERO)
    }
    #[doc = "MIB logic is disabled. The MIB logic halts and does not update any MIB counters."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(MIB_DIS_A::ONE)
    }
}
impl R {
    #[doc = "Bit 29 - MIB Clear"]
    #[inline(always)]
    pub fn mib_clear(&self) -> MIB_CLEAR_R {
        MIB_CLEAR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MIB Idle"]
    #[inline(always)]
    pub fn mib_idle(&self) -> MIB_IDLE_R {
        MIB_IDLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable MIB Logic"]
    #[inline(always)]
    pub fn mib_dis(&self) -> MIB_DIS_R {
        MIB_DIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - MIB Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mib_clear(&mut self) -> MIB_CLEAR_W<29> {
        MIB_CLEAR_W::new(self)
    }
    #[doc = "Bit 31 - Disable MIB Logic"]
    #[inline(always)]
    #[must_use]
    pub fn mib_dis(&mut self) -> MIB_DIS_W<31> {
        MIB_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIB Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mibc](index.html) module"]
pub struct MIBC_SPEC;
impl crate::RegisterSpec for MIBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mibc::R](R) reader structure"]
impl crate::Readable for MIBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mibc::W](W) writer structure"]
impl crate::Writable for MIBC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIBC to value 0xc000_0000"]
impl crate::Resettable for MIBC_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
