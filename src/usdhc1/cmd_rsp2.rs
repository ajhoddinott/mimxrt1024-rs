#[doc = "Register `CMD_RSP2` reader"]
pub struct R(crate::R<CMD_RSP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_RSP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_RSP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_RSP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRSP2` reader - Command response 2"]
pub type CMDRSP2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command response 2"]
    #[inline(always)]
    pub fn cmdrsp2(&self) -> CMDRSP2_R {
        CMDRSP2_R::new(self.bits)
    }
}
#[doc = "Command Response2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_rsp2](index.html) module"]
pub struct CMD_RSP2_SPEC;
impl crate::RegisterSpec for CMD_RSP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_rsp2::R](R) reader structure"]
impl crate::Readable for CMD_RSP2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMD_RSP2 to value 0"]
impl crate::Resettable for CMD_RSP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
