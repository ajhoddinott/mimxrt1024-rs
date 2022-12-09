#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDF` reader - Transmit Data Flag"]
pub type TDF_R = crate::BitReader<TDF_A>;
#[doc = "Transmit Data Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDF_A {
    #[doc = "0: Transmit data not requested"]
    TXDATA_NOT_REQST = 0,
    #[doc = "1: Transmit data is requested"]
    TXDATA_REQST = 1,
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
            false => TDF_A::TXDATA_NOT_REQST,
            true => TDF_A::TXDATA_REQST,
        }
    }
    #[doc = "Checks if the value of the field is `TXDATA_NOT_REQST`"]
    #[inline(always)]
    pub fn is_txdata_not_reqst(&self) -> bool {
        *self == TDF_A::TXDATA_NOT_REQST
    }
    #[doc = "Checks if the value of the field is `TXDATA_REQST`"]
    #[inline(always)]
    pub fn is_txdata_reqst(&self) -> bool {
        *self == TDF_A::TXDATA_REQST
    }
}
#[doc = "Field `RDF` reader - Receive Data Flag"]
pub type RDF_R = crate::BitReader<RDF_A>;
#[doc = "Receive Data Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDF_A {
    #[doc = "0: Receive Data is not ready"]
    NOTREADY = 0,
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
            false => RDF_A::NOTREADY,
            true => RDF_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == RDF_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RDF_A::READY
    }
}
#[doc = "Field `WCF` reader - Word Complete Flag"]
pub type WCF_R = crate::BitReader<WCF_A>;
#[doc = "Word Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WCF_A {
    #[doc = "0: Transfer of a received word has not yet completed"]
    NOT_COMPLETED = 0,
    #[doc = "1: Transfer of a received word has completed"]
    COMPLETED = 1,
}
impl From<WCF_A> for bool {
    #[inline(always)]
    fn from(variant: WCF_A) -> Self {
        variant as u8 != 0
    }
}
impl WCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCF_A {
        match self.bits {
            false => WCF_A::NOT_COMPLETED,
            true => WCF_A::COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETED`"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == WCF_A::NOT_COMPLETED
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == WCF_A::COMPLETED
    }
}
#[doc = "Field `WCF` writer - Word Complete Flag"]
pub type WCF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, WCF_A, O>;
impl<'a, const O: u8> WCF_W<'a, O> {
    #[doc = "Transfer of a received word has not yet completed"]
    #[inline(always)]
    pub fn not_completed(self) -> &'a mut W {
        self.variant(WCF_A::NOT_COMPLETED)
    }
    #[doc = "Transfer of a received word has completed"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut W {
        self.variant(WCF_A::COMPLETED)
    }
}
#[doc = "Field `FCF` reader - Frame Complete Flag"]
pub type FCF_R = crate::BitReader<FCF_A>;
#[doc = "Frame Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCF_A {
    #[doc = "0: Frame transfer has not completed"]
    NOT_COMPLETED = 0,
    #[doc = "1: Frame transfer has completed"]
    COMPLETED = 1,
}
impl From<FCF_A> for bool {
    #[inline(always)]
    fn from(variant: FCF_A) -> Self {
        variant as u8 != 0
    }
}
impl FCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCF_A {
        match self.bits {
            false => FCF_A::NOT_COMPLETED,
            true => FCF_A::COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETED`"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == FCF_A::NOT_COMPLETED
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == FCF_A::COMPLETED
    }
}
#[doc = "Field `FCF` writer - Frame Complete Flag"]
pub type FCF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, FCF_A, O>;
impl<'a, const O: u8> FCF_W<'a, O> {
    #[doc = "Frame transfer has not completed"]
    #[inline(always)]
    pub fn not_completed(self) -> &'a mut W {
        self.variant(FCF_A::NOT_COMPLETED)
    }
    #[doc = "Frame transfer has completed"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut W {
        self.variant(FCF_A::COMPLETED)
    }
}
#[doc = "Field `TCF` reader - Transfer Complete Flag"]
pub type TCF_R = crate::BitReader<TCF_A>;
#[doc = "Transfer Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF_A {
    #[doc = "0: All transfers have not completed"]
    NOT_COMPLETED = 0,
    #[doc = "1: All transfers have completed"]
    COMPLETED = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::NOT_COMPLETED,
            true => TCF_A::COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETED`"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == TCF_A::NOT_COMPLETED
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TCF_A::COMPLETED
    }
}
#[doc = "Field `TCF` writer - Transfer Complete Flag"]
pub type TCF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, TCF_A, O>;
impl<'a, const O: u8> TCF_W<'a, O> {
    #[doc = "All transfers have not completed"]
    #[inline(always)]
    pub fn not_completed(self) -> &'a mut W {
        self.variant(TCF_A::NOT_COMPLETED)
    }
    #[doc = "All transfers have completed"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut W {
        self.variant(TCF_A::COMPLETED)
    }
}
#[doc = "Field `TEF` reader - Transmit Error Flag"]
pub type TEF_R = crate::BitReader<TEF_A>;
#[doc = "Transmit Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEF_A {
    #[doc = "0: Transmit FIFO underrun has not occurred"]
    NO_UNDERRUN = 0,
    #[doc = "1: Transmit FIFO underrun has occurred"]
    UNDERRUN = 1,
}
impl From<TEF_A> for bool {
    #[inline(always)]
    fn from(variant: TEF_A) -> Self {
        variant as u8 != 0
    }
}
impl TEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEF_A {
        match self.bits {
            false => TEF_A::NO_UNDERRUN,
            true => TEF_A::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_UNDERRUN`"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == TEF_A::NO_UNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == TEF_A::UNDERRUN
    }
}
#[doc = "Field `TEF` writer - Transmit Error Flag"]
pub type TEF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, TEF_A, O>;
impl<'a, const O: u8> TEF_W<'a, O> {
    #[doc = "Transmit FIFO underrun has not occurred"]
    #[inline(always)]
    pub fn no_underrun(self) -> &'a mut W {
        self.variant(TEF_A::NO_UNDERRUN)
    }
    #[doc = "Transmit FIFO underrun has occurred"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut W {
        self.variant(TEF_A::UNDERRUN)
    }
}
#[doc = "Field `REF` reader - Receive Error Flag"]
pub type REF_R = crate::BitReader<REF_A>;
#[doc = "Receive Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REF_A {
    #[doc = "0: Receive FIFO has not overflowed"]
    NOT_OVERFLOWED = 0,
    #[doc = "1: Receive FIFO has overflowed"]
    OVERFLOWED = 1,
}
impl From<REF_A> for bool {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        variant as u8 != 0
    }
}
impl REF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_A {
        match self.bits {
            false => REF_A::NOT_OVERFLOWED,
            true => REF_A::OVERFLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OVERFLOWED`"]
    #[inline(always)]
    pub fn is_not_overflowed(&self) -> bool {
        *self == REF_A::NOT_OVERFLOWED
    }
    #[doc = "Checks if the value of the field is `OVERFLOWED`"]
    #[inline(always)]
    pub fn is_overflowed(&self) -> bool {
        *self == REF_A::OVERFLOWED
    }
}
#[doc = "Field `REF` writer - Receive Error Flag"]
pub type REF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, REF_A, O>;
impl<'a, const O: u8> REF_W<'a, O> {
    #[doc = "Receive FIFO has not overflowed"]
    #[inline(always)]
    pub fn not_overflowed(self) -> &'a mut W {
        self.variant(REF_A::NOT_OVERFLOWED)
    }
    #[doc = "Receive FIFO has overflowed"]
    #[inline(always)]
    pub fn overflowed(self) -> &'a mut W {
        self.variant(REF_A::OVERFLOWED)
    }
}
#[doc = "Field `DMF` reader - Data Match Flag"]
pub type DMF_R = crate::BitReader<DMF_A>;
#[doc = "Data Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMF_A {
    #[doc = "0: Have not received matching data"]
    NO_MATCH = 0,
    #[doc = "1: Have received matching data"]
    MATCH = 1,
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
            false => DMF_A::NO_MATCH,
            true => DMF_A::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MATCH`"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == DMF_A::NO_MATCH
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == DMF_A::MATCH
    }
}
#[doc = "Field `DMF` writer - Data Match Flag"]
pub type DMF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, DMF_A, O>;
impl<'a, const O: u8> DMF_W<'a, O> {
    #[doc = "Have not received matching data"]
    #[inline(always)]
    pub fn no_match(self) -> &'a mut W {
        self.variant(DMF_A::NO_MATCH)
    }
    #[doc = "Have received matching data"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(DMF_A::MATCH)
    }
}
#[doc = "Field `MBF` reader - Module Busy Flag"]
pub type MBF_R = crate::BitReader<MBF_A>;
#[doc = "Module Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBF_A {
    #[doc = "0: LPSPI is idle"]
    IDLE = 0,
    #[doc = "1: LPSPI is busy"]
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
    #[doc = "Bit 8 - Word Complete Flag"]
    #[inline(always)]
    pub fn wcf(&self) -> WCF_R {
        WCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Complete Flag"]
    #[inline(always)]
    pub fn fcf(&self) -> FCF_R {
        FCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Error Flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive Error Flag"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data Match Flag"]
    #[inline(always)]
    pub fn dmf(&self) -> DMF_R {
        DMF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - Module Busy Flag"]
    #[inline(always)]
    pub fn mbf(&self) -> MBF_R {
        MBF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Word Complete Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wcf(&mut self) -> WCF_W<8> {
        WCF_W::new(self)
    }
    #[doc = "Bit 9 - Frame Complete Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fcf(&mut self) -> FCF_W<9> {
        FCF_W::new(self)
    }
    #[doc = "Bit 10 - Transfer Complete Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<10> {
        TCF_W::new(self)
    }
    #[doc = "Bit 11 - Transmit Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tef(&mut self) -> TEF_W<11> {
        TEF_W::new(self)
    }
    #[doc = "Bit 12 - Receive Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ref_(&mut self) -> REF_W<12> {
        REF_W::new(self)
    }
    #[doc = "Bit 13 - Data Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmf(&mut self) -> DMF_W<13> {
        DMF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x3f00;
}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
