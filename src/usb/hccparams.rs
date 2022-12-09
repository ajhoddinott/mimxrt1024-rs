#[doc = "Register `HCCPARAMS` reader"]
pub struct R(crate::R<HCCPARAMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCPARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCPARAMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCPARAMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADC` reader - 64-bit Addressing Capability This bit is set '0b' in all controller core, no 64-bit addressing capability is supported"]
pub type ADC_R = crate::BitReader<bool>;
#[doc = "Field `PFL` reader - Programmable Frame List Flag If this bit is set to zero, then the system software must use a frame list length of 1024 elements with this host controller"]
pub type PFL_R = crate::BitReader<bool>;
#[doc = "Field `ASP` reader - Asynchronous Schedule Park Capability If this bit is set to a one, then the host controller supports the park feature for high-speed queue heads in the Asynchronous Schedule"]
pub type ASP_R = crate::BitReader<bool>;
#[doc = "Field `IST` reader - Isochronous Scheduling Threshold"]
pub type IST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EECP` reader - EHCI Extended Capabilities Pointer"]
pub type EECP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - 64-bit Addressing Capability This bit is set '0b' in all controller core, no 64-bit addressing capability is supported"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Programmable Frame List Flag If this bit is set to zero, then the system software must use a frame list length of 1024 elements with this host controller"]
    #[inline(always)]
    pub fn pfl(&self) -> PFL_R {
        PFL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Asynchronous Schedule Park Capability If this bit is set to a one, then the host controller supports the park feature for high-speed queue heads in the Asynchronous Schedule"]
    #[inline(always)]
    pub fn asp(&self) -> ASP_R {
        ASP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Isochronous Scheduling Threshold"]
    #[inline(always)]
    pub fn ist(&self) -> IST_R {
        IST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - EHCI Extended Capabilities Pointer"]
    #[inline(always)]
    pub fn eecp(&self) -> EECP_R {
        EECP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Host Controller Capability Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccparams](index.html) module"]
pub struct HCCPARAMS_SPEC;
impl crate::RegisterSpec for HCCPARAMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccparams::R](R) reader structure"]
impl crate::Readable for HCCPARAMS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCCPARAMS to value 0x06"]
impl crate::Resettable for HCCPARAMS_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
