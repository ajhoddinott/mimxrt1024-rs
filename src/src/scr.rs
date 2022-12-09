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
#[doc = "Field `lockup_rst` reader - lockup reset enable bit"]
pub type LOCKUP_RST_R = crate::BitReader<LOCKUP_RST_A>;
#[doc = "lockup reset enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKUP_RST_A {
    #[doc = "0: disabled"]
    LOCKUP_RST_0 = 0,
    #[doc = "1: enabled"]
    LOCKUP_RST_1 = 1,
}
impl From<LOCKUP_RST_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCKUP_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_RST_A {
        match self.bits {
            false => LOCKUP_RST_A::LOCKUP_RST_0,
            true => LOCKUP_RST_A::LOCKUP_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKUP_RST_0`"]
    #[inline(always)]
    pub fn is_lockup_rst_0(&self) -> bool {
        *self == LOCKUP_RST_A::LOCKUP_RST_0
    }
    #[doc = "Checks if the value of the field is `LOCKUP_RST_1`"]
    #[inline(always)]
    pub fn is_lockup_rst_1(&self) -> bool {
        *self == LOCKUP_RST_A::LOCKUP_RST_1
    }
}
#[doc = "Field `lockup_rst` writer - lockup reset enable bit"]
pub type LOCKUP_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, LOCKUP_RST_A, O>;
impl<'a, const O: u8> LOCKUP_RST_W<'a, O> {
    #[doc = "disabled"]
    #[inline(always)]
    pub fn lockup_rst_0(self) -> &'a mut W {
        self.variant(LOCKUP_RST_A::LOCKUP_RST_0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn lockup_rst_1(self) -> &'a mut W {
        self.variant(LOCKUP_RST_A::LOCKUP_RST_1)
    }
}
#[doc = "Field `mask_wdog_rst` reader - Mask wdog_rst_b source"]
pub type MASK_WDOG_RST_R = crate::FieldReader<u8, MASK_WDOG_RST_A>;
#[doc = "Mask wdog_rst_b source\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASK_WDOG_RST_A {
    #[doc = "5: wdog_rst_b is masked"]
    MASK_WDOG_RST_5 = 5,
    #[doc = "10: wdog_rst_b is not masked (default)"]
    MASK_WDOG_RST_10 = 10,
}
impl From<MASK_WDOG_RST_A> for u8 {
    #[inline(always)]
    fn from(variant: MASK_WDOG_RST_A) -> Self {
        variant as _
    }
}
impl MASK_WDOG_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MASK_WDOG_RST_A> {
        match self.bits {
            5 => Some(MASK_WDOG_RST_A::MASK_WDOG_RST_5),
            10 => Some(MASK_WDOG_RST_A::MASK_WDOG_RST_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG_RST_5`"]
    #[inline(always)]
    pub fn is_mask_wdog_rst_5(&self) -> bool {
        *self == MASK_WDOG_RST_A::MASK_WDOG_RST_5
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG_RST_10`"]
    #[inline(always)]
    pub fn is_mask_wdog_rst_10(&self) -> bool {
        *self == MASK_WDOG_RST_A::MASK_WDOG_RST_10
    }
}
#[doc = "Field `mask_wdog_rst` writer - Mask wdog_rst_b source"]
pub type MASK_WDOG_RST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR_SPEC, u8, MASK_WDOG_RST_A, 4, O>;
impl<'a, const O: u8> MASK_WDOG_RST_W<'a, O> {
    #[doc = "wdog_rst_b is masked"]
    #[inline(always)]
    pub fn mask_wdog_rst_5(self) -> &'a mut W {
        self.variant(MASK_WDOG_RST_A::MASK_WDOG_RST_5)
    }
    #[doc = "wdog_rst_b is not masked (default)"]
    #[inline(always)]
    pub fn mask_wdog_rst_10(self) -> &'a mut W {
        self.variant(MASK_WDOG_RST_A::MASK_WDOG_RST_10)
    }
}
#[doc = "Field `core0_rst` reader - Software reset for core0 only"]
pub type CORE0_RST_R = crate::BitReader<CORE0_RST_A>;
#[doc = "Software reset for core0 only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CORE0_RST_A {
    #[doc = "0: do not assert core0 reset"]
    CORE0_RST_0 = 0,
    #[doc = "1: assert core0 reset"]
    CORE0_RST_1 = 1,
}
impl From<CORE0_RST_A> for bool {
    #[inline(always)]
    fn from(variant: CORE0_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl CORE0_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CORE0_RST_A {
        match self.bits {
            false => CORE0_RST_A::CORE0_RST_0,
            true => CORE0_RST_A::CORE0_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CORE0_RST_0`"]
    #[inline(always)]
    pub fn is_core0_rst_0(&self) -> bool {
        *self == CORE0_RST_A::CORE0_RST_0
    }
    #[doc = "Checks if the value of the field is `CORE0_RST_1`"]
    #[inline(always)]
    pub fn is_core0_rst_1(&self) -> bool {
        *self == CORE0_RST_A::CORE0_RST_1
    }
}
#[doc = "Field `core0_rst` writer - Software reset for core0 only"]
pub type CORE0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, CORE0_RST_A, O>;
impl<'a, const O: u8> CORE0_RST_W<'a, O> {
    #[doc = "do not assert core0 reset"]
    #[inline(always)]
    pub fn core0_rst_0(self) -> &'a mut W {
        self.variant(CORE0_RST_A::CORE0_RST_0)
    }
    #[doc = "assert core0 reset"]
    #[inline(always)]
    pub fn core0_rst_1(self) -> &'a mut W {
        self.variant(CORE0_RST_A::CORE0_RST_1)
    }
}
#[doc = "Field `core0_dbg_rst` reader - Software reset for core0 debug only"]
pub type CORE0_DBG_RST_R = crate::BitReader<CORE0_DBG_RST_A>;
#[doc = "Software reset for core0 debug only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CORE0_DBG_RST_A {
    #[doc = "0: do not assert core0 debug reset"]
    CORE0_DBG_RST_0 = 0,
    #[doc = "1: assert core0 debug reset"]
    CORE0_DBG_RST_1 = 1,
}
impl From<CORE0_DBG_RST_A> for bool {
    #[inline(always)]
    fn from(variant: CORE0_DBG_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl CORE0_DBG_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CORE0_DBG_RST_A {
        match self.bits {
            false => CORE0_DBG_RST_A::CORE0_DBG_RST_0,
            true => CORE0_DBG_RST_A::CORE0_DBG_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CORE0_DBG_RST_0`"]
    #[inline(always)]
    pub fn is_core0_dbg_rst_0(&self) -> bool {
        *self == CORE0_DBG_RST_A::CORE0_DBG_RST_0
    }
    #[doc = "Checks if the value of the field is `CORE0_DBG_RST_1`"]
    #[inline(always)]
    pub fn is_core0_dbg_rst_1(&self) -> bool {
        *self == CORE0_DBG_RST_A::CORE0_DBG_RST_1
    }
}
#[doc = "Field `core0_dbg_rst` writer - Software reset for core0 debug only"]
pub type CORE0_DBG_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, CORE0_DBG_RST_A, O>;
impl<'a, const O: u8> CORE0_DBG_RST_W<'a, O> {
    #[doc = "do not assert core0 debug reset"]
    #[inline(always)]
    pub fn core0_dbg_rst_0(self) -> &'a mut W {
        self.variant(CORE0_DBG_RST_A::CORE0_DBG_RST_0)
    }
    #[doc = "assert core0 debug reset"]
    #[inline(always)]
    pub fn core0_dbg_rst_1(self) -> &'a mut W {
        self.variant(CORE0_DBG_RST_A::CORE0_DBG_RST_1)
    }
}
#[doc = "Field `dbg_rst_msk_pg` reader - Do not assert debug resets after power gating event of core"]
pub type DBG_RST_MSK_PG_R = crate::BitReader<DBG_RST_MSK_PG_A>;
#[doc = "Do not assert debug resets after power gating event of core\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_RST_MSK_PG_A {
    #[doc = "0: do not mask core debug resets (debug resets will be asserted after power gating event)"]
    DBG_RST_MSK_PG_0 = 0,
    #[doc = "1: mask core debug resets (debug resets won't be asserted after power gating event)"]
    DBG_RST_MSK_PG_1 = 1,
}
impl From<DBG_RST_MSK_PG_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_RST_MSK_PG_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_RST_MSK_PG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_RST_MSK_PG_A {
        match self.bits {
            false => DBG_RST_MSK_PG_A::DBG_RST_MSK_PG_0,
            true => DBG_RST_MSK_PG_A::DBG_RST_MSK_PG_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBG_RST_MSK_PG_0`"]
    #[inline(always)]
    pub fn is_dbg_rst_msk_pg_0(&self) -> bool {
        *self == DBG_RST_MSK_PG_A::DBG_RST_MSK_PG_0
    }
    #[doc = "Checks if the value of the field is `DBG_RST_MSK_PG_1`"]
    #[inline(always)]
    pub fn is_dbg_rst_msk_pg_1(&self) -> bool {
        *self == DBG_RST_MSK_PG_A::DBG_RST_MSK_PG_1
    }
}
#[doc = "Field `dbg_rst_msk_pg` writer - Do not assert debug resets after power gating event of core"]
pub type DBG_RST_MSK_PG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCR_SPEC, DBG_RST_MSK_PG_A, O>;
impl<'a, const O: u8> DBG_RST_MSK_PG_W<'a, O> {
    #[doc = "do not mask core debug resets (debug resets will be asserted after power gating event)"]
    #[inline(always)]
    pub fn dbg_rst_msk_pg_0(self) -> &'a mut W {
        self.variant(DBG_RST_MSK_PG_A::DBG_RST_MSK_PG_0)
    }
    #[doc = "mask core debug resets (debug resets won't be asserted after power gating event)"]
    #[inline(always)]
    pub fn dbg_rst_msk_pg_1(self) -> &'a mut W {
        self.variant(DBG_RST_MSK_PG_A::DBG_RST_MSK_PG_1)
    }
}
#[doc = "Field `mask_wdog3_rst` reader - Mask wdog3_rst_b source"]
pub type MASK_WDOG3_RST_R = crate::FieldReader<u8, MASK_WDOG3_RST_A>;
#[doc = "Mask wdog3_rst_b source\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASK_WDOG3_RST_A {
    #[doc = "5: wdog3_rst_b is masked"]
    MASK_WDOG3_RST_5 = 5,
    #[doc = "10: wdog3_rst_b is not masked"]
    MASK_WDOG3_RST_10 = 10,
}
impl From<MASK_WDOG3_RST_A> for u8 {
    #[inline(always)]
    fn from(variant: MASK_WDOG3_RST_A) -> Self {
        variant as _
    }
}
impl MASK_WDOG3_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MASK_WDOG3_RST_A> {
        match self.bits {
            5 => Some(MASK_WDOG3_RST_A::MASK_WDOG3_RST_5),
            10 => Some(MASK_WDOG3_RST_A::MASK_WDOG3_RST_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG3_RST_5`"]
    #[inline(always)]
    pub fn is_mask_wdog3_rst_5(&self) -> bool {
        *self == MASK_WDOG3_RST_A::MASK_WDOG3_RST_5
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG3_RST_10`"]
    #[inline(always)]
    pub fn is_mask_wdog3_rst_10(&self) -> bool {
        *self == MASK_WDOG3_RST_A::MASK_WDOG3_RST_10
    }
}
#[doc = "Field `mask_wdog3_rst` writer - Mask wdog3_rst_b source"]
pub type MASK_WDOG3_RST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR_SPEC, u8, MASK_WDOG3_RST_A, 4, O>;
impl<'a, const O: u8> MASK_WDOG3_RST_W<'a, O> {
    #[doc = "wdog3_rst_b is masked"]
    #[inline(always)]
    pub fn mask_wdog3_rst_5(self) -> &'a mut W {
        self.variant(MASK_WDOG3_RST_A::MASK_WDOG3_RST_5)
    }
    #[doc = "wdog3_rst_b is not masked"]
    #[inline(always)]
    pub fn mask_wdog3_rst_10(self) -> &'a mut W {
        self.variant(MASK_WDOG3_RST_A::MASK_WDOG3_RST_10)
    }
}
impl R {
    #[doc = "Bit 4 - lockup reset enable bit"]
    #[inline(always)]
    pub fn lockup_rst(&self) -> LOCKUP_RST_R {
        LOCKUP_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 7:10 - Mask wdog_rst_b source"]
    #[inline(always)]
    pub fn mask_wdog_rst(&self) -> MASK_WDOG_RST_R {
        MASK_WDOG_RST_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Software reset for core0 only"]
    #[inline(always)]
    pub fn core0_rst(&self) -> CORE0_RST_R {
        CORE0_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - Software reset for core0 debug only"]
    #[inline(always)]
    pub fn core0_dbg_rst(&self) -> CORE0_DBG_RST_R {
        CORE0_DBG_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 25 - Do not assert debug resets after power gating event of core"]
    #[inline(always)]
    pub fn dbg_rst_msk_pg(&self) -> DBG_RST_MSK_PG_R {
        DBG_RST_MSK_PG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Mask wdog3_rst_b source"]
    #[inline(always)]
    pub fn mask_wdog3_rst(&self) -> MASK_WDOG3_RST_R {
        MASK_WDOG3_RST_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - lockup reset enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lockup_rst(&mut self) -> LOCKUP_RST_W<4> {
        LOCKUP_RST_W::new(self)
    }
    #[doc = "Bits 7:10 - Mask wdog_rst_b source"]
    #[inline(always)]
    #[must_use]
    pub fn mask_wdog_rst(&mut self) -> MASK_WDOG_RST_W<7> {
        MASK_WDOG_RST_W::new(self)
    }
    #[doc = "Bit 13 - Software reset for core0 only"]
    #[inline(always)]
    #[must_use]
    pub fn core0_rst(&mut self) -> CORE0_RST_W<13> {
        CORE0_RST_W::new(self)
    }
    #[doc = "Bit 17 - Software reset for core0 debug only"]
    #[inline(always)]
    #[must_use]
    pub fn core0_dbg_rst(&mut self) -> CORE0_DBG_RST_W<17> {
        CORE0_DBG_RST_W::new(self)
    }
    #[doc = "Bit 25 - Do not assert debug resets after power gating event of core"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rst_msk_pg(&mut self) -> DBG_RST_MSK_PG_W<25> {
        DBG_RST_MSK_PG_W::new(self)
    }
    #[doc = "Bits 28:31 - Mask wdog3_rst_b source"]
    #[inline(always)]
    #[must_use]
    pub fn mask_wdog3_rst(&mut self) -> MASK_WDOG3_RST_W<28> {
        MASK_WDOG3_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
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
#[doc = "`reset()` method sets SCR to value 0xa048_0520"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xa048_0520;
}
