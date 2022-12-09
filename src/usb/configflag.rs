#[doc = "Register `CONFIGFLAG` reader"]
pub struct R(crate::R<CONFIGFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIGFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIGFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIGFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CF` reader - Configure Flag Host software sets this bit as the last action in its process of configuring the Host Controller"]
pub type CF_R = crate::BitReader<CF_A>;
#[doc = "Configure Flag Host software sets this bit as the last action in its process of configuring the Host Controller\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF_A {
    #[doc = "0: Port routing control logic default-routes each port to an implementation dependent classic host controller."]
    CF_0 = 0,
    #[doc = "1: Port routing control logic default-routes all ports to this host controller."]
    CF_1 = 1,
}
impl From<CF_A> for bool {
    #[inline(always)]
    fn from(variant: CF_A) -> Self {
        variant as u8 != 0
    }
}
impl CF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF_A {
        match self.bits {
            false => CF_A::CF_0,
            true => CF_A::CF_1,
        }
    }
    #[doc = "Checks if the value of the field is `CF_0`"]
    #[inline(always)]
    pub fn is_cf_0(&self) -> bool {
        *self == CF_A::CF_0
    }
    #[doc = "Checks if the value of the field is `CF_1`"]
    #[inline(always)]
    pub fn is_cf_1(&self) -> bool {
        *self == CF_A::CF_1
    }
}
impl R {
    #[doc = "Bit 0 - Configure Flag Host software sets this bit as the last action in its process of configuring the Host Controller"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Configure Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [configflag](index.html) module"]
pub struct CONFIGFLAG_SPEC;
impl crate::RegisterSpec for CONFIGFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [configflag::R](R) reader structure"]
impl crate::Readable for CONFIGFLAG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONFIGFLAG to value 0x01"]
impl crate::Resettable for CONFIGFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
