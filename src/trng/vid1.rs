#[doc = "Register `VID1` reader"]
pub struct R(crate::R<VID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MIN_REV` reader - Shows the IP's Minor revision of the TRNG."]
pub type MIN_REV_R = crate::FieldReader<u8, MIN_REV_A>;
#[doc = "Shows the IP's Minor revision of the TRNG.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MIN_REV_A {
    #[doc = "0: Minor revision number for TRNG."]
    MIN_REV_VAL = 0,
}
impl From<MIN_REV_A> for u8 {
    #[inline(always)]
    fn from(variant: MIN_REV_A) -> Self {
        variant as _
    }
}
impl MIN_REV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MIN_REV_A> {
        match self.bits {
            0 => Some(MIN_REV_A::MIN_REV_VAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN_REV_VAL`"]
    #[inline(always)]
    pub fn is_min_rev_val(&self) -> bool {
        *self == MIN_REV_A::MIN_REV_VAL
    }
}
#[doc = "Field `MAJ_REV` reader - Shows the IP's Major revision of the TRNG."]
pub type MAJ_REV_R = crate::FieldReader<u8, MAJ_REV_A>;
#[doc = "Shows the IP's Major revision of the TRNG.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAJ_REV_A {
    #[doc = "1: Major revision number for TRNG."]
    MAJ_REV_VAL = 1,
}
impl From<MAJ_REV_A> for u8 {
    #[inline(always)]
    fn from(variant: MAJ_REV_A) -> Self {
        variant as _
    }
}
impl MAJ_REV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAJ_REV_A> {
        match self.bits {
            1 => Some(MAJ_REV_A::MAJ_REV_VAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAJ_REV_VAL`"]
    #[inline(always)]
    pub fn is_maj_rev_val(&self) -> bool {
        *self == MAJ_REV_A::MAJ_REV_VAL
    }
}
#[doc = "Field `IP_ID` reader - Shows the IP ID."]
pub type IP_ID_R = crate::FieldReader<u16, IP_ID_A>;
#[doc = "Shows the IP ID.\n\nValue on reset: 48"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum IP_ID_A {
    #[doc = "48: ID for TRNG."]
    IP_ID_VAL = 48,
}
impl From<IP_ID_A> for u16 {
    #[inline(always)]
    fn from(variant: IP_ID_A) -> Self {
        variant as _
    }
}
impl IP_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IP_ID_A> {
        match self.bits {
            48 => Some(IP_ID_A::IP_ID_VAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IP_ID_VAL`"]
    #[inline(always)]
    pub fn is_ip_id_val(&self) -> bool {
        *self == IP_ID_A::IP_ID_VAL
    }
}
impl R {
    #[doc = "Bits 0:7 - Shows the IP's Minor revision of the TRNG."]
    #[inline(always)]
    pub fn min_rev(&self) -> MIN_REV_R {
        MIN_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Shows the IP's Major revision of the TRNG."]
    #[inline(always)]
    pub fn maj_rev(&self) -> MAJ_REV_R {
        MAJ_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Shows the IP ID."]
    #[inline(always)]
    pub fn ip_id(&self) -> IP_ID_R {
        IP_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Version ID Register (MS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vid1](index.html) module"]
pub struct VID1_SPEC;
impl crate::RegisterSpec for VID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vid1::R](R) reader structure"]
impl crate::Readable for VID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VID1 to value 0x0030_0301"]
impl crate::Resettable for VID1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0030_0301;
}
