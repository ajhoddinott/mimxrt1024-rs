#[doc = "Register `GPR18` reader"]
pub struct R(crate::R<GPR18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR18` writer"]
pub struct W(crate::W<GPR18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR18_SPEC>;
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
impl From<crate::W<GPR18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_M7_APC_AC_R0_BOT` reader - lock M7_APC_AC_R0_BOT field for changes"]
pub type LOCK_M7_APC_AC_R0_BOT_R = crate::BitReader<LOCK_M7_APC_AC_R0_BOT_A>;
#[doc = "lock M7_APC_AC_R0_BOT field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_M7_APC_AC_R0_BOT_A {
    #[doc = "0: M7_APC_AC_R0_BOT is not locked"]
    LOCK_M7_APC_AC_R0_BOT_0 = 0,
    #[doc = "1: M7_APC_AC_R0_BOT is locked (read access only)"]
    LOCK_M7_APC_AC_R0_BOT_1 = 1,
}
impl From<LOCK_M7_APC_AC_R0_BOT_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_M7_APC_AC_R0_BOT_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_M7_APC_AC_R0_BOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_M7_APC_AC_R0_BOT_A {
        match self.bits {
            false => LOCK_M7_APC_AC_R0_BOT_A::LOCK_M7_APC_AC_R0_BOT_0,
            true => LOCK_M7_APC_AC_R0_BOT_A::LOCK_M7_APC_AC_R0_BOT_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R0_BOT_0`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r0_bot_0(&self) -> bool {
        *self == LOCK_M7_APC_AC_R0_BOT_A::LOCK_M7_APC_AC_R0_BOT_0
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R0_BOT_1`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r0_bot_1(&self) -> bool {
        *self == LOCK_M7_APC_AC_R0_BOT_A::LOCK_M7_APC_AC_R0_BOT_1
    }
}
#[doc = "Field `LOCK_M7_APC_AC_R0_BOT` writer - lock M7_APC_AC_R0_BOT field for changes"]
pub type LOCK_M7_APC_AC_R0_BOT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR18_SPEC, LOCK_M7_APC_AC_R0_BOT_A, O>;
impl<'a, const O: u8> LOCK_M7_APC_AC_R0_BOT_W<'a, O> {
    #[doc = "M7_APC_AC_R0_BOT is not locked"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r0_bot_0(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R0_BOT_A::LOCK_M7_APC_AC_R0_BOT_0)
    }
    #[doc = "M7_APC_AC_R0_BOT is locked (read access only)"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r0_bot_1(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R0_BOT_A::LOCK_M7_APC_AC_R0_BOT_1)
    }
}
#[doc = "Field `M7_APC_AC_R0_BOT` reader - Access Permission Controller (APC) end address of memory region-0"]
pub type M7_APC_AC_R0_BOT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `M7_APC_AC_R0_BOT` writer - Access Permission Controller (APC) end address of memory region-0"]
pub type M7_APC_AC_R0_BOT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR18_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - lock M7_APC_AC_R0_BOT field for changes"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r0_bot(&self) -> LOCK_M7_APC_AC_R0_BOT_R {
        LOCK_M7_APC_AC_R0_BOT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:31 - Access Permission Controller (APC) end address of memory region-0"]
    #[inline(always)]
    pub fn m7_apc_ac_r0_bot(&self) -> M7_APC_AC_R0_BOT_R {
        M7_APC_AC_R0_BOT_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - lock M7_APC_AC_R0_BOT field for changes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_m7_apc_ac_r0_bot(&mut self) -> LOCK_M7_APC_AC_R0_BOT_W<0> {
        LOCK_M7_APC_AC_R0_BOT_W::new(self)
    }
    #[doc = "Bits 3:31 - Access Permission Controller (APC) end address of memory region-0"]
    #[inline(always)]
    #[must_use]
    pub fn m7_apc_ac_r0_bot(&mut self) -> M7_APC_AC_R0_BOT_W<3> {
        M7_APC_AC_R0_BOT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR18 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr18](index.html) module"]
pub struct GPR18_SPEC;
impl crate::RegisterSpec for GPR18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr18::R](R) reader structure"]
impl crate::Readable for GPR18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr18::W](W) writer structure"]
impl crate::Writable for GPR18_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR18 to value 0"]
impl crate::Resettable for GPR18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
