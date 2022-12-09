#[doc = "Register `HW_OCOTP_LOCK` reader"]
pub struct R(crate::R<HW_OCOTP_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_OCOTP_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_OCOTP_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_OCOTP_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_OCOTP_LOCK` writer"]
pub struct W(crate::W<HW_OCOTP_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_OCOTP_LOCK_SPEC>;
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
impl From<crate::W<HW_OCOTP_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_OCOTP_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_CFG` reader - BOOT_CFG Write Lock Status"]
pub type BOOT_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SJC_RESP` reader - SJC_RESP Lock Status"]
pub type SJC_RESP_R = crate::BitReader<SJC_RESP_A>;
#[doc = "SJC_RESP Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SJC_RESP_A {
    #[doc = "0: The writing or reading of this region's shadow register and OTP fuse word are not blocked."]
    NOTBLOCKED = 0,
    #[doc = "1: When set, the writing of this region's shadow register and OTP fuse word are blocked. The read of this region's shadow register and OTP fuse word are also blocked"]
    BLOCKED = 1,
}
impl From<SJC_RESP_A> for bool {
    #[inline(always)]
    fn from(variant: SJC_RESP_A) -> Self {
        variant as u8 != 0
    }
}
impl SJC_RESP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SJC_RESP_A {
        match self.bits {
            false => SJC_RESP_A::NOTBLOCKED,
            true => SJC_RESP_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBLOCKED`"]
    #[inline(always)]
    pub fn is_notblocked(&self) -> bool {
        *self == SJC_RESP_A::NOTBLOCKED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SJC_RESP_A::BLOCKED
    }
}
#[doc = "Field `MAC_ADDR` reader - MAC_ADDR Write Lock Status"]
pub type MAC_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GP1` reader - GP1 Write Lock Status"]
pub type GP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GP2` reader - GP2 Write Lock Status"]
pub type GP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_GP1` reader - SW_GP1 Write Lock Status"]
pub type SW_GP1_R = crate::BitReader<SW_GP1_A>;
#[doc = "SW_GP1 Write Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW_GP1_A {
    #[doc = "0: Writing of this region's shadow register and OTP fuse word are not blocked."]
    NOTBLOCKED = 0,
    #[doc = "1: When set, the writing of this region's shadow register and OTP fuse word are blocked."]
    BLOCKED = 1,
}
impl From<SW_GP1_A> for bool {
    #[inline(always)]
    fn from(variant: SW_GP1_A) -> Self {
        variant as u8 != 0
    }
}
impl SW_GP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_GP1_A {
        match self.bits {
            false => SW_GP1_A::NOTBLOCKED,
            true => SW_GP1_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBLOCKED`"]
    #[inline(always)]
    pub fn is_notblocked(&self) -> bool {
        *self == SW_GP1_A::NOTBLOCKED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SW_GP1_A::BLOCKED
    }
}
#[doc = "Field `ANALOG` reader - ANALOG Write Lock Status"]
pub type ANALOG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_GP2_LOCK` reader - SW_GP2 Write Lock Status"]
pub type SW_GP2_LOCK_R = crate::BitReader<SW_GP2_LOCK_A>;
#[doc = "SW_GP2 Write Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW_GP2_LOCK_A {
    #[doc = "0: Writing of this region's shadow register and OTP fuse word are not blocked."]
    NOTBLOCKED = 0,
    #[doc = "1: When set, the writing of this region's shadow register and OTP fuse word are blocked."]
    BLOCKED = 1,
}
impl From<SW_GP2_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SW_GP2_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl SW_GP2_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_GP2_LOCK_A {
        match self.bits {
            false => SW_GP2_LOCK_A::NOTBLOCKED,
            true => SW_GP2_LOCK_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBLOCKED`"]
    #[inline(always)]
    pub fn is_notblocked(&self) -> bool {
        *self == SW_GP2_LOCK_A::NOTBLOCKED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SW_GP2_LOCK_A::BLOCKED
    }
}
#[doc = "Field `MISC_CONF` reader - MISC_CONF Write Lock Status"]
pub type MISC_CONF_R = crate::BitReader<MISC_CONF_A>;
#[doc = "MISC_CONF Write Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISC_CONF_A {
    #[doc = "0: Writing of this region's shadow register and OTP fuse word are not blocked."]
    NOTBLOCKED = 0,
    #[doc = "1: When set, the writing of this region's shadow register and OTP fuse word are blocked."]
    BLOCKED = 1,
}
impl From<MISC_CONF_A> for bool {
    #[inline(always)]
    fn from(variant: MISC_CONF_A) -> Self {
        variant as u8 != 0
    }
}
impl MISC_CONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MISC_CONF_A {
        match self.bits {
            false => MISC_CONF_A::NOTBLOCKED,
            true => MISC_CONF_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBLOCKED`"]
    #[inline(always)]
    pub fn is_notblocked(&self) -> bool {
        *self == MISC_CONF_A::NOTBLOCKED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == MISC_CONF_A::BLOCKED
    }
}
#[doc = "Field `SW_GP2_RLOCK` reader - SW_GP2 Read Lock Status"]
pub type SW_GP2_RLOCK_R = crate::BitReader<SW_GP2_RLOCK_A>;
#[doc = "SW_GP2 Read Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW_GP2_RLOCK_A {
    #[doc = "0: The reading of this region's shadow register and OTP fuse word are not blocked."]
    NOTBLOCKED = 0,
    #[doc = "1: When set, the reading of this region's shadow register and OTP fuse word are blocked."]
    BLOCKED = 1,
}
impl From<SW_GP2_RLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SW_GP2_RLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl SW_GP2_RLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_GP2_RLOCK_A {
        match self.bits {
            false => SW_GP2_RLOCK_A::NOTBLOCKED,
            true => SW_GP2_RLOCK_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBLOCKED`"]
    #[inline(always)]
    pub fn is_notblocked(&self) -> bool {
        *self == SW_GP2_RLOCK_A::NOTBLOCKED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SW_GP2_RLOCK_A::BLOCKED
    }
}
#[doc = "Field `GP3` reader - GP3 Write Lock Status"]
pub type GP3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIELD_RETURN` reader - FIELD RETURN Status"]
pub type FIELD_RETURN_R = crate::BitReader<FIELD_RETURN_A>;
#[doc = "FIELD RETURN Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIELD_RETURN_A {
    #[doc = "0: The device is a functional part."]
    FUNCTIONAL = 0,
    #[doc = "1: The device is a field returned part."]
    FIELD_RETURNED = 1,
}
impl From<FIELD_RETURN_A> for bool {
    #[inline(always)]
    fn from(variant: FIELD_RETURN_A) -> Self {
        variant as u8 != 0
    }
}
impl FIELD_RETURN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELD_RETURN_A {
        match self.bits {
            false => FIELD_RETURN_A::FUNCTIONAL,
            true => FIELD_RETURN_A::FIELD_RETURNED,
        }
    }
    #[doc = "Checks if the value of the field is `FUNCTIONAL`"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == FIELD_RETURN_A::FUNCTIONAL
    }
    #[doc = "Checks if the value of the field is `FIELD_RETURNED`"]
    #[inline(always)]
    pub fn is_field_returned(&self) -> bool {
        *self == FIELD_RETURN_A::FIELD_RETURNED
    }
}
#[doc = "Field `FIELD_RETURN` writer - FIELD RETURN Status"]
pub type FIELD_RETURN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HW_OCOTP_LOCK_SPEC, FIELD_RETURN_A, O>;
impl<'a, const O: u8> FIELD_RETURN_W<'a, O> {
    #[doc = "The device is a functional part."]
    #[inline(always)]
    pub fn functional(self) -> &'a mut W {
        self.variant(FIELD_RETURN_A::FUNCTIONAL)
    }
    #[doc = "The device is a field returned part."]
    #[inline(always)]
    pub fn field_returned(self) -> &'a mut W {
        self.variant(FIELD_RETURN_A::FIELD_RETURNED)
    }
}
impl R {
    #[doc = "Bits 2:3 - BOOT_CFG Write Lock Status"]
    #[inline(always)]
    pub fn boot_cfg(&self) -> BOOT_CFG_R {
        BOOT_CFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - SJC_RESP Lock Status"]
    #[inline(always)]
    pub fn sjc_resp(&self) -> SJC_RESP_R {
        SJC_RESP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - MAC_ADDR Write Lock Status"]
    #[inline(always)]
    pub fn mac_addr(&self) -> MAC_ADDR_R {
        MAC_ADDR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GP1 Write Lock Status"]
    #[inline(always)]
    pub fn gp1(&self) -> GP1_R {
        GP1_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GP2 Write Lock Status"]
    #[inline(always)]
    pub fn gp2(&self) -> GP2_R {
        GP2_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - SW_GP1 Write Lock Status"]
    #[inline(always)]
    pub fn sw_gp1(&self) -> SW_GP1_R {
        SW_GP1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - ANALOG Write Lock Status"]
    #[inline(always)]
    pub fn analog(&self) -> ANALOG_R {
        ANALOG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - SW_GP2 Write Lock Status"]
    #[inline(always)]
    pub fn sw_gp2_lock(&self) -> SW_GP2_LOCK_R {
        SW_GP2_LOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MISC_CONF Write Lock Status"]
    #[inline(always)]
    pub fn misc_conf(&self) -> MISC_CONF_R {
        MISC_CONF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SW_GP2 Read Lock Status"]
    #[inline(always)]
    pub fn sw_gp2_rlock(&self) -> SW_GP2_RLOCK_R {
        SW_GP2_RLOCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:27 - GP3 Write Lock Status"]
    #[inline(always)]
    pub fn gp3(&self) -> GP3_R {
        GP3_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 31 - FIELD RETURN Status"]
    #[inline(always)]
    pub fn field_return(&self) -> FIELD_RETURN_R {
        FIELD_RETURN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - FIELD RETURN Status"]
    #[inline(always)]
    #[must_use]
    pub fn field_return(&mut self) -> FIELD_RETURN_W<31> {
        FIELD_RETURN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Value of OTP Bank0 Word0 (Lock controls)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_ocotp_lock](index.html) module"]
pub struct HW_OCOTP_LOCK_SPEC;
impl crate::RegisterSpec for HW_OCOTP_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_ocotp_lock::R](R) reader structure"]
impl crate::Readable for HW_OCOTP_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_ocotp_lock::W](W) writer structure"]
impl crate::Writable for HW_OCOTP_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HW_OCOTP_LOCK to value 0x4012_8003"]
impl crate::Resettable for HW_OCOTP_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x4012_8003;
}
