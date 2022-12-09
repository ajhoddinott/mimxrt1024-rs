#[doc = "Register `INT_SIGNAL_EN` reader"]
pub struct R(crate::R<INT_SIGNAL_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_SIGNAL_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_SIGNAL_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_SIGNAL_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_SIGNAL_EN` writer"]
pub struct W(crate::W<INT_SIGNAL_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SIGNAL_EN_SPEC>;
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
impl From<crate::W<INT_SIGNAL_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SIGNAL_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCIEN` reader - Command complete interrupt enable"]
pub type CCIEN_R = crate::BitReader<CCIEN_A>;
#[doc = "Command complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCIEN_A {
    #[doc = "0: Masked"]
    CCIEN_0 = 0,
    #[doc = "1: Enabled"]
    CCIEN_1 = 1,
}
impl From<CCIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIEN_A {
        match self.bits {
            false => CCIEN_A::CCIEN_0,
            true => CCIEN_A::CCIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIEN_0`"]
    #[inline(always)]
    pub fn is_ccien_0(&self) -> bool {
        *self == CCIEN_A::CCIEN_0
    }
    #[doc = "Checks if the value of the field is `CCIEN_1`"]
    #[inline(always)]
    pub fn is_ccien_1(&self) -> bool {
        *self == CCIEN_A::CCIEN_1
    }
}
#[doc = "Field `CCIEN` writer - Command complete interrupt enable"]
pub type CCIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, CCIEN_A, O>;
impl<'a, const O: u8> CCIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ccien_0(self) -> &'a mut W {
        self.variant(CCIEN_A::CCIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ccien_1(self) -> &'a mut W {
        self.variant(CCIEN_A::CCIEN_1)
    }
}
#[doc = "Field `TCIEN` reader - Transfer complete interrupt enable"]
pub type TCIEN_R = crate::BitReader<TCIEN_A>;
#[doc = "Transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIEN_A {
    #[doc = "0: Masked"]
    TCIEN_0 = 0,
    #[doc = "1: Enabled"]
    TCIEN_1 = 1,
}
impl From<TCIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TCIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIEN_A {
        match self.bits {
            false => TCIEN_A::TCIEN_0,
            true => TCIEN_A::TCIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCIEN_0`"]
    #[inline(always)]
    pub fn is_tcien_0(&self) -> bool {
        *self == TCIEN_A::TCIEN_0
    }
    #[doc = "Checks if the value of the field is `TCIEN_1`"]
    #[inline(always)]
    pub fn is_tcien_1(&self) -> bool {
        *self == TCIEN_A::TCIEN_1
    }
}
#[doc = "Field `TCIEN` writer - Transfer complete interrupt enable"]
pub type TCIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, TCIEN_A, O>;
impl<'a, const O: u8> TCIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tcien_0(self) -> &'a mut W {
        self.variant(TCIEN_A::TCIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tcien_1(self) -> &'a mut W {
        self.variant(TCIEN_A::TCIEN_1)
    }
}
#[doc = "Field `BGEIEN` reader - Block gap event interrupt enable"]
pub type BGEIEN_R = crate::BitReader<BGEIEN_A>;
#[doc = "Block gap event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGEIEN_A {
    #[doc = "0: Masked"]
    BGEIEN_0 = 0,
    #[doc = "1: Enabled"]
    BGEIEN_1 = 1,
}
impl From<BGEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BGEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BGEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGEIEN_A {
        match self.bits {
            false => BGEIEN_A::BGEIEN_0,
            true => BGEIEN_A::BGEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BGEIEN_0`"]
    #[inline(always)]
    pub fn is_bgeien_0(&self) -> bool {
        *self == BGEIEN_A::BGEIEN_0
    }
    #[doc = "Checks if the value of the field is `BGEIEN_1`"]
    #[inline(always)]
    pub fn is_bgeien_1(&self) -> bool {
        *self == BGEIEN_A::BGEIEN_1
    }
}
#[doc = "Field `BGEIEN` writer - Block gap event interrupt enable"]
pub type BGEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, BGEIEN_A, O>;
impl<'a, const O: u8> BGEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn bgeien_0(self) -> &'a mut W {
        self.variant(BGEIEN_A::BGEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bgeien_1(self) -> &'a mut W {
        self.variant(BGEIEN_A::BGEIEN_1)
    }
}
#[doc = "Field `DINTIEN` reader - DMA interrupt enable"]
pub type DINTIEN_R = crate::BitReader<DINTIEN_A>;
#[doc = "DMA interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINTIEN_A {
    #[doc = "0: Masked"]
    DINTIEN_0 = 0,
    #[doc = "1: Enabled"]
    DINTIEN_1 = 1,
}
impl From<DINTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DINTIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DINTIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINTIEN_A {
        match self.bits {
            false => DINTIEN_A::DINTIEN_0,
            true => DINTIEN_A::DINTIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DINTIEN_0`"]
    #[inline(always)]
    pub fn is_dintien_0(&self) -> bool {
        *self == DINTIEN_A::DINTIEN_0
    }
    #[doc = "Checks if the value of the field is `DINTIEN_1`"]
    #[inline(always)]
    pub fn is_dintien_1(&self) -> bool {
        *self == DINTIEN_A::DINTIEN_1
    }
}
#[doc = "Field `DINTIEN` writer - DMA interrupt enable"]
pub type DINTIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, DINTIEN_A, O>;
impl<'a, const O: u8> DINTIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dintien_0(self) -> &'a mut W {
        self.variant(DINTIEN_A::DINTIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dintien_1(self) -> &'a mut W {
        self.variant(DINTIEN_A::DINTIEN_1)
    }
}
#[doc = "Field `BWRIEN` reader - Buffer write ready interrupt enable"]
pub type BWRIEN_R = crate::BitReader<BWRIEN_A>;
#[doc = "Buffer write ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWRIEN_A {
    #[doc = "0: Masked"]
    BWRIEN_0 = 0,
    #[doc = "1: Enabled"]
    BWRIEN_1 = 1,
}
impl From<BWRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWRIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BWRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWRIEN_A {
        match self.bits {
            false => BWRIEN_A::BWRIEN_0,
            true => BWRIEN_A::BWRIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BWRIEN_0`"]
    #[inline(always)]
    pub fn is_bwrien_0(&self) -> bool {
        *self == BWRIEN_A::BWRIEN_0
    }
    #[doc = "Checks if the value of the field is `BWRIEN_1`"]
    #[inline(always)]
    pub fn is_bwrien_1(&self) -> bool {
        *self == BWRIEN_A::BWRIEN_1
    }
}
#[doc = "Field `BWRIEN` writer - Buffer write ready interrupt enable"]
pub type BWRIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, BWRIEN_A, O>;
impl<'a, const O: u8> BWRIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn bwrien_0(self) -> &'a mut W {
        self.variant(BWRIEN_A::BWRIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bwrien_1(self) -> &'a mut W {
        self.variant(BWRIEN_A::BWRIEN_1)
    }
}
#[doc = "Field `BRRIEN` reader - Buffer read ready interrupt enable"]
pub type BRRIEN_R = crate::BitReader<BRRIEN_A>;
#[doc = "Buffer read ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRRIEN_A {
    #[doc = "0: Masked"]
    BRRIEN_0 = 0,
    #[doc = "1: Enabled"]
    BRRIEN_1 = 1,
}
impl From<BRRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRRIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BRRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRRIEN_A {
        match self.bits {
            false => BRRIEN_A::BRRIEN_0,
            true => BRRIEN_A::BRRIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BRRIEN_0`"]
    #[inline(always)]
    pub fn is_brrien_0(&self) -> bool {
        *self == BRRIEN_A::BRRIEN_0
    }
    #[doc = "Checks if the value of the field is `BRRIEN_1`"]
    #[inline(always)]
    pub fn is_brrien_1(&self) -> bool {
        *self == BRRIEN_A::BRRIEN_1
    }
}
#[doc = "Field `BRRIEN` writer - Buffer read ready interrupt enable"]
pub type BRRIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, BRRIEN_A, O>;
impl<'a, const O: u8> BRRIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn brrien_0(self) -> &'a mut W {
        self.variant(BRRIEN_A::BRRIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn brrien_1(self) -> &'a mut W {
        self.variant(BRRIEN_A::BRRIEN_1)
    }
}
#[doc = "Field `CINSIEN` reader - Card insertion interrupt enable"]
pub type CINSIEN_R = crate::BitReader<CINSIEN_A>;
#[doc = "Card insertion interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINSIEN_A {
    #[doc = "0: Masked"]
    CINSIEN_0 = 0,
    #[doc = "1: Enabled"]
    CINSIEN_1 = 1,
}
impl From<CINSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CINSIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CINSIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINSIEN_A {
        match self.bits {
            false => CINSIEN_A::CINSIEN_0,
            true => CINSIEN_A::CINSIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINSIEN_0`"]
    #[inline(always)]
    pub fn is_cinsien_0(&self) -> bool {
        *self == CINSIEN_A::CINSIEN_0
    }
    #[doc = "Checks if the value of the field is `CINSIEN_1`"]
    #[inline(always)]
    pub fn is_cinsien_1(&self) -> bool {
        *self == CINSIEN_A::CINSIEN_1
    }
}
#[doc = "Field `CINSIEN` writer - Card insertion interrupt enable"]
pub type CINSIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, CINSIEN_A, O>;
impl<'a, const O: u8> CINSIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cinsien_0(self) -> &'a mut W {
        self.variant(CINSIEN_A::CINSIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cinsien_1(self) -> &'a mut W {
        self.variant(CINSIEN_A::CINSIEN_1)
    }
}
#[doc = "Field `CRMIEN` reader - Card removal interrupt enable"]
pub type CRMIEN_R = crate::BitReader<CRMIEN_A>;
#[doc = "Card removal interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRMIEN_A {
    #[doc = "0: Masked"]
    CRMIEN_0 = 0,
    #[doc = "1: Enabled"]
    CRMIEN_1 = 1,
}
impl From<CRMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRMIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRMIEN_A {
        match self.bits {
            false => CRMIEN_A::CRMIEN_0,
            true => CRMIEN_A::CRMIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRMIEN_0`"]
    #[inline(always)]
    pub fn is_crmien_0(&self) -> bool {
        *self == CRMIEN_A::CRMIEN_0
    }
    #[doc = "Checks if the value of the field is `CRMIEN_1`"]
    #[inline(always)]
    pub fn is_crmien_1(&self) -> bool {
        *self == CRMIEN_A::CRMIEN_1
    }
}
#[doc = "Field `CRMIEN` writer - Card removal interrupt enable"]
pub type CRMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, CRMIEN_A, O>;
impl<'a, const O: u8> CRMIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn crmien_0(self) -> &'a mut W {
        self.variant(CRMIEN_A::CRMIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn crmien_1(self) -> &'a mut W {
        self.variant(CRMIEN_A::CRMIEN_1)
    }
}
#[doc = "Field `CINTIEN` reader - Card interrupt enable"]
pub type CINTIEN_R = crate::BitReader<CINTIEN_A>;
#[doc = "Card interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINTIEN_A {
    #[doc = "0: Masked"]
    CINTIEN_0 = 0,
    #[doc = "1: Enabled"]
    CINTIEN_1 = 1,
}
impl From<CINTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CINTIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CINTIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINTIEN_A {
        match self.bits {
            false => CINTIEN_A::CINTIEN_0,
            true => CINTIEN_A::CINTIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINTIEN_0`"]
    #[inline(always)]
    pub fn is_cintien_0(&self) -> bool {
        *self == CINTIEN_A::CINTIEN_0
    }
    #[doc = "Checks if the value of the field is `CINTIEN_1`"]
    #[inline(always)]
    pub fn is_cintien_1(&self) -> bool {
        *self == CINTIEN_A::CINTIEN_1
    }
}
#[doc = "Field `CINTIEN` writer - Card interrupt enable"]
pub type CINTIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, CINTIEN_A, O>;
impl<'a, const O: u8> CINTIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cintien_0(self) -> &'a mut W {
        self.variant(CINTIEN_A::CINTIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cintien_1(self) -> &'a mut W {
        self.variant(CINTIEN_A::CINTIEN_1)
    }
}
#[doc = "Field `RTEIEN` reader - Re-tuning event interrupt enable"]
pub type RTEIEN_R = crate::BitReader<RTEIEN_A>;
#[doc = "Re-tuning event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTEIEN_A {
    #[doc = "0: Masked"]
    RTEIEN_0 = 0,
    #[doc = "1: Enabled"]
    RTEIEN_1 = 1,
}
impl From<RTEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTEIEN_A {
        match self.bits {
            false => RTEIEN_A::RTEIEN_0,
            true => RTEIEN_A::RTEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTEIEN_0`"]
    #[inline(always)]
    pub fn is_rteien_0(&self) -> bool {
        *self == RTEIEN_A::RTEIEN_0
    }
    #[doc = "Checks if the value of the field is `RTEIEN_1`"]
    #[inline(always)]
    pub fn is_rteien_1(&self) -> bool {
        *self == RTEIEN_A::RTEIEN_1
    }
}
#[doc = "Field `RTEIEN` writer - Re-tuning event interrupt enable"]
pub type RTEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, RTEIEN_A, O>;
impl<'a, const O: u8> RTEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn rteien_0(self) -> &'a mut W {
        self.variant(RTEIEN_A::RTEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn rteien_1(self) -> &'a mut W {
        self.variant(RTEIEN_A::RTEIEN_1)
    }
}
#[doc = "Field `TPIEN` reader - Tuning Pass interrupt enable"]
pub type TPIEN_R = crate::BitReader<TPIEN_A>;
#[doc = "Tuning Pass interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPIEN_A {
    #[doc = "0: Masked"]
    TPIEN_0 = 0,
    #[doc = "1: Enabled"]
    TPIEN_1 = 1,
}
impl From<TPIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TPIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPIEN_A {
        match self.bits {
            false => TPIEN_A::TPIEN_0,
            true => TPIEN_A::TPIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TPIEN_0`"]
    #[inline(always)]
    pub fn is_tpien_0(&self) -> bool {
        *self == TPIEN_A::TPIEN_0
    }
    #[doc = "Checks if the value of the field is `TPIEN_1`"]
    #[inline(always)]
    pub fn is_tpien_1(&self) -> bool {
        *self == TPIEN_A::TPIEN_1
    }
}
#[doc = "Field `TPIEN` writer - Tuning Pass interrupt enable"]
pub type TPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, TPIEN_A, O>;
impl<'a, const O: u8> TPIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tpien_0(self) -> &'a mut W {
        self.variant(TPIEN_A::TPIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tpien_1(self) -> &'a mut W {
        self.variant(TPIEN_A::TPIEN_1)
    }
}
#[doc = "Field `CTOEIEN` reader - Command timeout error interrupt enable"]
pub type CTOEIEN_R = crate::BitReader<CTOEIEN_A>;
#[doc = "Command timeout error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTOEIEN_A {
    #[doc = "0: Masked"]
    CTOEIEN_0 = 0,
    #[doc = "1: Enabled"]
    CTOEIEN_1 = 1,
}
impl From<CTOEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTOEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTOEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTOEIEN_A {
        match self.bits {
            false => CTOEIEN_A::CTOEIEN_0,
            true => CTOEIEN_A::CTOEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTOEIEN_0`"]
    #[inline(always)]
    pub fn is_ctoeien_0(&self) -> bool {
        *self == CTOEIEN_A::CTOEIEN_0
    }
    #[doc = "Checks if the value of the field is `CTOEIEN_1`"]
    #[inline(always)]
    pub fn is_ctoeien_1(&self) -> bool {
        *self == CTOEIEN_A::CTOEIEN_1
    }
}
#[doc = "Field `CTOEIEN` writer - Command timeout error interrupt enable"]
pub type CTOEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, CTOEIEN_A, O>;
impl<'a, const O: u8> CTOEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ctoeien_0(self) -> &'a mut W {
        self.variant(CTOEIEN_A::CTOEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ctoeien_1(self) -> &'a mut W {
        self.variant(CTOEIEN_A::CTOEIEN_1)
    }
}
#[doc = "Field `CCEIEN` reader - Command CRC error interrupt enable"]
pub type CCEIEN_R = crate::BitReader<CCEIEN_A>;
#[doc = "Command CRC error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCEIEN_A {
    #[doc = "0: Masked"]
    CCEIEN_0 = 0,
    #[doc = "1: Enabled"]
    CCEIEN_1 = 1,
}
impl From<CCEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCEIEN_A {
        match self.bits {
            false => CCEIEN_A::CCEIEN_0,
            true => CCEIEN_A::CCEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCEIEN_0`"]
    #[inline(always)]
    pub fn is_cceien_0(&self) -> bool {
        *self == CCEIEN_A::CCEIEN_0
    }
    #[doc = "Checks if the value of the field is `CCEIEN_1`"]
    #[inline(always)]
    pub fn is_cceien_1(&self) -> bool {
        *self == CCEIEN_A::CCEIEN_1
    }
}
#[doc = "Field `CCEIEN` writer - Command CRC error interrupt enable"]
pub type CCEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, CCEIEN_A, O>;
impl<'a, const O: u8> CCEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cceien_0(self) -> &'a mut W {
        self.variant(CCEIEN_A::CCEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cceien_1(self) -> &'a mut W {
        self.variant(CCEIEN_A::CCEIEN_1)
    }
}
#[doc = "Field `CEBEIEN` reader - Command end bit error interrupt enable"]
pub type CEBEIEN_R = crate::BitReader<CEBEIEN_A>;
#[doc = "Command end bit error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEBEIEN_A {
    #[doc = "0: Masked"]
    CEBEIEN_0 = 0,
    #[doc = "1: Enabled"]
    CEBEIEN_1 = 1,
}
impl From<CEBEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEBEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CEBEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEBEIEN_A {
        match self.bits {
            false => CEBEIEN_A::CEBEIEN_0,
            true => CEBEIEN_A::CEBEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEBEIEN_0`"]
    #[inline(always)]
    pub fn is_cebeien_0(&self) -> bool {
        *self == CEBEIEN_A::CEBEIEN_0
    }
    #[doc = "Checks if the value of the field is `CEBEIEN_1`"]
    #[inline(always)]
    pub fn is_cebeien_1(&self) -> bool {
        *self == CEBEIEN_A::CEBEIEN_1
    }
}
#[doc = "Field `CEBEIEN` writer - Command end bit error interrupt enable"]
pub type CEBEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, CEBEIEN_A, O>;
impl<'a, const O: u8> CEBEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cebeien_0(self) -> &'a mut W {
        self.variant(CEBEIEN_A::CEBEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cebeien_1(self) -> &'a mut W {
        self.variant(CEBEIEN_A::CEBEIEN_1)
    }
}
#[doc = "Field `CIEIEN` reader - Command index error interrupt enable"]
pub type CIEIEN_R = crate::BitReader<CIEIEN_A>;
#[doc = "Command index error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIEIEN_A {
    #[doc = "0: Masked"]
    CIEIEN_0 = 0,
    #[doc = "1: Enabled"]
    CIEIEN_1 = 1,
}
impl From<CIEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CIEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CIEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIEIEN_A {
        match self.bits {
            false => CIEIEN_A::CIEIEN_0,
            true => CIEIEN_A::CIEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIEIEN_0`"]
    #[inline(always)]
    pub fn is_cieien_0(&self) -> bool {
        *self == CIEIEN_A::CIEIEN_0
    }
    #[doc = "Checks if the value of the field is `CIEIEN_1`"]
    #[inline(always)]
    pub fn is_cieien_1(&self) -> bool {
        *self == CIEIEN_A::CIEIEN_1
    }
}
#[doc = "Field `CIEIEN` writer - Command index error interrupt enable"]
pub type CIEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, CIEIEN_A, O>;
impl<'a, const O: u8> CIEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cieien_0(self) -> &'a mut W {
        self.variant(CIEIEN_A::CIEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cieien_1(self) -> &'a mut W {
        self.variant(CIEIEN_A::CIEIEN_1)
    }
}
#[doc = "Field `DTOEIEN` reader - Data timeout error interrupt enable"]
pub type DTOEIEN_R = crate::BitReader<DTOEIEN_A>;
#[doc = "Data timeout error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTOEIEN_A {
    #[doc = "0: Masked"]
    DTOEIEN_0 = 0,
    #[doc = "1: Enabled"]
    DTOEIEN_1 = 1,
}
impl From<DTOEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTOEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DTOEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOEIEN_A {
        match self.bits {
            false => DTOEIEN_A::DTOEIEN_0,
            true => DTOEIEN_A::DTOEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTOEIEN_0`"]
    #[inline(always)]
    pub fn is_dtoeien_0(&self) -> bool {
        *self == DTOEIEN_A::DTOEIEN_0
    }
    #[doc = "Checks if the value of the field is `DTOEIEN_1`"]
    #[inline(always)]
    pub fn is_dtoeien_1(&self) -> bool {
        *self == DTOEIEN_A::DTOEIEN_1
    }
}
#[doc = "Field `DTOEIEN` writer - Data timeout error interrupt enable"]
pub type DTOEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, DTOEIEN_A, O>;
impl<'a, const O: u8> DTOEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dtoeien_0(self) -> &'a mut W {
        self.variant(DTOEIEN_A::DTOEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dtoeien_1(self) -> &'a mut W {
        self.variant(DTOEIEN_A::DTOEIEN_1)
    }
}
#[doc = "Field `DCEIEN` reader - Data CRC error interrupt enable"]
pub type DCEIEN_R = crate::BitReader<DCEIEN_A>;
#[doc = "Data CRC error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCEIEN_A {
    #[doc = "0: Masked"]
    DCEIEN_0 = 0,
    #[doc = "1: Enabled"]
    DCEIEN_1 = 1,
}
impl From<DCEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCEIEN_A {
        match self.bits {
            false => DCEIEN_A::DCEIEN_0,
            true => DCEIEN_A::DCEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCEIEN_0`"]
    #[inline(always)]
    pub fn is_dceien_0(&self) -> bool {
        *self == DCEIEN_A::DCEIEN_0
    }
    #[doc = "Checks if the value of the field is `DCEIEN_1`"]
    #[inline(always)]
    pub fn is_dceien_1(&self) -> bool {
        *self == DCEIEN_A::DCEIEN_1
    }
}
#[doc = "Field `DCEIEN` writer - Data CRC error interrupt enable"]
pub type DCEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, DCEIEN_A, O>;
impl<'a, const O: u8> DCEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dceien_0(self) -> &'a mut W {
        self.variant(DCEIEN_A::DCEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dceien_1(self) -> &'a mut W {
        self.variant(DCEIEN_A::DCEIEN_1)
    }
}
#[doc = "Field `DEBEIEN` reader - Data end bit error interrupt enable"]
pub type DEBEIEN_R = crate::BitReader<DEBEIEN_A>;
#[doc = "Data end bit error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBEIEN_A {
    #[doc = "0: Masked"]
    DEBEIEN_0 = 0,
    #[doc = "1: Enabled"]
    DEBEIEN_1 = 1,
}
impl From<DEBEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DEBEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBEIEN_A {
        match self.bits {
            false => DEBEIEN_A::DEBEIEN_0,
            true => DEBEIEN_A::DEBEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEBEIEN_0`"]
    #[inline(always)]
    pub fn is_debeien_0(&self) -> bool {
        *self == DEBEIEN_A::DEBEIEN_0
    }
    #[doc = "Checks if the value of the field is `DEBEIEN_1`"]
    #[inline(always)]
    pub fn is_debeien_1(&self) -> bool {
        *self == DEBEIEN_A::DEBEIEN_1
    }
}
#[doc = "Field `DEBEIEN` writer - Data end bit error interrupt enable"]
pub type DEBEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, DEBEIEN_A, O>;
impl<'a, const O: u8> DEBEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn debeien_0(self) -> &'a mut W {
        self.variant(DEBEIEN_A::DEBEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn debeien_1(self) -> &'a mut W {
        self.variant(DEBEIEN_A::DEBEIEN_1)
    }
}
#[doc = "Field `AC12EIEN` reader - Auto CMD12 error interrupt enable"]
pub type AC12EIEN_R = crate::BitReader<AC12EIEN_A>;
#[doc = "Auto CMD12 error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12EIEN_A {
    #[doc = "0: Masked"]
    AC12EIEN_0 = 0,
    #[doc = "1: Enabled"]
    AC12EIEN_1 = 1,
}
impl From<AC12EIEN_A> for bool {
    #[inline(always)]
    fn from(variant: AC12EIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12EIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12EIEN_A {
        match self.bits {
            false => AC12EIEN_A::AC12EIEN_0,
            true => AC12EIEN_A::AC12EIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12EIEN_0`"]
    #[inline(always)]
    pub fn is_ac12eien_0(&self) -> bool {
        *self == AC12EIEN_A::AC12EIEN_0
    }
    #[doc = "Checks if the value of the field is `AC12EIEN_1`"]
    #[inline(always)]
    pub fn is_ac12eien_1(&self) -> bool {
        *self == AC12EIEN_A::AC12EIEN_1
    }
}
#[doc = "Field `AC12EIEN` writer - Auto CMD12 error interrupt enable"]
pub type AC12EIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, AC12EIEN_A, O>;
impl<'a, const O: u8> AC12EIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ac12eien_0(self) -> &'a mut W {
        self.variant(AC12EIEN_A::AC12EIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ac12eien_1(self) -> &'a mut W {
        self.variant(AC12EIEN_A::AC12EIEN_1)
    }
}
#[doc = "Field `TNEIEN` reader - Tuning error interrupt enable"]
pub type TNEIEN_R = crate::BitReader<TNEIEN_A>;
#[doc = "Tuning error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TNEIEN_A {
    #[doc = "0: Masked"]
    TNEIEN_0 = 0,
    #[doc = "1: Enabled"]
    TNEIEN_1 = 1,
}
impl From<TNEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TNEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TNEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNEIEN_A {
        match self.bits {
            false => TNEIEN_A::TNEIEN_0,
            true => TNEIEN_A::TNEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TNEIEN_0`"]
    #[inline(always)]
    pub fn is_tneien_0(&self) -> bool {
        *self == TNEIEN_A::TNEIEN_0
    }
    #[doc = "Checks if the value of the field is `TNEIEN_1`"]
    #[inline(always)]
    pub fn is_tneien_1(&self) -> bool {
        *self == TNEIEN_A::TNEIEN_1
    }
}
#[doc = "Field `TNEIEN` writer - Tuning error interrupt enable"]
pub type TNEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, TNEIEN_A, O>;
impl<'a, const O: u8> TNEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tneien_0(self) -> &'a mut W {
        self.variant(TNEIEN_A::TNEIEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tneien_1(self) -> &'a mut W {
        self.variant(TNEIEN_A::TNEIEN_1)
    }
}
#[doc = "Field `DMAEIEN` reader - DMA error interrupt enable"]
pub type DMAEIEN_R = crate::BitReader<DMAEIEN_A>;
#[doc = "DMA error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEIEN_A {
    #[doc = "0: Masked"]
    DMAEIEN_0 = 0,
    #[doc = "1: Enable"]
    DMAEIEN_1 = 1,
}
impl From<DMAEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEIEN_A {
        match self.bits {
            false => DMAEIEN_A::DMAEIEN_0,
            true => DMAEIEN_A::DMAEIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAEIEN_0`"]
    #[inline(always)]
    pub fn is_dmaeien_0(&self) -> bool {
        *self == DMAEIEN_A::DMAEIEN_0
    }
    #[doc = "Checks if the value of the field is `DMAEIEN_1`"]
    #[inline(always)]
    pub fn is_dmaeien_1(&self) -> bool {
        *self == DMAEIEN_A::DMAEIEN_1
    }
}
#[doc = "Field `DMAEIEN` writer - DMA error interrupt enable"]
pub type DMAEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SIGNAL_EN_SPEC, DMAEIEN_A, O>;
impl<'a, const O: u8> DMAEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dmaeien_0(self) -> &'a mut W {
        self.variant(DMAEIEN_A::DMAEIEN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmaeien_1(self) -> &'a mut W {
        self.variant(DMAEIEN_A::DMAEIEN_1)
    }
}
impl R {
    #[doc = "Bit 0 - Command complete interrupt enable"]
    #[inline(always)]
    pub fn ccien(&self) -> CCIEN_R {
        CCIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcien(&self) -> TCIEN_R {
        TCIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block gap event interrupt enable"]
    #[inline(always)]
    pub fn bgeien(&self) -> BGEIEN_R {
        BGEIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA interrupt enable"]
    #[inline(always)]
    pub fn dintien(&self) -> DINTIEN_R {
        DINTIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer write ready interrupt enable"]
    #[inline(always)]
    pub fn bwrien(&self) -> BWRIEN_R {
        BWRIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer read ready interrupt enable"]
    #[inline(always)]
    pub fn brrien(&self) -> BRRIEN_R {
        BRRIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card insertion interrupt enable"]
    #[inline(always)]
    pub fn cinsien(&self) -> CINSIEN_R {
        CINSIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card removal interrupt enable"]
    #[inline(always)]
    pub fn crmien(&self) -> CRMIEN_R {
        CRMIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card interrupt enable"]
    #[inline(always)]
    pub fn cintien(&self) -> CINTIEN_R {
        CINTIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-tuning event interrupt enable"]
    #[inline(always)]
    pub fn rteien(&self) -> RTEIEN_R {
        RTEIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Tuning Pass interrupt enable"]
    #[inline(always)]
    pub fn tpien(&self) -> TPIEN_R {
        TPIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Command timeout error interrupt enable"]
    #[inline(always)]
    pub fn ctoeien(&self) -> CTOEIEN_R {
        CTOEIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC error interrupt enable"]
    #[inline(always)]
    pub fn cceien(&self) -> CCEIEN_R {
        CCEIEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command end bit error interrupt enable"]
    #[inline(always)]
    pub fn cebeien(&self) -> CEBEIEN_R {
        CEBEIEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command index error interrupt enable"]
    #[inline(always)]
    pub fn cieien(&self) -> CIEIEN_R {
        CIEIEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data timeout error interrupt enable"]
    #[inline(always)]
    pub fn dtoeien(&self) -> DTOEIEN_R {
        DTOEIEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC error interrupt enable"]
    #[inline(always)]
    pub fn dceien(&self) -> DCEIEN_R {
        DCEIEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data end bit error interrupt enable"]
    #[inline(always)]
    pub fn debeien(&self) -> DEBEIEN_R {
        DEBEIEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 error interrupt enable"]
    #[inline(always)]
    pub fn ac12eien(&self) -> AC12EIEN_R {
        AC12EIEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning error interrupt enable"]
    #[inline(always)]
    pub fn tneien(&self) -> TNEIEN_R {
        TNEIEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA error interrupt enable"]
    #[inline(always)]
    pub fn dmaeien(&self) -> DMAEIEN_R {
        DMAEIEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccien(&mut self) -> CCIEN_W<0> {
        CCIEN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcien(&mut self) -> TCIEN_W<1> {
        TCIEN_W::new(self)
    }
    #[doc = "Bit 2 - Block gap event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bgeien(&mut self) -> BGEIEN_W<2> {
        BGEIEN_W::new(self)
    }
    #[doc = "Bit 3 - DMA interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dintien(&mut self) -> DINTIEN_W<3> {
        DINTIEN_W::new(self)
    }
    #[doc = "Bit 4 - Buffer write ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwrien(&mut self) -> BWRIEN_W<4> {
        BWRIEN_W::new(self)
    }
    #[doc = "Bit 5 - Buffer read ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn brrien(&mut self) -> BRRIEN_W<5> {
        BRRIEN_W::new(self)
    }
    #[doc = "Bit 6 - Card insertion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cinsien(&mut self) -> CINSIEN_W<6> {
        CINSIEN_W::new(self)
    }
    #[doc = "Bit 7 - Card removal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn crmien(&mut self) -> CRMIEN_W<7> {
        CRMIEN_W::new(self)
    }
    #[doc = "Bit 8 - Card interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cintien(&mut self) -> CINTIEN_W<8> {
        CINTIEN_W::new(self)
    }
    #[doc = "Bit 12 - Re-tuning event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rteien(&mut self) -> RTEIEN_W<12> {
        RTEIEN_W::new(self)
    }
    #[doc = "Bit 14 - Tuning Pass interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpien(&mut self) -> TPIEN_W<14> {
        TPIEN_W::new(self)
    }
    #[doc = "Bit 16 - Command timeout error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctoeien(&mut self) -> CTOEIEN_W<16> {
        CTOEIEN_W::new(self)
    }
    #[doc = "Bit 17 - Command CRC error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cceien(&mut self) -> CCEIEN_W<17> {
        CCEIEN_W::new(self)
    }
    #[doc = "Bit 18 - Command end bit error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cebeien(&mut self) -> CEBEIEN_W<18> {
        CEBEIEN_W::new(self)
    }
    #[doc = "Bit 19 - Command index error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cieien(&mut self) -> CIEIEN_W<19> {
        CIEIEN_W::new(self)
    }
    #[doc = "Bit 20 - Data timeout error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoeien(&mut self) -> DTOEIEN_W<20> {
        DTOEIEN_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dceien(&mut self) -> DCEIEN_W<21> {
        DCEIEN_W::new(self)
    }
    #[doc = "Bit 22 - Data end bit error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn debeien(&mut self) -> DEBEIEN_W<22> {
        DEBEIEN_W::new(self)
    }
    #[doc = "Bit 24 - Auto CMD12 error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac12eien(&mut self) -> AC12EIEN_W<24> {
        AC12EIEN_W::new(self)
    }
    #[doc = "Bit 26 - Tuning error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tneien(&mut self) -> TNEIEN_W<26> {
        TNEIEN_W::new(self)
    }
    #[doc = "Bit 28 - DMA error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaeien(&mut self) -> DMAEIEN_W<28> {
        DMAEIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Signal Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_signal_en](index.html) module"]
pub struct INT_SIGNAL_EN_SPEC;
impl crate::RegisterSpec for INT_SIGNAL_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_signal_en::R](R) reader structure"]
impl crate::Readable for INT_SIGNAL_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_signal_en::W](W) writer structure"]
impl crate::Writable for INT_SIGNAL_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_SIGNAL_EN to value 0"]
impl crate::Resettable for INT_SIGNAL_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
