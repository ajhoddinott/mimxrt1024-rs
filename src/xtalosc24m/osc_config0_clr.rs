#[doc = "Register `OSC_CONFIG0_CLR` reader"]
pub struct R(crate::R<OSC_CONFIG0_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CONFIG0_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CONFIG0_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CONFIG0_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CONFIG0_CLR` writer"]
pub struct W(crate::W<OSC_CONFIG0_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CONFIG0_CLR_SPEC>;
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
impl From<crate::W<OSC_CONFIG0_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CONFIG0_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_CONFIG0_CLR_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enables the tuning logic to calculate new RC tuning values"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enables the tuning logic to calculate new RC tuning values"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_CONFIG0_CLR_SPEC, bool, O>;
#[doc = "Field `BYPASS` reader - Bypasses any calculated RC tuning value and uses the programmed register value."]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - Bypasses any calculated RC tuning value and uses the programmed register value."]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_CONFIG0_CLR_SPEC, bool, O>;
#[doc = "Field `INVERT` reader - Invert the stepping of the calculated RC tuning value."]
pub type INVERT_R = crate::BitReader<bool>;
#[doc = "Field `INVERT` writer - Invert the stepping of the calculated RC tuning value."]
pub type INVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_CONFIG0_CLR_SPEC, bool, O>;
#[doc = "Field `RC_OSC_PROG` reader - RC osc. tuning values."]
pub type RC_OSC_PROG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RC_OSC_PROG` writer - RC osc. tuning values."]
pub type RC_OSC_PROG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONFIG0_CLR_SPEC, u8, u8, 8, O>;
#[doc = "Field `HYST_PLUS` reader - Positive hysteresis value"]
pub type HYST_PLUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HYST_PLUS` writer - Positive hysteresis value"]
pub type HYST_PLUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONFIG0_CLR_SPEC, u8, u8, 4, O>;
#[doc = "Field `HYST_MINUS` reader - Negative hysteresis value"]
pub type HYST_MINUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HYST_MINUS` writer - Negative hysteresis value"]
pub type HYST_MINUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONFIG0_CLR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RC_OSC_PROG_CUR` reader - The current tuning value in use."]
pub type RC_OSC_PROG_CUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RC_OSC_PROG_CUR` writer - The current tuning value in use."]
pub type RC_OSC_PROG_CUR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONFIG0_CLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub fn invert(&self) -> INVERT_R {
        INVERT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - RC osc. tuning values."]
    #[inline(always)]
    pub fn rc_osc_prog(&self) -> RC_OSC_PROG_R {
        RC_OSC_PROG_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - Positive hysteresis value"]
    #[inline(always)]
    pub fn hyst_plus(&self) -> HYST_PLUS_R {
        HYST_PLUS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Negative hysteresis value"]
    #[inline(always)]
    pub fn hyst_minus(&self) -> HYST_MINUS_R {
        HYST_MINUS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - The current tuning value in use."]
    #[inline(always)]
    pub fn rc_osc_prog_cur(&self) -> RC_OSC_PROG_CUR_R {
        RC_OSC_PROG_CUR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<2> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 3 - Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    #[must_use]
    pub fn invert(&mut self) -> INVERT_W<3> {
        INVERT_W::new(self)
    }
    #[doc = "Bits 4:11 - RC osc. tuning values."]
    #[inline(always)]
    #[must_use]
    pub fn rc_osc_prog(&mut self) -> RC_OSC_PROG_W<4> {
        RC_OSC_PROG_W::new(self)
    }
    #[doc = "Bits 12:15 - Positive hysteresis value"]
    #[inline(always)]
    #[must_use]
    pub fn hyst_plus(&mut self) -> HYST_PLUS_W<12> {
        HYST_PLUS_W::new(self)
    }
    #[doc = "Bits 16:19 - Negative hysteresis value"]
    #[inline(always)]
    #[must_use]
    pub fn hyst_minus(&mut self) -> HYST_MINUS_W<16> {
        HYST_MINUS_W::new(self)
    }
    #[doc = "Bits 24:31 - The current tuning value in use."]
    #[inline(always)]
    #[must_use]
    pub fn rc_osc_prog_cur(&mut self) -> RC_OSC_PROG_CUR_W<24> {
        RC_OSC_PROG_CUR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTAL OSC Configuration 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config0_clr](index.html) module"]
pub struct OSC_CONFIG0_CLR_SPEC;
impl crate::RegisterSpec for OSC_CONFIG0_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_config0_clr::R](R) reader structure"]
impl crate::Readable for OSC_CONFIG0_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_config0_clr::W](W) writer structure"]
impl crate::Writable for OSC_CONFIG0_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC_CONFIG0_CLR to value 0x1020"]
impl crate::Resettable for OSC_CONFIG0_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1020;
}
