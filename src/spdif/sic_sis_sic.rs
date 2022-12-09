#[doc = "Register `SIC` writer"]
pub struct W(crate::W<SIC_SIS_SIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIC_SIS_SIC_SPEC>;
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
impl From<crate::W<SIC_SIS_SIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIC_SIS_SIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LockLoss` writer - SPDIF receiver loss of lock"]
pub type LOCK_LOSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `RxFIFOResyn` writer - Rx FIFO resync"]
pub type RX_FIFORESYN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `RxFIFOUnOv` writer - Rx FIFO underrun/overrun"]
pub type RX_FIFOUN_OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `UQErr` writer - U/Q Channel framing error"]
pub type UQERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `UQSync` writer - U/Q Channel sync found"]
pub type UQSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `QRxOv` writer - Q Channel receive register overrun"]
pub type QRX_OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `URxOv` writer - U Channel receive register overrun"]
pub type URX_OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `BitErr` writer - SPDIF receiver found parity bit error"]
pub type BIT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `SymErr` writer - SPDIF receiver found illegal symbol"]
pub type SYM_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `ValNoGood` writer - SPDIF validity flag no good"]
pub type VAL_NO_GOOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `CNew` writer - SPDIF receive change in value of control channel"]
pub type CNEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `TxResyn` writer - SPDIF Tx FIFO resync"]
pub type TX_RESYN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `TxUnOv` writer - SPDIF Tx FIFO under/overrun"]
pub type TX_UN_OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
#[doc = "Field `Lock` writer - SPDIF receiver's DPLL is locked"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIC_SIS_SIC_SPEC, bool, O>;
impl W {
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
    #[doc = "Bit 9 - U Channel receive register overrun"]
    #[inline(always)]
    #[must_use]
    pub fn urx_ov(&mut self) -> URX_OV_W<9> {
        URX_OV_W::new(self)
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
#[doc = "InterruptClear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sic_sis_sic](index.html) module"]
pub struct SIC_SIS_SIC_SPEC;
impl crate::RegisterSpec for SIC_SIS_SIC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sic_sis_sic::W](W) writer structure"]
impl crate::Writable for SIC_SIS_SIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIC to value 0"]
impl crate::Resettable for SIC_SIS_SIC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
