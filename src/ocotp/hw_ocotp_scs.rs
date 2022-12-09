#[doc = "Register `HW_OCOTP_SCS` reader"]
pub struct R(crate::R<HW_OCOTP_SCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_OCOTP_SCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_OCOTP_SCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_OCOTP_SCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_OCOTP_SCS` writer"]
pub struct W(crate::W<HW_OCOTP_SCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_OCOTP_SCS_SPEC>;
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
impl From<crate::W<HW_OCOTP_SCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_OCOTP_SCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HAB_JDE` reader - HAB JTAG Debug Enable"]
pub type HAB_JDE_R = crate::BitReader<HAB_JDE_A>;
#[doc = "HAB JTAG Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HAB_JDE_A {
    #[doc = "0: JTAG debugging is not enabled by the HAB (it may still be enabled by other mechanisms)."]
    JTAG_DBG_DIS = 0,
    #[doc = "1: JTAG debugging is enabled by the HAB (though this signal may be gated off)."]
    JTAG_DBG_EN = 1,
}
impl From<HAB_JDE_A> for bool {
    #[inline(always)]
    fn from(variant: HAB_JDE_A) -> Self {
        variant as u8 != 0
    }
}
impl HAB_JDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HAB_JDE_A {
        match self.bits {
            false => HAB_JDE_A::JTAG_DBG_DIS,
            true => HAB_JDE_A::JTAG_DBG_EN,
        }
    }
    #[doc = "Checks if the value of the field is `JTAG_DBG_DIS`"]
    #[inline(always)]
    pub fn is_jtag_dbg_dis(&self) -> bool {
        *self == HAB_JDE_A::JTAG_DBG_DIS
    }
    #[doc = "Checks if the value of the field is `JTAG_DBG_EN`"]
    #[inline(always)]
    pub fn is_jtag_dbg_en(&self) -> bool {
        *self == HAB_JDE_A::JTAG_DBG_EN
    }
}
#[doc = "Field `HAB_JDE` writer - HAB JTAG Debug Enable"]
pub type HAB_JDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HW_OCOTP_SCS_SPEC, HAB_JDE_A, O>;
impl<'a, const O: u8> HAB_JDE_W<'a, O> {
    #[doc = "JTAG debugging is not enabled by the HAB (it may still be enabled by other mechanisms)."]
    #[inline(always)]
    pub fn jtag_dbg_dis(self) -> &'a mut W {
        self.variant(HAB_JDE_A::JTAG_DBG_DIS)
    }
    #[doc = "JTAG debugging is enabled by the HAB (though this signal may be gated off)."]
    #[inline(always)]
    pub fn jtag_dbg_en(self) -> &'a mut W {
        self.variant(HAB_JDE_A::JTAG_DBG_EN)
    }
}
#[doc = "Field `SPARE` reader - Spare"]
pub type SPARE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPARE` writer - Spare"]
pub type SPARE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HW_OCOTP_SCS_SPEC, u32, u32, 30, O>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: Bits in this register are unlocked."]
    UNLOCKED = 0,
    #[doc = "1: Bits in this register are locked. When set, all of the bits in this register are locked and can not be changed through SW programming. After this bit is set, it can only be cleared by a POR."]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HW_OCOTP_SCS_SPEC, LOCK_A, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    #[doc = "Bits in this register are unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "Bits in this register are locked. When set, all of the bits in this register are locked and can not be changed through SW programming. After this bit is set, it can only be cleared by a POR."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bit 0 - HAB JTAG Debug Enable"]
    #[inline(always)]
    pub fn hab_jde(&self) -> HAB_JDE_R {
        HAB_JDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:30 - Spare"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new((self.bits >> 1) & 0x3fff_ffff)
    }
    #[doc = "Bit 31 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HAB JTAG Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hab_jde(&mut self) -> HAB_JDE_W<0> {
        HAB_JDE_W::new(self)
    }
    #[doc = "Bits 1:30 - Spare"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<1> {
        SPARE_W::new(self)
    }
    #[doc = "Bit 31 - Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Controllable Signals Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_ocotp_scs](index.html) module"]
pub struct HW_OCOTP_SCS_SPEC;
impl crate::RegisterSpec for HW_OCOTP_SCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_ocotp_scs::R](R) reader structure"]
impl crate::Readable for HW_OCOTP_SCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_ocotp_scs::W](W) writer structure"]
impl crate::Writable for HW_OCOTP_SCS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HW_OCOTP_SCS to value 0"]
impl crate::Resettable for HW_OCOTP_SCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
