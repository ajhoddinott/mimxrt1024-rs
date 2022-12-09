#[doc = "Register `IPCR0` reader"]
pub struct R(crate::R<IPCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCR0` writer"]
pub struct W(crate::W<IPCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCR0_SPEC>;
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
impl From<crate::W<IPCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFAR` reader - Serial Flash Address for IP command."]
pub type SFAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SFAR` writer - Serial Flash Address for IP command."]
pub type SFAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPCR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Serial Flash Address for IP command."]
    #[inline(always)]
    pub fn sfar(&self) -> SFAR_R {
        SFAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Serial Flash Address for IP command."]
    #[inline(always)]
    #[must_use]
    pub fn sfar(&mut self) -> SFAR_W<0> {
        SFAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcr0](index.html) module"]
pub struct IPCR0_SPEC;
impl crate::RegisterSpec for IPCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcr0::R](R) reader structure"]
impl crate::Readable for IPCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipcr0::W](W) writer structure"]
impl crate::Writable for IPCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPCR0 to value 0"]
impl crate::Resettable for IPCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
