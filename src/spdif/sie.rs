#[doc = "Register `SIE` reader"]
pub struct R(crate::R<SIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIE` writer"]
pub struct W(crate::W<SIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIE_SPEC>;
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
impl From<crate::W<SIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RxFIFOFul` reader - SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
pub type RX_FIFOFUL_R = crate::BitReader<bool>;
#[doc = "Field `RxFIFOFul` writer - SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
pub type RX_FIFOFUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `TxEm` reader - SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
pub type TX_EM_R = crate::BitReader<bool>;
#[doc = "Field `TxEm` writer - SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
pub type TX_EM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `LockLoss` reader - SPDIF receiver loss of lock"]
pub type LOCK_LOSS_R = crate::BitReader<bool>;
#[doc = "Field `LockLoss` writer - SPDIF receiver loss of lock"]
pub type LOCK_LOSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `RxFIFOResyn` reader - Rx FIFO resync"]
pub type RX_FIFORESYN_R = crate::BitReader<bool>;
#[doc = "Field `RxFIFOResyn` writer - Rx FIFO resync"]
pub type RX_FIFORESYN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `RxFIFOUnOv` reader - Rx FIFO underrun/overrun"]
pub type RX_FIFOUN_OV_R = crate::BitReader<bool>;
#[doc = "Field `RxFIFOUnOv` writer - Rx FIFO underrun/overrun"]
pub type RX_FIFOUN_OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `UQErr` reader - U/Q Channel framing error"]
pub type UQERR_R = crate::BitReader<bool>;
#[doc = "Field `UQErr` writer - U/Q Channel framing error"]
pub type UQERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `UQSync` reader - U/Q Channel sync found"]
pub type UQSYNC_R = crate::BitReader<bool>;
#[doc = "Field `UQSync` writer - U/Q Channel sync found"]
pub type UQSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `QRxOv` reader - Q Channel receive register overrun"]
pub type QRX_OV_R = crate::BitReader<bool>;
#[doc = "Field `QRxOv` writer - Q Channel receive register overrun"]
pub type QRX_OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `QRxFul` reader - Q Channel receive register full, can't be cleared with reg"]
pub type QRX_FUL_R = crate::BitReader<bool>;
#[doc = "Field `QRxFul` writer - Q Channel receive register full, can't be cleared with reg"]
pub type QRX_FUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `URxOv` reader - U Channel receive register overrun"]
pub type URX_OV_R = crate::BitReader<bool>;
#[doc = "Field `URxOv` writer - U Channel receive register overrun"]
pub type URX_OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `URxFul` reader - U Channel receive register full, can't be cleared with reg"]
pub type URX_FUL_R = crate::BitReader<bool>;
#[doc = "Field `URxFul` writer - U Channel receive register full, can't be cleared with reg"]
pub type URX_FUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `BitErr` reader - SPDIF receiver found parity bit error"]
pub type BIT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `BitErr` writer - SPDIF receiver found parity bit error"]
pub type BIT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `SymErr` reader - SPDIF receiver found illegal symbol"]
pub type SYM_ERR_R = crate::BitReader<bool>;
#[doc = "Field `SymErr` writer - SPDIF receiver found illegal symbol"]
pub type SYM_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `ValNoGood` reader - SPDIF validity flag no good"]
pub type VAL_NO_GOOD_R = crate::BitReader<bool>;
#[doc = "Field `ValNoGood` writer - SPDIF validity flag no good"]
pub type VAL_NO_GOOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `CNew` reader - SPDIF receive change in value of control channel"]
pub type CNEW_R = crate::BitReader<bool>;
#[doc = "Field `CNew` writer - SPDIF receive change in value of control channel"]
pub type CNEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `TxResyn` reader - SPDIF Tx FIFO resync"]
pub type TX_RESYN_R = crate::BitReader<bool>;
#[doc = "Field `TxResyn` writer - SPDIF Tx FIFO resync"]
pub type TX_RESYN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `TxUnOv` reader - SPDIF Tx FIFO under/overrun"]
pub type TX_UN_OV_R = crate::BitReader<bool>;
#[doc = "Field `TxUnOv` writer - SPDIF Tx FIFO under/overrun"]
pub type TX_UN_OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
#[doc = "Field `Lock` reader - SPDIF receiver's DPLL is locked"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `Lock` writer - SPDIF receiver's DPLL is locked"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
    #[inline(always)]
    pub fn rx_fifoful(&self) -> RX_FIFOFUL_R {
        RX_FIFOFUL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
    #[inline(always)]
    pub fn tx_em(&self) -> TX_EM_R {
        TX_EM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPDIF receiver loss of lock"]
    #[inline(always)]
    pub fn lock_loss(&self) -> LOCK_LOSS_R {
        LOCK_LOSS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO resync"]
    #[inline(always)]
    pub fn rx_fiforesyn(&self) -> RX_FIFORESYN_R {
        RX_FIFORESYN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO underrun/overrun"]
    #[inline(always)]
    pub fn rx_fifoun_ov(&self) -> RX_FIFOUN_OV_R {
        RX_FIFOUN_OV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - U/Q Channel framing error"]
    #[inline(always)]
    pub fn uqerr(&self) -> UQERR_R {
        UQERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - U/Q Channel sync found"]
    #[inline(always)]
    pub fn uqsync(&self) -> UQSYNC_R {
        UQSYNC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Q Channel receive register overrun"]
    #[inline(always)]
    pub fn qrx_ov(&self) -> QRX_OV_R {
        QRX_OV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Q Channel receive register full, can't be cleared with reg"]
    #[inline(always)]
    pub fn qrx_ful(&self) -> QRX_FUL_R {
        QRX_FUL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - U Channel receive register overrun"]
    #[inline(always)]
    pub fn urx_ov(&self) -> URX_OV_R {
        URX_OV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - U Channel receive register full, can't be cleared with reg"]
    #[inline(always)]
    pub fn urx_ful(&self) -> URX_FUL_R {
        URX_FUL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - SPDIF receiver found parity bit error"]
    #[inline(always)]
    pub fn bit_err(&self) -> BIT_ERR_R {
        BIT_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPDIF receiver found illegal symbol"]
    #[inline(always)]
    pub fn sym_err(&self) -> SYM_ERR_R {
        SYM_ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPDIF validity flag no good"]
    #[inline(always)]
    pub fn val_no_good(&self) -> VAL_NO_GOOD_R {
        VAL_NO_GOOD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPDIF receive change in value of control channel"]
    #[inline(always)]
    pub fn cnew(&self) -> CNEW_R {
        CNEW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SPDIF Tx FIFO resync"]
    #[inline(always)]
    pub fn tx_resyn(&self) -> TX_RESYN_R {
        TX_RESYN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SPDIF Tx FIFO under/overrun"]
    #[inline(always)]
    pub fn tx_un_ov(&self) -> TX_UN_OV_R {
        TX_UN_OV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SPDIF receiver's DPLL is locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifoful(&mut self) -> RX_FIFOFUL_W<0> {
        RX_FIFOFUL_W::new(self)
    }
    #[doc = "Bit 1 - SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx_em(&mut self) -> TX_EM_W<1> {
        TX_EM_W::new(self)
    }
    #[doc = "Bit 2 - SPDIF receiver loss of lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock_loss(&mut self) -> LOCK_LOSS_W<2> {
        LOCK_LOSS_W::new(self)
    }
    #[doc = "Bit 3 - Rx FIFO resync"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fiforesyn(&mut self) -> RX_FIFORESYN_W<3> {
        RX_FIFORESYN_W::new(self)
    }
    #[doc = "Bit 4 - Rx FIFO underrun/overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifoun_ov(&mut self) -> RX_FIFOUN_OV_W<4> {
        RX_FIFOUN_OV_W::new(self)
    }
    #[doc = "Bit 5 - U/Q Channel framing error"]
    #[inline(always)]
    #[must_use]
    pub fn uqerr(&mut self) -> UQERR_W<5> {
        UQERR_W::new(self)
    }
    #[doc = "Bit 6 - U/Q Channel sync found"]
    #[inline(always)]
    #[must_use]
    pub fn uqsync(&mut self) -> UQSYNC_W<6> {
        UQSYNC_W::new(self)
    }
    #[doc = "Bit 7 - Q Channel receive register overrun"]
    #[inline(always)]
    #[must_use]
    pub fn qrx_ov(&mut self) -> QRX_OV_W<7> {
        QRX_OV_W::new(self)
    }
    #[doc = "Bit 8 - Q Channel receive register full, can't be cleared with reg"]
    #[inline(always)]
    #[must_use]
    pub fn qrx_ful(&mut self) -> QRX_FUL_W<8> {
        QRX_FUL_W::new(self)
    }
    #[doc = "Bit 9 - U Channel receive register overrun"]
    #[inline(always)]
    #[must_use]
    pub fn urx_ov(&mut self) -> URX_OV_W<9> {
        URX_OV_W::new(self)
    }
    #[doc = "Bit 10 - U Channel receive register full, can't be cleared with reg"]
    #[inline(always)]
    #[must_use]
    pub fn urx_ful(&mut self) -> URX_FUL_W<10> {
        URX_FUL_W::new(self)
    }
    #[doc = "Bit 14 - SPDIF receiver found parity bit error"]
    #[inline(always)]
    #[must_use]
    pub fn bit_err(&mut self) -> BIT_ERR_W<14> {
        BIT_ERR_W::new(self)
    }
    #[doc = "Bit 15 - SPDIF receiver found illegal symbol"]
    #[inline(always)]
    #[must_use]
    pub fn sym_err(&mut self) -> SYM_ERR_W<15> {
        SYM_ERR_W::new(self)
    }
    #[doc = "Bit 16 - SPDIF validity flag no good"]
    #[inline(always)]
    #[must_use]
    pub fn val_no_good(&mut self) -> VAL_NO_GOOD_W<16> {
        VAL_NO_GOOD_W::new(self)
    }
    #[doc = "Bit 17 - SPDIF receive change in value of control channel"]
    #[inline(always)]
    #[must_use]
    pub fn cnew(&mut self) -> CNEW_W<17> {
        CNEW_W::new(self)
    }
    #[doc = "Bit 18 - SPDIF Tx FIFO resync"]
    #[inline(always)]
    #[must_use]
    pub fn tx_resyn(&mut self) -> TX_RESYN_W<18> {
        TX_RESYN_W::new(self)
    }
    #[doc = "Bit 19 - SPDIF Tx FIFO under/overrun"]
    #[inline(always)]
    #[must_use]
    pub fn tx_un_ov(&mut self) -> TX_UN_OV_W<19> {
        TX_UN_OV_W::new(self)
    }
    #[doc = "Bit 20 - SPDIF receiver's DPLL is locked"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<20> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "InterruptEn Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie](index.html) module"]
pub struct SIE_SPEC;
impl crate::RegisterSpec for SIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sie::R](R) reader structure"]
impl crate::Readable for SIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sie::W](W) writer structure"]
impl crate::Writable for SIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIE to value 0"]
impl crate::Resettable for SIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
