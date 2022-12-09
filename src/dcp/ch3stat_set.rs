#[doc = "Register `CH3STAT_SET` reader"]
pub struct R(crate::R<CH3STAT_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3STAT_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3STAT_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3STAT_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3STAT_SET` writer"]
pub struct W(crate::W<CH3STAT_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3STAT_SET_SPEC>;
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
impl From<crate::W<CH3STAT_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3STAT_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HASH_MISMATCH` reader - This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
pub type HASH_MISMATCH_R = crate::BitReader<bool>;
#[doc = "Field `HASH_MISMATCH` writer - This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
pub type HASH_MISMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH3STAT_SET_SPEC, bool, O>;
#[doc = "Field `ERROR_SETUP` reader - This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
pub type ERROR_SETUP_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_SETUP` writer - This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
pub type ERROR_SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH3STAT_SET_SPEC, bool, O>;
#[doc = "Field `ERROR_PACKET` reader - This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
pub type ERROR_PACKET_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_PACKET` writer - This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
pub type ERROR_PACKET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH3STAT_SET_SPEC, bool, O>;
#[doc = "Field `ERROR_SRC` reader - This bit indicates that a bus error occurred when reading from the source buffer"]
pub type ERROR_SRC_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_SRC` writer - This bit indicates that a bus error occurred when reading from the source buffer"]
pub type ERROR_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH3STAT_SET_SPEC, bool, O>;
#[doc = "Field `ERROR_DST` reader - This bit indicates that a bus error occurred when storing to the destination buffer"]
pub type ERROR_DST_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_DST` writer - This bit indicates that a bus error occurred when storing to the destination buffer"]
pub type ERROR_DST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH3STAT_SET_SPEC, bool, O>;
#[doc = "Field `ERROR_PAGEFAULT` reader - This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
pub type ERROR_PAGEFAULT_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_PAGEFAULT` writer - This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
pub type ERROR_PAGEFAULT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH3STAT_SET_SPEC, bool, O>;
#[doc = "Field `ERROR_CODE` reader - Indicates additional error codes for some of the error conditions."]
pub type ERROR_CODE_R = crate::FieldReader<u8, ERROR_CODE_A>;
#[doc = "Indicates additional error codes for some of the error conditions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERROR_CODE_A {
    #[doc = "1: Error is signalled because the next pointer is 0x00000000."]
    NEXT_CHAIN_IS_0 = 1,
    #[doc = "2: Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    NO_CHAIN = 2,
    #[doc = "3: Error is signalled because an error was reported while reading/writing the context buffer."]
    CONTEXT_ERROR = 3,
    #[doc = "4: Error is signalled because an error was reported while reading/writing the payload."]
    PAYLOAD_ERROR = 4,
    #[doc = "5: Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    INVALID_MODE = 5,
}
impl From<ERROR_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERROR_CODE_A) -> Self {
        variant as _
    }
}
impl ERROR_CODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERROR_CODE_A> {
        match self.bits {
            1 => Some(ERROR_CODE_A::NEXT_CHAIN_IS_0),
            2 => Some(ERROR_CODE_A::NO_CHAIN),
            3 => Some(ERROR_CODE_A::CONTEXT_ERROR),
            4 => Some(ERROR_CODE_A::PAYLOAD_ERROR),
            5 => Some(ERROR_CODE_A::INVALID_MODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NEXT_CHAIN_IS_0`"]
    #[inline(always)]
    pub fn is_next_chain_is_0(&self) -> bool {
        *self == ERROR_CODE_A::NEXT_CHAIN_IS_0
    }
    #[doc = "Checks if the value of the field is `NO_CHAIN`"]
    #[inline(always)]
    pub fn is_no_chain(&self) -> bool {
        *self == ERROR_CODE_A::NO_CHAIN
    }
    #[doc = "Checks if the value of the field is `CONTEXT_ERROR`"]
    #[inline(always)]
    pub fn is_context_error(&self) -> bool {
        *self == ERROR_CODE_A::CONTEXT_ERROR
    }
    #[doc = "Checks if the value of the field is `PAYLOAD_ERROR`"]
    #[inline(always)]
    pub fn is_payload_error(&self) -> bool {
        *self == ERROR_CODE_A::PAYLOAD_ERROR
    }
    #[doc = "Checks if the value of the field is `INVALID_MODE`"]
    #[inline(always)]
    pub fn is_invalid_mode(&self) -> bool {
        *self == ERROR_CODE_A::INVALID_MODE
    }
}
#[doc = "Field `ERROR_CODE` writer - Indicates additional error codes for some of the error conditions."]
pub type ERROR_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH3STAT_SET_SPEC, u8, ERROR_CODE_A, 8, O>;
impl<'a, const O: u8> ERROR_CODE_W<'a, O> {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    #[inline(always)]
    pub fn next_chain_is_0(self) -> &'a mut W {
        self.variant(ERROR_CODE_A::NEXT_CHAIN_IS_0)
    }
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    #[inline(always)]
    pub fn no_chain(self) -> &'a mut W {
        self.variant(ERROR_CODE_A::NO_CHAIN)
    }
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    #[inline(always)]
    pub fn context_error(self) -> &'a mut W {
        self.variant(ERROR_CODE_A::CONTEXT_ERROR)
    }
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    #[inline(always)]
    pub fn payload_error(self) -> &'a mut W {
        self.variant(ERROR_CODE_A::PAYLOAD_ERROR)
    }
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
    #[inline(always)]
    pub fn invalid_mode(self) -> &'a mut W {
        self.variant(ERROR_CODE_A::INVALID_MODE)
    }
}
#[doc = "Field `TAG` reader - Indicates the tag from the last completed packet in the command structure."]
pub type TAG_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 1 - This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub fn hash_mismatch(&self) -> HASH_MISMATCH_R {
        HASH_MISMATCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub fn error_setup(&self) -> ERROR_SETUP_R {
        ERROR_SETUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    #[inline(always)]
    pub fn error_packet(&self) -> ERROR_PACKET_R {
        ERROR_PACKET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub fn error_src(&self) -> ERROR_SRC_R {
        ERROR_SRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub fn error_dst(&self) -> ERROR_DST_R {
        ERROR_DST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub fn error_pagefault(&self) -> ERROR_PAGEFAULT_R {
        ERROR_PAGEFAULT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    pub fn error_code(&self) -> ERROR_CODE_R {
        ERROR_CODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Indicates the tag from the last completed packet in the command structure."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    #[must_use]
    pub fn hash_mismatch(&mut self) -> HASH_MISMATCH_W<1> {
        HASH_MISMATCH_W::new(self)
    }
    #[doc = "Bit 2 - This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    #[must_use]
    pub fn error_setup(&mut self) -> ERROR_SETUP_W<2> {
        ERROR_SETUP_W::new(self)
    }
    #[doc = "Bit 3 - This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    #[inline(always)]
    #[must_use]
    pub fn error_packet(&mut self) -> ERROR_PACKET_W<3> {
        ERROR_PACKET_W::new(self)
    }
    #[doc = "Bit 4 - This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    #[must_use]
    pub fn error_src(&mut self) -> ERROR_SRC_W<4> {
        ERROR_SRC_W::new(self)
    }
    #[doc = "Bit 5 - This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    #[must_use]
    pub fn error_dst(&mut self) -> ERROR_DST_W<5> {
        ERROR_DST_W::new(self)
    }
    #[doc = "Bit 6 - This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    #[must_use]
    pub fn error_pagefault(&mut self) -> ERROR_PAGEFAULT_W<6> {
        ERROR_PAGEFAULT_W::new(self)
    }
    #[doc = "Bits 16:23 - Indicates additional error codes for some of the error conditions."]
    #[inline(always)]
    #[must_use]
    pub fn error_code(&mut self) -> ERROR_CODE_W<16> {
        ERROR_CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP channel 3 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3stat_set](index.html) module"]
pub struct CH3STAT_SET_SPEC;
impl crate::RegisterSpec for CH3STAT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3stat_set::R](R) reader structure"]
impl crate::Readable for CH3STAT_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3stat_set::W](W) writer structure"]
impl crate::Writable for CH3STAT_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3STAT_SET to value 0"]
impl crate::Resettable for CH3STAT_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
