#[doc = "Register `CM7_CACR` reader"]
pub struct R(crate::R<CM7_CACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM7_CACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM7_CACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM7_CACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM7_CACR` writer"]
pub struct W(crate::W<CM7_CACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM7_CACR_SPEC>;
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
impl From<crate::W<CM7_CACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM7_CACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIWT` reader - Shared cacheable-is-WT for data cache. Enables limited cache coherency usage."]
pub type SIWT_R = crate::BitReader<SIWT_A>;
#[doc = "Shared cacheable-is-WT for data cache. Enables limited cache coherency usage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIWT_A {
    #[doc = "0: Normal Cacheable Shared locations are treated as being Non-cacheable. Default mode of operation for Shared memory."]
    SIWT_0 = 0,
    #[doc = "1: Normal Cacheable shared locations are treated as Write-Through."]
    SIWT_1 = 1,
}
impl From<SIWT_A> for bool {
    #[inline(always)]
    fn from(variant: SIWT_A) -> Self {
        variant as u8 != 0
    }
}
impl SIWT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIWT_A {
        match self.bits {
            false => SIWT_A::SIWT_0,
            true => SIWT_A::SIWT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SIWT_0`"]
    #[inline(always)]
    pub fn is_siwt_0(&self) -> bool {
        *self == SIWT_A::SIWT_0
    }
    #[doc = "Checks if the value of the field is `SIWT_1`"]
    #[inline(always)]
    pub fn is_siwt_1(&self) -> bool {
        *self == SIWT_A::SIWT_1
    }
}
#[doc = "Field `SIWT` writer - Shared cacheable-is-WT for data cache. Enables limited cache coherency usage."]
pub type SIWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_CACR_SPEC, SIWT_A, O>;
impl<'a, const O: u8> SIWT_W<'a, O> {
    #[doc = "Normal Cacheable Shared locations are treated as being Non-cacheable. Default mode of operation for Shared memory."]
    #[inline(always)]
    pub fn siwt_0(self) -> &'a mut W {
        self.variant(SIWT_A::SIWT_0)
    }
    #[doc = "Normal Cacheable shared locations are treated as Write-Through."]
    #[inline(always)]
    pub fn siwt_1(self) -> &'a mut W {
        self.variant(SIWT_A::SIWT_1)
    }
}
#[doc = "Field `ECCDIS` reader - Enables ECC in the instruction and data cache."]
pub type ECCDIS_R = crate::BitReader<ECCDIS_A>;
#[doc = "Enables ECC in the instruction and data cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCDIS_A {
    #[doc = "0: Enables ECC in the instruction and data cache."]
    ECCDIS_0 = 0,
    #[doc = "1: Disables ECC in the instruction and data cache."]
    ECCDIS_1 = 1,
}
impl From<ECCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ECCDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCDIS_A {
        match self.bits {
            false => ECCDIS_A::ECCDIS_0,
            true => ECCDIS_A::ECCDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECCDIS_0`"]
    #[inline(always)]
    pub fn is_eccdis_0(&self) -> bool {
        *self == ECCDIS_A::ECCDIS_0
    }
    #[doc = "Checks if the value of the field is `ECCDIS_1`"]
    #[inline(always)]
    pub fn is_eccdis_1(&self) -> bool {
        *self == ECCDIS_A::ECCDIS_1
    }
}
#[doc = "Field `ECCDIS` writer - Enables ECC in the instruction and data cache."]
pub type ECCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_CACR_SPEC, ECCDIS_A, O>;
impl<'a, const O: u8> ECCDIS_W<'a, O> {
    #[doc = "Enables ECC in the instruction and data cache."]
    #[inline(always)]
    pub fn eccdis_0(self) -> &'a mut W {
        self.variant(ECCDIS_A::ECCDIS_0)
    }
    #[doc = "Disables ECC in the instruction and data cache."]
    #[inline(always)]
    pub fn eccdis_1(self) -> &'a mut W {
        self.variant(ECCDIS_A::ECCDIS_1)
    }
}
#[doc = "Field `FORCEWT` reader - Enables Force Write-Through in the data cache."]
pub type FORCEWT_R = crate::BitReader<FORCEWT_A>;
#[doc = "Enables Force Write-Through in the data cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCEWT_A {
    #[doc = "0: Disables Force Write-Through."]
    FORCEWT_0 = 0,
    #[doc = "1: Enables Force Write-Through. All Cacheable memory regions are treated as Write-Through."]
    FORCEWT_1 = 1,
}
impl From<FORCEWT_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEWT_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCEWT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEWT_A {
        match self.bits {
            false => FORCEWT_A::FORCEWT_0,
            true => FORCEWT_A::FORCEWT_1,
        }
    }
    #[doc = "Checks if the value of the field is `FORCEWT_0`"]
    #[inline(always)]
    pub fn is_forcewt_0(&self) -> bool {
        *self == FORCEWT_A::FORCEWT_0
    }
    #[doc = "Checks if the value of the field is `FORCEWT_1`"]
    #[inline(always)]
    pub fn is_forcewt_1(&self) -> bool {
        *self == FORCEWT_A::FORCEWT_1
    }
}
#[doc = "Field `FORCEWT` writer - Enables Force Write-Through in the data cache."]
pub type FORCEWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_CACR_SPEC, FORCEWT_A, O>;
impl<'a, const O: u8> FORCEWT_W<'a, O> {
    #[doc = "Disables Force Write-Through."]
    #[inline(always)]
    pub fn forcewt_0(self) -> &'a mut W {
        self.variant(FORCEWT_A::FORCEWT_0)
    }
    #[doc = "Enables Force Write-Through. All Cacheable memory regions are treated as Write-Through."]
    #[inline(always)]
    pub fn forcewt_1(self) -> &'a mut W {
        self.variant(FORCEWT_A::FORCEWT_1)
    }
}
impl R {
    #[doc = "Bit 0 - Shared cacheable-is-WT for data cache. Enables limited cache coherency usage."]
    #[inline(always)]
    pub fn siwt(&self) -> SIWT_R {
        SIWT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables ECC in the instruction and data cache."]
    #[inline(always)]
    pub fn eccdis(&self) -> ECCDIS_R {
        ECCDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables Force Write-Through in the data cache."]
    #[inline(always)]
    pub fn forcewt(&self) -> FORCEWT_R {
        FORCEWT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shared cacheable-is-WT for data cache. Enables limited cache coherency usage."]
    #[inline(always)]
    #[must_use]
    pub fn siwt(&mut self) -> SIWT_W<0> {
        SIWT_W::new(self)
    }
    #[doc = "Bit 1 - Enables ECC in the instruction and data cache."]
    #[inline(always)]
    #[must_use]
    pub fn eccdis(&mut self) -> ECCDIS_W<1> {
        ECCDIS_W::new(self)
    }
    #[doc = "Bit 2 - Enables Force Write-Through in the data cache."]
    #[inline(always)]
    #[must_use]
    pub fn forcewt(&mut self) -> FORCEWT_W<2> {
        FORCEWT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "L1 Cache Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_cacr](index.html) module"]
pub struct CM7_CACR_SPEC;
impl crate::RegisterSpec for CM7_CACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm7_cacr::R](R) reader structure"]
impl crate::Readable for CM7_CACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm7_cacr::W](W) writer structure"]
impl crate::Writable for CM7_CACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM7_CACR to value 0"]
impl crate::Resettable for CM7_CACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
