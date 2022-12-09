#[doc = "Register `FTST0` reader"]
pub struct R(crate::R<FTST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTST0` writer"]
pub struct W(crate::W<FTST0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTST0_SPEC>;
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
impl From<crate::W<FTST0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTST0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTEST` reader - Fault Test"]
pub type FTEST_R = crate::BitReader<FTEST_A>;
#[doc = "Fault Test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTEST_A {
    #[doc = "0: No fault"]
    NO_FAULT = 0,
    #[doc = "1: Cause a simulated fault"]
    FAULT = 1,
}
impl From<FTEST_A> for bool {
    #[inline(always)]
    fn from(variant: FTEST_A) -> Self {
        variant as u8 != 0
    }
}
impl FTEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTEST_A {
        match self.bits {
            false => FTEST_A::NO_FAULT,
            true => FTEST_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FAULT`"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == FTEST_A::NO_FAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == FTEST_A::FAULT
    }
}
#[doc = "Field `FTEST` writer - Fault Test"]
pub type FTEST_W<'a, const O: u8> = crate::BitWriter<'a, u16, FTST0_SPEC, FTEST_A, O>;
impl<'a, const O: u8> FTEST_W<'a, O> {
    #[doc = "No fault"]
    #[inline(always)]
    pub fn no_fault(self) -> &'a mut W {
        self.variant(FTEST_A::NO_FAULT)
    }
    #[doc = "Cause a simulated fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(FTEST_A::FAULT)
    }
}
impl R {
    #[doc = "Bit 0 - Fault Test"]
    #[inline(always)]
    pub fn ftest(&self) -> FTEST_R {
        FTEST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Test"]
    #[inline(always)]
    #[must_use]
    pub fn ftest(&mut self) -> FTEST_W<0> {
        FTEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftst0](index.html) module"]
pub struct FTST0_SPEC;
impl crate::RegisterSpec for FTST0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ftst0::R](R) reader structure"]
impl crate::Readable for FTST0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftst0::W](W) writer structure"]
impl crate::Writable for FTST0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTST0 to value 0"]
impl crate::Resettable for FTST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
