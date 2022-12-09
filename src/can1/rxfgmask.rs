#[doc = "Register `RXFGMASK` reader"]
pub struct R(crate::R<RXFGMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFGMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFGMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFGMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFGMASK` writer"]
pub struct W(crate::W<RXFGMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFGMASK_SPEC>;
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
impl From<crate::W<RXFGMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFGMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FGM` reader - These bits mask the ID Filter Table elements bits in a perfect alignment"]
pub type FGM_R = crate::FieldReader<u32, FGM_A>;
#[doc = "These bits mask the ID Filter Table elements bits in a perfect alignment\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum FGM_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care\""]
    FGM_0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked"]
    FGM_1 = 1,
}
impl From<FGM_A> for u32 {
    #[inline(always)]
    fn from(variant: FGM_A) -> Self {
        variant as _
    }
}
impl FGM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FGM_A> {
        match self.bits {
            0 => Some(FGM_A::FGM_0),
            1 => Some(FGM_A::FGM_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FGM_0`"]
    #[inline(always)]
    pub fn is_fgm_0(&self) -> bool {
        *self == FGM_A::FGM_0
    }
    #[doc = "Checks if the value of the field is `FGM_1`"]
    #[inline(always)]
    pub fn is_fgm_1(&self) -> bool {
        *self == FGM_A::FGM_1
    }
}
#[doc = "Field `FGM` writer - These bits mask the ID Filter Table elements bits in a perfect alignment"]
pub type FGM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXFGMASK_SPEC, u32, FGM_A, 32, O>;
impl<'a, const O: u8> FGM_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care\""]
    #[inline(always)]
    pub fn fgm_0(self) -> &'a mut W {
        self.variant(FGM_A::FGM_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline(always)]
    pub fn fgm_1(self) -> &'a mut W {
        self.variant(FGM_A::FGM_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - These bits mask the ID Filter Table elements bits in a perfect alignment"]
    #[inline(always)]
    pub fn fgm(&self) -> FGM_R {
        FGM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits mask the ID Filter Table elements bits in a perfect alignment"]
    #[inline(always)]
    #[must_use]
    pub fn fgm(&mut self) -> FGM_W<0> {
        FGM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx FIFO Global Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfgmask](index.html) module"]
pub struct RXFGMASK_SPEC;
impl crate::RegisterSpec for RXFGMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfgmask::R](R) reader structure"]
impl crate::Readable for RXFGMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxfgmask::W](W) writer structure"]
impl crate::Writable for RXFGMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFGMASK to value 0xffff_ffff"]
impl crate::Resettable for RXFGMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
