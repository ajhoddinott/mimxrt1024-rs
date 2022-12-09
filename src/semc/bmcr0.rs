#[doc = "Register `BMCR0` reader"]
pub struct R(crate::R<BMCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMCR0` writer"]
pub struct W(crate::W<BMCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMCR0_SPEC>;
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
impl From<crate::W<BMCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WQOS` reader - Weight of QOS"]
pub type WQOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WQOS` writer - Weight of QOS"]
pub type WQOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `WAGE` reader - Weight of AGE"]
pub type WAGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAGE` writer - Weight of AGE"]
pub type WAGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `WSH` reader - Weight of Slave Hit without read/write switch"]
pub type WSH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WSH` writer - Weight of Slave Hit without read/write switch"]
pub type WSH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `WRWS` reader - Weight of slave hit with Read/Write Switch"]
pub type WRWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRWS` writer - Weight of slave hit with Read/Write Switch"]
pub type WRWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - Weight of QOS"]
    #[inline(always)]
    pub fn wqos(&self) -> WQOS_R {
        WQOS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight of AGE"]
    #[inline(always)]
    pub fn wage(&self) -> WAGE_R {
        WAGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Weight of Slave Hit without read/write switch"]
    #[inline(always)]
    pub fn wsh(&self) -> WSH_R {
        WSH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Weight of slave hit with Read/Write Switch"]
    #[inline(always)]
    pub fn wrws(&self) -> WRWS_R {
        WRWS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight of QOS"]
    #[inline(always)]
    #[must_use]
    pub fn wqos(&mut self) -> WQOS_W<0> {
        WQOS_W::new(self)
    }
    #[doc = "Bits 4:7 - Weight of AGE"]
    #[inline(always)]
    #[must_use]
    pub fn wage(&mut self) -> WAGE_W<4> {
        WAGE_W::new(self)
    }
    #[doc = "Bits 8:15 - Weight of Slave Hit without read/write switch"]
    #[inline(always)]
    #[must_use]
    pub fn wsh(&mut self) -> WSH_W<8> {
        WSH_W::new(self)
    }
    #[doc = "Bits 16:23 - Weight of slave hit with Read/Write Switch"]
    #[inline(always)]
    #[must_use]
    pub fn wrws(&mut self) -> WRWS_W<16> {
        WRWS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus (AXI) Master Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmcr0](index.html) module"]
pub struct BMCR0_SPEC;
impl crate::RegisterSpec for BMCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmcr0::R](R) reader structure"]
impl crate::Readable for BMCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmcr0::W](W) writer structure"]
impl crate::Writable for BMCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMCR0 to value 0"]
impl crate::Resettable for BMCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
