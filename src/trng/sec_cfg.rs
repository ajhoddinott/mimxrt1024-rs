#[doc = "Register `SEC_CFG` reader"]
pub struct R(crate::R<SEC_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CFG` writer"]
pub struct W(crate::W<SEC_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CFG_SPEC>;
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
impl From<crate::W<SEC_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNUSED0` reader - This bit is unused. Ignore."]
pub type UNUSED0_R = crate::BitReader<bool>;
#[doc = "Field `UNUSED0` writer - This bit is unused. Ignore."]
pub type UNUSED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEC_CFG_SPEC, bool, O>;
#[doc = "Field `NO_PRGM` reader - If set, the TRNG registers cannot be programmed"]
pub type NO_PRGM_R = crate::BitReader<NO_PRGM_A>;
#[doc = "If set, the TRNG registers cannot be programmed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NO_PRGM_A {
    #[doc = "0: Programability of registers controlled only by the Miscellaneous Control Register's access mode bit."]
    NO_PRGM_OFF = 0,
    #[doc = "1: Overides Miscellaneous Control Register access mode and prevents TRNG register programming."]
    NO_PRGM_ON = 1,
}
impl From<NO_PRGM_A> for bool {
    #[inline(always)]
    fn from(variant: NO_PRGM_A) -> Self {
        variant as u8 != 0
    }
}
impl NO_PRGM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NO_PRGM_A {
        match self.bits {
            false => NO_PRGM_A::NO_PRGM_OFF,
            true => NO_PRGM_A::NO_PRGM_ON,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PRGM_OFF`"]
    #[inline(always)]
    pub fn is_no_prgm_off(&self) -> bool {
        *self == NO_PRGM_A::NO_PRGM_OFF
    }
    #[doc = "Checks if the value of the field is `NO_PRGM_ON`"]
    #[inline(always)]
    pub fn is_no_prgm_on(&self) -> bool {
        *self == NO_PRGM_A::NO_PRGM_ON
    }
}
#[doc = "Field `NO_PRGM` writer - If set, the TRNG registers cannot be programmed"]
pub type NO_PRGM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEC_CFG_SPEC, NO_PRGM_A, O>;
impl<'a, const O: u8> NO_PRGM_W<'a, O> {
    #[doc = "Programability of registers controlled only by the Miscellaneous Control Register's access mode bit."]
    #[inline(always)]
    pub fn no_prgm_off(self) -> &'a mut W {
        self.variant(NO_PRGM_A::NO_PRGM_OFF)
    }
    #[doc = "Overides Miscellaneous Control Register access mode and prevents TRNG register programming."]
    #[inline(always)]
    pub fn no_prgm_on(self) -> &'a mut W {
        self.variant(NO_PRGM_A::NO_PRGM_ON)
    }
}
#[doc = "Field `UNUSED2` reader - This bit is unused. Ignore."]
pub type UNUSED2_R = crate::BitReader<bool>;
#[doc = "Field `UNUSED2` writer - This bit is unused. Ignore."]
pub type UNUSED2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEC_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is unused. Ignore."]
    #[inline(always)]
    pub fn unused0(&self) -> UNUSED0_R {
        UNUSED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If set, the TRNG registers cannot be programmed"]
    #[inline(always)]
    pub fn no_prgm(&self) -> NO_PRGM_R {
        NO_PRGM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is unused. Ignore."]
    #[inline(always)]
    pub fn unused2(&self) -> UNUSED2_R {
        UNUSED2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is unused. Ignore."]
    #[inline(always)]
    #[must_use]
    pub fn unused0(&mut self) -> UNUSED0_W<0> {
        UNUSED0_W::new(self)
    }
    #[doc = "Bit 1 - If set, the TRNG registers cannot be programmed"]
    #[inline(always)]
    #[must_use]
    pub fn no_prgm(&mut self) -> NO_PRGM_W<1> {
        NO_PRGM_W::new(self)
    }
    #[doc = "Bit 2 - This bit is unused. Ignore."]
    #[inline(always)]
    #[must_use]
    pub fn unused2(&mut self) -> UNUSED2_W<2> {
        UNUSED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_cfg](index.html) module"]
pub struct SEC_CFG_SPEC;
impl crate::RegisterSpec for SEC_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_cfg::R](R) reader structure"]
impl crate::Readable for SEC_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_cfg::W](W) writer structure"]
impl crate::Writable for SEC_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC_CFG to value 0"]
impl crate::Resettable for SEC_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
