#[doc = "Register `PWD_TOG` reader"]
pub struct R(crate::R<PWD_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWD_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWD_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWD_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWD_TOG` writer"]
pub struct W(crate::W<PWD_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWD_TOG_SPEC>;
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
impl From<crate::W<PWD_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWD_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSVD0` reader - Reserved."]
pub type RSVD0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXPWDFS` reader - 0 = Normal operation"]
pub type TXPWDFS_R = crate::BitReader<bool>;
#[doc = "Field `TXPWDFS` writer - 0 = Normal operation"]
pub type TXPWDFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, bool, O>;
#[doc = "Field `TXPWDIBIAS` reader - 0 = Normal operation"]
pub type TXPWDIBIAS_R = crate::BitReader<bool>;
#[doc = "Field `TXPWDIBIAS` writer - 0 = Normal operation"]
pub type TXPWDIBIAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, bool, O>;
#[doc = "Field `TXPWDV2I` reader - 0 = Normal operation"]
pub type TXPWDV2I_R = crate::BitReader<bool>;
#[doc = "Field `TXPWDV2I` writer - 0 = Normal operation"]
pub type TXPWDV2I_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, bool, O>;
#[doc = "Field `RSVD1` reader - Reserved."]
pub type RSVD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXPWDENV` reader - 0 = Normal operation"]
pub type RXPWDENV_R = crate::BitReader<bool>;
#[doc = "Field `RXPWDENV` writer - 0 = Normal operation"]
pub type RXPWDENV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, bool, O>;
#[doc = "Field `RXPWD1PT1` reader - 0 = Normal operation"]
pub type RXPWD1PT1_R = crate::BitReader<bool>;
#[doc = "Field `RXPWD1PT1` writer - 0 = Normal operation"]
pub type RXPWD1PT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, bool, O>;
#[doc = "Field `RXPWDDIFF` reader - 0 = Normal operation"]
pub type RXPWDDIFF_R = crate::BitReader<bool>;
#[doc = "Field `RXPWDDIFF` writer - 0 = Normal operation"]
pub type RXPWDDIFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, bool, O>;
#[doc = "Field `RXPWDRX` reader - 0 = Normal operation"]
pub type RXPWDRX_R = crate::BitReader<bool>;
#[doc = "Field `RXPWDRX` writer - 0 = Normal operation"]
pub type RXPWDRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, bool, O>;
#[doc = "Field `RSVD2` reader - Reserved."]
pub type RSVD2_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - 0 = Normal operation"]
    #[inline(always)]
    pub fn txpwdfs(&self) -> TXPWDFS_R {
        TXPWDFS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0 = Normal operation"]
    #[inline(always)]
    pub fn txpwdibias(&self) -> TXPWDIBIAS_R {
        TXPWDIBIAS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 0 = Normal operation"]
    #[inline(always)]
    pub fn txpwdv2i(&self) -> TXPWDV2I_R {
        TXPWDV2I_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxpwdenv(&self) -> RXPWDENV_R {
        RXPWDENV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxpwd1pt1(&self) -> RXPWD1PT1_R {
        RXPWD1PT1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxpwddiff(&self) -> RXPWDDIFF_R {
        RXPWDDIFF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxpwdrx(&self) -> RXPWDRX_R {
        RXPWDRX_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - Reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 10 - 0 = Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn txpwdfs(&mut self) -> TXPWDFS_W<10> {
        TXPWDFS_W::new(self)
    }
    #[doc = "Bit 11 - 0 = Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn txpwdibias(&mut self) -> TXPWDIBIAS_W<11> {
        TXPWDIBIAS_W::new(self)
    }
    #[doc = "Bit 12 - 0 = Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn txpwdv2i(&mut self) -> TXPWDV2I_W<12> {
        TXPWDV2I_W::new(self)
    }
    #[doc = "Bit 17 - 0 = Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn rxpwdenv(&mut self) -> RXPWDENV_W<17> {
        RXPWDENV_W::new(self)
    }
    #[doc = "Bit 18 - 0 = Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn rxpwd1pt1(&mut self) -> RXPWD1PT1_W<18> {
        RXPWD1PT1_W::new(self)
    }
    #[doc = "Bit 19 - 0 = Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn rxpwddiff(&mut self) -> RXPWDDIFF_W<19> {
        RXPWDDIFF_W::new(self)
    }
    #[doc = "Bit 20 - 0 = Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn rxpwdrx(&mut self) -> RXPWDRX_W<20> {
        RXPWDRX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Power-Down Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd_tog](index.html) module"]
pub struct PWD_TOG_SPEC;
impl crate::RegisterSpec for PWD_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwd_tog::R](R) reader structure"]
impl crate::Readable for PWD_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwd_tog::W](W) writer structure"]
impl crate::Writable for PWD_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWD_TOG to value 0x001e_1c00"]
impl crate::Resettable for PWD_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0x001e_1c00;
}
