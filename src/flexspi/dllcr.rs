#[doc = "Register `DLLCR%s` reader"]
pub struct R(crate::R<DLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLLCR%s` writer"]
pub struct W(crate::W<DLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLLCR_SPEC>;
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
impl From<crate::W<DLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLLEN` reader - DLL calibration enable."]
pub type DLLEN_R = crate::BitReader<bool>;
#[doc = "Field `DLLEN` writer - DLL calibration enable."]
pub type DLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCR_SPEC, bool, O>;
#[doc = "Field `DLLRESET` reader - Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
pub type DLLRESET_R = crate::BitReader<bool>;
#[doc = "Field `DLLRESET` writer - Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
pub type DLLRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCR_SPEC, bool, O>;
#[doc = "Field `SLVDLYTARGET` reader - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
pub type SLVDLYTARGET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLVDLYTARGET` writer - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
pub type SLVDLYTARGET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLLCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `OVRDEN` reader - Slave clock delay line delay cell number selection override enable."]
pub type OVRDEN_R = crate::BitReader<bool>;
#[doc = "Field `OVRDEN` writer - Slave clock delay line delay cell number selection override enable."]
pub type OVRDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCR_SPEC, bool, O>;
#[doc = "Field `OVRDVAL` reader - Slave clock delay line delay cell number selection override value."]
pub type OVRDVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVRDVAL` writer - Slave clock delay line delay cell number selection override value."]
pub type OVRDVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLLCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - DLL calibration enable."]
    #[inline(always)]
    pub fn dllen(&self) -> DLLEN_R {
        DLLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
    #[inline(always)]
    pub fn dllreset(&self) -> DLLRESET_R {
        DLLRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:6 - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
    #[inline(always)]
    pub fn slvdlytarget(&self) -> SLVDLYTARGET_R {
        SLVDLYTARGET_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Slave clock delay line delay cell number selection override enable."]
    #[inline(always)]
    pub fn ovrden(&self) -> OVRDEN_R {
        OVRDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:14 - Slave clock delay line delay cell number selection override value."]
    #[inline(always)]
    pub fn ovrdval(&self) -> OVRDVAL_R {
        OVRDVAL_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DLL calibration enable."]
    #[inline(always)]
    #[must_use]
    pub fn dllen(&mut self) -> DLLEN_W<0> {
        DLLEN_W::new(self)
    }
    #[doc = "Bit 1 - Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
    #[inline(always)]
    #[must_use]
    pub fn dllreset(&mut self) -> DLLRESET_W<1> {
        DLLRESET_W::new(self)
    }
    #[doc = "Bits 3:6 - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
    #[inline(always)]
    #[must_use]
    pub fn slvdlytarget(&mut self) -> SLVDLYTARGET_W<3> {
        SLVDLYTARGET_W::new(self)
    }
    #[doc = "Bit 8 - Slave clock delay line delay cell number selection override enable."]
    #[inline(always)]
    #[must_use]
    pub fn ovrden(&mut self) -> OVRDEN_W<8> {
        OVRDEN_W::new(self)
    }
    #[doc = "Bits 9:14 - Slave clock delay line delay cell number selection override value."]
    #[inline(always)]
    #[must_use]
    pub fn ovrdval(&mut self) -> OVRDVAL_W<9> {
        OVRDVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DLL Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dllcr](index.html) module"]
pub struct DLLCR_SPEC;
impl crate::RegisterSpec for DLLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dllcr::R](R) reader structure"]
impl crate::Readable for DLLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dllcr::W](W) writer structure"]
impl crate::Writable for DLLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLLCR%s to value 0x0100"]
impl crate::Resettable for DLLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
