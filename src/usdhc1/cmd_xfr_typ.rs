#[doc = "Register `CMD_XFR_TYP` reader"]
pub struct R(crate::R<CMD_XFR_TYP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_XFR_TYP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_XFR_TYP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_XFR_TYP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD_XFR_TYP` writer"]
pub struct W(crate::W<CMD_XFR_TYP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_XFR_TYP_SPEC>;
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
impl From<crate::W<CMD_XFR_TYP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_XFR_TYP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSPTYP` reader - Response type select"]
pub type RSPTYP_R = crate::FieldReader<u8, RSPTYP_A>;
#[doc = "Response type select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSPTYP_A {
    #[doc = "0: No response"]
    RSPTYP_0 = 0,
    #[doc = "1: Response length 136"]
    RSPTYP_1 = 1,
    #[doc = "2: Response length 48"]
    RSPTYP_2 = 2,
    #[doc = "3: Response length 48, check busy after response"]
    RSPTYP_3 = 3,
}
impl From<RSPTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: RSPTYP_A) -> Self {
        variant as _
    }
}
impl RSPTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPTYP_A {
        match self.bits {
            0 => RSPTYP_A::RSPTYP_0,
            1 => RSPTYP_A::RSPTYP_1,
            2 => RSPTYP_A::RSPTYP_2,
            3 => RSPTYP_A::RSPTYP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RSPTYP_0`"]
    #[inline(always)]
    pub fn is_rsptyp_0(&self) -> bool {
        *self == RSPTYP_A::RSPTYP_0
    }
    #[doc = "Checks if the value of the field is `RSPTYP_1`"]
    #[inline(always)]
    pub fn is_rsptyp_1(&self) -> bool {
        *self == RSPTYP_A::RSPTYP_1
    }
    #[doc = "Checks if the value of the field is `RSPTYP_2`"]
    #[inline(always)]
    pub fn is_rsptyp_2(&self) -> bool {
        *self == RSPTYP_A::RSPTYP_2
    }
    #[doc = "Checks if the value of the field is `RSPTYP_3`"]
    #[inline(always)]
    pub fn is_rsptyp_3(&self) -> bool {
        *self == RSPTYP_A::RSPTYP_3
    }
}
#[doc = "Field `RSPTYP` writer - Response type select"]
pub type RSPTYP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CMD_XFR_TYP_SPEC, u8, RSPTYP_A, 2, O>;
impl<'a, const O: u8> RSPTYP_W<'a, O> {
    #[doc = "No response"]
    #[inline(always)]
    pub fn rsptyp_0(self) -> &'a mut W {
        self.variant(RSPTYP_A::RSPTYP_0)
    }
    #[doc = "Response length 136"]
    #[inline(always)]
    pub fn rsptyp_1(self) -> &'a mut W {
        self.variant(RSPTYP_A::RSPTYP_1)
    }
    #[doc = "Response length 48"]
    #[inline(always)]
    pub fn rsptyp_2(self) -> &'a mut W {
        self.variant(RSPTYP_A::RSPTYP_2)
    }
    #[doc = "Response length 48, check busy after response"]
    #[inline(always)]
    pub fn rsptyp_3(self) -> &'a mut W {
        self.variant(RSPTYP_A::RSPTYP_3)
    }
}
#[doc = "Field `CCCEN` reader - Command CRC check enable"]
pub type CCCEN_R = crate::BitReader<CCCEN_A>;
#[doc = "Command CRC check enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCCEN_A {
    #[doc = "0: Disables command CRC check"]
    CCCEN_0 = 0,
    #[doc = "1: Enables command CRC check"]
    CCCEN_1 = 1,
}
impl From<CCCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCCEN_A {
        match self.bits {
            false => CCCEN_A::CCCEN_0,
            true => CCCEN_A::CCCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCCEN_0`"]
    #[inline(always)]
    pub fn is_cccen_0(&self) -> bool {
        *self == CCCEN_A::CCCEN_0
    }
    #[doc = "Checks if the value of the field is `CCCEN_1`"]
    #[inline(always)]
    pub fn is_cccen_1(&self) -> bool {
        *self == CCCEN_A::CCCEN_1
    }
}
#[doc = "Field `CCCEN` writer - Command CRC check enable"]
pub type CCCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFR_TYP_SPEC, CCCEN_A, O>;
impl<'a, const O: u8> CCCEN_W<'a, O> {
    #[doc = "Disables command CRC check"]
    #[inline(always)]
    pub fn cccen_0(self) -> &'a mut W {
        self.variant(CCCEN_A::CCCEN_0)
    }
    #[doc = "Enables command CRC check"]
    #[inline(always)]
    pub fn cccen_1(self) -> &'a mut W {
        self.variant(CCCEN_A::CCCEN_1)
    }
}
#[doc = "Field `CICEN` reader - Command index check enable"]
pub type CICEN_R = crate::BitReader<CICEN_A>;
#[doc = "Command index check enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CICEN_A {
    #[doc = "0: Disable command index check"]
    CICEN_0 = 0,
    #[doc = "1: Enables command index check"]
    CICEN_1 = 1,
}
impl From<CICEN_A> for bool {
    #[inline(always)]
    fn from(variant: CICEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CICEN_A {
        match self.bits {
            false => CICEN_A::CICEN_0,
            true => CICEN_A::CICEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CICEN_0`"]
    #[inline(always)]
    pub fn is_cicen_0(&self) -> bool {
        *self == CICEN_A::CICEN_0
    }
    #[doc = "Checks if the value of the field is `CICEN_1`"]
    #[inline(always)]
    pub fn is_cicen_1(&self) -> bool {
        *self == CICEN_A::CICEN_1
    }
}
#[doc = "Field `CICEN` writer - Command index check enable"]
pub type CICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFR_TYP_SPEC, CICEN_A, O>;
impl<'a, const O: u8> CICEN_W<'a, O> {
    #[doc = "Disable command index check"]
    #[inline(always)]
    pub fn cicen_0(self) -> &'a mut W {
        self.variant(CICEN_A::CICEN_0)
    }
    #[doc = "Enables command index check"]
    #[inline(always)]
    pub fn cicen_1(self) -> &'a mut W {
        self.variant(CICEN_A::CICEN_1)
    }
}
#[doc = "Field `DPSEL` reader - Data present select"]
pub type DPSEL_R = crate::BitReader<DPSEL_A>;
#[doc = "Data present select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPSEL_A {
    #[doc = "0: No data present"]
    DPSEL_0 = 0,
    #[doc = "1: Data present"]
    DPSEL_1 = 1,
}
impl From<DPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DPSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPSEL_A {
        match self.bits {
            false => DPSEL_A::DPSEL_0,
            true => DPSEL_A::DPSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DPSEL_0`"]
    #[inline(always)]
    pub fn is_dpsel_0(&self) -> bool {
        *self == DPSEL_A::DPSEL_0
    }
    #[doc = "Checks if the value of the field is `DPSEL_1`"]
    #[inline(always)]
    pub fn is_dpsel_1(&self) -> bool {
        *self == DPSEL_A::DPSEL_1
    }
}
#[doc = "Field `DPSEL` writer - Data present select"]
pub type DPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFR_TYP_SPEC, DPSEL_A, O>;
impl<'a, const O: u8> DPSEL_W<'a, O> {
    #[doc = "No data present"]
    #[inline(always)]
    pub fn dpsel_0(self) -> &'a mut W {
        self.variant(DPSEL_A::DPSEL_0)
    }
    #[doc = "Data present"]
    #[inline(always)]
    pub fn dpsel_1(self) -> &'a mut W {
        self.variant(DPSEL_A::DPSEL_1)
    }
}
#[doc = "Field `CMDTYP` reader - Command type"]
pub type CMDTYP_R = crate::FieldReader<u8, CMDTYP_A>;
#[doc = "Command type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDTYP_A {
    #[doc = "0: Normal other commands"]
    CMDTYP_0 = 0,
    #[doc = "1: Suspend CMD52 for writing bus suspend in CCCR"]
    CMDTYP_1 = 1,
    #[doc = "2: Resume CMD52 for writing function select in CCCR"]
    CMDTYP_2 = 2,
    #[doc = "3: Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
    CMDTYP_3 = 3,
}
impl From<CMDTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDTYP_A) -> Self {
        variant as _
    }
}
impl CMDTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTYP_A {
        match self.bits {
            0 => CMDTYP_A::CMDTYP_0,
            1 => CMDTYP_A::CMDTYP_1,
            2 => CMDTYP_A::CMDTYP_2,
            3 => CMDTYP_A::CMDTYP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CMDTYP_0`"]
    #[inline(always)]
    pub fn is_cmdtyp_0(&self) -> bool {
        *self == CMDTYP_A::CMDTYP_0
    }
    #[doc = "Checks if the value of the field is `CMDTYP_1`"]
    #[inline(always)]
    pub fn is_cmdtyp_1(&self) -> bool {
        *self == CMDTYP_A::CMDTYP_1
    }
    #[doc = "Checks if the value of the field is `CMDTYP_2`"]
    #[inline(always)]
    pub fn is_cmdtyp_2(&self) -> bool {
        *self == CMDTYP_A::CMDTYP_2
    }
    #[doc = "Checks if the value of the field is `CMDTYP_3`"]
    #[inline(always)]
    pub fn is_cmdtyp_3(&self) -> bool {
        *self == CMDTYP_A::CMDTYP_3
    }
}
#[doc = "Field `CMDTYP` writer - Command type"]
pub type CMDTYP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CMD_XFR_TYP_SPEC, u8, CMDTYP_A, 2, O>;
impl<'a, const O: u8> CMDTYP_W<'a, O> {
    #[doc = "Normal other commands"]
    #[inline(always)]
    pub fn cmdtyp_0(self) -> &'a mut W {
        self.variant(CMDTYP_A::CMDTYP_0)
    }
    #[doc = "Suspend CMD52 for writing bus suspend in CCCR"]
    #[inline(always)]
    pub fn cmdtyp_1(self) -> &'a mut W {
        self.variant(CMDTYP_A::CMDTYP_1)
    }
    #[doc = "Resume CMD52 for writing function select in CCCR"]
    #[inline(always)]
    pub fn cmdtyp_2(self) -> &'a mut W {
        self.variant(CMDTYP_A::CMDTYP_2)
    }
    #[doc = "Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
    #[inline(always)]
    pub fn cmdtyp_3(self) -> &'a mut W {
        self.variant(CMDTYP_A::CMDTYP_3)
    }
}
#[doc = "Field `CMDINX` reader - Command index"]
pub type CMDINX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDINX` writer - Command index"]
pub type CMDINX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_XFR_TYP_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 16:17 - Response type select"]
    #[inline(always)]
    pub fn rsptyp(&self) -> RSPTYP_R {
        RSPTYP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Command CRC check enable"]
    #[inline(always)]
    pub fn cccen(&self) -> CCCEN_R {
        CCCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Command index check enable"]
    #[inline(always)]
    pub fn cicen(&self) -> CICEN_R {
        CICEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data present select"]
    #[inline(always)]
    pub fn dpsel(&self) -> DPSEL_R {
        DPSEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Command type"]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CMDTYP_R {
        CMDTYP_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - Command index"]
    #[inline(always)]
    pub fn cmdinx(&self) -> CMDINX_R {
        CMDINX_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Response type select"]
    #[inline(always)]
    #[must_use]
    pub fn rsptyp(&mut self) -> RSPTYP_W<16> {
        RSPTYP_W::new(self)
    }
    #[doc = "Bit 19 - Command CRC check enable"]
    #[inline(always)]
    #[must_use]
    pub fn cccen(&mut self) -> CCCEN_W<19> {
        CCCEN_W::new(self)
    }
    #[doc = "Bit 20 - Command index check enable"]
    #[inline(always)]
    #[must_use]
    pub fn cicen(&mut self) -> CICEN_W<20> {
        CICEN_W::new(self)
    }
    #[doc = "Bit 21 - Data present select"]
    #[inline(always)]
    #[must_use]
    pub fn dpsel(&mut self) -> DPSEL_W<21> {
        DPSEL_W::new(self)
    }
    #[doc = "Bits 22:23 - Command type"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtyp(&mut self) -> CMDTYP_W<22> {
        CMDTYP_W::new(self)
    }
    #[doc = "Bits 24:29 - Command index"]
    #[inline(always)]
    #[must_use]
    pub fn cmdinx(&mut self) -> CMDINX_W<24> {
        CMDINX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Transfer Type\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_xfr_typ](index.html) module"]
pub struct CMD_XFR_TYP_SPEC;
impl crate::RegisterSpec for CMD_XFR_TYP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_xfr_typ::R](R) reader structure"]
impl crate::Readable for CMD_XFR_TYP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd_xfr_typ::W](W) writer structure"]
impl crate::Writable for CMD_XFR_TYP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD_XFR_TYP to value 0"]
impl crate::Resettable for CMD_XFR_TYP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
