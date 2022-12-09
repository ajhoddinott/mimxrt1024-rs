#[doc = "Register `MCTL` reader"]
pub struct R(crate::R<MCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTL` writer"]
pub struct W(crate::W<MCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTL_SPEC>;
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
impl From<crate::W<MCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMP_MODE` reader - Sample Mode"]
pub type SAMP_MODE_R = crate::FieldReader<u8, SAMP_MODE_A>;
#[doc = "Sample Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAMP_MODE_A {
    #[doc = "0: use Von Neumann data into both Entropy shifter and Statistical Checker"]
    SAMP_MODE_VON_BOTH = 0,
    #[doc = "1: use raw data into both Entropy shifter and Statistical Checker"]
    SAMP_MODE_RAW_BOTH = 1,
    #[doc = "2: use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    SAMP_MODE_VON_ENT = 2,
    #[doc = "3: undefined/reserved."]
    SAMP_MODE_UNDEF = 3,
}
impl From<SAMP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMP_MODE_A) -> Self {
        variant as _
    }
}
impl SAMP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMP_MODE_A {
        match self.bits {
            0 => SAMP_MODE_A::SAMP_MODE_VON_BOTH,
            1 => SAMP_MODE_A::SAMP_MODE_RAW_BOTH,
            2 => SAMP_MODE_A::SAMP_MODE_VON_ENT,
            3 => SAMP_MODE_A::SAMP_MODE_UNDEF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAMP_MODE_VON_BOTH`"]
    #[inline(always)]
    pub fn is_samp_mode_von_both(&self) -> bool {
        *self == SAMP_MODE_A::SAMP_MODE_VON_BOTH
    }
    #[doc = "Checks if the value of the field is `SAMP_MODE_RAW_BOTH`"]
    #[inline(always)]
    pub fn is_samp_mode_raw_both(&self) -> bool {
        *self == SAMP_MODE_A::SAMP_MODE_RAW_BOTH
    }
    #[doc = "Checks if the value of the field is `SAMP_MODE_VON_ENT`"]
    #[inline(always)]
    pub fn is_samp_mode_von_ent(&self) -> bool {
        *self == SAMP_MODE_A::SAMP_MODE_VON_ENT
    }
    #[doc = "Checks if the value of the field is `SAMP_MODE_UNDEF`"]
    #[inline(always)]
    pub fn is_samp_mode_undef(&self) -> bool {
        *self == SAMP_MODE_A::SAMP_MODE_UNDEF
    }
}
#[doc = "Field `SAMP_MODE` writer - Sample Mode"]
pub type SAMP_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MCTL_SPEC, u8, SAMP_MODE_A, 2, O>;
impl<'a, const O: u8> SAMP_MODE_W<'a, O> {
    #[doc = "use Von Neumann data into both Entropy shifter and Statistical Checker"]
    #[inline(always)]
    pub fn samp_mode_von_both(self) -> &'a mut W {
        self.variant(SAMP_MODE_A::SAMP_MODE_VON_BOTH)
    }
    #[doc = "use raw data into both Entropy shifter and Statistical Checker"]
    #[inline(always)]
    pub fn samp_mode_raw_both(self) -> &'a mut W {
        self.variant(SAMP_MODE_A::SAMP_MODE_RAW_BOTH)
    }
    #[doc = "use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    #[inline(always)]
    pub fn samp_mode_von_ent(self) -> &'a mut W {
        self.variant(SAMP_MODE_A::SAMP_MODE_VON_ENT)
    }
    #[doc = "undefined/reserved."]
    #[inline(always)]
    pub fn samp_mode_undef(self) -> &'a mut W {
        self.variant(SAMP_MODE_A::SAMP_MODE_UNDEF)
    }
}
#[doc = "Field `OSC_DIV` reader - Oscillator Divide"]
pub type OSC_DIV_R = crate::FieldReader<u8, OSC_DIV_A>;
#[doc = "Oscillator Divide\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSC_DIV_A {
    #[doc = "0: use ring oscillator with no divide"]
    OSC_DIV_DIV1 = 0,
    #[doc = "1: use ring oscillator divided-by-2"]
    OSC_DIV_DIV2 = 1,
    #[doc = "2: use ring oscillator divided-by-4"]
    OSC_DIV_DIV4 = 2,
    #[doc = "3: use ring oscillator divided-by-8"]
    OSC_DIV_DIV8 = 3,
}
impl From<OSC_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC_DIV_A) -> Self {
        variant as _
    }
}
impl OSC_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC_DIV_A {
        match self.bits {
            0 => OSC_DIV_A::OSC_DIV_DIV1,
            1 => OSC_DIV_A::OSC_DIV_DIV2,
            2 => OSC_DIV_A::OSC_DIV_DIV4,
            3 => OSC_DIV_A::OSC_DIV_DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OSC_DIV_DIV1`"]
    #[inline(always)]
    pub fn is_osc_div_div1(&self) -> bool {
        *self == OSC_DIV_A::OSC_DIV_DIV1
    }
    #[doc = "Checks if the value of the field is `OSC_DIV_DIV2`"]
    #[inline(always)]
    pub fn is_osc_div_div2(&self) -> bool {
        *self == OSC_DIV_A::OSC_DIV_DIV2
    }
    #[doc = "Checks if the value of the field is `OSC_DIV_DIV4`"]
    #[inline(always)]
    pub fn is_osc_div_div4(&self) -> bool {
        *self == OSC_DIV_A::OSC_DIV_DIV4
    }
    #[doc = "Checks if the value of the field is `OSC_DIV_DIV8`"]
    #[inline(always)]
    pub fn is_osc_div_div8(&self) -> bool {
        *self == OSC_DIV_A::OSC_DIV_DIV8
    }
}
#[doc = "Field `OSC_DIV` writer - Oscillator Divide"]
pub type OSC_DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MCTL_SPEC, u8, OSC_DIV_A, 2, O>;
impl<'a, const O: u8> OSC_DIV_W<'a, O> {
    #[doc = "use ring oscillator with no divide"]
    #[inline(always)]
    pub fn osc_div_div1(self) -> &'a mut W {
        self.variant(OSC_DIV_A::OSC_DIV_DIV1)
    }
    #[doc = "use ring oscillator divided-by-2"]
    #[inline(always)]
    pub fn osc_div_div2(self) -> &'a mut W {
        self.variant(OSC_DIV_A::OSC_DIV_DIV2)
    }
    #[doc = "use ring oscillator divided-by-4"]
    #[inline(always)]
    pub fn osc_div_div4(self) -> &'a mut W {
        self.variant(OSC_DIV_A::OSC_DIV_DIV4)
    }
    #[doc = "use ring oscillator divided-by-8"]
    #[inline(always)]
    pub fn osc_div_div8(self) -> &'a mut W {
        self.variant(OSC_DIV_A::OSC_DIV_DIV8)
    }
}
#[doc = "Field `UNUSED4` reader - This bit is unused. Always reads zero."]
pub type UNUSED4_R = crate::BitReader<bool>;
#[doc = "Field `UNUSED5` reader - This bit is unused. Always reads zero."]
pub type UNUSED5_R = crate::BitReader<bool>;
#[doc = "Field `RST_DEF` writer - Reset Defaults"]
pub type RST_DEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCTL_SPEC, bool, O>;
#[doc = "Field `FOR_SCLK` reader - Force System Clock"]
pub type FOR_SCLK_R = crate::BitReader<bool>;
#[doc = "Field `FOR_SCLK` writer - Force System Clock"]
pub type FOR_SCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCTL_SPEC, bool, O>;
#[doc = "Field `FCT_FAIL` reader - Read only: Frequency Count Fail"]
pub type FCT_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `FCT_VAL` reader - Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from FRQCNT."]
pub type FCT_VAL_R = crate::BitReader<bool>;
#[doc = "Field `ENT_VAL` reader - Read only: Entropy Valid"]
pub type ENT_VAL_R = crate::BitReader<bool>;
#[doc = "Field `TST_OUT` reader - Read only: Test point inside ring oscillator."]
pub type TST_OUT_R = crate::BitReader<bool>;
#[doc = "Field `ERR` reader - Read: Error status"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - Read: Error status"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MCTL_SPEC, bool, O>;
#[doc = "Field `TSTOP_OK` reader - TRNG_OK_TO_STOP"]
pub type TSTOP_OK_R = crate::BitReader<bool>;
#[doc = "Field `LRUN_CONT` reader - Long run count continues between entropy generations"]
pub type LRUN_CONT_R = crate::BitReader<bool>;
#[doc = "Field `LRUN_CONT` writer - Long run count continues between entropy generations"]
pub type LRUN_CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCTL_SPEC, bool, O>;
#[doc = "Field `PRGM` reader - Programming Mode Select"]
pub type PRGM_R = crate::BitReader<bool>;
#[doc = "Field `PRGM` writer - Programming Mode Select"]
pub type PRGM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Sample Mode"]
    #[inline(always)]
    pub fn samp_mode(&self) -> SAMP_MODE_R {
        SAMP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Oscillator Divide"]
    #[inline(always)]
    pub fn osc_div(&self) -> OSC_DIV_R {
        OSC_DIV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - This bit is unused. Always reads zero."]
    #[inline(always)]
    pub fn unused4(&self) -> UNUSED4_R {
        UNUSED4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is unused. Always reads zero."]
    #[inline(always)]
    pub fn unused5(&self) -> UNUSED5_R {
        UNUSED5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Force System Clock"]
    #[inline(always)]
    pub fn for_sclk(&self) -> FOR_SCLK_R {
        FOR_SCLK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read only: Frequency Count Fail"]
    #[inline(always)]
    pub fn fct_fail(&self) -> FCT_FAIL_R {
        FCT_FAIL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from FRQCNT."]
    #[inline(always)]
    pub fn fct_val(&self) -> FCT_VAL_R {
        FCT_VAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read only: Entropy Valid"]
    #[inline(always)]
    pub fn ent_val(&self) -> ENT_VAL_R {
        ENT_VAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Read only: Test point inside ring oscillator."]
    #[inline(always)]
    pub fn tst_out(&self) -> TST_OUT_R {
        TST_OUT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Read: Error status"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TRNG_OK_TO_STOP"]
    #[inline(always)]
    pub fn tstop_ok(&self) -> TSTOP_OK_R {
        TSTOP_OK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Long run count continues between entropy generations"]
    #[inline(always)]
    pub fn lrun_cont(&self) -> LRUN_CONT_R {
        LRUN_CONT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Programming Mode Select"]
    #[inline(always)]
    pub fn prgm(&self) -> PRGM_R {
        PRGM_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sample Mode"]
    #[inline(always)]
    #[must_use]
    pub fn samp_mode(&mut self) -> SAMP_MODE_W<0> {
        SAMP_MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Oscillator Divide"]
    #[inline(always)]
    #[must_use]
    pub fn osc_div(&mut self) -> OSC_DIV_W<2> {
        OSC_DIV_W::new(self)
    }
    #[doc = "Bit 6 - Reset Defaults"]
    #[inline(always)]
    #[must_use]
    pub fn rst_def(&mut self) -> RST_DEF_W<6> {
        RST_DEF_W::new(self)
    }
    #[doc = "Bit 7 - Force System Clock"]
    #[inline(always)]
    #[must_use]
    pub fn for_sclk(&mut self) -> FOR_SCLK_W<7> {
        FOR_SCLK_W::new(self)
    }
    #[doc = "Bit 12 - Read: Error status"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<12> {
        ERR_W::new(self)
    }
    #[doc = "Bit 14 - Long run count continues between entropy generations"]
    #[inline(always)]
    #[must_use]
    pub fn lrun_cont(&mut self) -> LRUN_CONT_W<14> {
        LRUN_CONT_W::new(self)
    }
    #[doc = "Bit 16 - Programming Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn prgm(&mut self) -> PRGM_W<16> {
        PRGM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctl](index.html) module"]
pub struct MCTL_SPEC;
impl crate::RegisterSpec for MCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctl::R](R) reader structure"]
impl crate::Readable for MCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctl::W](W) writer structure"]
impl crate::Writable for MCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1000;
}
#[doc = "`reset()` method sets MCTL to value 0x0001_2001"]
impl crate::Resettable for MCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_2001;
}
