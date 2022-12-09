#[doc = "Register `GPR21` reader"]
pub struct R(crate::R<GPR21_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR21_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR21_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR21_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR21` writer"]
pub struct W(crate::W<GPR21_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR21_SPEC>;
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
impl From<crate::W<GPR21_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR21_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_M7_APC_AC_R1_TOP` reader - lock M7_APC_AC_R1_TOP field for changes"]
pub type LOCK_M7_APC_AC_R1_TOP_R = crate::BitReader<LOCK_M7_APC_AC_R1_TOP_A>;
#[doc = "lock M7_APC_AC_R1_TOP field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_M7_APC_AC_R1_TOP_A {
    #[doc = "0: M7_APC_AC_R1_TOP is not locked"]
    LOCK_M7_APC_AC_R1_TOP_0 = 0,
    #[doc = "1: M7_APC_AC_R1_TOP is locked (read access only)"]
    LOCK_M7_APC_AC_R1_TOP_1 = 1,
}
impl From<LOCK_M7_APC_AC_R1_TOP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_M7_APC_AC_R1_TOP_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_M7_APC_AC_R1_TOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_M7_APC_AC_R1_TOP_A {
        match self.bits {
            false => LOCK_M7_APC_AC_R1_TOP_A::LOCK_M7_APC_AC_R1_TOP_0,
            true => LOCK_M7_APC_AC_R1_TOP_A::LOCK_M7_APC_AC_R1_TOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R1_TOP_0`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r1_top_0(&self) -> bool {
        *self == LOCK_M7_APC_AC_R1_TOP_A::LOCK_M7_APC_AC_R1_TOP_0
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R1_TOP_1`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r1_top_1(&self) -> bool {
        *self == LOCK_M7_APC_AC_R1_TOP_A::LOCK_M7_APC_AC_R1_TOP_1
    }
}
#[doc = "Field `LOCK_M7_APC_AC_R1_TOP` writer - lock M7_APC_AC_R1_TOP field for changes"]
pub type LOCK_M7_APC_AC_R1_TOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR21_SPEC, LOCK_M7_APC_AC_R1_TOP_A, O>;
impl<'a, const O: u8> LOCK_M7_APC_AC_R1_TOP_W<'a, O> {
    #[doc = "M7_APC_AC_R1_TOP is not locked"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r1_top_0(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R1_TOP_A::LOCK_M7_APC_AC_R1_TOP_0)
    }
    #[doc = "M7_APC_AC_R1_TOP is locked (read access only)"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r1_top_1(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R1_TOP_A::LOCK_M7_APC_AC_R1_TOP_1)
    }
}
#[doc = "Field `M7_APC_AC_R1_TOP` reader - Access Permission Controller (APC) start address of memory region-1"]
pub type M7_APC_AC_R1_TOP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `M7_APC_AC_R1_TOP` writer - Access Permission Controller (APC) start address of memory region-1"]
pub type M7_APC_AC_R1_TOP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR21_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - lock M7_APC_AC_R1_TOP field for changes"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r1_top(&self) -> LOCK_M7_APC_AC_R1_TOP_R {
        LOCK_M7_APC_AC_R1_TOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:31 - Access Permission Controller (APC) start address of memory region-1"]
    #[inline(always)]
    pub fn m7_apc_ac_r1_top(&self) -> M7_APC_AC_R1_TOP_R {
        M7_APC_AC_R1_TOP_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - lock M7_APC_AC_R1_TOP field for changes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_m7_apc_ac_r1_top(&mut self) -> LOCK_M7_APC_AC_R1_TOP_W<0> {
        LOCK_M7_APC_AC_R1_TOP_W::new(self)
    }
    #[doc = "Bits 3:31 - Access Permission Controller (APC) start address of memory region-1"]
    #[inline(always)]
    #[must_use]
    pub fn m7_apc_ac_r1_top(&mut self) -> M7_APC_AC_R1_TOP_W<3> {
        M7_APC_AC_R1_TOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR21 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr21](index.html) module"]
pub struct GPR21_SPEC;
impl crate::RegisterSpec for GPR21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr21::R](R) reader structure"]
impl crate::Readable for GPR21_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr21::W](W) writer structure"]
impl crate::Writable for GPR21_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR21 to value 0"]
impl crate::Resettable for GPR21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
