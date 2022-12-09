#[doc = "Register `IPRXDAT` reader"]
pub struct R(crate::R<IPRXDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPRXDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPRXDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPRXDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DAT` reader - Data returned by device for an IP read command."]
pub type DAT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data returned by device for an IP read command."]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new(self.bits)
    }
}
#[doc = "RX DATA Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprxdat](index.html) module"]
pub struct IPRXDAT_SPEC;
impl crate::RegisterSpec for IPRXDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iprxdat::R](R) reader structure"]
impl crate::Readable for IPRXDAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPRXDAT to value 0"]
impl crate::Resettable for IPRXDAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
