#[doc = "Register `GPR3` reader"]
pub struct R(crate::R<GPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR3` writer"]
pub struct W(crate::W<GPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR3_SPEC>;
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
impl From<crate::W<GPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCP_KEY_SEL` reader - Select 128-bit DCP key from 256-bit key from SNVS Master Key"]
pub type DCP_KEY_SEL_R = crate::BitReader<DCP_KEY_SEL_A>;
#[doc = "Select 128-bit DCP key from 256-bit key from SNVS Master Key\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCP_KEY_SEL_A {
    #[doc = "0: Select \\[127:0\\]
from SNVS Master Key as DCP key"]
    DCP_KEY_SEL_0 = 0,
    #[doc = "1: Select \\[255:128\\]
from SNVS Master Key as DCP key"]
    DCP_KEY_SEL_1 = 1,
}
impl From<DCP_KEY_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DCP_KEY_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DCP_KEY_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCP_KEY_SEL_A {
        match self.bits {
            false => DCP_KEY_SEL_A::DCP_KEY_SEL_0,
            true => DCP_KEY_SEL_A::DCP_KEY_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCP_KEY_SEL_0`"]
    #[inline(always)]
    pub fn is_dcp_key_sel_0(&self) -> bool {
        *self == DCP_KEY_SEL_A::DCP_KEY_SEL_0
    }
    #[doc = "Checks if the value of the field is `DCP_KEY_SEL_1`"]
    #[inline(always)]
    pub fn is_dcp_key_sel_1(&self) -> bool {
        *self == DCP_KEY_SEL_A::DCP_KEY_SEL_1
    }
}
#[doc = "Field `DCP_KEY_SEL` writer - Select 128-bit DCP key from 256-bit key from SNVS Master Key"]
pub type DCP_KEY_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR3_SPEC, DCP_KEY_SEL_A, O>;
impl<'a, const O: u8> DCP_KEY_SEL_W<'a, O> {
    #[doc = "Select \\[127:0\\]
from SNVS Master Key as DCP key"]
    #[inline(always)]
    pub fn dcp_key_sel_0(self) -> &'a mut W {
        self.variant(DCP_KEY_SEL_A::DCP_KEY_SEL_0)
    }
    #[doc = "Select \\[255:128\\]
from SNVS Master Key as DCP key"]
    #[inline(always)]
    pub fn dcp_key_sel_1(self) -> &'a mut W {
        self.variant(DCP_KEY_SEL_A::DCP_KEY_SEL_1)
    }
}
impl R {
    #[doc = "Bit 4 - Select 128-bit DCP key from 256-bit key from SNVS Master Key"]
    #[inline(always)]
    pub fn dcp_key_sel(&self) -> DCP_KEY_SEL_R {
        DCP_KEY_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Select 128-bit DCP key from 256-bit key from SNVS Master Key"]
    #[inline(always)]
    #[must_use]
    pub fn dcp_key_sel(&mut self) -> DCP_KEY_SEL_W<4> {
        DCP_KEY_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR3 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr3](index.html) module"]
pub struct GPR3_SPEC;
impl crate::RegisterSpec for GPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr3::R](R) reader structure"]
impl crate::Readable for GPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr3::W](W) writer structure"]
impl crate::Writable for GPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR3 to value 0xfff0"]
impl crate::Resettable for GPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0xfff0;
}
