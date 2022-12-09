#[doc = "Register `PFD_528` reader"]
pub struct R(crate::R<PFD_528_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFD_528_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFD_528_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFD_528_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFD_528` writer"]
pub struct W(crate::W<PFD_528_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFD_528_SPEC>;
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
impl From<crate::W<PFD_528_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFD_528_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFD0_FRAC` reader - This field controls the fractional divide value"]
pub type PFD0_FRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFD0_FRAC` writer - This field controls the fractional divide value"]
pub type PFD0_FRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFD_528_SPEC, u8, u8, 6, O>;
#[doc = "Field `PFD0_STABLE` reader - This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
pub type PFD0_STABLE_R = crate::BitReader<bool>;
#[doc = "Field `PFD0_CLKGATE` reader - If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
pub type PFD0_CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `PFD0_CLKGATE` writer - If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
pub type PFD0_CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFD_528_SPEC, bool, O>;
#[doc = "Field `PFD1_FRAC` reader - This field controls the fractional divide value"]
pub type PFD1_FRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFD1_FRAC` writer - This field controls the fractional divide value"]
pub type PFD1_FRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFD_528_SPEC, u8, u8, 6, O>;
#[doc = "Field `PFD1_STABLE` reader - This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
pub type PFD1_STABLE_R = crate::BitReader<bool>;
#[doc = "Field `PFD1_CLKGATE` reader - IO Clock Gate"]
pub type PFD1_CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `PFD1_CLKGATE` writer - IO Clock Gate"]
pub type PFD1_CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFD_528_SPEC, bool, O>;
#[doc = "Field `PFD2_FRAC` reader - This field controls the fractional divide value"]
pub type PFD2_FRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFD2_FRAC` writer - This field controls the fractional divide value"]
pub type PFD2_FRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFD_528_SPEC, u8, u8, 6, O>;
#[doc = "Field `PFD2_STABLE` reader - This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
pub type PFD2_STABLE_R = crate::BitReader<bool>;
#[doc = "Field `PFD2_CLKGATE` reader - IO Clock Gate"]
pub type PFD2_CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `PFD2_CLKGATE` writer - IO Clock Gate"]
pub type PFD2_CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFD_528_SPEC, bool, O>;
#[doc = "Field `PFD3_FRAC` reader - This field controls the fractional divide value"]
pub type PFD3_FRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFD3_FRAC` writer - This field controls the fractional divide value"]
pub type PFD3_FRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFD_528_SPEC, u8, u8, 6, O>;
#[doc = "Field `PFD3_STABLE` reader - This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
pub type PFD3_STABLE_R = crate::BitReader<bool>;
#[doc = "Field `PFD3_CLKGATE` reader - IO Clock Gate"]
pub type PFD3_CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `PFD3_CLKGATE` writer - IO Clock Gate"]
pub type PFD3_CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFD_528_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - This field controls the fractional divide value"]
    #[inline(always)]
    pub fn pfd0_frac(&self) -> PFD0_FRAC_R {
        PFD0_FRAC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub fn pfd0_stable(&self) -> PFD0_STABLE_R {
        PFD0_STABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[inline(always)]
    pub fn pfd0_clkgate(&self) -> PFD0_CLKGATE_R {
        PFD0_CLKGATE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - This field controls the fractional divide value"]
    #[inline(always)]
    pub fn pfd1_frac(&self) -> PFD1_FRAC_R {
        PFD1_FRAC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub fn pfd1_stable(&self) -> PFD1_STABLE_R {
        PFD1_STABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IO Clock Gate"]
    #[inline(always)]
    pub fn pfd1_clkgate(&self) -> PFD1_CLKGATE_R {
        PFD1_CLKGATE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - This field controls the fractional divide value"]
    #[inline(always)]
    pub fn pfd2_frac(&self) -> PFD2_FRAC_R {
        PFD2_FRAC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub fn pfd2_stable(&self) -> PFD2_STABLE_R {
        PFD2_STABLE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IO Clock Gate"]
    #[inline(always)]
    pub fn pfd2_clkgate(&self) -> PFD2_CLKGATE_R {
        PFD2_CLKGATE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - This field controls the fractional divide value"]
    #[inline(always)]
    pub fn pfd3_frac(&self) -> PFD3_FRAC_R {
        PFD3_FRAC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub fn pfd3_stable(&self) -> PFD3_STABLE_R {
        PFD3_STABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IO Clock Gate"]
    #[inline(always)]
    pub fn pfd3_clkgate(&self) -> PFD3_CLKGATE_R {
        PFD3_CLKGATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - This field controls the fractional divide value"]
    #[inline(always)]
    #[must_use]
    pub fn pfd0_frac(&mut self) -> PFD0_FRAC_W<0> {
        PFD0_FRAC_W::new(self)
    }
    #[doc = "Bit 7 - If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[inline(always)]
    #[must_use]
    pub fn pfd0_clkgate(&mut self) -> PFD0_CLKGATE_W<7> {
        PFD0_CLKGATE_W::new(self)
    }
    #[doc = "Bits 8:13 - This field controls the fractional divide value"]
    #[inline(always)]
    #[must_use]
    pub fn pfd1_frac(&mut self) -> PFD1_FRAC_W<8> {
        PFD1_FRAC_W::new(self)
    }
    #[doc = "Bit 15 - IO Clock Gate"]
    #[inline(always)]
    #[must_use]
    pub fn pfd1_clkgate(&mut self) -> PFD1_CLKGATE_W<15> {
        PFD1_CLKGATE_W::new(self)
    }
    #[doc = "Bits 16:21 - This field controls the fractional divide value"]
    #[inline(always)]
    #[must_use]
    pub fn pfd2_frac(&mut self) -> PFD2_FRAC_W<16> {
        PFD2_FRAC_W::new(self)
    }
    #[doc = "Bit 23 - IO Clock Gate"]
    #[inline(always)]
    #[must_use]
    pub fn pfd2_clkgate(&mut self) -> PFD2_CLKGATE_W<23> {
        PFD2_CLKGATE_W::new(self)
    }
    #[doc = "Bits 24:29 - This field controls the fractional divide value"]
    #[inline(always)]
    #[must_use]
    pub fn pfd3_frac(&mut self) -> PFD3_FRAC_W<24> {
        PFD3_FRAC_W::new(self)
    }
    #[doc = "Bit 31 - IO Clock Gate"]
    #[inline(always)]
    #[must_use]
    pub fn pfd3_clkgate(&mut self) -> PFD3_CLKGATE_W<31> {
        PFD3_CLKGATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfd_528](index.html) module"]
pub struct PFD_528_SPEC;
impl crate::RegisterSpec for PFD_528_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfd_528::R](R) reader structure"]
impl crate::Readable for PFD_528_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfd_528::W](W) writer structure"]
impl crate::Writable for PFD_528_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFD_528 to value 0x1018_101b"]
impl crate::Resettable for PFD_528_SPEC {
    const RESET_VALUE: Self::Ux = 0x1018_101b;
}
