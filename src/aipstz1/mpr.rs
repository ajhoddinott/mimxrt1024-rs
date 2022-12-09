#[doc = "Register `MPR` reader"]
pub struct R(crate::R<MPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPR` writer"]
pub struct W(crate::W<MPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPR_SPEC>;
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
impl From<crate::W<MPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPROT3` reader - Master 3 Priviledge, Buffer, Read, Write Control."]
pub type MPROT3_R = crate::FieldReader<u8, MPROT3_A>;
#[doc = "Master 3 Priviledge, Buffer, Read, Write Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MPROT3_A {
    #[doc = "0: Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    MPL0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    MPL1 = 1,
}
impl From<MPROT3_A> for u8 {
    #[inline(always)]
    fn from(variant: MPROT3_A) -> Self {
        variant as _
    }
}
impl MPROT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MPROT3_A> {
        match self.bits {
            0 => Some(MPROT3_A::MPL0),
            1 => Some(MPROT3_A::MPL1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline(always)]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT3_A::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline(always)]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT3_A::MPL1
    }
}
#[doc = "Field `MPROT3` writer - Master 3 Priviledge, Buffer, Read, Write Control."]
pub type MPROT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPR_SPEC, u8, MPROT3_A, 4, O>;
impl<'a, const O: u8> MPROT3_W<'a, O> {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    #[inline(always)]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT3_A::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    #[inline(always)]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT3_A::MPL1)
    }
}
#[doc = "Field `MPROT2` reader - Master 2 Priviledge, Buffer, Read, Write Control"]
pub type MPROT2_R = crate::FieldReader<u8, MPROT2_A>;
#[doc = "Master 2 Priviledge, Buffer, Read, Write Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MPROT2_A {
    #[doc = "0: Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    MPL0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    MPL1 = 1,
}
impl From<MPROT2_A> for u8 {
    #[inline(always)]
    fn from(variant: MPROT2_A) -> Self {
        variant as _
    }
}
impl MPROT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MPROT2_A> {
        match self.bits {
            0 => Some(MPROT2_A::MPL0),
            1 => Some(MPROT2_A::MPL1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline(always)]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT2_A::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline(always)]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT2_A::MPL1
    }
}
#[doc = "Field `MPROT2` writer - Master 2 Priviledge, Buffer, Read, Write Control"]
pub type MPROT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPR_SPEC, u8, MPROT2_A, 4, O>;
impl<'a, const O: u8> MPROT2_W<'a, O> {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    #[inline(always)]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT2_A::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    #[inline(always)]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT2_A::MPL1)
    }
}
#[doc = "Field `MPROT1` reader - Master 1 Priviledge, Buffer, Read, Write Control"]
pub type MPROT1_R = crate::FieldReader<u8, MPROT1_A>;
#[doc = "Master 1 Priviledge, Buffer, Read, Write Control\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MPROT1_A {
    #[doc = "0: Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    MPL0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    MPL1 = 1,
}
impl From<MPROT1_A> for u8 {
    #[inline(always)]
    fn from(variant: MPROT1_A) -> Self {
        variant as _
    }
}
impl MPROT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MPROT1_A> {
        match self.bits {
            0 => Some(MPROT1_A::MPL0),
            1 => Some(MPROT1_A::MPL1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline(always)]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT1_A::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline(always)]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT1_A::MPL1
    }
}
#[doc = "Field `MPROT1` writer - Master 1 Priviledge, Buffer, Read, Write Control"]
pub type MPROT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPR_SPEC, u8, MPROT1_A, 4, O>;
impl<'a, const O: u8> MPROT1_W<'a, O> {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    #[inline(always)]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT1_A::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    #[inline(always)]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT1_A::MPL1)
    }
}
#[doc = "Field `MPROT0` reader - Master 0 Priviledge, Buffer, Read, Write Control"]
pub type MPROT0_R = crate::FieldReader<u8, MPROT0_A>;
#[doc = "Master 0 Priviledge, Buffer, Read, Write Control\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MPROT0_A {
    #[doc = "0: Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    MPL0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    MPL1 = 1,
}
impl From<MPROT0_A> for u8 {
    #[inline(always)]
    fn from(variant: MPROT0_A) -> Self {
        variant as _
    }
}
impl MPROT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MPROT0_A> {
        match self.bits {
            0 => Some(MPROT0_A::MPL0),
            1 => Some(MPROT0_A::MPL1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline(always)]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT0_A::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline(always)]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT0_A::MPL1
    }
}
#[doc = "Field `MPROT0` writer - Master 0 Priviledge, Buffer, Read, Write Control"]
pub type MPROT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPR_SPEC, u8, MPROT0_A, 4, O>;
impl<'a, const O: u8> MPROT0_W<'a, O> {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    #[inline(always)]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT0_A::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    #[inline(always)]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT0_A::MPL1)
    }
}
impl R {
    #[doc = "Bits 16:19 - Master 3 Priviledge, Buffer, Read, Write Control."]
    #[inline(always)]
    pub fn mprot3(&self) -> MPROT3_R {
        MPROT3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Master 2 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    pub fn mprot2(&self) -> MPROT2_R {
        MPROT2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Master 1 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    pub fn mprot1(&self) -> MPROT1_R {
        MPROT1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Master 0 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    pub fn mprot0(&self) -> MPROT0_R {
        MPROT0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Master 3 Priviledge, Buffer, Read, Write Control."]
    #[inline(always)]
    #[must_use]
    pub fn mprot3(&mut self) -> MPROT3_W<16> {
        MPROT3_W::new(self)
    }
    #[doc = "Bits 20:23 - Master 2 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    #[must_use]
    pub fn mprot2(&mut self) -> MPROT2_W<20> {
        MPROT2_W::new(self)
    }
    #[doc = "Bits 24:27 - Master 1 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    #[must_use]
    pub fn mprot1(&mut self) -> MPROT1_W<24> {
        MPROT1_W::new(self)
    }
    #[doc = "Bits 28:31 - Master 0 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    #[must_use]
    pub fn mprot0(&mut self) -> MPROT0_W<28> {
        MPROT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Priviledge Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpr](index.html) module"]
pub struct MPR_SPEC;
impl crate::RegisterSpec for MPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpr::R](R) reader structure"]
impl crate::Readable for MPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpr::W](W) writer structure"]
impl crate::Writable for MPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPR to value 0x7700_0000"]
impl crate::Resettable for MPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7700_0000;
}
