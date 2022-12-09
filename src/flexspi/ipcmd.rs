#[doc = "Register `IPCMD` reader"]
pub struct R(crate::R<IPCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCMD` writer"]
pub struct W(crate::W<IPCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCMD_SPEC>;
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
impl From<crate::W<IPCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRG` reader - Setting this bit will trigger an IP Command."]
pub type TRG_R = crate::BitReader<bool>;
#[doc = "Field `TRG` writer - Setting this bit will trigger an IP Command."]
pub type TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPCMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Setting this bit will trigger an IP Command."]
    #[inline(always)]
    pub fn trg(&self) -> TRG_R {
        TRG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit will trigger an IP Command."]
    #[inline(always)]
    #[must_use]
    pub fn trg(&mut self) -> TRG_W<0> {
        TRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcmd](index.html) module"]
pub struct IPCMD_SPEC;
impl crate::RegisterSpec for IPCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcmd::R](R) reader structure"]
impl crate::Readable for IPCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipcmd::W](W) writer structure"]
impl crate::Writable for IPCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPCMD to value 0"]
impl crate::Resettable for IPCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
