#[doc = "Register `CMD_RSP0` reader"]
pub struct R(crate::R<CMD_RSP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_RSP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_RSP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_RSP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRSP0` reader - Command response 0"]
pub type CMDRSP0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command response 0"]
    #[inline(always)]
    pub fn cmdrsp0(&self) -> CMDRSP0_R {
        CMDRSP0_R::new(self.bits)
    }
}
#[doc = "Command Response0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_rsp0](index.html) module"]
pub struct CMD_RSP0_SPEC;
impl crate::RegisterSpec for CMD_RSP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_rsp0::R](R) reader structure"]
impl crate::Readable for CMD_RSP0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMD_RSP0 to value 0"]
impl crate::Resettable for CMD_RSP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
