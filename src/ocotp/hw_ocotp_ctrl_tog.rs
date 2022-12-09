#[doc = "Register `HW_OCOTP_CTRL_TOG` reader"]
pub struct R(crate::R<HW_OCOTP_CTRL_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_OCOTP_CTRL_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_OCOTP_CTRL_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_OCOTP_CTRL_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_OCOTP_CTRL_TOG` writer"]
pub struct W(crate::W<HW_OCOTP_CTRL_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_OCOTP_CTRL_TOG_SPEC>;
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
impl From<crate::W<HW_OCOTP_CTRL_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_OCOTP_CTRL_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - OTP write and read access address register"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - OTP write and read access address register"]
pub type ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HW_OCOTP_CTRL_TOG_SPEC, u8, u8, 6, O>;
#[doc = "Field `BUSY` reader - OTP controller status bit"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` reader - Locked Region Access Error"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Locked Region Access Error"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter1T<'a, u32, HW_OCOTP_CTRL_TOG_SPEC, bool, O>;
#[doc = "Field `RELOAD_SHADOWS` reader - Reload Shadow Registers"]
pub type RELOAD_SHADOWS_R = crate::BitReader<bool>;
#[doc = "Field `RELOAD_SHADOWS` writer - Reload Shadow Registers"]
pub type RELOAD_SHADOWS_W<'a, const O: u8> =
    crate::BitWriter1T<'a, u32, HW_OCOTP_CTRL_TOG_SPEC, bool, O>;
#[doc = "Field `WR_UNLOCK` reader - Write Unlock"]
pub type WR_UNLOCK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WR_UNLOCK` writer - Write Unlock"]
pub type WR_UNLOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HW_OCOTP_CTRL_TOG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:5 - OTP write and read access address register"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - OTP controller status bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Locked Region Access Error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reload Shadow Registers"]
    #[inline(always)]
    pub fn reload_shadows(&self) -> RELOAD_SHADOWS_R {
        RELOAD_SHADOWS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Write Unlock"]
    #[inline(always)]
    pub fn wr_unlock(&self) -> WR_UNLOCK_R {
        WR_UNLOCK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - OTP write and read access address register"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 9 - Locked Region Access Error"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<9> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 10 - Reload Shadow Registers"]
    #[inline(always)]
    #[must_use]
    pub fn reload_shadows(&mut self) -> RELOAD_SHADOWS_W<10> {
        RELOAD_SHADOWS_W::new(self)
    }
    #[doc = "Bits 16:31 - Write Unlock"]
    #[inline(always)]
    #[must_use]
    pub fn wr_unlock(&mut self) -> WR_UNLOCK_W<16> {
        WR_UNLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTP Controller Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_ocotp_ctrl_tog](index.html) module"]
pub struct HW_OCOTP_CTRL_TOG_SPEC;
impl crate::RegisterSpec for HW_OCOTP_CTRL_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_ocotp_ctrl_tog::R](R) reader structure"]
impl crate::Readable for HW_OCOTP_CTRL_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_ocotp_ctrl_tog::W](W) writer structure"]
impl crate::Writable for HW_OCOTP_CTRL_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_063f;
}
#[doc = "`reset()` method sets HW_OCOTP_CTRL_TOG to value 0"]
impl crate::Resettable for HW_OCOTP_CTRL_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
