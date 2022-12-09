#[doc = "Register `HW_OCOTP_READ_CTRL` reader"]
pub struct R(crate::R<HW_OCOTP_READ_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_OCOTP_READ_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_OCOTP_READ_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_OCOTP_READ_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_OCOTP_READ_CTRL` writer"]
pub struct W(crate::W<HW_OCOTP_READ_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_OCOTP_READ_CTRL_SPEC>;
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
impl From<crate::W<HW_OCOTP_READ_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_OCOTP_READ_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_FUSE` reader - Read Fuse"]
pub type READ_FUSE_R = crate::BitReader<bool>;
#[doc = "Field `READ_FUSE` writer - Read Fuse"]
pub type READ_FUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HW_OCOTP_READ_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Read Fuse"]
    #[inline(always)]
    pub fn read_fuse(&self) -> READ_FUSE_R {
        READ_FUSE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read Fuse"]
    #[inline(always)]
    #[must_use]
    pub fn read_fuse(&mut self) -> READ_FUSE_W<0> {
        READ_FUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTP Controller Write Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_ocotp_read_ctrl](index.html) module"]
pub struct HW_OCOTP_READ_CTRL_SPEC;
impl crate::RegisterSpec for HW_OCOTP_READ_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_ocotp_read_ctrl::R](R) reader structure"]
impl crate::Readable for HW_OCOTP_READ_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_ocotp_read_ctrl::W](W) writer structure"]
impl crate::Writable for HW_OCOTP_READ_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HW_OCOTP_READ_CTRL to value 0"]
impl crate::Resettable for HW_OCOTP_READ_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
