#[doc = "Register `CLK_TUNE_CTRL_STATUS` reader"]
pub struct R(crate::R<CLK_TUNE_CTRL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TUNE_CTRL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TUNE_CTRL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TUNE_CTRL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TUNE_CTRL_STATUS` writer"]
pub struct W(crate::W<CLK_TUNE_CTRL_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TUNE_CTRL_STATUS_SPEC>;
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
impl From<crate::W<CLK_TUNE_CTRL_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TUNE_CTRL_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY_CELL_SET_POST` reader - Delay cells on the feedback clock between CLK_OUT and CLK_POST"]
pub type DLY_CELL_SET_POST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLY_CELL_SET_POST` writer - Delay cells on the feedback clock between CLK_OUT and CLK_POST"]
pub type DLY_CELL_SET_POST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TUNE_CTRL_STATUS_SPEC, u8, u8, 4, O>;
#[doc = "Field `DLY_CELL_SET_OUT` reader - Delay cells on the feedback clock between CLK_PRE and CLK_OUT"]
pub type DLY_CELL_SET_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLY_CELL_SET_OUT` writer - Delay cells on the feedback clock between CLK_PRE and CLK_OUT"]
pub type DLY_CELL_SET_OUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TUNE_CTRL_STATUS_SPEC, u8, u8, 4, O>;
#[doc = "Field `DLY_CELL_SET_PRE` reader - delay cells on the feedback clock between the feedback clock and CLK_PRE"]
pub type DLY_CELL_SET_PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLY_CELL_SET_PRE` writer - delay cells on the feedback clock between the feedback clock and CLK_PRE"]
pub type DLY_CELL_SET_PRE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TUNE_CTRL_STATUS_SPEC, u8, u8, 7, O>;
#[doc = "Field `NXT_ERR` reader - NXT error"]
pub type NXT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `TAP_SEL_POST` reader - Delay cells added on the feedback clock between CLK_OUT and CLK_POST"]
pub type TAP_SEL_POST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAP_SEL_OUT` reader - Delay cells added on the feedback clock between CLK_PRE and CLK_OUT"]
pub type TAP_SEL_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAP_SEL_PRE` reader - TAP_SEL_PRE"]
pub type TAP_SEL_PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE_ERR` reader - PRE error"]
pub type PRE_ERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Delay cells on the feedback clock between CLK_OUT and CLK_POST"]
    #[inline(always)]
    pub fn dly_cell_set_post(&self) -> DLY_CELL_SET_POST_R {
        DLY_CELL_SET_POST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Delay cells on the feedback clock between CLK_PRE and CLK_OUT"]
    #[inline(always)]
    pub fn dly_cell_set_out(&self) -> DLY_CELL_SET_OUT_R {
        DLY_CELL_SET_OUT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - delay cells on the feedback clock between the feedback clock and CLK_PRE"]
    #[inline(always)]
    pub fn dly_cell_set_pre(&self) -> DLY_CELL_SET_PRE_R {
        DLY_CELL_SET_PRE_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - NXT error"]
    #[inline(always)]
    pub fn nxt_err(&self) -> NXT_ERR_R {
        NXT_ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Delay cells added on the feedback clock between CLK_OUT and CLK_POST"]
    #[inline(always)]
    pub fn tap_sel_post(&self) -> TAP_SEL_POST_R {
        TAP_SEL_POST_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Delay cells added on the feedback clock between CLK_PRE and CLK_OUT"]
    #[inline(always)]
    pub fn tap_sel_out(&self) -> TAP_SEL_OUT_R {
        TAP_SEL_OUT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - TAP_SEL_PRE"]
    #[inline(always)]
    pub fn tap_sel_pre(&self) -> TAP_SEL_PRE_R {
        TAP_SEL_PRE_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - PRE error"]
    #[inline(always)]
    pub fn pre_err(&self) -> PRE_ERR_R {
        PRE_ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Delay cells on the feedback clock between CLK_OUT and CLK_POST"]
    #[inline(always)]
    #[must_use]
    pub fn dly_cell_set_post(&mut self) -> DLY_CELL_SET_POST_W<0> {
        DLY_CELL_SET_POST_W::new(self)
    }
    #[doc = "Bits 4:7 - Delay cells on the feedback clock between CLK_PRE and CLK_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn dly_cell_set_out(&mut self) -> DLY_CELL_SET_OUT_W<4> {
        DLY_CELL_SET_OUT_W::new(self)
    }
    #[doc = "Bits 8:14 - delay cells on the feedback clock between the feedback clock and CLK_PRE"]
    #[inline(always)]
    #[must_use]
    pub fn dly_cell_set_pre(&mut self) -> DLY_CELL_SET_PRE_W<8> {
        DLY_CELL_SET_PRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLK Tuning Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_tune_ctrl_status](index.html) module"]
pub struct CLK_TUNE_CTRL_STATUS_SPEC;
impl crate::RegisterSpec for CLK_TUNE_CTRL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_tune_ctrl_status::R](R) reader structure"]
impl crate::Readable for CLK_TUNE_CTRL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_tune_ctrl_status::W](W) writer structure"]
impl crate::Writable for CLK_TUNE_CTRL_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_TUNE_CTRL_STATUS to value 0"]
impl crate::Resettable for CLK_TUNE_CTRL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
