#[doc = "Register `TUNING_CTRL` reader"]
pub struct R(crate::R<TUNING_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TUNING_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TUNING_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TUNING_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TUNING_CTRL` writer"]
pub struct W(crate::W<TUNING_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TUNING_CTRL_SPEC>;
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
impl From<crate::W<TUNING_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TUNING_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING_START_TAP` reader - Tuning start"]
pub type TUNING_START_TAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING_START_TAP` writer - Tuning start"]
pub type TUNING_START_TAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TUNING_CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `DIS_CMD_CHK_FOR_STD_TUNING` reader - Disable command check for standard tuning"]
pub type DIS_CMD_CHK_FOR_STD_TUNING_R = crate::BitReader<bool>;
#[doc = "Field `DIS_CMD_CHK_FOR_STD_TUNING` writer - Disable command check for standard tuning"]
pub type DIS_CMD_CHK_FOR_STD_TUNING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TUNING_CTRL_SPEC, bool, O>;
#[doc = "Field `TUNING_COUNTER` reader - Tuning counter"]
pub type TUNING_COUNTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING_COUNTER` writer - Tuning counter"]
pub type TUNING_COUNTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TUNING_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `TUNING_STEP` reader - TUNING_STEP"]
pub type TUNING_STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING_STEP` writer - TUNING_STEP"]
pub type TUNING_STEP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TUNING_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `TUNING_WINDOW` reader - Data window"]
pub type TUNING_WINDOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING_WINDOW` writer - Data window"]
pub type TUNING_WINDOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TUNING_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `STD_TUNING_EN` reader - Standard tuning circuit and procedure enable"]
pub type STD_TUNING_EN_R = crate::BitReader<bool>;
#[doc = "Field `STD_TUNING_EN` writer - Standard tuning circuit and procedure enable"]
pub type STD_TUNING_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNING_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Tuning start"]
    #[inline(always)]
    pub fn tuning_start_tap(&self) -> TUNING_START_TAP_R {
        TUNING_START_TAP_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Disable command check for standard tuning"]
    #[inline(always)]
    pub fn dis_cmd_chk_for_std_tuning(&self) -> DIS_CMD_CHK_FOR_STD_TUNING_R {
        DIS_CMD_CHK_FOR_STD_TUNING_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Tuning counter"]
    #[inline(always)]
    pub fn tuning_counter(&self) -> TUNING_COUNTER_R {
        TUNING_COUNTER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - TUNING_STEP"]
    #[inline(always)]
    pub fn tuning_step(&self) -> TUNING_STEP_R {
        TUNING_STEP_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Data window"]
    #[inline(always)]
    pub fn tuning_window(&self) -> TUNING_WINDOW_R {
        TUNING_WINDOW_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Standard tuning circuit and procedure enable"]
    #[inline(always)]
    pub fn std_tuning_en(&self) -> STD_TUNING_EN_R {
        STD_TUNING_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Tuning start"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_start_tap(&mut self) -> TUNING_START_TAP_W<0> {
        TUNING_START_TAP_W::new(self)
    }
    #[doc = "Bit 7 - Disable command check for standard tuning"]
    #[inline(always)]
    #[must_use]
    pub fn dis_cmd_chk_for_std_tuning(&mut self) -> DIS_CMD_CHK_FOR_STD_TUNING_W<7> {
        DIS_CMD_CHK_FOR_STD_TUNING_W::new(self)
    }
    #[doc = "Bits 8:15 - Tuning counter"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_counter(&mut self) -> TUNING_COUNTER_W<8> {
        TUNING_COUNTER_W::new(self)
    }
    #[doc = "Bits 16:18 - TUNING_STEP"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_step(&mut self) -> TUNING_STEP_W<16> {
        TUNING_STEP_W::new(self)
    }
    #[doc = "Bits 20:22 - Data window"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_window(&mut self) -> TUNING_WINDOW_W<20> {
        TUNING_WINDOW_W::new(self)
    }
    #[doc = "Bit 24 - Standard tuning circuit and procedure enable"]
    #[inline(always)]
    #[must_use]
    pub fn std_tuning_en(&mut self) -> STD_TUNING_EN_W<24> {
        STD_TUNING_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tuning Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tuning_ctrl](index.html) module"]
pub struct TUNING_CTRL_SPEC;
impl crate::RegisterSpec for TUNING_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tuning_ctrl::R](R) reader structure"]
impl crate::Readable for TUNING_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tuning_ctrl::W](W) writer structure"]
impl crate::Writable for TUNING_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TUNING_CTRL to value 0x0021_2800"]
impl crate::Resettable for TUNING_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0021_2800;
}
