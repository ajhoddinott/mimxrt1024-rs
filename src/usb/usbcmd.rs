#[doc = "Register `USBCMD` reader"]
pub struct R(crate::R<USBCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCMD` writer"]
pub struct W(crate::W<USBCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCMD_SPEC>;
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
impl From<crate::W<USBCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS` reader - Run/Stop (RS) - Read/Write"]
pub type RS_R = crate::BitReader<bool>;
#[doc = "Field `RS` writer - Run/Stop (RS) - Read/Write"]
pub type RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `RST` reader - Controller Reset (RESET) - Read/Write"]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - Controller Reset (RESET) - Read/Write"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `FS_1` reader - See description at bit 15"]
pub type FS_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS_1` writer - See description at bit 15"]
pub type FS_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBCMD_SPEC, u8, u8, 2, O>;
#[doc = "Field `PSE` reader - Periodic Schedule Enable- Read/Write"]
pub type PSE_R = crate::BitReader<PSE_A>;
#[doc = "Periodic Schedule Enable- Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSE_A {
    #[doc = "0: Do not process the Periodic Schedule"]
    PSE_0 = 0,
    #[doc = "1: Use the PERIODICLISTBASE register to access the Periodic Schedule."]
    PSE_1 = 1,
}
impl From<PSE_A> for bool {
    #[inline(always)]
    fn from(variant: PSE_A) -> Self {
        variant as u8 != 0
    }
}
impl PSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSE_A {
        match self.bits {
            false => PSE_A::PSE_0,
            true => PSE_A::PSE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PSE_0`"]
    #[inline(always)]
    pub fn is_pse_0(&self) -> bool {
        *self == PSE_A::PSE_0
    }
    #[doc = "Checks if the value of the field is `PSE_1`"]
    #[inline(always)]
    pub fn is_pse_1(&self) -> bool {
        *self == PSE_A::PSE_1
    }
}
#[doc = "Field `PSE` writer - Periodic Schedule Enable- Read/Write"]
pub type PSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, PSE_A, O>;
impl<'a, const O: u8> PSE_W<'a, O> {
    #[doc = "Do not process the Periodic Schedule"]
    #[inline(always)]
    pub fn pse_0(self) -> &'a mut W {
        self.variant(PSE_A::PSE_0)
    }
    #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule."]
    #[inline(always)]
    pub fn pse_1(self) -> &'a mut W {
        self.variant(PSE_A::PSE_1)
    }
}
#[doc = "Field `ASE` reader - Asynchronous Schedule Enable - Read/Write"]
pub type ASE_R = crate::BitReader<ASE_A>;
#[doc = "Asynchronous Schedule Enable - Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASE_A {
    #[doc = "0: Do not process the Asynchronous Schedule."]
    ASE_0 = 0,
    #[doc = "1: Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
    ASE_1 = 1,
}
impl From<ASE_A> for bool {
    #[inline(always)]
    fn from(variant: ASE_A) -> Self {
        variant as u8 != 0
    }
}
impl ASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASE_A {
        match self.bits {
            false => ASE_A::ASE_0,
            true => ASE_A::ASE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ASE_0`"]
    #[inline(always)]
    pub fn is_ase_0(&self) -> bool {
        *self == ASE_A::ASE_0
    }
    #[doc = "Checks if the value of the field is `ASE_1`"]
    #[inline(always)]
    pub fn is_ase_1(&self) -> bool {
        *self == ASE_A::ASE_1
    }
}
#[doc = "Field `ASE` writer - Asynchronous Schedule Enable - Read/Write"]
pub type ASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, ASE_A, O>;
impl<'a, const O: u8> ASE_W<'a, O> {
    #[doc = "Do not process the Asynchronous Schedule."]
    #[inline(always)]
    pub fn ase_0(self) -> &'a mut W {
        self.variant(ASE_A::ASE_0)
    }
    #[doc = "Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
    #[inline(always)]
    pub fn ase_1(self) -> &'a mut W {
        self.variant(ASE_A::ASE_1)
    }
}
#[doc = "Field `IAA` reader - Interrupt on Async Advance Doorbell - Read/Write"]
pub type IAA_R = crate::BitReader<bool>;
#[doc = "Field `IAA` writer - Interrupt on Async Advance Doorbell - Read/Write"]
pub type IAA_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `ASP` reader - Asynchronous Schedule Park Mode Count - Read/Write"]
pub type ASP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ASP` writer - Asynchronous Schedule Park Mode Count - Read/Write"]
pub type ASP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBCMD_SPEC, u8, u8, 2, O>;
#[doc = "Field `ASPE` reader - Asynchronous Schedule Park Mode Enable - Read/Write"]
pub type ASPE_R = crate::BitReader<bool>;
#[doc = "Field `ASPE` writer - Asynchronous Schedule Park Mode Enable - Read/Write"]
pub type ASPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `SUTW` reader - Setup TripWire - Read/Write"]
pub type SUTW_R = crate::BitReader<bool>;
#[doc = "Field `SUTW` writer - Setup TripWire - Read/Write"]
pub type SUTW_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `ATDTW` reader - Add dTD TripWire - Read/Write"]
pub type ATDTW_R = crate::BitReader<bool>;
#[doc = "Field `ATDTW` writer - Add dTD TripWire - Read/Write"]
pub type ATDTW_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `FS_2` reader - Frame List Size - (Read/Write or Read Only)"]
pub type FS_2_R = crate::BitReader<FS_2_A>;
#[doc = "Frame List Size - (Read/Write or Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FS_2_A {
    #[doc = "0: 1024 elements (4096 bytes) Default value"]
    FS_2_0 = 0,
    #[doc = "1: 512 elements (2048 bytes)"]
    FS_2_1 = 1,
}
impl From<FS_2_A> for bool {
    #[inline(always)]
    fn from(variant: FS_2_A) -> Self {
        variant as u8 != 0
    }
}
impl FS_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS_2_A {
        match self.bits {
            false => FS_2_A::FS_2_0,
            true => FS_2_A::FS_2_1,
        }
    }
    #[doc = "Checks if the value of the field is `FS_2_0`"]
    #[inline(always)]
    pub fn is_fs_2_0(&self) -> bool {
        *self == FS_2_A::FS_2_0
    }
    #[doc = "Checks if the value of the field is `FS_2_1`"]
    #[inline(always)]
    pub fn is_fs_2_1(&self) -> bool {
        *self == FS_2_A::FS_2_1
    }
}
#[doc = "Field `FS_2` writer - Frame List Size - (Read/Write or Read Only)"]
pub type FS_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, FS_2_A, O>;
impl<'a, const O: u8> FS_2_W<'a, O> {
    #[doc = "1024 elements (4096 bytes) Default value"]
    #[inline(always)]
    pub fn fs_2_0(self) -> &'a mut W {
        self.variant(FS_2_A::FS_2_0)
    }
    #[doc = "512 elements (2048 bytes)"]
    #[inline(always)]
    pub fn fs_2_1(self) -> &'a mut W {
        self.variant(FS_2_A::FS_2_1)
    }
}
#[doc = "Field `ITC` reader - Interrupt Threshold Control -Read/Write"]
pub type ITC_R = crate::FieldReader<u8, ITC_A>;
#[doc = "Interrupt Threshold Control -Read/Write\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ITC_A {
    #[doc = "0: Immediate (no threshold)"]
    ITC_0 = 0,
    #[doc = "1: 1 micro-frame"]
    ITC_1 = 1,
    #[doc = "2: 2 micro-frames"]
    ITC_2 = 2,
    #[doc = "4: 4 micro-frames"]
    ITC_4 = 4,
    #[doc = "8: 8 micro-frames"]
    ITC_8 = 8,
    #[doc = "16: 16 micro-frames"]
    ITC_16 = 16,
    #[doc = "32: 32 micro-frames"]
    ITC_32 = 32,
    #[doc = "64: 64 micro-frames"]
    ITC_64 = 64,
}
impl From<ITC_A> for u8 {
    #[inline(always)]
    fn from(variant: ITC_A) -> Self {
        variant as _
    }
}
impl ITC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ITC_A> {
        match self.bits {
            0 => Some(ITC_A::ITC_0),
            1 => Some(ITC_A::ITC_1),
            2 => Some(ITC_A::ITC_2),
            4 => Some(ITC_A::ITC_4),
            8 => Some(ITC_A::ITC_8),
            16 => Some(ITC_A::ITC_16),
            32 => Some(ITC_A::ITC_32),
            64 => Some(ITC_A::ITC_64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ITC_0`"]
    #[inline(always)]
    pub fn is_itc_0(&self) -> bool {
        *self == ITC_A::ITC_0
    }
    #[doc = "Checks if the value of the field is `ITC_1`"]
    #[inline(always)]
    pub fn is_itc_1(&self) -> bool {
        *self == ITC_A::ITC_1
    }
    #[doc = "Checks if the value of the field is `ITC_2`"]
    #[inline(always)]
    pub fn is_itc_2(&self) -> bool {
        *self == ITC_A::ITC_2
    }
    #[doc = "Checks if the value of the field is `ITC_4`"]
    #[inline(always)]
    pub fn is_itc_4(&self) -> bool {
        *self == ITC_A::ITC_4
    }
    #[doc = "Checks if the value of the field is `ITC_8`"]
    #[inline(always)]
    pub fn is_itc_8(&self) -> bool {
        *self == ITC_A::ITC_8
    }
    #[doc = "Checks if the value of the field is `ITC_16`"]
    #[inline(always)]
    pub fn is_itc_16(&self) -> bool {
        *self == ITC_A::ITC_16
    }
    #[doc = "Checks if the value of the field is `ITC_32`"]
    #[inline(always)]
    pub fn is_itc_32(&self) -> bool {
        *self == ITC_A::ITC_32
    }
    #[doc = "Checks if the value of the field is `ITC_64`"]
    #[inline(always)]
    pub fn is_itc_64(&self) -> bool {
        *self == ITC_A::ITC_64
    }
}
#[doc = "Field `ITC` writer - Interrupt Threshold Control -Read/Write"]
pub type ITC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBCMD_SPEC, u8, ITC_A, 8, O>;
impl<'a, const O: u8> ITC_W<'a, O> {
    #[doc = "Immediate (no threshold)"]
    #[inline(always)]
    pub fn itc_0(self) -> &'a mut W {
        self.variant(ITC_A::ITC_0)
    }
    #[doc = "1 micro-frame"]
    #[inline(always)]
    pub fn itc_1(self) -> &'a mut W {
        self.variant(ITC_A::ITC_1)
    }
    #[doc = "2 micro-frames"]
    #[inline(always)]
    pub fn itc_2(self) -> &'a mut W {
        self.variant(ITC_A::ITC_2)
    }
    #[doc = "4 micro-frames"]
    #[inline(always)]
    pub fn itc_4(self) -> &'a mut W {
        self.variant(ITC_A::ITC_4)
    }
    #[doc = "8 micro-frames"]
    #[inline(always)]
    pub fn itc_8(self) -> &'a mut W {
        self.variant(ITC_A::ITC_8)
    }
    #[doc = "16 micro-frames"]
    #[inline(always)]
    pub fn itc_16(self) -> &'a mut W {
        self.variant(ITC_A::ITC_16)
    }
    #[doc = "32 micro-frames"]
    #[inline(always)]
    pub fn itc_32(self) -> &'a mut W {
        self.variant(ITC_A::ITC_32)
    }
    #[doc = "64 micro-frames"]
    #[inline(always)]
    pub fn itc_64(self) -> &'a mut W {
        self.variant(ITC_A::ITC_64)
    }
}
impl R {
    #[doc = "Bit 0 - Run/Stop (RS) - Read/Write"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controller Reset (RESET) - Read/Write"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - See description at bit 15"]
    #[inline(always)]
    pub fn fs_1(&self) -> FS_1_R {
        FS_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Periodic Schedule Enable- Read/Write"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable - Read/Write"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on Async Advance Doorbell - Read/Write"]
    #[inline(always)]
    pub fn iaa(&self) -> IAA_R {
        IAA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park Mode Count - Read/Write"]
    #[inline(always)]
    pub fn asp(&self) -> ASP_R {
        ASP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable - Read/Write"]
    #[inline(always)]
    pub fn aspe(&self) -> ASPE_R {
        ASPE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Setup TripWire - Read/Write"]
    #[inline(always)]
    pub fn sutw(&self) -> SUTW_R {
        SUTW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Add dTD TripWire - Read/Write"]
    #[inline(always)]
    pub fn atdtw(&self) -> ATDTW_R {
        ATDTW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Frame List Size - (Read/Write or Read Only)"]
    #[inline(always)]
    pub fn fs_2(&self) -> FS_2_R {
        FS_2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control -Read/Write"]
    #[inline(always)]
    pub fn itc(&self) -> ITC_R {
        ITC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop (RS) - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<0> {
        RS_W::new(self)
    }
    #[doc = "Bit 1 - Controller Reset (RESET) - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<1> {
        RST_W::new(self)
    }
    #[doc = "Bits 2:3 - See description at bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn fs_1(&mut self) -> FS_1_W<2> {
        FS_1_W::new(self)
    }
    #[doc = "Bit 4 - Periodic Schedule Enable- Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PSE_W<4> {
        PSE_W::new(self)
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn ase(&mut self) -> ASE_W<5> {
        ASE_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt on Async Advance Doorbell - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn iaa(&mut self) -> IAA_W<6> {
        IAA_W::new(self)
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park Mode Count - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn asp(&mut self) -> ASP_W<8> {
        ASP_W::new(self)
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn aspe(&mut self) -> ASPE_W<11> {
        ASPE_W::new(self)
    }
    #[doc = "Bit 13 - Setup TripWire - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn sutw(&mut self) -> SUTW_W<13> {
        SUTW_W::new(self)
    }
    #[doc = "Bit 14 - Add dTD TripWire - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn atdtw(&mut self) -> ATDTW_W<14> {
        ATDTW_W::new(self)
    }
    #[doc = "Bit 15 - Frame List Size - (Read/Write or Read Only)"]
    #[inline(always)]
    #[must_use]
    pub fn fs_2(&mut self) -> FS_2_W<15> {
        FS_2_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control -Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn itc(&mut self) -> ITC_W<16> {
        ITC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcmd](index.html) module"]
pub struct USBCMD_SPEC;
impl crate::RegisterSpec for USBCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbcmd::R](R) reader structure"]
impl crate::Readable for USBCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbcmd::W](W) writer structure"]
impl crate::Writable for USBCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCMD to value 0x0008_0000"]
impl crate::Resettable for USBCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0000;
}
