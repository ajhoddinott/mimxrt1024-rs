#[doc = "Register `SSR` reader"]
pub struct R(crate::R<SSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSR` writer"]
pub struct W(crate::W<SSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSR_SPEC>;
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
impl From<crate::W<SSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDF` reader - Transmit Data Flag"]
pub type TDF_R = crate::BitReader<TDF_A>;
#[doc = "Transmit Data Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDF_A {
    #[doc = "0: Transmit data not requested"]
    NO_FLAG = 0,
    #[doc = "1: Transmit data is requested"]
    FLAG = 1,
}
impl From<TDF_A> for bool {
    #[inline(always)]
    fn from(variant: TDF_A) -> Self {
        variant as u8 != 0
    }
}
impl TDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDF_A {
        match self.bits {
            false => TDF_A::NO_FLAG,
            true => TDF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == TDF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == TDF_A::FLAG
    }
}
#[doc = "Field `RDF` reader - Receive Data Flag"]
pub type RDF_R = crate::BitReader<RDF_A>;
#[doc = "Receive Data Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDF_A {
    #[doc = "0: Receive data is not ready"]
    NOT_READY = 0,
    #[doc = "1: Receive data is ready"]
    READY = 1,
}
impl From<RDF_A> for bool {
    #[inline(always)]
    fn from(variant: RDF_A) -> Self {
        variant as u8 != 0
    }
}
impl RDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDF_A {
        match self.bits {
            false => RDF_A::NOT_READY,
            true => RDF_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == RDF_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RDF_A::READY
    }
}
#[doc = "Field `AVF` reader - Address Valid Flag"]
pub type AVF_R = crate::BitReader<AVF_A>;
#[doc = "Address Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVF_A {
    #[doc = "0: Address Status Register is not valid"]
    NOT_VALID = 0,
    #[doc = "1: Address Status Register is valid"]
    VALID = 1,
}
impl From<AVF_A> for bool {
    #[inline(always)]
    fn from(variant: AVF_A) -> Self {
        variant as u8 != 0
    }
}
impl AVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVF_A {
        match self.bits {
            false => AVF_A::NOT_VALID,
            true => AVF_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == AVF_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == AVF_A::VALID
    }
}
#[doc = "Field `TAF` reader - Transmit ACK Flag"]
pub type TAF_R = crate::BitReader<TAF_A>;
#[doc = "Transmit ACK Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAF_A {
    #[doc = "0: Transmit ACK/NACK is not required"]
    NOT_REQUIRED = 0,
    #[doc = "1: Transmit ACK/NACK is required"]
    REQUIRED = 1,
}
impl From<TAF_A> for bool {
    #[inline(always)]
    fn from(variant: TAF_A) -> Self {
        variant as u8 != 0
    }
}
impl TAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAF_A {
        match self.bits {
            false => TAF_A::NOT_REQUIRED,
            true => TAF_A::REQUIRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_REQUIRED`"]
    #[inline(always)]
    pub fn is_not_required(&self) -> bool {
        *self == TAF_A::NOT_REQUIRED
    }
    #[doc = "Checks if the value of the field is `REQUIRED`"]
    #[inline(always)]
    pub fn is_required(&self) -> bool {
        *self == TAF_A::REQUIRED
    }
}
#[doc = "Field `RSF` reader - Repeated Start Flag"]
pub type RSF_R = crate::BitReader<RSF_A>;
#[doc = "Repeated Start Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSF_A {
    #[doc = "0: Slave has not detected a Repeated START condition"]
    NO_FLAG = 0,
    #[doc = "1: Slave has detected a Repeated START condition"]
    FLAG = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
impl RSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::NO_FLAG,
            true => RSF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == RSF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == RSF_A::FLAG
    }
}
#[doc = "Field `RSF` writer - Repeated Start Flag"]
pub type RSF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SSR_SPEC, RSF_A, O>;
impl<'a, const O: u8> RSF_W<'a, O> {
    #[doc = "Slave has not detected a Repeated START condition"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(RSF_A::NO_FLAG)
    }
    #[doc = "Slave has detected a Repeated START condition"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(RSF_A::FLAG)
    }
}
#[doc = "Field `SDF` reader - STOP Detect Flag"]
pub type SDF_R = crate::BitReader<SDF_A>;
#[doc = "STOP Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDF_A {
    #[doc = "0: Slave has not detected a STOP condition"]
    NO_FLAG = 0,
    #[doc = "1: Slave has detected a STOP condition"]
    FLAG = 1,
}
impl From<SDF_A> for bool {
    #[inline(always)]
    fn from(variant: SDF_A) -> Self {
        variant as u8 != 0
    }
}
impl SDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDF_A {
        match self.bits {
            false => SDF_A::NO_FLAG,
            true => SDF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == SDF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == SDF_A::FLAG
    }
}
#[doc = "Field `SDF` writer - STOP Detect Flag"]
pub type SDF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SSR_SPEC, SDF_A, O>;
impl<'a, const O: u8> SDF_W<'a, O> {
    #[doc = "Slave has not detected a STOP condition"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(SDF_A::NO_FLAG)
    }
    #[doc = "Slave has detected a STOP condition"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(SDF_A::FLAG)
    }
}
#[doc = "Field `BEF` reader - Bit Error Flag"]
pub type BEF_R = crate::BitReader<BEF_A>;
#[doc = "Bit Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEF_A {
    #[doc = "0: Slave has not detected a bit error"]
    NO_FLAG = 0,
    #[doc = "1: Slave has detected a bit error"]
    FLAG = 1,
}
impl From<BEF_A> for bool {
    #[inline(always)]
    fn from(variant: BEF_A) -> Self {
        variant as u8 != 0
    }
}
impl BEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEF_A {
        match self.bits {
            false => BEF_A::NO_FLAG,
            true => BEF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == BEF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == BEF_A::FLAG
    }
}
#[doc = "Field `BEF` writer - Bit Error Flag"]
pub type BEF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SSR_SPEC, BEF_A, O>;
impl<'a, const O: u8> BEF_W<'a, O> {
    #[doc = "Slave has not detected a bit error"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(BEF_A::NO_FLAG)
    }
    #[doc = "Slave has detected a bit error"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(BEF_A::FLAG)
    }
}
#[doc = "Field `FEF` reader - FIFO Error Flag"]
pub type FEF_R = crate::BitReader<FEF_A>;
#[doc = "FIFO Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEF_A {
    #[doc = "0: FIFO underflow or overflow was not detected"]
    NO_FLAG = 0,
    #[doc = "1: FIFO underflow or overflow was detected"]
    FLAG = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
impl FEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::NO_FLAG,
            true => FEF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FEF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FEF_A::FLAG
    }
}
#[doc = "Field `FEF` writer - FIFO Error Flag"]
pub type FEF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SSR_SPEC, FEF_A, O>;
impl<'a, const O: u8> FEF_W<'a, O> {
    #[doc = "FIFO underflow or overflow was not detected"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FEF_A::NO_FLAG)
    }
    #[doc = "FIFO underflow or overflow was detected"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FEF_A::FLAG)
    }
}
#[doc = "Field `AM0F` reader - Address Match 0 Flag"]
pub type AM0F_R = crate::BitReader<AM0F_A>;
#[doc = "Address Match 0 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AM0F_A {
    #[doc = "0: Have not received an ADDR0 matching address"]
    NO_FLAG = 0,
    #[doc = "1: Have received an ADDR0 matching address"]
    FLAG = 1,
}
impl From<AM0F_A> for bool {
    #[inline(always)]
    fn from(variant: AM0F_A) -> Self {
        variant as u8 != 0
    }
}
impl AM0F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AM0F_A {
        match self.bits {
            false => AM0F_A::NO_FLAG,
            true => AM0F_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == AM0F_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == AM0F_A::FLAG
    }
}
#[doc = "Field `AM1F` reader - Address Match 1 Flag"]
pub type AM1F_R = crate::BitReader<AM1F_A>;
#[doc = "Address Match 1 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AM1F_A {
    #[doc = "0: Have not received an ADDR1 or ADDR0/ADDR1 range matching address"]
    NO_FLAG = 0,
    #[doc = "1: Have received an ADDR1 or ADDR0/ADDR1 range matching address"]
    FLAG = 1,
}
impl From<AM1F_A> for bool {
    #[inline(always)]
    fn from(variant: AM1F_A) -> Self {
        variant as u8 != 0
    }
}
impl AM1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AM1F_A {
        match self.bits {
            false => AM1F_A::NO_FLAG,
            true => AM1F_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == AM1F_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == AM1F_A::FLAG
    }
}
#[doc = "Field `GCF` reader - General Call Flag"]
pub type GCF_R = crate::BitReader<GCF_A>;
#[doc = "General Call Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCF_A {
    #[doc = "0: Slave has not detected the General Call Address or the General Call Address is disabled"]
    NO_FLAG = 0,
    #[doc = "1: Slave has detected the General Call Address"]
    FLAG = 1,
}
impl From<GCF_A> for bool {
    #[inline(always)]
    fn from(variant: GCF_A) -> Self {
        variant as u8 != 0
    }
}
impl GCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCF_A {
        match self.bits {
            false => GCF_A::NO_FLAG,
            true => GCF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == GCF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == GCF_A::FLAG
    }
}
#[doc = "Field `SARF` reader - SMBus Alert Response Flag"]
pub type SARF_R = crate::BitReader<SARF_A>;
#[doc = "SMBus Alert Response Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SARF_A {
    #[doc = "0: SMBus Alert Response is disabled or not detected"]
    NO_FLAG = 0,
    #[doc = "1: SMBus Alert Response is enabled and detected"]
    FLAG = 1,
}
impl From<SARF_A> for bool {
    #[inline(always)]
    fn from(variant: SARF_A) -> Self {
        variant as u8 != 0
    }
}
impl SARF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SARF_A {
        match self.bits {
            false => SARF_A::NO_FLAG,
            true => SARF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == SARF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == SARF_A::FLAG
    }
}
#[doc = "Field `SBF` reader - Slave Busy Flag"]
pub type SBF_R = crate::BitReader<SBF_A>;
#[doc = "Slave Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBF_A {
    #[doc = "0: I2C Slave is idle"]
    IDLE = 0,
    #[doc = "1: I2C Slave is busy"]
    BUSY = 1,
}
impl From<SBF_A> for bool {
    #[inline(always)]
    fn from(variant: SBF_A) -> Self {
        variant as u8 != 0
    }
}
impl SBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBF_A {
        match self.bits {
            false => SBF_A::IDLE,
            true => SBF_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SBF_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SBF_A::BUSY
    }
}
#[doc = "Field `BBF` reader - Bus Busy Flag"]
pub type BBF_R = crate::BitReader<BBF_A>;
#[doc = "Bus Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BBF_A {
    #[doc = "0: I2C Bus is idle"]
    IDLE = 0,
    #[doc = "1: I2C Bus is busy"]
    BUSY = 1,
}
impl From<BBF_A> for bool {
    #[inline(always)]
    fn from(variant: BBF_A) -> Self {
        variant as u8 != 0
    }
}
impl BBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BBF_A {
        match self.bits {
            false => BBF_A::IDLE,
            true => BBF_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BBF_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BBF_A::BUSY
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Data Flag"]
    #[inline(always)]
    pub fn tdf(&self) -> TDF_R {
        TDF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data Flag"]
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Address Valid Flag"]
    #[inline(always)]
    pub fn avf(&self) -> AVF_R {
        AVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit ACK Flag"]
    #[inline(always)]
    pub fn taf(&self) -> TAF_R {
        TAF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Repeated Start Flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline(always)]
    pub fn sdf(&self) -> SDF_R {
        SDF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bit Error Flag"]
    #[inline(always)]
    pub fn bef(&self) -> BEF_R {
        BEF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Address Match 0 Flag"]
    #[inline(always)]
    pub fn am0f(&self) -> AM0F_R {
        AM0F_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Address Match 1 Flag"]
    #[inline(always)]
    pub fn am1f(&self) -> AM1F_R {
        AM1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - General Call Flag"]
    #[inline(always)]
    pub fn gcf(&self) -> GCF_R {
        GCF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBus Alert Response Flag"]
    #[inline(always)]
    pub fn sarf(&self) -> SARF_R {
        SARF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Slave Busy Flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus Busy Flag"]
    #[inline(always)]
    pub fn bbf(&self) -> BBF_R {
        BBF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Repeated Start Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<8> {
        RSF_W::new(self)
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sdf(&mut self) -> SDF_W<9> {
        SDF_W::new(self)
    }
    #[doc = "Bit 10 - Bit Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bef(&mut self) -> BEF_W<10> {
        BEF_W::new(self)
    }
    #[doc = "Bit 11 - FIFO Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FEF_W<11> {
        FEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssr](index.html) module"]
pub struct SSR_SPEC;
impl crate::RegisterSpec for SSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssr::R](R) reader structure"]
impl crate::Readable for SSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssr::W](W) writer structure"]
impl crate::Writable for SSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f00;
}
#[doc = "`reset()` method sets SSR to value 0"]
impl crate::Resettable for SSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
