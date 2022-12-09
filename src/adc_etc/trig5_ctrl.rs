#[doc = "Register `TRIG5_CTRL` reader"]
pub struct R(crate::R<TRIG5_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIG5_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIG5_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIG5_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIG5_CTRL` writer"]
pub struct W(crate::W<TRIG5_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIG5_CTRL_SPEC>;
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
impl From<crate::W<TRIG5_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIG5_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_TRIG` reader - Software trigger. This field is self-clearing."]
pub type SW_TRIG_R = crate::BitReader<SW_TRIG_A>;
#[doc = "Software trigger. This field is self-clearing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW_TRIG_A {
    #[doc = "0: No software trigger event generated."]
    SW_TRIG_0 = 0,
    #[doc = "1: Software trigger event generated."]
    SW_TRIG_1 = 1,
}
impl From<SW_TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: SW_TRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl SW_TRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_TRIG_A {
        match self.bits {
            false => SW_TRIG_A::SW_TRIG_0,
            true => SW_TRIG_A::SW_TRIG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SW_TRIG_0`"]
    #[inline(always)]
    pub fn is_sw_trig_0(&self) -> bool {
        *self == SW_TRIG_A::SW_TRIG_0
    }
    #[doc = "Checks if the value of the field is `SW_TRIG_1`"]
    #[inline(always)]
    pub fn is_sw_trig_1(&self) -> bool {
        *self == SW_TRIG_A::SW_TRIG_1
    }
}
#[doc = "Field `SW_TRIG` writer - Software trigger. This field is self-clearing."]
pub type SW_TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIG5_CTRL_SPEC, SW_TRIG_A, O>;
impl<'a, const O: u8> SW_TRIG_W<'a, O> {
    #[doc = "No software trigger event generated."]
    #[inline(always)]
    pub fn sw_trig_0(self) -> &'a mut W {
        self.variant(SW_TRIG_A::SW_TRIG_0)
    }
    #[doc = "Software trigger event generated."]
    #[inline(always)]
    pub fn sw_trig_1(self) -> &'a mut W {
        self.variant(SW_TRIG_A::SW_TRIG_1)
    }
}
#[doc = "Field `TRIG_MODE` reader - Trigger mode selection."]
pub type TRIG_MODE_R = crate::BitReader<TRIG_MODE_A>;
#[doc = "Trigger mode selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG_MODE_A {
    #[doc = "0: Hardware trigger. The softerware trigger will be ignored."]
    TRIG_MODE_0 = 0,
    #[doc = "1: Software trigger. The hardware trigger will be ignored."]
    TRIG_MODE_1 = 1,
}
impl From<TRIG_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG_MODE_A {
        match self.bits {
            false => TRIG_MODE_A::TRIG_MODE_0,
            true => TRIG_MODE_A::TRIG_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_MODE_0`"]
    #[inline(always)]
    pub fn is_trig_mode_0(&self) -> bool {
        *self == TRIG_MODE_A::TRIG_MODE_0
    }
    #[doc = "Checks if the value of the field is `TRIG_MODE_1`"]
    #[inline(always)]
    pub fn is_trig_mode_1(&self) -> bool {
        *self == TRIG_MODE_A::TRIG_MODE_1
    }
}
#[doc = "Field `TRIG_MODE` writer - Trigger mode selection."]
pub type TRIG_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIG5_CTRL_SPEC, TRIG_MODE_A, O>;
impl<'a, const O: u8> TRIG_MODE_W<'a, O> {
    #[doc = "Hardware trigger. The softerware trigger will be ignored."]
    #[inline(always)]
    pub fn trig_mode_0(self) -> &'a mut W {
        self.variant(TRIG_MODE_A::TRIG_MODE_0)
    }
    #[doc = "Software trigger. The hardware trigger will be ignored."]
    #[inline(always)]
    pub fn trig_mode_1(self) -> &'a mut W {
        self.variant(TRIG_MODE_A::TRIG_MODE_1)
    }
}
#[doc = "Field `TRIG_CHAIN` reader - The number of segments inside the trigger chain of TRIGa."]
pub type TRIG_CHAIN_R = crate::FieldReader<u8, TRIG_CHAIN_A>;
#[doc = "The number of segments inside the trigger chain of TRIGa.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIG_CHAIN_A {
    #[doc = "0: Trigger chain length is 1"]
    TRIG_CHAIN_0 = 0,
    #[doc = "1: Trigger chain length is 2"]
    TRIG_CHAIN_1 = 1,
    #[doc = "2: Trigger chain length is 3"]
    TRIG_CHAIN_2 = 2,
    #[doc = "3: Trigger chain length is 4"]
    TRIG_CHAIN_3 = 3,
    #[doc = "4: Trigger chain length is 5"]
    TRIG_CHAIN_4 = 4,
    #[doc = "5: Trigger chain length is 6"]
    TRIG_CHAIN_5 = 5,
    #[doc = "6: Trigger chain length is 7"]
    TRIG_CHAIN_6 = 6,
    #[doc = "7: Trigger chain length is 8"]
    TRIG_CHAIN_7 = 7,
}
impl From<TRIG_CHAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIG_CHAIN_A) -> Self {
        variant as _
    }
}
impl TRIG_CHAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG_CHAIN_A {
        match self.bits {
            0 => TRIG_CHAIN_A::TRIG_CHAIN_0,
            1 => TRIG_CHAIN_A::TRIG_CHAIN_1,
            2 => TRIG_CHAIN_A::TRIG_CHAIN_2,
            3 => TRIG_CHAIN_A::TRIG_CHAIN_3,
            4 => TRIG_CHAIN_A::TRIG_CHAIN_4,
            5 => TRIG_CHAIN_A::TRIG_CHAIN_5,
            6 => TRIG_CHAIN_A::TRIG_CHAIN_6,
            7 => TRIG_CHAIN_A::TRIG_CHAIN_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_CHAIN_0`"]
    #[inline(always)]
    pub fn is_trig_chain_0(&self) -> bool {
        *self == TRIG_CHAIN_A::TRIG_CHAIN_0
    }
    #[doc = "Checks if the value of the field is `TRIG_CHAIN_1`"]
    #[inline(always)]
    pub fn is_trig_chain_1(&self) -> bool {
        *self == TRIG_CHAIN_A::TRIG_CHAIN_1
    }
    #[doc = "Checks if the value of the field is `TRIG_CHAIN_2`"]
    #[inline(always)]
    pub fn is_trig_chain_2(&self) -> bool {
        *self == TRIG_CHAIN_A::TRIG_CHAIN_2
    }
    #[doc = "Checks if the value of the field is `TRIG_CHAIN_3`"]
    #[inline(always)]
    pub fn is_trig_chain_3(&self) -> bool {
        *self == TRIG_CHAIN_A::TRIG_CHAIN_3
    }
    #[doc = "Checks if the value of the field is `TRIG_CHAIN_4`"]
    #[inline(always)]
    pub fn is_trig_chain_4(&self) -> bool {
        *self == TRIG_CHAIN_A::TRIG_CHAIN_4
    }
    #[doc = "Checks if the value of the field is `TRIG_CHAIN_5`"]
    #[inline(always)]
    pub fn is_trig_chain_5(&self) -> bool {
        *self == TRIG_CHAIN_A::TRIG_CHAIN_5
    }
    #[doc = "Checks if the value of the field is `TRIG_CHAIN_6`"]
    #[inline(always)]
    pub fn is_trig_chain_6(&self) -> bool {
        *self == TRIG_CHAIN_A::TRIG_CHAIN_6
    }
    #[doc = "Checks if the value of the field is `TRIG_CHAIN_7`"]
    #[inline(always)]
    pub fn is_trig_chain_7(&self) -> bool {
        *self == TRIG_CHAIN_A::TRIG_CHAIN_7
    }
}
#[doc = "Field `TRIG_CHAIN` writer - The number of segments inside the trigger chain of TRIGa."]
pub type TRIG_CHAIN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG5_CTRL_SPEC, u8, TRIG_CHAIN_A, 3, O>;
impl<'a, const O: u8> TRIG_CHAIN_W<'a, O> {
    #[doc = "Trigger chain length is 1"]
    #[inline(always)]
    pub fn trig_chain_0(self) -> &'a mut W {
        self.variant(TRIG_CHAIN_A::TRIG_CHAIN_0)
    }
    #[doc = "Trigger chain length is 2"]
    #[inline(always)]
    pub fn trig_chain_1(self) -> &'a mut W {
        self.variant(TRIG_CHAIN_A::TRIG_CHAIN_1)
    }
    #[doc = "Trigger chain length is 3"]
    #[inline(always)]
    pub fn trig_chain_2(self) -> &'a mut W {
        self.variant(TRIG_CHAIN_A::TRIG_CHAIN_2)
    }
    #[doc = "Trigger chain length is 4"]
    #[inline(always)]
    pub fn trig_chain_3(self) -> &'a mut W {
        self.variant(TRIG_CHAIN_A::TRIG_CHAIN_3)
    }
    #[doc = "Trigger chain length is 5"]
    #[inline(always)]
    pub fn trig_chain_4(self) -> &'a mut W {
        self.variant(TRIG_CHAIN_A::TRIG_CHAIN_4)
    }
    #[doc = "Trigger chain length is 6"]
    #[inline(always)]
    pub fn trig_chain_5(self) -> &'a mut W {
        self.variant(TRIG_CHAIN_A::TRIG_CHAIN_5)
    }
    #[doc = "Trigger chain length is 7"]
    #[inline(always)]
    pub fn trig_chain_6(self) -> &'a mut W {
        self.variant(TRIG_CHAIN_A::TRIG_CHAIN_6)
    }
    #[doc = "Trigger chain length is 8"]
    #[inline(always)]
    pub fn trig_chain_7(self) -> &'a mut W {
        self.variant(TRIG_CHAIN_A::TRIG_CHAIN_7)
    }
}
#[doc = "Field `TRIG_PRIORITY` reader - External trigger priority, 7 is highest priority, while 0 is lowest"]
pub type TRIG_PRIORITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIG_PRIORITY` writer - External trigger priority, 7 is highest priority, while 0 is lowest"]
pub type TRIG_PRIORITY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIG5_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SYNC_MODE` reader - Trigger synchronization mode selection"]
pub type SYNC_MODE_R = crate::BitReader<SYNC_MODE_A>;
#[doc = "Trigger synchronization mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_MODE_A {
    #[doc = "0: Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
    SYNC_MODE_0 = 0,
    #[doc = "1: Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
    SYNC_MODE_1 = 1,
}
impl From<SYNC_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_MODE_A {
        match self.bits {
            false => SYNC_MODE_A::SYNC_MODE_0,
            true => SYNC_MODE_A::SYNC_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC_MODE_0`"]
    #[inline(always)]
    pub fn is_sync_mode_0(&self) -> bool {
        *self == SYNC_MODE_A::SYNC_MODE_0
    }
    #[doc = "Checks if the value of the field is `SYNC_MODE_1`"]
    #[inline(always)]
    pub fn is_sync_mode_1(&self) -> bool {
        *self == SYNC_MODE_A::SYNC_MODE_1
    }
}
#[doc = "Field `SYNC_MODE` writer - Trigger synchronization mode selection"]
pub type SYNC_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIG5_CTRL_SPEC, SYNC_MODE_A, O>;
impl<'a, const O: u8> SYNC_MODE_W<'a, O> {
    #[doc = "Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently."]
    #[inline(always)]
    pub fn sync_mode_0(self) -> &'a mut W {
        self.variant(SYNC_MODE_A::SYNC_MODE_0)
    }
    #[doc = "Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously."]
    #[inline(always)]
    pub fn sync_mode_1(self) -> &'a mut W {
        self.variant(SYNC_MODE_A::SYNC_MODE_1)
    }
}
impl R {
    #[doc = "Bit 0 - Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub fn sw_trig(&self) -> SW_TRIG_R {
        SW_TRIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Trigger mode selection."]
    #[inline(always)]
    pub fn trig_mode(&self) -> TRIG_MODE_R {
        TRIG_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub fn trig_chain(&self) -> TRIG_CHAIN_R {
        TRIG_CHAIN_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub fn trig_priority(&self) -> TRIG_PRIORITY_R {
        TRIG_PRIORITY_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Trigger synchronization mode selection"]
    #[inline(always)]
    pub fn sync_mode(&self) -> SYNC_MODE_R {
        SYNC_MODE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software trigger. This field is self-clearing."]
    #[inline(always)]
    #[must_use]
    pub fn sw_trig(&mut self) -> SW_TRIG_W<0> {
        SW_TRIG_W::new(self)
    }
    #[doc = "Bit 4 - Trigger mode selection."]
    #[inline(always)]
    #[must_use]
    pub fn trig_mode(&mut self) -> TRIG_MODE_W<4> {
        TRIG_MODE_W::new(self)
    }
    #[doc = "Bits 8:10 - The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    #[must_use]
    pub fn trig_chain(&mut self) -> TRIG_CHAIN_W<8> {
        TRIG_CHAIN_W::new(self)
    }
    #[doc = "Bits 12:14 - External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    #[must_use]
    pub fn trig_priority(&mut self) -> TRIG_PRIORITY_W<12> {
        TRIG_PRIORITY_W::new(self)
    }
    #[doc = "Bit 16 - Trigger synchronization mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn sync_mode(&mut self) -> SYNC_MODE_W<16> {
        SYNC_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETC_TRIG Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig5_ctrl](index.html) module"]
pub struct TRIG5_CTRL_SPEC;
impl crate::RegisterSpec for TRIG5_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trig5_ctrl::R](R) reader structure"]
impl crate::Readable for TRIG5_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trig5_ctrl::W](W) writer structure"]
impl crate::Writable for TRIG5_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIG5_CTRL to value 0"]
impl crate::Resettable for TRIG5_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
