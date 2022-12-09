#[doc = "Register `VID2` reader"]
pub struct R(crate::R<VID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONFIG_OPT` reader - Shows the IP's Configuaration options for the TRNG."]
pub type CONFIG_OPT_R = crate::FieldReader<u8, CONFIG_OPT_A>;
#[doc = "Shows the IP's Configuaration options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONFIG_OPT_A {
    #[doc = "0: TRNG_CONFIG_OPT for TRNG."]
    CONFIG_OPT_VAL = 0,
}
impl From<CONFIG_OPT_A> for u8 {
    #[inline(always)]
    fn from(variant: CONFIG_OPT_A) -> Self {
        variant as _
    }
}
impl CONFIG_OPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CONFIG_OPT_A> {
        match self.bits {
            0 => Some(CONFIG_OPT_A::CONFIG_OPT_VAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONFIG_OPT_VAL`"]
    #[inline(always)]
    pub fn is_config_opt_val(&self) -> bool {
        *self == CONFIG_OPT_A::CONFIG_OPT_VAL
    }
}
#[doc = "Field `ECO_REV` reader - Shows the IP's ECO revision of the TRNG."]
pub type ECO_REV_R = crate::FieldReader<u8, ECO_REV_A>;
#[doc = "Shows the IP's ECO revision of the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ECO_REV_A {
    #[doc = "0: TRNG_ECO_REV for TRNG."]
    ECO_REV_VAL = 0,
}
impl From<ECO_REV_A> for u8 {
    #[inline(always)]
    fn from(variant: ECO_REV_A) -> Self {
        variant as _
    }
}
impl ECO_REV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECO_REV_A> {
        match self.bits {
            0 => Some(ECO_REV_A::ECO_REV_VAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ECO_REV_VAL`"]
    #[inline(always)]
    pub fn is_eco_rev_val(&self) -> bool {
        *self == ECO_REV_A::ECO_REV_VAL
    }
}
#[doc = "Field `INTG_OPT` reader - Shows the integration options for the TRNG"]
pub type INTG_OPT_R = crate::FieldReader<u8, INTG_OPT_A>;
#[doc = "Shows the integration options for the TRNG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTG_OPT_A {
    #[doc = "0: INTG_OPT for TRNG."]
    INTG_OPT_VAL = 0,
}
impl From<INTG_OPT_A> for u8 {
    #[inline(always)]
    fn from(variant: INTG_OPT_A) -> Self {
        variant as _
    }
}
impl INTG_OPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INTG_OPT_A> {
        match self.bits {
            0 => Some(INTG_OPT_A::INTG_OPT_VAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INTG_OPT_VAL`"]
    #[inline(always)]
    pub fn is_intg_opt_val(&self) -> bool {
        *self == INTG_OPT_A::INTG_OPT_VAL
    }
}
#[doc = "Field `ERA` reader - Shows the compile options for the TRNG."]
pub type ERA_R = crate::FieldReader<u8, ERA_A>;
#[doc = "Shows the compile options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERA_A {
    #[doc = "0: COMPILE_OPT for TRNG."]
    ERA_VAL = 0,
}
impl From<ERA_A> for u8 {
    #[inline(always)]
    fn from(variant: ERA_A) -> Self {
        variant as _
    }
}
impl ERA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERA_A> {
        match self.bits {
            0 => Some(ERA_A::ERA_VAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ERA_VAL`"]
    #[inline(always)]
    pub fn is_era_val(&self) -> bool {
        *self == ERA_A::ERA_VAL
    }
}
impl R {
    #[doc = "Bits 0:7 - Shows the IP's Configuaration options for the TRNG."]
    #[inline(always)]
    pub fn config_opt(&self) -> CONFIG_OPT_R {
        CONFIG_OPT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Shows the IP's ECO revision of the TRNG."]
    #[inline(always)]
    pub fn eco_rev(&self) -> ECO_REV_R {
        ECO_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Shows the integration options for the TRNG"]
    #[inline(always)]
    pub fn intg_opt(&self) -> INTG_OPT_R {
        INTG_OPT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Shows the compile options for the TRNG."]
    #[inline(always)]
    pub fn era(&self) -> ERA_R {
        ERA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Version ID Register (LS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vid2](index.html) module"]
pub struct VID2_SPEC;
impl crate::RegisterSpec for VID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vid2::R](R) reader structure"]
impl crate::Readable for VID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VID2 to value 0"]
impl crate::Resettable for VID2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
