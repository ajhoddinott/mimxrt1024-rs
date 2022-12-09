#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USrc_Sel` reader - no description available"]
pub type USRC_SEL_R = crate::FieldReader<u8, USRC_SEL_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USRC_SEL_A {
    #[doc = "0: No embedded U channel"]
    USRC_SEL_0 = 0,
    #[doc = "1: U channel from SPDIF receive block (CD mode)"]
    USRC_SEL_1 = 1,
    #[doc = "3: U channel from on chip transmitter"]
    USRC_SEL_3 = 3,
}
impl From<USRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USRC_SEL_A) -> Self {
        variant as _
    }
}
impl USRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USRC_SEL_A> {
        match self.bits {
            0 => Some(USRC_SEL_A::USRC_SEL_0),
            1 => Some(USRC_SEL_A::USRC_SEL_1),
            3 => Some(USRC_SEL_A::USRC_SEL_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USRC_SEL_0`"]
    #[inline(always)]
    pub fn is_usrc_sel_0(&self) -> bool {
        *self == USRC_SEL_A::USRC_SEL_0
    }
    #[doc = "Checks if the value of the field is `USRC_SEL_1`"]
    #[inline(always)]
    pub fn is_usrc_sel_1(&self) -> bool {
        *self == USRC_SEL_A::USRC_SEL_1
    }
    #[doc = "Checks if the value of the field is `USRC_SEL_3`"]
    #[inline(always)]
    pub fn is_usrc_sel_3(&self) -> bool {
        *self == USRC_SEL_A::USRC_SEL_3
    }
}
#[doc = "Field `USrc_Sel` writer - no description available"]
pub type USRC_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCR_SPEC, u8, USRC_SEL_A, 2, O>;
impl<'a, const O: u8> USRC_SEL_W<'a, O> {
    #[doc = "No embedded U channel"]
    #[inline(always)]
    pub fn usrc_sel_0(self) -> &'a mut W {
        self.variant(USRC_SEL_A::USRC_SEL_0)
    }
    #[doc = "U channel from SPDIF receive block (CD mode)"]
    #[inline(always)]
    pub fn usrc_sel_1(self) -> &'a mut W {
        self.variant(USRC_SEL_A::USRC_SEL_1)
    }
    #[doc = "U channel from on chip transmitter"]
    #[inline(always)]
    pub fn usrc_sel_3(self) -> &'a mut W {
        self.variant(USRC_SEL_A::USRC_SEL_3)
    }
}
#[doc = "Field `TxSel` reader - no description available"]
pub type TX_SEL_R = crate::FieldReader<u8, TX_SEL_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_SEL_A {
    #[doc = "0: Off and output 0"]
    TX_SEL_0 = 0,
    #[doc = "1: Feed-through SPDIFIN"]
    TX_SEL_1 = 1,
    #[doc = "5: Tx Normal operation"]
    TX_SEL_5 = 5,
}
impl From<TX_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_SEL_A) -> Self {
        variant as _
    }
}
impl TX_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_SEL_A> {
        match self.bits {
            0 => Some(TX_SEL_A::TX_SEL_0),
            1 => Some(TX_SEL_A::TX_SEL_1),
            5 => Some(TX_SEL_A::TX_SEL_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TX_SEL_0`"]
    #[inline(always)]
    pub fn is_tx_sel_0(&self) -> bool {
        *self == TX_SEL_A::TX_SEL_0
    }
    #[doc = "Checks if the value of the field is `TX_SEL_1`"]
    #[inline(always)]
    pub fn is_tx_sel_1(&self) -> bool {
        *self == TX_SEL_A::TX_SEL_1
    }
    #[doc = "Checks if the value of the field is `TX_SEL_5`"]
    #[inline(always)]
    pub fn is_tx_sel_5(&self) -> bool {
        *self == TX_SEL_A::TX_SEL_5
    }
}
#[doc = "Field `TxSel` writer - no description available"]
pub type TX_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCR_SPEC, u8, TX_SEL_A, 3, O>;
impl<'a, const O: u8> TX_SEL_W<'a, O> {
    #[doc = "Off and output 0"]
    #[inline(always)]
    pub fn tx_sel_0(self) -> &'a mut W {
        self.variant(TX_SEL_A::TX_SEL_0)
    }
    #[doc = "Feed-through SPDIFIN"]
    #[inline(always)]
    pub fn tx_sel_1(self) -> &'a mut W {
        self.variant(TX_SEL_A::TX_SEL_1)
    }
    #[doc = "Tx Normal operation"]
    #[inline(always)]
    pub fn tx_sel_5(self) -> &'a mut W {
        self.variant(TX_SEL_A::TX_SEL_5)
    }
}
#[doc = "Field `ValCtrl` reader - no description available"]
pub type VAL_CTRL_R = crate::BitReader<VAL_CTRL_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VAL_CTRL_A {
    #[doc = "0: Outgoing Validity always set"]
    VAL_CTRL_0 = 0,
    #[doc = "1: Outgoing Validity always clear"]
    VAL_CTRL_1 = 1,
}
impl From<VAL_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: VAL_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl VAL_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VAL_CTRL_A {
        match self.bits {
            false => VAL_CTRL_A::VAL_CTRL_0,
            true => VAL_CTRL_A::VAL_CTRL_1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL_CTRL_0`"]
    #[inline(always)]
    pub fn is_val_ctrl_0(&self) -> bool {
        *self == VAL_CTRL_A::VAL_CTRL_0
    }
    #[doc = "Checks if the value of the field is `VAL_CTRL_1`"]
    #[inline(always)]
    pub fn is_val_ctrl_1(&self) -> bool {
        *self == VAL_CTRL_A::VAL_CTRL_1
    }
}
#[doc = "Field `ValCtrl` writer - no description available"]
pub type VAL_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, VAL_CTRL_A, O>;
impl<'a, const O: u8> VAL_CTRL_W<'a, O> {
    #[doc = "Outgoing Validity always set"]
    #[inline(always)]
    pub fn val_ctrl_0(self) -> &'a mut W {
        self.variant(VAL_CTRL_A::VAL_CTRL_0)
    }
    #[doc = "Outgoing Validity always clear"]
    #[inline(always)]
    pub fn val_ctrl_1(self) -> &'a mut W {
        self.variant(VAL_CTRL_A::VAL_CTRL_1)
    }
}
#[doc = "Field `DMA_TX_En` reader - DMA Transmit Request Enable (Tx FIFO empty)"]
pub type DMA_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_TX_En` writer - DMA Transmit Request Enable (Tx FIFO empty)"]
pub type DMA_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `DMA_Rx_En` reader - DMA Receive Request Enable (RX FIFO full)"]
pub type DMA_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_Rx_En` writer - DMA Receive Request Enable (RX FIFO full)"]
pub type DMA_RX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `TxFIFO_Ctrl` reader - no description available"]
pub type TX_FIFO_CTRL_R = crate::FieldReader<u8, TX_FIFO_CTRL_A>;
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_FIFO_CTRL_A {
    #[doc = "0: Send out digital zero on SPDIF Tx"]
    TX_FIFO_CTRL_0 = 0,
    #[doc = "1: Tx Normal operation"]
    TX_FIFO_CTRL_1 = 1,
    #[doc = "2: Reset to 1 sample remaining"]
    TX_FIFO_CTRL_2 = 2,
}
impl From<TX_FIFO_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_FIFO_CTRL_A) -> Self {
        variant as _
    }
}
impl TX_FIFO_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_FIFO_CTRL_A> {
        match self.bits {
            0 => Some(TX_FIFO_CTRL_A::TX_FIFO_CTRL_0),
            1 => Some(TX_FIFO_CTRL_A::TX_FIFO_CTRL_1),
            2 => Some(TX_FIFO_CTRL_A::TX_FIFO_CTRL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TX_FIFO_CTRL_0`"]
    #[inline(always)]
    pub fn is_tx_fifo_ctrl_0(&self) -> bool {
        *self == TX_FIFO_CTRL_A::TX_FIFO_CTRL_0
    }
    #[doc = "Checks if the value of the field is `TX_FIFO_CTRL_1`"]
    #[inline(always)]
    pub fn is_tx_fifo_ctrl_1(&self) -> bool {
        *self == TX_FIFO_CTRL_A::TX_FIFO_CTRL_1
    }
    #[doc = "Checks if the value of the field is `TX_FIFO_CTRL_2`"]
    #[inline(always)]
    pub fn is_tx_fifo_ctrl_2(&self) -> bool {
        *self == TX_FIFO_CTRL_A::TX_FIFO_CTRL_2
    }
}
#[doc = "Field `TxFIFO_Ctrl` writer - no description available"]
pub type TX_FIFO_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR_SPEC, u8, TX_FIFO_CTRL_A, 2, O>;
impl<'a, const O: u8> TX_FIFO_CTRL_W<'a, O> {
    #[doc = "Send out digital zero on SPDIF Tx"]
    #[inline(always)]
    pub fn tx_fifo_ctrl_0(self) -> &'a mut W {
        self.variant(TX_FIFO_CTRL_A::TX_FIFO_CTRL_0)
    }
    #[doc = "Tx Normal operation"]
    #[inline(always)]
    pub fn tx_fifo_ctrl_1(self) -> &'a mut W {
        self.variant(TX_FIFO_CTRL_A::TX_FIFO_CTRL_1)
    }
    #[doc = "Reset to 1 sample remaining"]
    #[inline(always)]
    pub fn tx_fifo_ctrl_2(self) -> &'a mut W {
        self.variant(TX_FIFO_CTRL_A::TX_FIFO_CTRL_2)
    }
}
#[doc = "Field `soft_reset` reader - When write 1 to this bit, it will cause SPDIF software reset"]
pub type SOFT_RESET_R = crate::BitReader<bool>;
#[doc = "Field `soft_reset` writer - When write 1 to this bit, it will cause SPDIF software reset"]
pub type SOFT_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `LOW_POWER` reader - When write 1 to this bit, it will cause SPDIF enter low-power mode"]
pub type LOW_POWER_R = crate::BitReader<bool>;
#[doc = "Field `LOW_POWER` writer - When write 1 to this bit, it will cause SPDIF enter low-power mode"]
pub type LOW_POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `TxFIFOEmpty_Sel` reader - no description available"]
pub type TX_FIFOEMPTY_SEL_R = crate::FieldReader<u8, TX_FIFOEMPTY_SEL_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_FIFOEMPTY_SEL_A {
    #[doc = "0: Empty interrupt if 0 sample in Tx left and right FIFOs"]
    TX_FIFOEMPTY_SEL_0 = 0,
    #[doc = "1: Empty interrupt if at most 4 sample in Tx left and right FIFOs"]
    TX_FIFOEMPTY_SEL_1 = 1,
    #[doc = "2: Empty interrupt if at most 8 sample in Tx left and right FIFOs"]
    TX_FIFOEMPTY_SEL_2 = 2,
    #[doc = "3: Empty interrupt if at most 12 sample in Tx left and right FIFOs"]
    TX_FIFOEMPTY_SEL_3 = 3,
}
impl From<TX_FIFOEMPTY_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_FIFOEMPTY_SEL_A) -> Self {
        variant as _
    }
}
impl TX_FIFOEMPTY_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FIFOEMPTY_SEL_A {
        match self.bits {
            0 => TX_FIFOEMPTY_SEL_A::TX_FIFOEMPTY_SEL_0,
            1 => TX_FIFOEMPTY_SEL_A::TX_FIFOEMPTY_SEL_1,
            2 => TX_FIFOEMPTY_SEL_A::TX_FIFOEMPTY_SEL_2,
            3 => TX_FIFOEMPTY_SEL_A::TX_FIFOEMPTY_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TX_FIFOEMPTY_SEL_0`"]
    #[inline(always)]
    pub fn is_tx_fifoempty_sel_0(&self) -> bool {
        *self == TX_FIFOEMPTY_SEL_A::TX_FIFOEMPTY_SEL_0
    }
    #[doc = "Checks if the value of the field is `TX_FIFOEMPTY_SEL_1`"]
    #[inline(always)]
    pub fn is_tx_fifoempty_sel_1(&self) -> bool {
        *self == TX_FIFOEMPTY_SEL_A::TX_FIFOEMPTY_SEL_1
    }
    #[doc = "Checks if the value of the field is `TX_FIFOEMPTY_SEL_2`"]
    #[inline(always)]
    pub fn is_tx_fifoempty_sel_2(&self) -> bool {
        *self == TX_FIFOEMPTY_SEL_A::TX_FIFOEMPTY_SEL_2
    }
    #[doc = "Checks if the value of the field is `TX_FIFOEMPTY_SEL_3`"]
    #[inline(always)]
    pub fn is_tx_fifoempty_sel_3(&self) -> bool {
        *self == TX_FIFOEMPTY_SEL_A::TX_FIFOEMPTY_SEL_3
    }
}
#[doc = "Field `TxFIFOEmpty_Sel` writer - no description available"]
pub type TX_FIFOEMPTY_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SCR_SPEC, u8, TX_FIFOEMPTY_SEL_A, 2, O>;
impl<'a, const O: u8> TX_FIFOEMPTY_SEL_W<'a, O> {
    #[doc = "Empty interrupt if 0 sample in Tx left and right FIFOs"]
    #[inline(always)]
    pub fn tx_fifoempty_sel_0(self) -> &'a mut W {
        self.variant(TX_FIFOEMPTY_SEL_A::TX_FIFOEMPTY_SEL_0)
    }
    #[doc = "Empty interrupt if at most 4 sample in Tx left and right FIFOs"]
    #[inline(always)]
    pub fn tx_fifoempty_sel_1(self) -> &'a mut W {
        self.variant(TX_FIFOEMPTY_SEL_A::TX_FIFOEMPTY_SEL_1)
    }
    #[doc = "Empty interrupt if at most 8 sample in Tx left and right FIFOs"]
    #[inline(always)]
    pub fn tx_fifoempty_sel_2(self) -> &'a mut W {
        self.variant(TX_FIFOEMPTY_SEL_A::TX_FIFOEMPTY_SEL_2)
    }
    #[doc = "Empty interrupt if at most 12 sample in Tx left and right FIFOs"]
    #[inline(always)]
    pub fn tx_fifoempty_sel_3(self) -> &'a mut W {
        self.variant(TX_FIFOEMPTY_SEL_A::TX_FIFOEMPTY_SEL_3)
    }
}
#[doc = "Field `TxAutoSync` reader - no description available"]
pub type TX_AUTO_SYNC_R = crate::BitReader<TX_AUTO_SYNC_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_AUTO_SYNC_A {
    #[doc = "0: Tx FIFO auto sync off"]
    TX_AUTO_SYNC_0 = 0,
    #[doc = "1: Tx FIFO auto sync on"]
    TX_AUTO_SYNC_1 = 1,
}
impl From<TX_AUTO_SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TX_AUTO_SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_AUTO_SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_AUTO_SYNC_A {
        match self.bits {
            false => TX_AUTO_SYNC_A::TX_AUTO_SYNC_0,
            true => TX_AUTO_SYNC_A::TX_AUTO_SYNC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TX_AUTO_SYNC_0`"]
    #[inline(always)]
    pub fn is_tx_auto_sync_0(&self) -> bool {
        *self == TX_AUTO_SYNC_A::TX_AUTO_SYNC_0
    }
    #[doc = "Checks if the value of the field is `TX_AUTO_SYNC_1`"]
    #[inline(always)]
    pub fn is_tx_auto_sync_1(&self) -> bool {
        *self == TX_AUTO_SYNC_A::TX_AUTO_SYNC_1
    }
}
#[doc = "Field `TxAutoSync` writer - no description available"]
pub type TX_AUTO_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, TX_AUTO_SYNC_A, O>;
impl<'a, const O: u8> TX_AUTO_SYNC_W<'a, O> {
    #[doc = "Tx FIFO auto sync off"]
    #[inline(always)]
    pub fn tx_auto_sync_0(self) -> &'a mut W {
        self.variant(TX_AUTO_SYNC_A::TX_AUTO_SYNC_0)
    }
    #[doc = "Tx FIFO auto sync on"]
    #[inline(always)]
    pub fn tx_auto_sync_1(self) -> &'a mut W {
        self.variant(TX_AUTO_SYNC_A::TX_AUTO_SYNC_1)
    }
}
#[doc = "Field `RxAutoSync` reader - no description available"]
pub type RX_AUTO_SYNC_R = crate::BitReader<RX_AUTO_SYNC_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_AUTO_SYNC_A {
    #[doc = "0: Rx FIFO auto sync off"]
    RX_AUTO_SYNC_0 = 0,
    #[doc = "1: RxFIFO auto sync on"]
    RX_AUTO_SYNC_1 = 1,
}
impl From<RX_AUTO_SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RX_AUTO_SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_AUTO_SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_AUTO_SYNC_A {
        match self.bits {
            false => RX_AUTO_SYNC_A::RX_AUTO_SYNC_0,
            true => RX_AUTO_SYNC_A::RX_AUTO_SYNC_1,
        }
    }
    #[doc = "Checks if the value of the field is `RX_AUTO_SYNC_0`"]
    #[inline(always)]
    pub fn is_rx_auto_sync_0(&self) -> bool {
        *self == RX_AUTO_SYNC_A::RX_AUTO_SYNC_0
    }
    #[doc = "Checks if the value of the field is `RX_AUTO_SYNC_1`"]
    #[inline(always)]
    pub fn is_rx_auto_sync_1(&self) -> bool {
        *self == RX_AUTO_SYNC_A::RX_AUTO_SYNC_1
    }
}
#[doc = "Field `RxAutoSync` writer - no description available"]
pub type RX_AUTO_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, RX_AUTO_SYNC_A, O>;
impl<'a, const O: u8> RX_AUTO_SYNC_W<'a, O> {
    #[doc = "Rx FIFO auto sync off"]
    #[inline(always)]
    pub fn rx_auto_sync_0(self) -> &'a mut W {
        self.variant(RX_AUTO_SYNC_A::RX_AUTO_SYNC_0)
    }
    #[doc = "RxFIFO auto sync on"]
    #[inline(always)]
    pub fn rx_auto_sync_1(self) -> &'a mut W {
        self.variant(RX_AUTO_SYNC_A::RX_AUTO_SYNC_1)
    }
}
#[doc = "Field `RxFIFOFull_Sel` reader - no description available"]
pub type RX_FIFOFULL_SEL_R = crate::FieldReader<u8, RX_FIFOFULL_SEL_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_FIFOFULL_SEL_A {
    #[doc = "0: Full interrupt if at least 1 sample in Rx left and right FIFOs"]
    RX_FIFOFULL_SEL_0 = 0,
    #[doc = "1: Full interrupt if at least 4 sample in Rx left and right FIFOs"]
    RX_FIFOFULL_SEL_1 = 1,
    #[doc = "2: Full interrupt if at least 8 sample in Rx left and right FIFOs"]
    RX_FIFOFULL_SEL_2 = 2,
    #[doc = "3: Full interrupt if at least 16 sample in Rx left and right FIFO"]
    RX_FIFOFULL_SEL_3 = 3,
}
impl From<RX_FIFOFULL_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_FIFOFULL_SEL_A) -> Self {
        variant as _
    }
}
impl RX_FIFOFULL_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFOFULL_SEL_A {
        match self.bits {
            0 => RX_FIFOFULL_SEL_A::RX_FIFOFULL_SEL_0,
            1 => RX_FIFOFULL_SEL_A::RX_FIFOFULL_SEL_1,
            2 => RX_FIFOFULL_SEL_A::RX_FIFOFULL_SEL_2,
            3 => RX_FIFOFULL_SEL_A::RX_FIFOFULL_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFOFULL_SEL_0`"]
    #[inline(always)]
    pub fn is_rx_fifofull_sel_0(&self) -> bool {
        *self == RX_FIFOFULL_SEL_A::RX_FIFOFULL_SEL_0
    }
    #[doc = "Checks if the value of the field is `RX_FIFOFULL_SEL_1`"]
    #[inline(always)]
    pub fn is_rx_fifofull_sel_1(&self) -> bool {
        *self == RX_FIFOFULL_SEL_A::RX_FIFOFULL_SEL_1
    }
    #[doc = "Checks if the value of the field is `RX_FIFOFULL_SEL_2`"]
    #[inline(always)]
    pub fn is_rx_fifofull_sel_2(&self) -> bool {
        *self == RX_FIFOFULL_SEL_A::RX_FIFOFULL_SEL_2
    }
    #[doc = "Checks if the value of the field is `RX_FIFOFULL_SEL_3`"]
    #[inline(always)]
    pub fn is_rx_fifofull_sel_3(&self) -> bool {
        *self == RX_FIFOFULL_SEL_A::RX_FIFOFULL_SEL_3
    }
}
#[doc = "Field `RxFIFOFull_Sel` writer - no description available"]
pub type RX_FIFOFULL_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SCR_SPEC, u8, RX_FIFOFULL_SEL_A, 2, O>;
impl<'a, const O: u8> RX_FIFOFULL_SEL_W<'a, O> {
    #[doc = "Full interrupt if at least 1 sample in Rx left and right FIFOs"]
    #[inline(always)]
    pub fn rx_fifofull_sel_0(self) -> &'a mut W {
        self.variant(RX_FIFOFULL_SEL_A::RX_FIFOFULL_SEL_0)
    }
    #[doc = "Full interrupt if at least 4 sample in Rx left and right FIFOs"]
    #[inline(always)]
    pub fn rx_fifofull_sel_1(self) -> &'a mut W {
        self.variant(RX_FIFOFULL_SEL_A::RX_FIFOFULL_SEL_1)
    }
    #[doc = "Full interrupt if at least 8 sample in Rx left and right FIFOs"]
    #[inline(always)]
    pub fn rx_fifofull_sel_2(self) -> &'a mut W {
        self.variant(RX_FIFOFULL_SEL_A::RX_FIFOFULL_SEL_2)
    }
    #[doc = "Full interrupt if at least 16 sample in Rx left and right FIFO"]
    #[inline(always)]
    pub fn rx_fifofull_sel_3(self) -> &'a mut W {
        self.variant(RX_FIFOFULL_SEL_A::RX_FIFOFULL_SEL_3)
    }
}
#[doc = "Field `RxFIFO_Rst` reader - no description available"]
pub type RX_FIFO_RST_R = crate::BitReader<RX_FIFO_RST_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFO_RST_A {
    #[doc = "0: Normal operation"]
    RX_FIFO_RST_0 = 0,
    #[doc = "1: Reset register to 1 sample remaining"]
    RX_FIFO_RST_1 = 1,
}
impl From<RX_FIFO_RST_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FIFO_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_RST_A {
        match self.bits {
            false => RX_FIFO_RST_A::RX_FIFO_RST_0,
            true => RX_FIFO_RST_A::RX_FIFO_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_RST_0`"]
    #[inline(always)]
    pub fn is_rx_fifo_rst_0(&self) -> bool {
        *self == RX_FIFO_RST_A::RX_FIFO_RST_0
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_RST_1`"]
    #[inline(always)]
    pub fn is_rx_fifo_rst_1(&self) -> bool {
        *self == RX_FIFO_RST_A::RX_FIFO_RST_1
    }
}
#[doc = "Field `RxFIFO_Rst` writer - no description available"]
pub type RX_FIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, RX_FIFO_RST_A, O>;
impl<'a, const O: u8> RX_FIFO_RST_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn rx_fifo_rst_0(self) -> &'a mut W {
        self.variant(RX_FIFO_RST_A::RX_FIFO_RST_0)
    }
    #[doc = "Reset register to 1 sample remaining"]
    #[inline(always)]
    pub fn rx_fifo_rst_1(self) -> &'a mut W {
        self.variant(RX_FIFO_RST_A::RX_FIFO_RST_1)
    }
}
#[doc = "Field `RxFIFO_Off_On` reader - no description available"]
pub type RX_FIFO_OFF_ON_R = crate::BitReader<RX_FIFO_OFF_ON_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFO_OFF_ON_A {
    #[doc = "0: SPDIF Rx FIFO is on"]
    RX_FIFO_OFF_ON_0 = 0,
    #[doc = "1: SPDIF Rx FIFO is off. Does not accept data from interface"]
    RX_FIFO_OFF_ON_1 = 1,
}
impl From<RX_FIFO_OFF_ON_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_OFF_ON_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FIFO_OFF_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_OFF_ON_A {
        match self.bits {
            false => RX_FIFO_OFF_ON_A::RX_FIFO_OFF_ON_0,
            true => RX_FIFO_OFF_ON_A::RX_FIFO_OFF_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_OFF_ON_0`"]
    #[inline(always)]
    pub fn is_rx_fifo_off_on_0(&self) -> bool {
        *self == RX_FIFO_OFF_ON_A::RX_FIFO_OFF_ON_0
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_OFF_ON_1`"]
    #[inline(always)]
    pub fn is_rx_fifo_off_on_1(&self) -> bool {
        *self == RX_FIFO_OFF_ON_A::RX_FIFO_OFF_ON_1
    }
}
#[doc = "Field `RxFIFO_Off_On` writer - no description available"]
pub type RX_FIFO_OFF_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCR_SPEC, RX_FIFO_OFF_ON_A, O>;
impl<'a, const O: u8> RX_FIFO_OFF_ON_W<'a, O> {
    #[doc = "SPDIF Rx FIFO is on"]
    #[inline(always)]
    pub fn rx_fifo_off_on_0(self) -> &'a mut W {
        self.variant(RX_FIFO_OFF_ON_A::RX_FIFO_OFF_ON_0)
    }
    #[doc = "SPDIF Rx FIFO is off. Does not accept data from interface"]
    #[inline(always)]
    pub fn rx_fifo_off_on_1(self) -> &'a mut W {
        self.variant(RX_FIFO_OFF_ON_A::RX_FIFO_OFF_ON_1)
    }
}
#[doc = "Field `RxFIFO_Ctrl` reader - no description available"]
pub type RX_FIFO_CTRL_R = crate::BitReader<RX_FIFO_CTRL_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFO_CTRL_A {
    #[doc = "0: Normal operation"]
    RX_FIFO_CTRL_0 = 0,
    #[doc = "1: Always read zero from Rx data register"]
    RX_FIFO_CTRL_1 = 1,
}
impl From<RX_FIFO_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FIFO_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_CTRL_A {
        match self.bits {
            false => RX_FIFO_CTRL_A::RX_FIFO_CTRL_0,
            true => RX_FIFO_CTRL_A::RX_FIFO_CTRL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_CTRL_0`"]
    #[inline(always)]
    pub fn is_rx_fifo_ctrl_0(&self) -> bool {
        *self == RX_FIFO_CTRL_A::RX_FIFO_CTRL_0
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_CTRL_1`"]
    #[inline(always)]
    pub fn is_rx_fifo_ctrl_1(&self) -> bool {
        *self == RX_FIFO_CTRL_A::RX_FIFO_CTRL_1
    }
}
#[doc = "Field `RxFIFO_Ctrl` writer - no description available"]
pub type RX_FIFO_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, RX_FIFO_CTRL_A, O>;
impl<'a, const O: u8> RX_FIFO_CTRL_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn rx_fifo_ctrl_0(self) -> &'a mut W {
        self.variant(RX_FIFO_CTRL_A::RX_FIFO_CTRL_0)
    }
    #[doc = "Always read zero from Rx data register"]
    #[inline(always)]
    pub fn rx_fifo_ctrl_1(self) -> &'a mut W {
        self.variant(RX_FIFO_CTRL_A::RX_FIFO_CTRL_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - no description available"]
    #[inline(always)]
    pub fn usrc_sel(&self) -> USRC_SEL_R {
        USRC_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - no description available"]
    #[inline(always)]
    pub fn tx_sel(&self) -> TX_SEL_R {
        TX_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn val_ctrl(&self) -> VAL_CTRL_R {
        VAL_CTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Transmit Request Enable (Tx FIFO empty)"]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA Receive Request Enable (RX FIFO full)"]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - no description available"]
    #[inline(always)]
    pub fn tx_fifo_ctrl(&self) -> TX_FIFO_CTRL_R {
        TX_FIFO_CTRL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - When write 1 to this bit, it will cause SPDIF software reset"]
    #[inline(always)]
    pub fn soft_reset(&self) -> SOFT_RESET_R {
        SOFT_RESET_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When write 1 to this bit, it will cause SPDIF enter low-power mode"]
    #[inline(always)]
    pub fn low_power(&self) -> LOW_POWER_R {
        LOW_POWER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 15:16 - no description available"]
    #[inline(always)]
    pub fn tx_fifoempty_sel(&self) -> TX_FIFOEMPTY_SEL_R {
        TX_FIFOEMPTY_SEL_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn tx_auto_sync(&self) -> TX_AUTO_SYNC_R {
        TX_AUTO_SYNC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn rx_auto_sync(&self) -> RX_AUTO_SYNC_R {
        RX_AUTO_SYNC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - no description available"]
    #[inline(always)]
    pub fn rx_fifofull_sel(&self) -> RX_FIFOFULL_SEL_R {
        RX_FIFOFULL_SEL_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> RX_FIFO_RST_R {
        RX_FIFO_RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn rx_fifo_off_on(&self) -> RX_FIFO_OFF_ON_R {
        RX_FIFO_OFF_ON_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn rx_fifo_ctrl(&self) -> RX_FIFO_CTRL_R {
        RX_FIFO_CTRL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn usrc_sel(&mut self) -> USRC_SEL_W<0> {
        USRC_SEL_W::new(self)
    }
    #[doc = "Bits 2:4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn tx_sel(&mut self) -> TX_SEL_W<2> {
        TX_SEL_W::new(self)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn val_ctrl(&mut self) -> VAL_CTRL_W<5> {
        VAL_CTRL_W::new(self)
    }
    #[doc = "Bit 8 - DMA Transmit Request Enable (Tx FIFO empty)"]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W<8> {
        DMA_TX_EN_W::new(self)
    }
    #[doc = "Bit 9 - DMA Receive Request Enable (RX FIFO full)"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W<9> {
        DMA_RX_EN_W::new(self)
    }
    #[doc = "Bits 10:11 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_ctrl(&mut self) -> TX_FIFO_CTRL_W<10> {
        TX_FIFO_CTRL_W::new(self)
    }
    #[doc = "Bit 12 - When write 1 to this bit, it will cause SPDIF software reset"]
    #[inline(always)]
    #[must_use]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W<12> {
        SOFT_RESET_W::new(self)
    }
    #[doc = "Bit 13 - When write 1 to this bit, it will cause SPDIF enter low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn low_power(&mut self) -> LOW_POWER_W<13> {
        LOW_POWER_W::new(self)
    }
    #[doc = "Bits 15:16 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifoempty_sel(&mut self) -> TX_FIFOEMPTY_SEL_W<15> {
        TX_FIFOEMPTY_SEL_W::new(self)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn tx_auto_sync(&mut self) -> TX_AUTO_SYNC_W<17> {
        TX_AUTO_SYNC_W::new(self)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn rx_auto_sync(&mut self) -> RX_AUTO_SYNC_W<18> {
        RX_AUTO_SYNC_W::new(self)
    }
    #[doc = "Bits 19:20 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifofull_sel(&mut self) -> RX_FIFOFULL_SEL_W<19> {
        RX_FIFOFULL_SEL_W::new(self)
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_rst(&mut self) -> RX_FIFO_RST_W<21> {
        RX_FIFO_RST_W::new(self)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_off_on(&mut self) -> RX_FIFO_OFF_ON_W<22> {
        RX_FIFO_OFF_ON_W::new(self)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_ctrl(&mut self) -> RX_FIFO_CTRL_W<23> {
        RX_FIFO_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPDIF Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0x0400"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
