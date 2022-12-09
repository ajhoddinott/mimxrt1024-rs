#[doc = "Register `SIS` reader"]
pub struct R(crate::R<SIC_SIS_SIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIC_SIS_SIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIC_SIS_SIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIC_SIS_SIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RxFIFOFul` reader - SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
pub type RX_FIFOFUL_R = crate::BitReader<bool>;
#[doc = "Field `TxEm` reader - SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
pub type TX_EM_R = crate::BitReader<bool>;
#[doc = "Field `LockLoss` reader - SPDIF receiver loss of lock"]
pub type LOCK_LOSS_R = crate::BitReader<bool>;
#[doc = "Field `RxFIFOResyn` reader - Rx FIFO resync"]
pub type RX_FIFORESYN_R = crate::BitReader<bool>;
#[doc = "Field `RxFIFOUnOv` reader - Rx FIFO underrun/overrun"]
pub type RX_FIFOUN_OV_R = crate::BitReader<bool>;
#[doc = "Field `UQErr` reader - U/Q Channel framing error"]
pub type UQERR_R = crate::BitReader<bool>;
#[doc = "Field `UQSync` reader - U/Q Channel sync found"]
pub type UQSYNC_R = crate::BitReader<bool>;
#[doc = "Field `QRxOv` reader - Q Channel receive register overrun"]
pub type QRX_OV_R = crate::BitReader<bool>;
#[doc = "Field `QRxFul` reader - Q Channel receive register full, can't be cleared with reg"]
pub type QRX_FUL_R = crate::BitReader<bool>;
#[doc = "Field `URxOv` reader - U Channel receive register overrun"]
pub type URX_OV_R = crate::BitReader<bool>;
#[doc = "Field `URxFul` reader - U Channel receive register full, can't be cleared with reg"]
pub type URX_FUL_R = crate::BitReader<bool>;
#[doc = "Field `BitErr` reader - SPDIF receiver found parity bit error"]
pub type BIT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `SymErr` reader - SPDIF receiver found illegal symbol"]
pub type SYM_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ValNoGood` reader - SPDIF validity flag no good"]
pub type VAL_NO_GOOD_R = crate::BitReader<bool>;
#[doc = "Field `CNew` reader - SPDIF receive change in value of control channel"]
pub type CNEW_R = crate::BitReader<bool>;
#[doc = "Field `TxResyn` reader - SPDIF Tx FIFO resync"]
pub type TX_RESYN_R = crate::BitReader<bool>;
#[doc = "Field `TxUnOv` reader - SPDIF Tx FIFO under/overrun"]
pub type TX_UN_OV_R = crate::BitReader<bool>;
#[doc = "Field `Lock` reader - SPDIF receiver's DPLL is locked"]
pub type LOCK_R = crate::BitReader<bool>;
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
#[doc = "InterruptStat Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sic_sis_sis](index.html) module"]
pub struct SIC_SIS_SIS_SPEC;
impl crate::RegisterSpec for SIC_SIS_SIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sic_sis_sis::R](R) reader structure"]
impl crate::Readable for SIC_SIS_SIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIS to value 0x02"]
impl crate::Resettable for SIC_SIS_SIS_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
