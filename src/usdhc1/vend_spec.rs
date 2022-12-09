#[doc = "Register `VEND_SPEC` reader"]
pub struct R(crate::R<VEND_SPEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VEND_SPEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VEND_SPEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VEND_SPEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VEND_SPEC` writer"]
pub struct W(crate::W<VEND_SPEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VEND_SPEC_SPEC>;
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
impl From<crate::W<VEND_SPEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VEND_SPEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSELECT` reader - Voltage selection"]
pub type VSELECT_R = crate::BitReader<VSELECT_A>;
#[doc = "Voltage selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSELECT_A {
    #[doc = "0: Change the voltage to high voltage range, around 3.0 V"]
    VSELECT_0 = 0,
    #[doc = "1: Change the voltage to low voltage range, around 1.8 V"]
    VSELECT_1 = 1,
}
impl From<VSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: VSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl VSELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSELECT_A {
        match self.bits {
            false => VSELECT_A::VSELECT_0,
            true => VSELECT_A::VSELECT_1,
        }
    }
    #[doc = "Checks if the value of the field is `VSELECT_0`"]
    #[inline(always)]
    pub fn is_vselect_0(&self) -> bool {
        *self == VSELECT_A::VSELECT_0
    }
    #[doc = "Checks if the value of the field is `VSELECT_1`"]
    #[inline(always)]
    pub fn is_vselect_1(&self) -> bool {
        *self == VSELECT_A::VSELECT_1
    }
}
#[doc = "Field `VSELECT` writer - Voltage selection"]
pub type VSELECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, VEND_SPEC_SPEC, VSELECT_A, O>;
impl<'a, const O: u8> VSELECT_W<'a, O> {
    #[doc = "Change the voltage to high voltage range, around 3.0 V"]
    #[inline(always)]
    pub fn vselect_0(self) -> &'a mut W {
        self.variant(VSELECT_A::VSELECT_0)
    }
    #[doc = "Change the voltage to low voltage range, around 1.8 V"]
    #[inline(always)]
    pub fn vselect_1(self) -> &'a mut W {
        self.variant(VSELECT_A::VSELECT_1)
    }
}
#[doc = "Field `CONFLICT_CHK_EN` reader - Conflict check enable"]
pub type CONFLICT_CHK_EN_R = crate::BitReader<CONFLICT_CHK_EN_A>;
#[doc = "Conflict check enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONFLICT_CHK_EN_A {
    #[doc = "0: Conflict check disable"]
    CONFLICT_CHK_EN_0 = 0,
    #[doc = "1: Conflict check enable"]
    CONFLICT_CHK_EN_1 = 1,
}
impl From<CONFLICT_CHK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CONFLICT_CHK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CONFLICT_CHK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONFLICT_CHK_EN_A {
        match self.bits {
            false => CONFLICT_CHK_EN_A::CONFLICT_CHK_EN_0,
            true => CONFLICT_CHK_EN_A::CONFLICT_CHK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONFLICT_CHK_EN_0`"]
    #[inline(always)]
    pub fn is_conflict_chk_en_0(&self) -> bool {
        *self == CONFLICT_CHK_EN_A::CONFLICT_CHK_EN_0
    }
    #[doc = "Checks if the value of the field is `CONFLICT_CHK_EN_1`"]
    #[inline(always)]
    pub fn is_conflict_chk_en_1(&self) -> bool {
        *self == CONFLICT_CHK_EN_A::CONFLICT_CHK_EN_1
    }
}
#[doc = "Field `CONFLICT_CHK_EN` writer - Conflict check enable"]
pub type CONFLICT_CHK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VEND_SPEC_SPEC, CONFLICT_CHK_EN_A, O>;
impl<'a, const O: u8> CONFLICT_CHK_EN_W<'a, O> {
    #[doc = "Conflict check disable"]
    #[inline(always)]
    pub fn conflict_chk_en_0(self) -> &'a mut W {
        self.variant(CONFLICT_CHK_EN_A::CONFLICT_CHK_EN_0)
    }
    #[doc = "Conflict check enable"]
    #[inline(always)]
    pub fn conflict_chk_en_1(self) -> &'a mut W {
        self.variant(CONFLICT_CHK_EN_A::CONFLICT_CHK_EN_1)
    }
}
#[doc = "Field `AC12_WR_CHKBUSY_EN` reader - Check busy enable"]
pub type AC12_WR_CHKBUSY_EN_R = crate::BitReader<AC12_WR_CHKBUSY_EN_A>;
#[doc = "Check busy enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12_WR_CHKBUSY_EN_A {
    #[doc = "0: Do not check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_0 = 0,
    #[doc = "1: Check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_1 = 1,
}
impl From<AC12_WR_CHKBUSY_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AC12_WR_CHKBUSY_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12_WR_CHKBUSY_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12_WR_CHKBUSY_EN_A {
        match self.bits {
            false => AC12_WR_CHKBUSY_EN_A::AC12_WR_CHKBUSY_EN_0,
            true => AC12_WR_CHKBUSY_EN_A::AC12_WR_CHKBUSY_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12_WR_CHKBUSY_EN_0`"]
    #[inline(always)]
    pub fn is_ac12_wr_chkbusy_en_0(&self) -> bool {
        *self == AC12_WR_CHKBUSY_EN_A::AC12_WR_CHKBUSY_EN_0
    }
    #[doc = "Checks if the value of the field is `AC12_WR_CHKBUSY_EN_1`"]
    #[inline(always)]
    pub fn is_ac12_wr_chkbusy_en_1(&self) -> bool {
        *self == AC12_WR_CHKBUSY_EN_A::AC12_WR_CHKBUSY_EN_1
    }
}
#[doc = "Field `AC12_WR_CHKBUSY_EN` writer - Check busy enable"]
pub type AC12_WR_CHKBUSY_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VEND_SPEC_SPEC, AC12_WR_CHKBUSY_EN_A, O>;
impl<'a, const O: u8> AC12_WR_CHKBUSY_EN_W<'a, O> {
    #[doc = "Do not check busy after auto CMD12 for write data packet"]
    #[inline(always)]
    pub fn ac12_wr_chkbusy_en_0(self) -> &'a mut W {
        self.variant(AC12_WR_CHKBUSY_EN_A::AC12_WR_CHKBUSY_EN_0)
    }
    #[doc = "Check busy after auto CMD12 for write data packet"]
    #[inline(always)]
    pub fn ac12_wr_chkbusy_en_1(self) -> &'a mut W {
        self.variant(AC12_WR_CHKBUSY_EN_A::AC12_WR_CHKBUSY_EN_1)
    }
}
#[doc = "Field `FRC_SDCLK_ON` reader - Force CLK"]
pub type FRC_SDCLK_ON_R = crate::BitReader<FRC_SDCLK_ON_A>;
#[doc = "Force CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRC_SDCLK_ON_A {
    #[doc = "0: CLK active or inactive is fully controlled by the hardware."]
    FRC_SDCLK_ON_0 = 0,
    #[doc = "1: Force CLK active"]
    FRC_SDCLK_ON_1 = 1,
}
impl From<FRC_SDCLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: FRC_SDCLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
impl FRC_SDCLK_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRC_SDCLK_ON_A {
        match self.bits {
            false => FRC_SDCLK_ON_A::FRC_SDCLK_ON_0,
            true => FRC_SDCLK_ON_A::FRC_SDCLK_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRC_SDCLK_ON_0`"]
    #[inline(always)]
    pub fn is_frc_sdclk_on_0(&self) -> bool {
        *self == FRC_SDCLK_ON_A::FRC_SDCLK_ON_0
    }
    #[doc = "Checks if the value of the field is `FRC_SDCLK_ON_1`"]
    #[inline(always)]
    pub fn is_frc_sdclk_on_1(&self) -> bool {
        *self == FRC_SDCLK_ON_A::FRC_SDCLK_ON_1
    }
}
#[doc = "Field `FRC_SDCLK_ON` writer - Force CLK"]
pub type FRC_SDCLK_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VEND_SPEC_SPEC, FRC_SDCLK_ON_A, O>;
impl<'a, const O: u8> FRC_SDCLK_ON_W<'a, O> {
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    #[inline(always)]
    pub fn frc_sdclk_on_0(self) -> &'a mut W {
        self.variant(FRC_SDCLK_ON_A::FRC_SDCLK_ON_0)
    }
    #[doc = "Force CLK active"]
    #[inline(always)]
    pub fn frc_sdclk_on_1(self) -> &'a mut W {
        self.variant(FRC_SDCLK_ON_A::FRC_SDCLK_ON_1)
    }
}
#[doc = "Field `CRC_CHK_DIS` reader - CRC Check Disable"]
pub type CRC_CHK_DIS_R = crate::BitReader<CRC_CHK_DIS_A>;
#[doc = "CRC Check Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC_CHK_DIS_A {
    #[doc = "0: Check CRC16 for every read data packet and check CRC fields for every write data packet"]
    CRC_CHK_DIS_0 = 0,
    #[doc = "1: Ignore CRC16 check for every read data packet and ignore CRC fields check for every write data packet"]
    CRC_CHK_DIS_1 = 1,
}
impl From<CRC_CHK_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_CHK_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC_CHK_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_CHK_DIS_A {
        match self.bits {
            false => CRC_CHK_DIS_A::CRC_CHK_DIS_0,
            true => CRC_CHK_DIS_A::CRC_CHK_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CHK_DIS_0`"]
    #[inline(always)]
    pub fn is_crc_chk_dis_0(&self) -> bool {
        *self == CRC_CHK_DIS_A::CRC_CHK_DIS_0
    }
    #[doc = "Checks if the value of the field is `CRC_CHK_DIS_1`"]
    #[inline(always)]
    pub fn is_crc_chk_dis_1(&self) -> bool {
        *self == CRC_CHK_DIS_A::CRC_CHK_DIS_1
    }
}
#[doc = "Field `CRC_CHK_DIS` writer - CRC Check Disable"]
pub type CRC_CHK_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VEND_SPEC_SPEC, CRC_CHK_DIS_A, O>;
impl<'a, const O: u8> CRC_CHK_DIS_W<'a, O> {
    #[doc = "Check CRC16 for every read data packet and check CRC fields for every write data packet"]
    #[inline(always)]
    pub fn crc_chk_dis_0(self) -> &'a mut W {
        self.variant(CRC_CHK_DIS_A::CRC_CHK_DIS_0)
    }
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC fields check for every write data packet"]
    #[inline(always)]
    pub fn crc_chk_dis_1(self) -> &'a mut W {
        self.variant(CRC_CHK_DIS_A::CRC_CHK_DIS_1)
    }
}
#[doc = "Field `CMD_BYTE_EN` reader - Byte access"]
pub type CMD_BYTE_EN_R = crate::BitReader<CMD_BYTE_EN_A>;
#[doc = "Byte access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_BYTE_EN_A {
    #[doc = "0: Disable"]
    CMD_BYTE_EN_0 = 0,
    #[doc = "1: Enable"]
    CMD_BYTE_EN_1 = 1,
}
impl From<CMD_BYTE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_BYTE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CMD_BYTE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_BYTE_EN_A {
        match self.bits {
            false => CMD_BYTE_EN_A::CMD_BYTE_EN_0,
            true => CMD_BYTE_EN_A::CMD_BYTE_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMD_BYTE_EN_0`"]
    #[inline(always)]
    pub fn is_cmd_byte_en_0(&self) -> bool {
        *self == CMD_BYTE_EN_A::CMD_BYTE_EN_0
    }
    #[doc = "Checks if the value of the field is `CMD_BYTE_EN_1`"]
    #[inline(always)]
    pub fn is_cmd_byte_en_1(&self) -> bool {
        *self == CMD_BYTE_EN_A::CMD_BYTE_EN_1
    }
}
#[doc = "Field `CMD_BYTE_EN` writer - Byte access"]
pub type CMD_BYTE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VEND_SPEC_SPEC, CMD_BYTE_EN_A, O>;
impl<'a, const O: u8> CMD_BYTE_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn cmd_byte_en_0(self) -> &'a mut W {
        self.variant(CMD_BYTE_EN_A::CMD_BYTE_EN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn cmd_byte_en_1(self) -> &'a mut W {
        self.variant(CMD_BYTE_EN_A::CMD_BYTE_EN_1)
    }
}
impl R {
    #[doc = "Bit 1 - Voltage selection"]
    #[inline(always)]
    pub fn vselect(&self) -> VSELECT_R {
        VSELECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Conflict check enable"]
    #[inline(always)]
    pub fn conflict_chk_en(&self) -> CONFLICT_CHK_EN_R {
        CONFLICT_CHK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Check busy enable"]
    #[inline(always)]
    pub fn ac12_wr_chkbusy_en(&self) -> AC12_WR_CHKBUSY_EN_R {
        AC12_WR_CHKBUSY_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Force CLK"]
    #[inline(always)]
    pub fn frc_sdclk_on(&self) -> FRC_SDCLK_ON_R {
        FRC_SDCLK_ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - CRC Check Disable"]
    #[inline(always)]
    pub fn crc_chk_dis(&self) -> CRC_CHK_DIS_R {
        CRC_CHK_DIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Byte access"]
    #[inline(always)]
    pub fn cmd_byte_en(&self) -> CMD_BYTE_EN_R {
        CMD_BYTE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Voltage selection"]
    #[inline(always)]
    #[must_use]
    pub fn vselect(&mut self) -> VSELECT_W<1> {
        VSELECT_W::new(self)
    }
    #[doc = "Bit 2 - Conflict check enable"]
    #[inline(always)]
    #[must_use]
    pub fn conflict_chk_en(&mut self) -> CONFLICT_CHK_EN_W<2> {
        CONFLICT_CHK_EN_W::new(self)
    }
    #[doc = "Bit 3 - Check busy enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac12_wr_chkbusy_en(&mut self) -> AC12_WR_CHKBUSY_EN_W<3> {
        AC12_WR_CHKBUSY_EN_W::new(self)
    }
    #[doc = "Bit 8 - Force CLK"]
    #[inline(always)]
    #[must_use]
    pub fn frc_sdclk_on(&mut self) -> FRC_SDCLK_ON_W<8> {
        FRC_SDCLK_ON_W::new(self)
    }
    #[doc = "Bit 15 - CRC Check Disable"]
    #[inline(always)]
    #[must_use]
    pub fn crc_chk_dis(&mut self) -> CRC_CHK_DIS_W<15> {
        CRC_CHK_DIS_W::new(self)
    }
    #[doc = "Bit 31 - Byte access"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_byte_en(&mut self) -> CMD_BYTE_EN_W<31> {
        CMD_BYTE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vendor Specific Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vend_spec](index.html) module"]
pub struct VEND_SPEC_SPEC;
impl crate::RegisterSpec for VEND_SPEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vend_spec::R](R) reader structure"]
impl crate::Readable for VEND_SPEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vend_spec::W](W) writer structure"]
impl crate::Writable for VEND_SPEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VEND_SPEC to value 0x3000_7809"]
impl crate::Resettable for VEND_SPEC_SPEC {
    const RESET_VALUE: Self::Ux = 0x3000_7809;
}
