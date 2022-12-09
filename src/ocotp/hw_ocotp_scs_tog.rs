#[doc = "Register `HW_OCOTP_SCS_TOG` reader"]
pub struct R(crate::R<HW_OCOTP_SCS_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_OCOTP_SCS_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_OCOTP_SCS_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_OCOTP_SCS_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_OCOTP_SCS_TOG` writer"]
pub struct W(crate::W<HW_OCOTP_SCS_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_OCOTP_SCS_TOG_SPEC>;
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
impl From<crate::W<HW_OCOTP_SCS_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_OCOTP_SCS_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HAB_JDE` reader - HAB JTAG Debug Enable"]
pub type HAB_JDE_R = crate::BitReader<bool>;
#[doc = "Field `HAB_JDE` writer - HAB JTAG Debug Enable"]
pub type HAB_JDE_W<'a, const O: u8> = crate::BitWriter1T<'a, u32, HW_OCOTP_SCS_TOG_SPEC, bool, O>;
#[doc = "Field `SPARE` reader - Spare"]
pub type SPARE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPARE` writer - Spare"]
pub type SPARE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HW_OCOTP_SCS_TOG_SPEC, u32, u32, 30, O>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter1T<'a, u32, HW_OCOTP_SCS_TOG_SPEC, bool, O>;
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
#[doc = "Software Controllable Signals Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_ocotp_scs_tog](index.html) module"]
pub struct HW_OCOTP_SCS_TOG_SPEC;
impl crate::RegisterSpec for HW_OCOTP_SCS_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_ocotp_scs_tog::R](R) reader structure"]
impl crate::Readable for HW_OCOTP_SCS_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_ocotp_scs_tog::W](W) writer structure"]
impl crate::Writable for HW_OCOTP_SCS_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets HW_OCOTP_SCS_TOG to value 0"]
impl crate::Resettable for HW_OCOTP_SCS_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
