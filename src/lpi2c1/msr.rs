#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSR` writer"]
pub struct W(crate::W<MSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSR_SPEC>;
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
impl From<crate::W<MSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDF` reader - Transmit Data Flag"]
pub type TDF_R = crate::BitReader<TDF_A>;
#[doc = "Transmit Data Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDF_A {
    #[doc = "0: Transmit data is not requested"]
    DISABLED = 0,
    #[doc = "1: Transmit data is requested"]
    ENABLED = 1,
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
            false => TDF_A::DISABLED,
            true => TDF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDF_A::ENABLED
    }
}
#[doc = "Field `RDF` reader - Receive Data Flag"]
pub type RDF_R = crate::BitReader<RDF_A>;
#[doc = "Receive Data Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDF_A {
    #[doc = "0: Receive Data is not ready"]
    DISABLED = 0,
    #[doc = "1: Receive data is ready"]
    ENABLED = 1,
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
            false => RDF_A::DISABLED,
            true => RDF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDF_A::ENABLED
    }
}
#[doc = "Field `EPF` reader - End Packet Flag"]
pub type EPF_R = crate::BitReader<EPF_A>;
#[doc = "End Packet Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPF_A {
    #[doc = "0: Master has not generated a STOP or Repeated START condition"]
    NO_FLAG = 0,
    #[doc = "1: Master has generated a STOP or Repeated START condition"]
    FLAG = 1,
}
impl From<EPF_A> for bool {
    #[inline(always)]
    fn from(variant: EPF_A) -> Self {
        variant as u8 != 0
    }
}
impl EPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPF_A {
        match self.bits {
            false => EPF_A::NO_FLAG,
            true => EPF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == EPF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == EPF_A::FLAG
    }
}
#[doc = "Field `EPF` writer - End Packet Flag"]
pub type EPF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MSR_SPEC, EPF_A, O>;
impl<'a, const O: u8> EPF_W<'a, O> {
    #[doc = "Master has not generated a STOP or Repeated START condition"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(EPF_A::NO_FLAG)
    }
    #[doc = "Master has generated a STOP or Repeated START condition"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(EPF_A::FLAG)
    }
}
#[doc = "Field `SDF` reader - STOP Detect Flag"]
pub type SDF_R = crate::BitReader<SDF_A>;
#[doc = "STOP Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDF_A {
    #[doc = "0: Master has not generated a STOP condition"]
    NO_FLAG = 0,
    #[doc = "1: Master has generated a STOP condition"]
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
pub type SDF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MSR_SPEC, SDF_A, O>;
impl<'a, const O: u8> SDF_W<'a, O> {
    #[doc = "Master has not generated a STOP condition"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(SDF_A::NO_FLAG)
    }
    #[doc = "Master has generated a STOP condition"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(SDF_A::FLAG)
    }
}
#[doc = "Field `NDF` reader - NACK Detect Flag"]
pub type NDF_R = crate::BitReader<NDF_A>;
#[doc = "NACK Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NDF_A {
    #[doc = "0: Unexpected NACK was not detected"]
    NO_FLAG = 0,
    #[doc = "1: Unexpected NACK was detected"]
    FLAG = 1,
}
impl From<NDF_A> for bool {
    #[inline(always)]
    fn from(variant: NDF_A) -> Self {
        variant as u8 != 0
    }
}
impl NDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDF_A {
        match self.bits {
            false => NDF_A::NO_FLAG,
            true => NDF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == NDF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == NDF_A::FLAG
    }
}
#[doc = "Field `NDF` writer - NACK Detect Flag"]
pub type NDF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MSR_SPEC, NDF_A, O>;
impl<'a, const O: u8> NDF_W<'a, O> {
    #[doc = "Unexpected NACK was not detected"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(NDF_A::NO_FLAG)
    }
    #[doc = "Unexpected NACK was detected"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(NDF_A::FLAG)
    }
}
#[doc = "Field `ALF` reader - Arbitration Lost Flag"]
pub type ALF_R = crate::BitReader<ALF_A>;
#[doc = "Arbitration Lost Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALF_A {
    #[doc = "0: Master has not lost arbitration"]
    NO_FLAG = 0,
    #[doc = "1: Master has lost arbitration"]
    FLAG = 1,
}
impl From<ALF_A> for bool {
    #[inline(always)]
    fn from(variant: ALF_A) -> Self {
        variant as u8 != 0
    }
}
impl ALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALF_A {
        match self.bits {
            false => ALF_A::NO_FLAG,
            true => ALF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == ALF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == ALF_A::FLAG
    }
}
#[doc = "Field `ALF` writer - Arbitration Lost Flag"]
pub type ALF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MSR_SPEC, ALF_A, O>;
impl<'a, const O: u8> ALF_W<'a, O> {
    #[doc = "Master has not lost arbitration"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(ALF_A::NO_FLAG)
    }
    #[doc = "Master has lost arbitration"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(ALF_A::FLAG)
    }
}
#[doc = "Field `FEF` reader - FIFO Error Flag"]
pub type FEF_R = crate::BitReader<FEF_A>;
#[doc = "FIFO Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEF_A {
    #[doc = "0: No error"]
    NO_FLAG = 0,
    #[doc = "1: Master sending or receiving data without a START condition"]
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
pub type FEF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MSR_SPEC, FEF_A, O>;
impl<'a, const O: u8> FEF_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FEF_A::NO_FLAG)
    }
    #[doc = "Master sending or receiving data without a START condition"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FEF_A::FLAG)
    }
}
#[doc = "Field `PLTF` reader - Pin Low Timeout Flag"]
pub type PLTF_R = crate::BitReader<PLTF_A>;
#[doc = "Pin Low Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLTF_A {
    #[doc = "0: Pin low timeout has not occurred or is disabled"]
    NO_FLAG = 0,
    #[doc = "1: Pin low timeout has occurred"]
    FLAG = 1,
}
impl From<PLTF_A> for bool {
    #[inline(always)]
    fn from(variant: PLTF_A) -> Self {
        variant as u8 != 0
    }
}
impl PLTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLTF_A {
        match self.bits {
            false => PLTF_A::NO_FLAG,
            true => PLTF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == PLTF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == PLTF_A::FLAG
    }
}
#[doc = "Field `PLTF` writer - Pin Low Timeout Flag"]
pub type PLTF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MSR_SPEC, PLTF_A, O>;
impl<'a, const O: u8> PLTF_W<'a, O> {
    #[doc = "Pin low timeout has not occurred or is disabled"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(PLTF_A::NO_FLAG)
    }
    #[doc = "Pin low timeout has occurred"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(PLTF_A::FLAG)
    }
}
#[doc = "Field `DMF` reader - Data Match Flag"]
pub type DMF_R = crate::BitReader<DMF_A>;
#[doc = "Data Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMF_A {
    #[doc = "0: Have not received matching data"]
    NO_FLAG = 0,
    #[doc = "1: Have received matching data"]
    FLAG = 1,
}
impl From<DMF_A> for bool {
    #[inline(always)]
    fn from(variant: DMF_A) -> Self {
        variant as u8 != 0
    }
}
impl DMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMF_A {
        match self.bits {
            false => DMF_A::NO_FLAG,
            true => DMF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == DMF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == DMF_A::FLAG
    }
}
#[doc = "Field `DMF` writer - Data Match Flag"]
pub type DMF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MSR_SPEC, DMF_A, O>;
impl<'a, const O: u8> DMF_W<'a, O> {
    #[doc = "Have not received matching data"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(DMF_A::NO_FLAG)
    }
    #[doc = "Have received matching data"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(DMF_A::FLAG)
    }
}
#[doc = "Field `MBF` reader - Master Busy Flag"]
pub type MBF_R = crate::BitReader<MBF_A>;
#[doc = "Master Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBF_A {
    #[doc = "0: I2C Master is idle"]
    IDLE = 0,
    #[doc = "1: I2C Master is busy"]
    BUSY = 1,
}
impl From<MBF_A> for bool {
    #[inline(always)]
    fn from(variant: MBF_A) -> Self {
        variant as u8 != 0
    }
}
impl MBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBF_A {
        match self.bits {
            false => MBF_A::IDLE,
            true => MBF_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == MBF_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == MBF_A::BUSY
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
    #[doc = "Bit 8 - End Packet Flag"]
    #[inline(always)]
    pub fn epf(&self) -> EPF_R {
        EPF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline(always)]
    pub fn sdf(&self) -> SDF_R {
        SDF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NACK Detect Flag"]
    #[inline(always)]
    pub fn ndf(&self) -> NDF_R {
        NDF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Arbitration Lost Flag"]
    #[inline(always)]
    pub fn alf(&self) -> ALF_R {
        ALF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin Low Timeout Flag"]
    #[inline(always)]
    pub fn pltf(&self) -> PLTF_R {
        PLTF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Data Match Flag"]
    #[inline(always)]
    pub fn dmf(&self) -> DMF_R {
        DMF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 24 - Master Busy Flag"]
    #[inline(always)]
    pub fn mbf(&self) -> MBF_R {
        MBF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus Busy Flag"]
    #[inline(always)]
    pub fn bbf(&self) -> BBF_R {
        BBF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - End Packet Flag"]
    #[inline(always)]
    #[must_use]
    pub fn epf(&mut self) -> EPF_W<8> {
        EPF_W::new(self)
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sdf(&mut self) -> SDF_W<9> {
        SDF_W::new(self)
    }
    #[doc = "Bit 10 - NACK Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ndf(&mut self) -> NDF_W<10> {
        NDF_W::new(self)
    }
    #[doc = "Bit 11 - Arbitration Lost Flag"]
    #[inline(always)]
    #[must_use]
    pub fn alf(&mut self) -> ALF_W<11> {
        ALF_W::new(self)
    }
    #[doc = "Bit 12 - FIFO Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FEF_W<12> {
        FEF_W::new(self)
    }
    #[doc = "Bit 13 - Pin Low Timeout Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pltf(&mut self) -> PLTF_W<13> {
        PLTF_W::new(self)
    }
    #[doc = "Bit 14 - Data Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmf(&mut self) -> DMF_W<14> {
        DMF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msr::W](W) writer structure"]
impl crate::Writable for MSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x7f00;
}
#[doc = "`reset()` method sets MSR to value 0x01"]
impl crate::Resettable for MSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
