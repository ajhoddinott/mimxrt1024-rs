#[doc = "Register `INT_STATUS` reader"]
pub struct R(crate::R<INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_STATUS` writer"]
pub struct W(crate::W<INT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_STATUS_SPEC>;
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
impl From<crate::W<INT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Reserved0` reader - Reserved"]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type RESERVED1_R = crate::BitReader<bool>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type RESERVED2_R = crate::BitReader<bool>;
#[doc = "Field `ITCM_ERR_STATUS` reader - ITCM Access Error Status"]
pub type ITCM_ERR_STATUS_R = crate::BitReader<ITCM_ERR_STATUS_A>;
#[doc = "ITCM Access Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITCM_ERR_STATUS_A {
    #[doc = "0: ITCM access error does not happen"]
    ITCM_ERR_STATUS_0 = 0,
    #[doc = "1: ITCM access error happens."]
    ITCM_ERR_STATUS_1 = 1,
}
impl From<ITCM_ERR_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ITCM_ERR_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl ITCM_ERR_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITCM_ERR_STATUS_A {
        match self.bits {
            false => ITCM_ERR_STATUS_A::ITCM_ERR_STATUS_0,
            true => ITCM_ERR_STATUS_A::ITCM_ERR_STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ITCM_ERR_STATUS_0`"]
    #[inline(always)]
    pub fn is_itcm_err_status_0(&self) -> bool {
        *self == ITCM_ERR_STATUS_A::ITCM_ERR_STATUS_0
    }
    #[doc = "Checks if the value of the field is `ITCM_ERR_STATUS_1`"]
    #[inline(always)]
    pub fn is_itcm_err_status_1(&self) -> bool {
        *self == ITCM_ERR_STATUS_A::ITCM_ERR_STATUS_1
    }
}
#[doc = "Field `ITCM_ERR_STATUS` writer - ITCM Access Error Status"]
pub type ITCM_ERR_STATUS_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, ITCM_ERR_STATUS_A, O>;
impl<'a, const O: u8> ITCM_ERR_STATUS_W<'a, O> {
    #[doc = "ITCM access error does not happen"]
    #[inline(always)]
    pub fn itcm_err_status_0(self) -> &'a mut W {
        self.variant(ITCM_ERR_STATUS_A::ITCM_ERR_STATUS_0)
    }
    #[doc = "ITCM access error happens."]
    #[inline(always)]
    pub fn itcm_err_status_1(self) -> &'a mut W {
        self.variant(ITCM_ERR_STATUS_A::ITCM_ERR_STATUS_1)
    }
}
#[doc = "Field `DTCM_ERR_STATUS` reader - DTCM Access Error Status"]
pub type DTCM_ERR_STATUS_R = crate::BitReader<DTCM_ERR_STATUS_A>;
#[doc = "DTCM Access Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCM_ERR_STATUS_A {
    #[doc = "0: DTCM access error does not happen"]
    DTCM_ERR_STATUS_0 = 0,
    #[doc = "1: DTCM access error happens."]
    DTCM_ERR_STATUS_1 = 1,
}
impl From<DTCM_ERR_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DTCM_ERR_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCM_ERR_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCM_ERR_STATUS_A {
        match self.bits {
            false => DTCM_ERR_STATUS_A::DTCM_ERR_STATUS_0,
            true => DTCM_ERR_STATUS_A::DTCM_ERR_STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTCM_ERR_STATUS_0`"]
    #[inline(always)]
    pub fn is_dtcm_err_status_0(&self) -> bool {
        *self == DTCM_ERR_STATUS_A::DTCM_ERR_STATUS_0
    }
    #[doc = "Checks if the value of the field is `DTCM_ERR_STATUS_1`"]
    #[inline(always)]
    pub fn is_dtcm_err_status_1(&self) -> bool {
        *self == DTCM_ERR_STATUS_A::DTCM_ERR_STATUS_1
    }
}
#[doc = "Field `DTCM_ERR_STATUS` writer - DTCM Access Error Status"]
pub type DTCM_ERR_STATUS_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, DTCM_ERR_STATUS_A, O>;
impl<'a, const O: u8> DTCM_ERR_STATUS_W<'a, O> {
    #[doc = "DTCM access error does not happen"]
    #[inline(always)]
    pub fn dtcm_err_status_0(self) -> &'a mut W {
        self.variant(DTCM_ERR_STATUS_A::DTCM_ERR_STATUS_0)
    }
    #[doc = "DTCM access error happens."]
    #[inline(always)]
    pub fn dtcm_err_status_1(self) -> &'a mut W {
        self.variant(DTCM_ERR_STATUS_A::DTCM_ERR_STATUS_1)
    }
}
#[doc = "Field `OCRAM_ERR_STATUS` reader - OCRAM Access Error Status"]
pub type OCRAM_ERR_STATUS_R = crate::BitReader<OCRAM_ERR_STATUS_A>;
#[doc = "OCRAM Access Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCRAM_ERR_STATUS_A {
    #[doc = "0: OCRAM access error does not happen"]
    OCRAM_ERR_STATUS_0 = 0,
    #[doc = "1: OCRAM access error happens."]
    OCRAM_ERR_STATUS_1 = 1,
}
impl From<OCRAM_ERR_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: OCRAM_ERR_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl OCRAM_ERR_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCRAM_ERR_STATUS_A {
        match self.bits {
            false => OCRAM_ERR_STATUS_A::OCRAM_ERR_STATUS_0,
            true => OCRAM_ERR_STATUS_A::OCRAM_ERR_STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM_ERR_STATUS_0`"]
    #[inline(always)]
    pub fn is_ocram_err_status_0(&self) -> bool {
        *self == OCRAM_ERR_STATUS_A::OCRAM_ERR_STATUS_0
    }
    #[doc = "Checks if the value of the field is `OCRAM_ERR_STATUS_1`"]
    #[inline(always)]
    pub fn is_ocram_err_status_1(&self) -> bool {
        *self == OCRAM_ERR_STATUS_A::OCRAM_ERR_STATUS_1
    }
}
#[doc = "Field `OCRAM_ERR_STATUS` writer - OCRAM Access Error Status"]
pub type OCRAM_ERR_STATUS_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, OCRAM_ERR_STATUS_A, O>;
impl<'a, const O: u8> OCRAM_ERR_STATUS_W<'a, O> {
    #[doc = "OCRAM access error does not happen"]
    #[inline(always)]
    pub fn ocram_err_status_0(self) -> &'a mut W {
        self.variant(OCRAM_ERR_STATUS_A::OCRAM_ERR_STATUS_0)
    }
    #[doc = "OCRAM access error happens."]
    #[inline(always)]
    pub fn ocram_err_status_1(self) -> &'a mut W {
        self.variant(OCRAM_ERR_STATUS_A::OCRAM_ERR_STATUS_1)
    }
}
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ITCM Access Error Status"]
    #[inline(always)]
    pub fn itcm_err_status(&self) -> ITCM_ERR_STATUS_R {
        ITCM_ERR_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DTCM Access Error Status"]
    #[inline(always)]
    pub fn dtcm_err_status(&self) -> DTCM_ERR_STATUS_R {
        DTCM_ERR_STATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OCRAM Access Error Status"]
    #[inline(always)]
    pub fn ocram_err_status(&self) -> OCRAM_ERR_STATUS_R {
        OCRAM_ERR_STATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ITCM Access Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn itcm_err_status(&mut self) -> ITCM_ERR_STATUS_W<3> {
        ITCM_ERR_STATUS_W::new(self)
    }
    #[doc = "Bit 4 - DTCM Access Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn dtcm_err_status(&mut self) -> DTCM_ERR_STATUS_W<4> {
        DTCM_ERR_STATUS_W::new(self)
    }
    #[doc = "Bit 5 - OCRAM Access Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn ocram_err_status(&mut self) -> OCRAM_ERR_STATUS_W<5> {
        OCRAM_ERR_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](index.html) module"]
pub struct INT_STATUS_SPEC;
impl crate::RegisterSpec for INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_status::R](R) reader structure"]
impl crate::Readable for INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_status::W](W) writer structure"]
impl crate::Writable for INT_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x38;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
