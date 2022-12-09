#[doc = "Register `CMD_RSP3` reader"]
pub struct R(crate::R<CMD_RSP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_RSP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_RSP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_RSP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRSP3` reader - Command response 3"]
pub type CMDRSP3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command response 3"]
    #[inline(always)]
    pub fn cmdrsp3(&self) -> CMDRSP3_R {
        CMDRSP3_R::new(self.bits)
    }
}
#[doc = "Command Response3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_rsp3](index.html) module"]
pub struct CMD_RSP3_SPEC;
impl crate::RegisterSpec for CMD_RSP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_rsp3::R](R) reader structure"]
impl crate::Readable for CMD_RSP3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMD_RSP3 to value 0"]
impl crate::Resettable for CMD_RSP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
