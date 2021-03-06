// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use cocoa::foundation::NSUInteger;
use objc::runtime::{YES, NO};
use objc_foundation::{INSString, NSString};

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLMutability {
    Default = 0,
    Mutable = 1,
    Immutable = 2,
}

impl Default for MTLMutability {
    #[inline]
    fn default() -> Self {
        MTLMutability::Default
    }
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLIndexType {
    UInt16 = 0,
    UInt32 = 1,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLAttributeFormat {
    Invalid = 0,
    UChar2 = 1,
    UChar3 = 2,
    UChar4 = 3,
    Char2 = 4,
    Char3 = 5,
    Char4 = 6,
    UChar2Normalized = 7,
    UChar3Normalized = 8,
    UChar4Normalized = 9,
    Char2Normalized = 10,
    Char3Normalized = 11,
    Char4Normalized = 12,
    UShort2 = 13,
    UShort3 = 14,
    UShort4 = 15,
    Short2 = 16,
    Short3 = 17,
    Short4 = 18,
    UShort2Normalized = 19,
    UShort3Normalized = 20,
    UShort4Normalized = 21,
    Short2Normalized = 22,
    Short3Normalized = 23,
    Short4Normalized = 24,
    Half2 = 25,
    Half3 = 26,
    Half4 = 27,
    Float = 28,
    Float2 = 29,
    Float3 = 30,
    Float4 = 31,
    Int = 32,
    Int2 = 33,
    Int3 = 34,
    Int4 = 35,
    UInt = 36,
    UInt2 = 37,
    UInt3 = 38,
    UInt4 = 39,
    Int1010102Normalized = 40,
    UInt1010102Normalized = 41,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLStepFunction {
    Constant = 0,
    PerInstance = 1,
    PerPatch = 2,
    PerPatchControlPoint = 3,
    PerVertex = 4,
    ThreadPositionInGridX = 5,
    ThreadPositionInGridXIndexed = 6,
    ThreadPositionInGridY = 7,
    ThreadPositionInGridYIndexed = 8,
} 

pub enum MTLComputePipelineDescriptor {}

foreign_obj_type! {
    type CType = MTLComputePipelineDescriptor;
    pub struct ComputePipelineDescriptor;
    pub struct ComputePipelineDescriptorRef;
}

impl ComputePipelineDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLComputePipelineDescriptor);
            msg_send![class, new]
        }
    }
}

impl ComputePipelineDescriptorRef {
    pub fn label(&self) -> &str {
        unsafe {
            let label: &NSString = msg_send![self, label];
            label.as_str()
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = NSString::from_str(label);
            msg_send![self, setLabel:nslabel]
        }
    }

    pub fn compute_function(&self) -> Option<&FunctionRef> {
        unsafe {
            msg_send![self, computeFunction]
        }
    }

    pub fn set_compute_function(&self, function: Option<&FunctionRef>) {
        unsafe {
            msg_send![self, setComputeFunction:function]
        }
    }

    pub fn thread_group_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe {
            match msg_send![self, threadGroupSizeIsMultipleOfThreadExecutionWidth] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    pub fn set_thread_group_size_is_multiple_of_thread_execution_width(&self, size_is_multiple_of_width: bool) {
        unsafe {
            msg_send![self, setThreadGroupSizeIsMultipleOfThreadExecutionWidth:size_is_multiple_of_width]
        }
    }

    pub fn stage_input_descriptor(&self) -> Option<&StageInputOutputDescriptorRef> {
        unsafe {
            msg_send![self, stageInputDescriptor]
        }
    }

    pub fn set_stage_input_descriptor(&self, descriptor: Option<&StageInputOutputDescriptorRef>) {
        unsafe {
            msg_send![self, setStageInputDescriptor:descriptor]
        }
    }

    pub fn buffers(&self) -> Option<&PipelineBufferDescriptorArrayRef> {
        unsafe {
            msg_send![self, buffers]
        }
    }

    pub fn reset(&self) {
        unsafe {
            msg_send![self, reset]
        }
    }
}

pub enum MTLComputePipelineState {}

foreign_obj_type! {
    type CType = MTLComputePipelineState;
    pub struct ComputePipelineState;
    pub struct ComputePipelineStateRef;
}

impl ComputePipelineStateRef {
    pub fn label(&self) -> &str {
        unsafe {
            let label: &NSString = msg_send![self, label];
            label.as_str()
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = NSString::from_str(label);
            msg_send![self, setLabel:nslabel]
        }
    }

    pub fn max_total_threads_per_group(&self) -> NSUInteger {
        unsafe {
            msg_send![self, maxTotalThreadsPerThreadgroup]
        }
    }

    pub fn thread_execution_width(&self) -> NSUInteger {
        unsafe {
            msg_send![self, threadExecutionWidth]
        }
    }

    pub fn static_threadgroup_memory_length(&self) -> NSUInteger {
        unsafe {
            msg_send![self, staticThreadgroupMemoryLength]
        }
    }
}

pub enum MTLPipelineBufferDescriptorArray {}

foreign_obj_type! {
    type CType = MTLPipelineBufferDescriptorArray;
    pub struct PipelineBufferDescriptorArray;
    pub struct PipelineBufferDescriptorArrayRef;
}

impl PipelineBufferDescriptorArrayRef {
    pub fn object_at(&self, index: usize) -> Option<&PipelineBufferDescriptorRef> {
        unsafe {
            msg_send![self, objectAtIndexedSubscript:index]
        }
    }

