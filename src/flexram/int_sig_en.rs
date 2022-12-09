#[doc = "Register `INT_SIG_EN` reader"]
pub struct R(crate::R<INT_SIG_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_SIG_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_SIG_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_SIG_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_SIG_EN` writer"]
pub struct W(crate::W<INT_SIG_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SIG_EN_SPEC>;
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
impl From<crate::W<INT_SIG_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SIG_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Reserved0` reader - Reserved"]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type RESERVED1_R = crate::BitReader<bool>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type RESERVED2_R = crate::BitReader<bool>;
#[doc = "Field `ITCM_ERR_SIG_EN` reader - ITCM Access Error Interrupt Enable"]
pub type ITCM_ERR_SIG_EN_R = crate::BitReader<ITCM_ERR_SIG_EN_A>;
#[doc = "ITCM Access Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITCM_ERR_SIG_EN_A {
    #[doc = "0: Masked"]
    ITCM_ERR_SIG_EN_0 = 0,
    #[doc = "1: Enabled"]
    ITCM_ERR_SIG_EN_1 = 1,
}
impl From<ITCM_ERR_SIG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ITCM_ERR_SIG_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ITCM_ERR_SIG_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITCM_ERR_SIG_EN_A {
        match self.bits {
            false => ITCM_ERR_SIG_EN_A::ITCM_ERR_SIG_EN_0,
            true => ITCM_ERR_SIG_EN_A::ITCM_ERR_SIG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ITCM_ERR_SIG_EN_0`"]
    #[inline(always)]
    pub fn is_itcm_err_sig_en_0(&self) -> bool {
        *self == ITCM_ERR_SIG_EN_A::ITCM_ERR_SIG_EN_0
    }
    #[doc = "Checks if the value of the field is `ITCM_ERR_SIG_EN_1`"]
    #[inline(always)]
    pub fn is_itcm_err_sig_en_1(&self) -> bool {
        *self == ITCM_ERR_SIG_EN_A::ITCM_ERR_SIG_EN_1
    }
}
#[doc = "Field `ITCM_ERR_SIG_EN` writer - ITCM Access Error Interrupt Enable"]
pub type ITCM_ERR_SIG_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_SIG_EN_SPEC, ITCM_ERR_SIG_EN_A, O>;
impl<'a, const O: u8> ITCM_ERR_SIG_EN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn itcm_err_sig_en_0(self) -> &'a mut W {
        self.variant(ITCM_ERR_SIG_EN_A::ITCM_ERR_SIG_EN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn itcm_err_sig_en_1(self) -> &'a mut W {
        self.variant(ITCM_ERR_SIG_EN_A::ITCM_ERR_SIG_EN_1)
    }
}
#[doc = "Field `DTCM_ERR_SIG_EN` reader - DTCM Access Error Interrupt Enable"]
pub type DTCM_ERR_SIG_EN_R = crate::BitReader<DTCM_ERR_SIG_EN_A>;
#[doc = "DTCM Access Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCM_ERR_SIG_EN_A {
    #[doc = "0: Masked"]
    DTCM_ERR_SIG_EN_0 = 0,
    #[doc = "1: Enabled"]
    DTCM_ERR_SIG_EN_1 = 1,
}
impl From<DTCM_ERR_SIG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DTCM_ERR_SIG_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCM_ERR_SIG_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCM_ERR_SIG_EN_A {
        match self.bits {
            false => DTCM_ERR_SIG_EN_A::DTCM_ERR_SIG_EN_0,
            true => DTCM_ERR_SIG_EN_A::DTCM_ERR_SIG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTCM_ERR_SIG_EN_0`"]
    #[inline(always)]
    pub fn is_dtcm_err_sig_en_0(&self) -> bool {
        *self == DTCM_ERR_SIG_EN_A::DTCM_ERR_SIG_EN_0
    }
    #[doc = "Checks if the value of the field is `DTCM_ERR_SIG_EN_1`"]
    #[inline(always)]
    pub fn is_dtcm_err_sig_en_1(&self) -> bool {
        *self == DTCM_ERR_SIG_EN_A::DTCM_ERR_SIG_EN_1
    }
}
#[doc = "Field `DTCM_ERR_SIG_EN` writer - DTCM Access Error Interrupt Enable"]
pub type DTCM_ERR_SIG_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_SIG_EN_SPEC, DTCM_ERR_SIG_EN_A, O>;
impl<'a, const O: u8> DTCM_ERR_SIG_EN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dtcm_err_sig_en_0(self) -> &'a mut W {
        self.variant(DTCM_ERR_SIG_EN_A::DTCM_ERR_SIG_EN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dtcm_err_sig_en_1(self) -> &'a mut W {
        self.variant(DTCM_ERR_SIG_EN_A::DTCM_ERR_SIG_EN_1)
    }
}
#[doc = "Field `OCRAM_ERR_SIG_EN` reader - OCRAM Access Error Interrupt Enable"]
pub type OCRAM_ERR_SIG_EN_R = crate::BitReader<OCRAM_ERR_SIG_EN_A>;
#[doc = "OCRAM Access Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCRAM_ERR_SIG_EN_A {
    #[doc = "0: Masked"]
    OCRAM_ERR_SIG_EN_0 = 0,
    #[doc = "1: Enabled"]
    OCRAM_ERR_SIG_EN_1 = 1,
}
impl From<OCRAM_ERR_SIG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OCRAM_ERR_SIG_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl OCRAM_ERR_SIG_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCRAM_ERR_SIG_EN_A {
        match self.bits {
            false => OCRAM_ERR_SIG_EN_A::OCRAM_ERR_SIG_EN_0,
            true => OCRAM_ERR_SIG_EN_A::OCRAM_ERR_SIG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM_ERR_SIG_EN_0`"]
    #[inline(always)]
    pub fn is_ocram_err_sig_en_0(&self) -> bool {
        *self == OCRAM_ERR_SIG_EN_A::OCRAM_ERR_SIG_EN_0
    }
    #[doc = "Checks if the value of the field is `OCRAM_ERR_SIG_EN_1`"]
    #[inline(always)]
    pub fn is_ocram_err_sig_en_1(&self) -> bool {
        *self == OCRAM_ERR_SIG_EN_A::OCRAM_ERR_SIG_EN_1
    }
}
#[doc = "Field `OCRAM_ERR_SIG_EN` writer - OCRAM Access Error Interrupt Enable"]
pub type OCRAM_ERR_SIG_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_SIG_EN_SPEC, OCRAM_ERR_SIG_EN_A, O>;
impl<'a, const O: u8> OCRAM_ERR_SIG_EN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ocram_err_sig_en_0(self) -> &'a mut W {
        self.variant(OCRAM_ERR_SIG_EN_A::OCRAM_ERR_SIG_EN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ocram_err_sig_en_1(self) -> &'a mut W {
        self.variant(OCRAM_ERR_SIG_EN_A::OCRAM_ERR_SIG_EN_1)
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
    #[doc = "Bit 3 - ITCM Access Error Interrupt Enable"]
    #[inline(always)]
    pub fn itcm_err_sig_en(&self) -> ITCM_ERR_SIG_EN_R {
        ITCM_ERR_SIG_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DTCM Access Error Interrupt Enable"]
    #[inline(always)]
    pub fn dtcm_err_sig_en(&self) -> DTCM_ERR_SIG_EN_R {
        DTCM_ERR_SIG_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OCRAM Access Error Interrupt Enable"]
    #[inline(always)]
    pub fn ocram_err_sig_en(&self) -> OCRAM_ERR_SIG_EN_R {
        OCRAM_ERR_SIG_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ITCM Access Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn itcm_err_sig_en(&mut self) -> ITCM_ERR_SIG_EN_W<3> {
        ITCM_ERR_SIG_EN_W::new(self)
    }
    #[doc = "Bit 4 - DTCM Access Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtcm_err_sig_en(&mut self) -> DTCM_ERR_SIG_EN_W<4> {
        DTCM_ERR_SIG_EN_W::new(self)
    }
    #[doc = "Bit 5 - OCRAM Access Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocram_err_sig_en(&mut self) -> OCRAM_ERR_SIG_EN_W<5> {
        OCRAM_ERR_SIG_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_sig_en](index.html) module"]
pub struct INT_SIG_EN_SPEC;
impl crate::RegisterSpec for INT_SIG_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_sig_en::R](R) reader structure"]
impl crate::Readable for INT_SIG_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_sig_en::W](W) writer structure"]
impl crate::Writable for INT_SIG_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_SIG_EN to value 0"]
impl crate::Resettable for INT_SIG_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
