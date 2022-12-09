#[doc = "Register `CSCDR3` reader"]
pub struct R(crate::R<CSCDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CCM Serial Clock Divider Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cscdr3](index.html) module"]
pub struct CSCDR3_SPEC;
impl crate::RegisterSpec for CSCDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cscdr3::R](R) reader structure"]
impl crate::Readable for CSCDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSCDR3 to value 0x0003_0841"]
impl crate::Resettable for CSCDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0841;
}