    pub fn set_object_at(&self, index: usize, buffer_desc: Option<&PipelineBufferDescriptorRef>) {
        unsafe {
            msg_send![self, setObject:buffer_desc atIndexedSubscript:index]
        }
    }
}

pub enum MTLPipelineBufferDescriptor {}

foreign_obj_type! {
    type CType = MTLPipelineBufferDescriptor;
    pub struct PipelineBufferDescriptor;
    pub struct PipelineBufferDescriptorRef;
}

impl PipelineBufferDescriptorRef {
    pub fn mutability(&self) -> MTLMutability {
        unsafe {
            msg_send![self, mutability]
        }
    }

    pub fn set_mutability(&self, new_mutability: MTLMutability) {
        unsafe {
            msg_send![self, setMutability:new_mutability]
        }
    }
}

pub enum MTLStageInputOutputDescriptor {}

foreign_obj_type! {
    type CType = MTLStageInputOutputDescriptor;
    pub struct StageInputOutputDescriptor;
    pub struct StageInputOutputDescriptorRef;
}

impl StageInputOutputDescriptor {
    pub fn new<'a>() -> &'a StageInputOutputDescriptorRef {
        unsafe {
            let class = class!(MTLStageInputOutputDescriptor);
            msg_send![class, stageInputOutputDescriptor]
        }
    }
}

impl StageInputOutputDescriptorRef {
    pub fn attributes(&self) -> Option<&AttributeDescriptorArrayRef> {
        unsafe {
            msg_send![self, attributes]
        }
    }

    pub fn index_buffer_index(&self) -> NSUInteger {
        unsafe {
            msg_send![self, indexBufferIndex]
        }
    }

    pub fn set_index_buffer_index(&self, idx_buffer_idx: NSUInteger) {
        unsafe {
            msg_send![self, setIndexBufferIndex:idx_buffer_idx]
        }
    }

    pub fn index_type(&self) -> MTLIndexType {
        unsafe {
            msg_send![self, indexType]
        }
    }

    pub fn set_index_type(&self, index_ty: MTLIndexType) {
        unsafe {
            msg_send![self, setIndexType:index_ty]
        }
    }

    pub fn layouts(&self) -> Option<&BufferLayoutDescriptorArrayRef> {
        unsafe {
            msg_send![self, layouts]
        }
    }

    pub fn reset(&self) {
        unsafe {
            msg_send![self, reset]
        }
    } 
}

pub enum MTLAttributeDescriptorArray {}

foreign_obj_type! {
    type CType = MTLAttributeDescriptorArray;
    pub struct AttributeDescriptorArray;
    pub struct AttributeDescriptorArrayRef;
}

impl AttributeDescriptorArrayRef {
    pub fn object_at(&self, index: usize) -> Option<&AttributeDescriptorRef> {
        unsafe {
            msg_send![self, objectAtIndexedSubscript:index]
        }
    }

    pub fn set_object_at(&self, index: usize, buffer_desc: Option<&AttributeDescriptorRef>) {
        unsafe {
            msg_send![self, setObject:buffer_desc atIndexedSubscript:index]
        }
    }
}

pub enum MTLAttributeDescriptor {}

foreign_obj_type! {
    type CType = MTLAttributeDescriptor;
    pub struct AttributeDescriptor;
    pub struct AttributeDescriptorRef;
}

impl AttributeDescriptorRef {
    pub fn buffer_index(&self) -> NSUInteger {
        unsafe {
            msg_send![self, bufferIndex]
        }
    }

    pub fn set_buffer_index(&self, buffer_index: NSUInteger) {
        unsafe {
            msg_send![self, setBufferIndex:buffer_index]
        }
    }

    pub fn format(&self) -> MTLAttributeFormat {
        unsafe {
            msg_send![self, format]
        }
    }

    pub fn set_format(&self, format: MTLAttributeFormat) {
        unsafe {
            msg_send![self, setFormat:format]
        }
    }

    pub fn offset(&self) -> NSUInteger {
        unsafe {
            msg_send![self, offset]
        }
    }

    pub fn set_offset(&self, offset: NSUInteger) {
        unsafe {
            msg_send![self, setOffset:offset]
        }
    }
}

pub enum MTLBufferLayoutDescriptorArray {}

foreign_obj_type! {
    type CType = MTLBufferLayoutDescriptorArray;
    pub struct BufferLayoutDescriptorArray;
    pub struct BufferLayoutDescriptorArrayRef;
}

impl BufferLayoutDescriptorArrayRef {
    pub fn object_at(&self, index: usize) -> Option<&BufferLayoutDescriptorRef> {
        unsafe {
            msg_send![self, objectAtIndexedSubscript:index]
        }
    }

    pub fn set_object_at(&self, index: usize, buffer_desc: Option<&BufferLayoutDescriptorRef>) {
        unsafe {
            msg_send![self, setObject:buffer_desc atIndexedSubscript:index]
        }
    }
}

pub enum MTLBufferLayoutDescriptor {}

foreign_obj_type! {
    type CType = MTLBufferLayoutDescriptor;
    pub struct BufferLayoutDescriptor;
    pub struct BufferLayoutDescriptorRef;
}

impl BufferLayoutDescriptorRef {
    pub fn step_function(&self) -> MTLStepFunction {
        unsafe {
            msg_send![self, stepFunction]
        }
    }

    pub fn set_step_function(&self, step_function: MTLStepFunction) {
        unsafe {
            msg_send![self, setStepFunction:step_function]
        }
    }

    pub fn step_rate(&self) -> NSUInteger {
        unsafe {
            msg_send![self, stepRate]
        }
    }

    pub fn set_step_rate(&self, step_rate: NSUInteger) {
        unsafe {
            msg_send![self, setStepRate:step_rate]
        }
    }

    pub fn stride(&self) -> NSUInteger {
        unsafe {
            msg_send![self, stride]
        }
    }

    pub fn set_stride(&self, stride: NSUInteger) {
        unsafe {
            msg_send![self, setStride:stride]
        }
    }
}
