#[doc = "Register `OSC_CONFIG2_TOG` reader"]
pub struct R(crate::R<OSC_CONFIG2_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CONFIG2_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CONFIG2_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CONFIG2_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CONFIG2_TOG` writer"]
pub struct W(crate::W<OSC_CONFIG2_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CONFIG2_TOG_SPEC>;
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
impl From<crate::W<OSC_CONFIG2_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CONFIG2_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT_1M_TRG` reader - The target count used to tune the 1MHz clock frequency"]
pub type COUNT_1M_TRG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT_1M_TRG` writer - The target count used to tune the 1MHz clock frequency"]
pub type COUNT_1M_TRG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONFIG2_TOG_SPEC, u16, u16, 12, O>;
#[doc = "Field `ENABLE_1M` reader - Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
pub type ENABLE_1M_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_1M` writer - Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
pub type ENABLE_1M_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_CONFIG2_TOG_SPEC, bool, O>;
#[doc = "Field `MUX_1M` reader - Mux the corrected or uncorrected 1MHz clock to the output"]
pub type MUX_1M_R = crate::BitReader<bool>;
#[doc = "Field `MUX_1M` writer - Mux the corrected or uncorrected 1MHz clock to the output"]
pub type MUX_1M_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_CONFIG2_TOG_SPEC, bool, O>;
#[doc = "Field `CLK_1M_ERR_FL` reader - Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
pub type CLK_1M_ERR_FL_R = crate::BitReader<bool>;
#[doc = "Field `CLK_1M_ERR_FL` writer - Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
pub type CLK_1M_ERR_FL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OSC_CONFIG2_TOG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub fn count_1m_trg(&self) -> COUNT_1M_TRG_R {
        COUNT_1M_TRG_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub fn enable_1m(&self) -> ENABLE_1M_R {
        ENABLE_1M_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub fn mux_1m(&self) -> MUX_1M_R {
        MUX_1M_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub fn clk_1m_err_fl(&self) -> CLK_1M_ERR_FL_R {
        CLK_1M_ERR_FL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    #[must_use]
    pub fn count_1m_trg(&mut self) -> COUNT_1M_TRG_W<0> {
        COUNT_1M_TRG_W::new(self)
    }
    #[doc = "Bit 16 - Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enable_1m(&mut self) -> ENABLE_1M_W<16> {
        ENABLE_1M_W::new(self)
    }
    #[doc = "Bit 17 - Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    #[must_use]
    pub fn mux_1m(&mut self) -> MUX_1M_W<17> {
        MUX_1M_W::new(self)
    }
    #[doc = "Bit 31 - Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    #[must_use]
    pub fn clk_1m_err_fl(&mut self) -> CLK_1M_ERR_FL_W<31> {
        CLK_1M_ERR_FL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTAL OSC Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config2_tog](index.html) module"]
pub struct OSC_CONFIG2_TOG_SPEC;
impl crate::RegisterSpec for OSC_CONFIG2_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_config2_tog::R](R) reader structure"]
impl crate::Readable for OSC_CONFIG2_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_config2_tog::W](W) writer structure"]
impl crate::Writable for OSC_CONFIG2_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC_CONFIG2_TOG to value 0x0001_02e2"]
impl crate::Resettable for OSC_CONFIG2_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_02e2;
}
