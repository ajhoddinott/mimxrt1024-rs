#[doc = "Register `WMCR` reader"]
pub struct R(crate::R<WMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WMCR` writer"]
pub struct W(crate::W<WMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WMCR_SPEC>;
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
impl From<crate::W<WMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDE` reader - PDE"]
pub type PDE_R = crate::BitReader<PDE_A>;
#[doc = "PDE\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDE_A {
    #[doc = "0: Power Down Counter of WDOG is disabled."]
    PDE_0 = 0,
    #[doc = "1: Power Down Counter of WDOG is enabled (Default)."]
    PDE_1 = 1,
}
impl From<PDE_A> for bool {
    #[inline(always)]
    fn from(variant: PDE_A) -> Self {
        variant as u8 != 0
    }
}
impl PDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDE_A {
        match self.bits {
            false => PDE_A::PDE_0,
            true => PDE_A::PDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDE_0`"]
    #[inline(always)]
    pub fn is_pde_0(&self) -> bool {
        *self == PDE_A::PDE_0
    }
    #[doc = "Checks if the value of the field is `PDE_1`"]
    #[inline(always)]
    pub fn is_pde_1(&self) -> bool {
        *self == PDE_A::PDE_1
    }
}
#[doc = "Field `PDE` writer - PDE"]
pub type PDE_W<'a, const O: u8> = crate::BitWriter<'a, u16, WMCR_SPEC, PDE_A, O>;
impl<'a, const O: u8> PDE_W<'a, O> {
    #[doc = "Power Down Counter of WDOG is disabled."]
    #[inline(always)]
    pub fn pde_0(self) -> &'a mut W {
        self.variant(PDE_A::PDE_0)
    }
    #[doc = "Power Down Counter of WDOG is enabled (Default)."]
    #[inline(always)]
    pub fn pde_1(self) -> &'a mut W {
        self.variant(PDE_A::PDE_1)
    }
}
impl R {
    #[doc = "Bit 0 - PDE"]
    #[inline(always)]
    pub fn pde(&self) -> PDE_R {
        PDE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDE"]
    #[inline(always)]
    #[must_use]
    pub fn pde(&mut self) -> PDE_W<0> {
        PDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmcr](index.html) module"]
pub struct WMCR_SPEC;
impl crate::RegisterSpec for WMCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wmcr::R](R) reader structure"]
impl crate::Readable for WMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wmcr::W](W) writer structure"]
impl crate::Writable for WMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WMCR to value 0x01"]
impl crate::Resettable for WMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
