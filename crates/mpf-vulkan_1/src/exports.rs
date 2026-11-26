/// How `dlls/vulkan-1/vulkan-1.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall name(args) winevulkan.name`: a forwarder to `winevulkan.dll`. The payload is
    /// the forward target exactly as written in the spec.
    Forward(&'static str),
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// One line of the export table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    /// The argument type list from the spec, for example `ptr int64 long`. Empty for stubs.
    pub args: &'static str,
    pub kind: ExportKind,
}

macro_rules! fwd {
    ($name:expr, $args:expr, $target:expr $(,)?) => {
        Export {
            name: $name,
            args: $args,
            kind: ExportKind::Forward($target),
        }
    };
}

macro_rules! stub {
    ($name:expr) => {
        Export {
            name: $name,
            args: "",
            kind: ExportKind::Stub,
        }
    };
}

/// The `vulkan-1.spec` export table, in spec order.
pub const EXPORTS: [Export; 262] = [
    fwd!(
        "vkAcquireNextImage2KHR",
        "ptr ptr ptr",
        "winevulkan.vkAcquireNextImage2KHR",
    ),
    fwd!(
        "vkAcquireNextImageKHR",
        "ptr int64 int64 int64 int64 ptr",
        "winevulkan.vkAcquireNextImageKHR",
    ),
    fwd!(
        "vkAllocateCommandBuffers",
        "ptr ptr ptr",
        "winevulkan.vkAllocateCommandBuffers",
    ),
    fwd!(
        "vkAllocateDescriptorSets",
        "ptr ptr ptr",
        "winevulkan.vkAllocateDescriptorSets",
    ),
    fwd!(
        "vkAllocateMemory",
        "ptr ptr ptr ptr",
        "winevulkan.vkAllocateMemory",
    ),
    fwd!(
        "vkBeginCommandBuffer",
        "ptr ptr",
        "winevulkan.vkBeginCommandBuffer",
    ),
    fwd!(
        "vkBindBufferMemory",
        "ptr int64 int64 int64",
        "winevulkan.vkBindBufferMemory",
    ),
    fwd!(
        "vkBindBufferMemory2",
        "ptr long ptr",
        "winevulkan.vkBindBufferMemory2",
    ),
    fwd!(
        "vkBindImageMemory",
        "ptr int64 int64 int64",
        "winevulkan.vkBindImageMemory",
    ),
    fwd!(
        "vkBindImageMemory2",
        "ptr long ptr",
        "winevulkan.vkBindImageMemory2",
    ),
    fwd!(
        "vkCmdBeginQuery",
        "ptr int64 long long",
        "winevulkan.vkCmdBeginQuery",
    ),
    fwd!(
        "vkCmdBeginRenderPass",
        "ptr ptr long",
        "winevulkan.vkCmdBeginRenderPass",
    ),
    fwd!(
        "vkCmdBeginRenderPass2",
        "ptr ptr ptr",
        "winevulkan.vkCmdBeginRenderPass2",
    ),
    fwd!(
        "vkCmdBeginRendering",
        "ptr ptr",
        "winevulkan.vkCmdBeginRendering",
    ),
    fwd!(
        "vkCmdBindDescriptorSets",
        "ptr long int64 long long ptr long ptr",
        "winevulkan.vkCmdBindDescriptorSets",
    ),
    fwd!(
        "vkCmdBindDescriptorSets2",
        "ptr ptr",
        "winevulkan.vkCmdBindDescriptorSets2",
    ),
    fwd!(
        "vkCmdBindIndexBuffer",
        "ptr int64 int64 long",
        "winevulkan.vkCmdBindIndexBuffer",
    ),
    fwd!(
        "vkCmdBindIndexBuffer2",
        "ptr int64 int64 int64 long",
        "winevulkan.vkCmdBindIndexBuffer2",
    ),
    fwd!(
        "vkCmdBindPipeline",
        "ptr long int64",
        "winevulkan.vkCmdBindPipeline",
    ),
    fwd!(
        "vkCmdBindVertexBuffers",
        "ptr long long ptr ptr",
        "winevulkan.vkCmdBindVertexBuffers",
    ),
    fwd!(
        "vkCmdBindVertexBuffers2",
        "ptr long long ptr ptr ptr ptr",
        "winevulkan.vkCmdBindVertexBuffers2",
    ),
    fwd!(
        "vkCmdBlitImage",
        "ptr int64 long int64 long long ptr long",
        "winevulkan.vkCmdBlitImage",
    ),
    fwd!("vkCmdBlitImage2", "ptr ptr", "winevulkan.vkCmdBlitImage2"),
    fwd!(
        "vkCmdClearAttachments",
        "ptr long ptr long ptr",
        "winevulkan.vkCmdClearAttachments",
    ),
    fwd!(
        "vkCmdClearColorImage",
        "ptr int64 long ptr long ptr",
        "winevulkan.vkCmdClearColorImage",
    ),
    fwd!(
        "vkCmdClearDepthStencilImage",
        "ptr int64 long ptr long ptr",
        "winevulkan.vkCmdClearDepthStencilImage",
    ),
    fwd!(
        "vkCmdCopyBuffer",
        "ptr int64 int64 long ptr",
        "winevulkan.vkCmdCopyBuffer",
    ),
    fwd!("vkCmdCopyBuffer2", "ptr ptr", "winevulkan.vkCmdCopyBuffer2"),
    fwd!(
        "vkCmdCopyBufferToImage",
        "ptr int64 int64 long long ptr",
        "winevulkan.vkCmdCopyBufferToImage",
    ),
    fwd!(
        "vkCmdCopyBufferToImage2",
        "ptr ptr",
        "winevulkan.vkCmdCopyBufferToImage2",
    ),
    fwd!(
        "vkCmdCopyImage",
        "ptr int64 long int64 long long ptr",
        "winevulkan.vkCmdCopyImage",
    ),
    fwd!("vkCmdCopyImage2", "ptr ptr", "winevulkan.vkCmdCopyImage2"),
    fwd!(
        "vkCmdCopyImageToBuffer",
        "ptr int64 long int64 long ptr",
        "winevulkan.vkCmdCopyImageToBuffer",
    ),
    fwd!(
        "vkCmdCopyImageToBuffer2",
        "ptr ptr",
        "winevulkan.vkCmdCopyImageToBuffer2",
    ),
    fwd!(
        "vkCmdCopyQueryPoolResults",
        "ptr int64 long long int64 int64 int64 long",
        "winevulkan.vkCmdCopyQueryPoolResults",
    ),
    fwd!(
        "vkCmdDispatch",
        "ptr long long long",
        "winevulkan.vkCmdDispatch",
    ),
    fwd!(
        "vkCmdDispatchBase",
        "ptr long long long long long long",
        "winevulkan.vkCmdDispatchBase",
    ),
    fwd!(
        "vkCmdDispatchIndirect",
        "ptr int64 int64",
        "winevulkan.vkCmdDispatchIndirect",
    ),
    fwd!(
        "vkCmdDraw",
        "ptr long long long long",
        "winevulkan.vkCmdDraw",
    ),
    fwd!(
        "vkCmdDrawIndexed",
        "ptr long long long long long",
        "winevulkan.vkCmdDrawIndexed",
    ),
    fwd!(
        "vkCmdDrawIndexedIndirect",
        "ptr int64 int64 long long",
        "winevulkan.vkCmdDrawIndexedIndirect",
    ),
    fwd!(
        "vkCmdDrawIndexedIndirectCount",
        "ptr int64 int64 int64 int64 long long",
        "winevulkan.vkCmdDrawIndexedIndirectCount",
    ),
    fwd!(
        "vkCmdDrawIndirect",
        "ptr int64 int64 long long",
        "winevulkan.vkCmdDrawIndirect",
    ),
    fwd!(
        "vkCmdDrawIndirectCount",
        "ptr int64 int64 int64 int64 long long",
        "winevulkan.vkCmdDrawIndirectCount",
    ),
    fwd!(
        "vkCmdEndQuery",
        "ptr int64 long",
        "winevulkan.vkCmdEndQuery",
    ),
    fwd!("vkCmdEndRenderPass", "ptr", "winevulkan.vkCmdEndRenderPass"),
    fwd!(
        "vkCmdEndRenderPass2",
        "ptr ptr",
        "winevulkan.vkCmdEndRenderPass2",
    ),
    fwd!("vkCmdEndRendering", "ptr", "winevulkan.vkCmdEndRendering"),
    fwd!(
        "vkCmdExecuteCommands",
        "ptr long ptr",
        "winevulkan.vkCmdExecuteCommands",
    ),
    fwd!(
        "vkCmdFillBuffer",
        "ptr int64 int64 int64 long",
        "winevulkan.vkCmdFillBuffer",
    ),
    fwd!(
        "vkCmdNextSubpass",
        "ptr long",
        "winevulkan.vkCmdNextSubpass",
    ),
    fwd!(
        "vkCmdNextSubpass2",
        "ptr ptr ptr",
        "winevulkan.vkCmdNextSubpass2",
    ),
    fwd!(
        "vkCmdPipelineBarrier",
        "ptr long long long long ptr long ptr long ptr",
        "winevulkan.vkCmdPipelineBarrier",
    ),
    fwd!(
        "vkCmdPipelineBarrier2",
        "ptr ptr",
        "winevulkan.vkCmdPipelineBarrier2",
    ),
    fwd!(
        "vkCmdPushConstants",
        "ptr int64 long long long ptr",
        "winevulkan.vkCmdPushConstants",
    ),
    fwd!(
        "vkCmdPushConstants2",
        "ptr ptr",
        "winevulkan.vkCmdPushConstants2",
    ),
    fwd!(
        "vkCmdPushDescriptorSet",
        "ptr long int64 long long ptr",
        "winevulkan.vkCmdPushDescriptorSet",
    ),
    fwd!(
        "vkCmdPushDescriptorSet2",
        "ptr ptr",
        "winevulkan.vkCmdPushDescriptorSet2",
    ),
    fwd!(
        "vkCmdPushDescriptorSetWithTemplate",
        "ptr int64 int64 long ptr",
        "winevulkan.vkCmdPushDescriptorSetWithTemplate",
    ),
    fwd!(
        "vkCmdPushDescriptorSetWithTemplate2",
        "ptr ptr",
        "winevulkan.vkCmdPushDescriptorSetWithTemplate2",
    ),
    fwd!(
        "vkCmdResetEvent",
        "ptr int64 long",
        "winevulkan.vkCmdResetEvent",
    ),
    fwd!(
        "vkCmdResetEvent2",
        "ptr int64 int64",
        "winevulkan.vkCmdResetEvent2",
    ),
    fwd!(
        "vkCmdResetQueryPool",
        "ptr int64 long long",
        "winevulkan.vkCmdResetQueryPool",
    ),
    fwd!(
        "vkCmdResolveImage",
        "ptr int64 long int64 long long ptr",
        "winevulkan.vkCmdResolveImage",
    ),
    fwd!(
        "vkCmdResolveImage2",
        "ptr ptr",
        "winevulkan.vkCmdResolveImage2",
    ),
    fwd!(
        "vkCmdSetBlendConstants",
        "ptr ptr",
        "winevulkan.vkCmdSetBlendConstants",
    ),
    fwd!(
        "vkCmdSetCullMode",
        "ptr long",
        "winevulkan.vkCmdSetCullMode",
    ),
    fwd!(
        "vkCmdSetDepthBias",
        "ptr float float float",
        "winevulkan.vkCmdSetDepthBias",
    ),
    fwd!(
        "vkCmdSetDepthBiasEnable",
        "ptr long",
        "winevulkan.vkCmdSetDepthBiasEnable",
    ),
    fwd!(
        "vkCmdSetDepthBounds",
        "ptr float float",
        "winevulkan.vkCmdSetDepthBounds",
    ),
    fwd!(
        "vkCmdSetDepthBoundsTestEnable",
        "ptr long",
        "winevulkan.vkCmdSetDepthBoundsTestEnable",
    ),
    fwd!(
        "vkCmdSetDepthCompareOp",
        "ptr long",
        "winevulkan.vkCmdSetDepthCompareOp",
    ),
    fwd!(
        "vkCmdSetDepthTestEnable",
        "ptr long",
        "winevulkan.vkCmdSetDepthTestEnable",
    ),
    fwd!(
        "vkCmdSetDepthWriteEnable",
        "ptr long",
        "winevulkan.vkCmdSetDepthWriteEnable",
    ),
    fwd!(
        "vkCmdSetDeviceMask",
        "ptr long",
        "winevulkan.vkCmdSetDeviceMask",
    ),
    fwd!(
        "vkCmdSetEvent",
        "ptr int64 long",
        "winevulkan.vkCmdSetEvent",
    ),
    fwd!(
        "vkCmdSetEvent2",
        "ptr int64 ptr",
        "winevulkan.vkCmdSetEvent2",
    ),
    fwd!(
        "vkCmdSetFrontFace",
        "ptr long",
        "winevulkan.vkCmdSetFrontFace",
    ),
    fwd!(
        "vkCmdSetLineStipple",
        "ptr long long",
        "winevulkan.vkCmdSetLineStipple",
    ),
    fwd!(
        "vkCmdSetLineWidth",
        "ptr float",
        "winevulkan.vkCmdSetLineWidth",
    ),
    fwd!(
        "vkCmdSetPrimitiveRestartEnable",
        "ptr long",
        "winevulkan.vkCmdSetPrimitiveRestartEnable",
    ),
    fwd!(
        "vkCmdSetPrimitiveTopology",
        "ptr long",
        "winevulkan.vkCmdSetPrimitiveTopology",
    ),
    fwd!(
        "vkCmdSetRasterizerDiscardEnable",
        "ptr long",
        "winevulkan.vkCmdSetRasterizerDiscardEnable",
    ),
    fwd!(
        "vkCmdSetRenderingAttachmentLocations",
        "ptr ptr",
        "winevulkan.vkCmdSetRenderingAttachmentLocations",
    ),
    fwd!(
        "vkCmdSetRenderingInputAttachmentIndices",
        "ptr ptr",
        "winevulkan.vkCmdSetRenderingInputAttachmentIndices",
    ),
    fwd!(
        "vkCmdSetScissor",
        "ptr long long ptr",
        "winevulkan.vkCmdSetScissor",
    ),
    fwd!(
        "vkCmdSetScissorWithCount",
        "ptr long ptr",
        "winevulkan.vkCmdSetScissorWithCount",
    ),
    fwd!(
        "vkCmdSetStencilCompareMask",
        "ptr long long",
        "winevulkan.vkCmdSetStencilCompareMask",
    ),
    fwd!(
        "vkCmdSetStencilOp",
        "ptr long long long long long",
        "winevulkan.vkCmdSetStencilOp",
    ),
    fwd!(
        "vkCmdSetStencilReference",
        "ptr long long",
        "winevulkan.vkCmdSetStencilReference",
    ),
    fwd!(
        "vkCmdSetStencilTestEnable",
        "ptr long",
        "winevulkan.vkCmdSetStencilTestEnable",
    ),
    fwd!(
        "vkCmdSetStencilWriteMask",
        "ptr long long",
        "winevulkan.vkCmdSetStencilWriteMask",
    ),
    fwd!(
        "vkCmdSetViewport",
        "ptr long long ptr",
        "winevulkan.vkCmdSetViewport",
    ),
    fwd!(
        "vkCmdSetViewportWithCount",
        "ptr long ptr",
        "winevulkan.vkCmdSetViewportWithCount",
    ),
    fwd!(
        "vkCmdUpdateBuffer",
        "ptr int64 int64 int64 ptr",
        "winevulkan.vkCmdUpdateBuffer",
    ),
    fwd!(
        "vkCmdWaitEvents",
        "ptr long ptr long long long ptr long ptr long ptr",
        "winevulkan.vkCmdWaitEvents",
    ),
    fwd!(
        "vkCmdWaitEvents2",
        "ptr long ptr ptr",
        "winevulkan.vkCmdWaitEvents2",
    ),
    fwd!(
        "vkCmdWriteTimestamp",
        "ptr long int64 long",
        "winevulkan.vkCmdWriteTimestamp",
    ),
    fwd!(
        "vkCmdWriteTimestamp2",
        "ptr int64 int64 long",
        "winevulkan.vkCmdWriteTimestamp2",
    ),
    fwd!(
        "vkCopyImageToImage",
        "ptr ptr",
        "winevulkan.vkCopyImageToImage",
    ),
    fwd!(
        "vkCopyImageToMemory",
        "ptr ptr",
        "winevulkan.vkCopyImageToMemory",
    ),
    fwd!(
        "vkCopyMemoryToImage",
        "ptr ptr",
        "winevulkan.vkCopyMemoryToImage",
    ),
    fwd!(
        "vkCreateBuffer",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateBuffer",
    ),
    fwd!(
        "vkCreateBufferView",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateBufferView",
    ),
    fwd!(
        "vkCreateCommandPool",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateCommandPool",
    ),
    fwd!(
        "vkCreateComputePipelines",
        "ptr int64 long ptr ptr ptr",
        "winevulkan.vkCreateComputePipelines",
    ),
    fwd!(
        "vkCreateDescriptorPool",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateDescriptorPool",
    ),
    fwd!(
        "vkCreateDescriptorSetLayout",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateDescriptorSetLayout",
    ),
    fwd!(
        "vkCreateDescriptorUpdateTemplate",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateDescriptorUpdateTemplate",
    ),
    fwd!(
        "vkCreateDevice",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateDevice",
    ),
    stub!("vkCreateDisplayModeKHR"),
    stub!("vkCreateDisplayPlaneSurfaceKHR"),
    fwd!(
        "vkCreateEvent",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateEvent",
    ),
    fwd!(
        "vkCreateFence",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateFence",
    ),
    fwd!(
        "vkCreateFramebuffer",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateFramebuffer",
    ),
    fwd!(
        "vkCreateGraphicsPipelines",
        "ptr int64 long ptr ptr ptr",
        "winevulkan.vkCreateGraphicsPipelines",
    ),
    fwd!(
        "vkCreateImage",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateImage",
    ),
    fwd!(
        "vkCreateImageView",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateImageView",
    ),
    fwd!(
        "vkCreateInstance",
        "ptr ptr ptr",
        "winevulkan.vkCreateInstance",
    ),
    fwd!(
        "vkCreatePipelineCache",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreatePipelineCache",
    ),
    fwd!(
        "vkCreatePipelineLayout",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreatePipelineLayout",
    ),
    fwd!(
        "vkCreatePrivateDataSlot",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreatePrivateDataSlot",
    ),
    fwd!(
        "vkCreateQueryPool",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateQueryPool",
    ),
    fwd!(
        "vkCreateRenderPass",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateRenderPass",
    ),
    fwd!(
        "vkCreateRenderPass2",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateRenderPass2",
    ),
    fwd!(
        "vkCreateSampler",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateSampler",
    ),
    fwd!(
        "vkCreateSamplerYcbcrConversion",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateSamplerYcbcrConversion",
    ),
    fwd!(
        "vkCreateSemaphore",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateSemaphore",
    ),
    fwd!(
        "vkCreateShaderModule",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateShaderModule",
    ),
    stub!("vkCreateSharedSwapchainsKHR"),
    fwd!(
        "vkCreateSwapchainKHR",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateSwapchainKHR",
    ),
    fwd!(
        "vkCreateWin32SurfaceKHR",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateWin32SurfaceKHR",
    ),
    fwd!(
        "vkDestroyBuffer",
        "ptr int64 ptr",
        "winevulkan.vkDestroyBuffer",
    ),
    fwd!(
        "vkDestroyBufferView",
        "ptr int64 ptr",
        "winevulkan.vkDestroyBufferView",
    ),
    fwd!(
        "vkDestroyCommandPool",
        "ptr int64 ptr",
        "winevulkan.vkDestroyCommandPool",
    ),
    fwd!(
        "vkDestroyDescriptorPool",
        "ptr int64 ptr",
        "winevulkan.vkDestroyDescriptorPool",
    ),
    fwd!(
        "vkDestroyDescriptorSetLayout",
        "ptr int64 ptr",
        "winevulkan.vkDestroyDescriptorSetLayout",
    ),
    fwd!(
        "vkDestroyDescriptorUpdateTemplate",
        "ptr int64 ptr",
        "winevulkan.vkDestroyDescriptorUpdateTemplate",
    ),
    fwd!("vkDestroyDevice", "ptr ptr", "winevulkan.vkDestroyDevice"),
    fwd!(
        "vkDestroyEvent",
        "ptr int64 ptr",
        "winevulkan.vkDestroyEvent",
    ),
    fwd!(
        "vkDestroyFence",
        "ptr int64 ptr",
        "winevulkan.vkDestroyFence",
    ),
    fwd!(
        "vkDestroyFramebuffer",
        "ptr int64 ptr",
        "winevulkan.vkDestroyFramebuffer",
    ),
    fwd!(
        "vkDestroyImage",
        "ptr int64 ptr",
        "winevulkan.vkDestroyImage",
    ),
    fwd!(
        "vkDestroyImageView",
        "ptr int64 ptr",
        "winevulkan.vkDestroyImageView",
    ),
    fwd!(
        "vkDestroyInstance",
        "ptr ptr",
        "winevulkan.vkDestroyInstance",
    ),
    fwd!(
        "vkDestroyPipeline",
        "ptr int64 ptr",
        "winevulkan.vkDestroyPipeline",
    ),
    fwd!(
        "vkDestroyPipelineCache",
        "ptr int64 ptr",
        "winevulkan.vkDestroyPipelineCache",
    ),
    fwd!(
        "vkDestroyPipelineLayout",
        "ptr int64 ptr",
        "winevulkan.vkDestroyPipelineLayout",
    ),
    fwd!(
        "vkDestroyPrivateDataSlot",
        "ptr int64 ptr",
        "winevulkan.vkDestroyPrivateDataSlot",
    ),
    fwd!(
        "vkDestroyQueryPool",
        "ptr int64 ptr",
        "winevulkan.vkDestroyQueryPool",
    ),
    fwd!(
        "vkDestroyRenderPass",
        "ptr int64 ptr",
        "winevulkan.vkDestroyRenderPass",
    ),
    fwd!(
        "vkDestroySampler",
        "ptr int64 ptr",
        "winevulkan.vkDestroySampler",
    ),
    fwd!(
        "vkDestroySamplerYcbcrConversion",
        "ptr int64 ptr",
        "winevulkan.vkDestroySamplerYcbcrConversion",
    ),
    fwd!(
        "vkDestroySemaphore",
        "ptr int64 ptr",
        "winevulkan.vkDestroySemaphore",
    ),
    fwd!(
        "vkDestroyShaderModule",
        "ptr int64 ptr",
        "winevulkan.vkDestroyShaderModule",
    ),
    fwd!(
        "vkDestroySurfaceKHR",
        "ptr int64 ptr",
        "winevulkan.vkDestroySurfaceKHR",
    ),
    fwd!(
        "vkDestroySwapchainKHR",
        "ptr int64 ptr",
        "winevulkan.vkDestroySwapchainKHR",
    ),
    fwd!("vkDeviceWaitIdle", "ptr", "winevulkan.vkDeviceWaitIdle"),
    fwd!("vkEndCommandBuffer", "ptr", "winevulkan.vkEndCommandBuffer"),
    fwd!(
        "vkEnumerateDeviceExtensionProperties",
        "ptr str ptr ptr",
        "winevulkan.vkEnumerateDeviceExtensionProperties",
    ),
    fwd!(
        "vkEnumerateDeviceLayerProperties",
        "ptr ptr ptr",
        "winevulkan.vkEnumerateDeviceLayerProperties",
    ),
    fwd!(
        "vkEnumerateInstanceExtensionProperties",
        "str ptr ptr",
        "winevulkan.vkEnumerateInstanceExtensionProperties",
    ),
    fwd!(
        "vkEnumerateInstanceLayerProperties",
        "ptr ptr",
        "winevulkan.vkEnumerateInstanceLayerProperties",
    ),
    fwd!(
        "vkEnumerateInstanceVersion",
        "ptr",
        "winevulkan.vkEnumerateInstanceVersion",
    ),
    fwd!(
        "vkEnumeratePhysicalDeviceGroups",
        "ptr ptr ptr",
        "winevulkan.vkEnumeratePhysicalDeviceGroups",
    ),
    fwd!(
        "vkEnumeratePhysicalDevices",
        "ptr ptr ptr",
        "winevulkan.vkEnumeratePhysicalDevices",
    ),
    fwd!(
        "vkFlushMappedMemoryRanges",
        "ptr long ptr",
        "winevulkan.vkFlushMappedMemoryRanges",
    ),
    fwd!(
        "vkFreeCommandBuffers",
        "ptr int64 long ptr",
        "winevulkan.vkFreeCommandBuffers",
    ),
    fwd!(
        "vkFreeDescriptorSets",
        "ptr int64 long ptr",
        "winevulkan.vkFreeDescriptorSets",
    ),
    fwd!("vkFreeMemory", "ptr int64 ptr", "winevulkan.vkFreeMemory"),
    fwd!(
        "vkGetBufferDeviceAddress",
        "ptr ptr",
        "winevulkan.vkGetBufferDeviceAddress",
    ),
    fwd!(
        "vkGetBufferMemoryRequirements",
        "ptr int64 ptr",
        "winevulkan.vkGetBufferMemoryRequirements",
    ),
    fwd!(
        "vkGetBufferMemoryRequirements2",
        "ptr ptr ptr",
        "winevulkan.vkGetBufferMemoryRequirements2",
    ),
    fwd!(
        "vkGetBufferOpaqueCaptureAddress",
        "ptr ptr",
        "winevulkan.vkGetBufferOpaqueCaptureAddress",
    ),
    stub!("vkGetCommandPoolMemoryConsumption"),
    fwd!(
        "vkGetDescriptorSetLayoutSupport",
        "ptr ptr ptr",
        "winevulkan.vkGetDescriptorSetLayoutSupport",
    ),
    fwd!(
        "vkGetDeviceBufferMemoryRequirements",
        "ptr ptr ptr",
        "winevulkan.vkGetDeviceBufferMemoryRequirements",
    ),
    fwd!(
        "vkGetDeviceGroupPeerMemoryFeatures",
        "ptr long long long ptr",
        "winevulkan.vkGetDeviceGroupPeerMemoryFeatures",
    ),
    fwd!(
        "vkGetDeviceGroupPresentCapabilitiesKHR",
        "ptr ptr",
        "winevulkan.vkGetDeviceGroupPresentCapabilitiesKHR",
    ),
    fwd!(
        "vkGetDeviceGroupSurfacePresentModesKHR",
        "ptr int64 ptr",
        "winevulkan.vkGetDeviceGroupSurfacePresentModesKHR",
    ),
    fwd!(
        "vkGetDeviceImageMemoryRequirements",
        "ptr ptr ptr",
        "winevulkan.vkGetDeviceImageMemoryRequirements",
    ),
    fwd!(
        "vkGetDeviceImageSparseMemoryRequirements",
        "ptr ptr ptr ptr",
        "winevulkan.vkGetDeviceImageSparseMemoryRequirements",
    ),
    fwd!(
        "vkGetDeviceImageSubresourceLayout",
        "ptr ptr ptr",
        "winevulkan.vkGetDeviceImageSubresourceLayout",
    ),
    fwd!(
        "vkGetDeviceMemoryCommitment",
        "ptr int64 ptr",
        "winevulkan.vkGetDeviceMemoryCommitment",
    ),
    fwd!(
        "vkGetDeviceMemoryOpaqueCaptureAddress",
        "ptr ptr",
        "winevulkan.vkGetDeviceMemoryOpaqueCaptureAddress",
    ),
    fwd!(
        "vkGetDeviceProcAddr",
        "ptr str",
        "winevulkan.vkGetDeviceProcAddr",
    ),
    fwd!(
        "vkGetDeviceQueue",
        "ptr long long ptr",
        "winevulkan.vkGetDeviceQueue",
    ),
    fwd!(
        "vkGetDeviceQueue2",
        "ptr ptr ptr",
        "winevulkan.vkGetDeviceQueue2",
    ),
    stub!("vkGetDisplayModePropertiesKHR"),
    stub!("vkGetDisplayPlaneCapabilitiesKHR"),
    stub!("vkGetDisplayPlaneSupportedDisplaysKHR"),
    fwd!(
        "vkGetEventStatus",
        "ptr int64",
        "winevulkan.vkGetEventStatus",
    ),
    stub!("vkGetFaultData"),
    fwd!(
        "vkGetFenceStatus",
        "ptr int64",
        "winevulkan.vkGetFenceStatus",
    ),
    fwd!(
        "vkGetImageMemoryRequirements",
        "ptr int64 ptr",
        "winevulkan.vkGetImageMemoryRequirements",
    ),
    fwd!(
        "vkGetImageMemoryRequirements2",
        "ptr ptr ptr",
        "winevulkan.vkGetImageMemoryRequirements2",
    ),
    fwd!(
        "vkGetImageSparseMemoryRequirements",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetImageSparseMemoryRequirements",
    ),
    fwd!(
        "vkGetImageSparseMemoryRequirements2",
        "ptr ptr ptr ptr",
        "winevulkan.vkGetImageSparseMemoryRequirements2",
    ),
    fwd!(
        "vkGetImageSubresourceLayout",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetImageSubresourceLayout",
    ),
    fwd!(
        "vkGetImageSubresourceLayout2",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetImageSubresourceLayout2",
    ),
    fwd!(
        "vkGetInstanceProcAddr",
        "ptr str",
        "winevulkan.vkGetInstanceProcAddr",
    ),
    stub!("vkGetPhysicalDeviceDisplayPlanePropertiesKHR"),
    stub!("vkGetPhysicalDeviceDisplayPropertiesKHR"),
    fwd!(
        "vkGetPhysicalDeviceExternalBufferProperties",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceExternalBufferProperties",
    ),
    fwd!(
        "vkGetPhysicalDeviceExternalFenceProperties",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceExternalFenceProperties",
    ),
    fwd!(
        "vkGetPhysicalDeviceExternalSemaphoreProperties",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceExternalSemaphoreProperties",
    ),
    fwd!(
        "vkGetPhysicalDeviceFeatures",
        "ptr ptr",
        "winevulkan.vkGetPhysicalDeviceFeatures",
    ),
    fwd!(
        "vkGetPhysicalDeviceFeatures2",
        "ptr ptr",
        "winevulkan.vkGetPhysicalDeviceFeatures2",
    ),
    fwd!(
        "vkGetPhysicalDeviceFormatProperties",
        "ptr long ptr",
        "winevulkan.vkGetPhysicalDeviceFormatProperties",
    ),
    fwd!(
        "vkGetPhysicalDeviceFormatProperties2",
        "ptr long ptr",
        "winevulkan.vkGetPhysicalDeviceFormatProperties2",
    ),
    fwd!(
        "vkGetPhysicalDeviceImageFormatProperties",
        "ptr long long long long long ptr",
        "winevulkan.vkGetPhysicalDeviceImageFormatProperties",
    ),
    fwd!(
        "vkGetPhysicalDeviceImageFormatProperties2",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceImageFormatProperties2",
    ),
    fwd!(
        "vkGetPhysicalDeviceMemoryProperties",
        "ptr ptr",
        "winevulkan.vkGetPhysicalDeviceMemoryProperties",
    ),
    fwd!(
        "vkGetPhysicalDeviceMemoryProperties2",
        "ptr ptr",
        "winevulkan.vkGetPhysicalDeviceMemoryProperties2",
    ),
    fwd!(
        "vkGetPhysicalDevicePresentRectanglesKHR",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetPhysicalDevicePresentRectanglesKHR",
    ),
    fwd!(
        "vkGetPhysicalDeviceProperties",
        "ptr ptr",
        "winevulkan.vkGetPhysicalDeviceProperties",
    ),
    fwd!(
        "vkGetPhysicalDeviceProperties2",
        "ptr ptr",
        "winevulkan.vkGetPhysicalDeviceProperties2",
    ),
    fwd!(
        "vkGetPhysicalDeviceQueueFamilyProperties",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceQueueFamilyProperties",
    ),
    fwd!(
        "vkGetPhysicalDeviceQueueFamilyProperties2",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceQueueFamilyProperties2",
    ),
    fwd!(
        "vkGetPhysicalDeviceSparseImageFormatProperties",
        "ptr long long long long long ptr ptr",
        "winevulkan.vkGetPhysicalDeviceSparseImageFormatProperties",
    ),
    fwd!(
        "vkGetPhysicalDeviceSparseImageFormatProperties2",
        "ptr ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceSparseImageFormatProperties2",
    ),
    fwd!(
        "vkGetPhysicalDeviceSurfaceCapabilities2KHR",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceSurfaceCapabilities2KHR",
    ),
    fwd!(
        "vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
        "ptr int64 ptr",
        "winevulkan.vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
    ),
    fwd!(
        "vkGetPhysicalDeviceSurfaceFormats2KHR",
        "ptr ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceSurfaceFormats2KHR",
    ),
    fwd!(
        "vkGetPhysicalDeviceSurfaceFormatsKHR",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetPhysicalDeviceSurfaceFormatsKHR",
    ),
    fwd!(
        "vkGetPhysicalDeviceSurfacePresentModesKHR",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetPhysicalDeviceSurfacePresentModesKHR",
    ),
    fwd!(
        "vkGetPhysicalDeviceSurfaceSupportKHR",
        "ptr long int64 ptr",
        "winevulkan.vkGetPhysicalDeviceSurfaceSupportKHR",
    ),
    fwd!(
        "vkGetPhysicalDeviceToolProperties",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceToolProperties",
    ),
    fwd!(
        "vkGetPhysicalDeviceWin32PresentationSupportKHR",
        "ptr long",
        "winevulkan.vkGetPhysicalDeviceWin32PresentationSupportKHR",
    ),
    fwd!(
        "vkGetPipelineCacheData",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetPipelineCacheData",
    ),
    fwd!(
        "vkGetPrivateData",
        "ptr long int64 int64 ptr",
        "winevulkan.vkGetPrivateData",
    ),
    fwd!(
        "vkGetQueryPoolResults",
        "ptr int64 long long long ptr int64 long",
        "winevulkan.vkGetQueryPoolResults",
    ),
    fwd!(
        "vkGetRenderAreaGranularity",
        "ptr int64 ptr",
        "winevulkan.vkGetRenderAreaGranularity",
    ),
    fwd!(
        "vkGetRenderingAreaGranularity",
        "ptr ptr ptr",
        "winevulkan.vkGetRenderingAreaGranularity",
    ),
    fwd!(
        "vkGetSemaphoreCounterValue",
        "ptr int64 ptr",
        "winevulkan.vkGetSemaphoreCounterValue",
    ),
    fwd!(
        "vkGetSwapchainImagesKHR",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetSwapchainImagesKHR",
    ),
    fwd!(
        "vkInvalidateMappedMemoryRanges",
        "ptr long ptr",
        "winevulkan.vkInvalidateMappedMemoryRanges",
    ),
    fwd!(
        "vkMapMemory",
        "ptr int64 int64 int64 long ptr",
        "winevulkan.vkMapMemory",
    ),
    fwd!("vkMapMemory2", "ptr ptr ptr", "winevulkan.vkMapMemory2"),
    fwd!(
        "vkMergePipelineCaches",
        "ptr int64 long ptr",
        "winevulkan.vkMergePipelineCaches",
    ),
    fwd!(
        "vkQueueBindSparse",
        "ptr long ptr int64",
        "winevulkan.vkQueueBindSparse",
    ),
    fwd!(
        "vkQueuePresentKHR",
        "ptr ptr",
        "winevulkan.vkQueuePresentKHR",
    ),
    fwd!(
        "vkQueueSubmit",
        "ptr long ptr int64",
        "winevulkan.vkQueueSubmit",
    ),
    fwd!(
        "vkQueueSubmit2",
        "ptr long ptr int64",
        "winevulkan.vkQueueSubmit2",
    ),
    fwd!("vkQueueWaitIdle", "ptr", "winevulkan.vkQueueWaitIdle"),
    fwd!(
        "vkResetCommandBuffer",
        "ptr long",
        "winevulkan.vkResetCommandBuffer",
    ),
    fwd!(
        "vkResetCommandPool",
        "ptr int64 long",
        "winevulkan.vkResetCommandPool",
    ),
    fwd!(
        "vkResetDescriptorPool",
        "ptr int64 long",
        "winevulkan.vkResetDescriptorPool",
    ),
    fwd!("vkResetEvent", "ptr int64", "winevulkan.vkResetEvent"),
    fwd!("vkResetFences", "ptr long ptr", "winevulkan.vkResetFences"),
    fwd!(
        "vkResetQueryPool",
        "ptr int64 long long",
        "winevulkan.vkResetQueryPool",
    ),
    fwd!("vkSetEvent", "ptr int64", "winevulkan.vkSetEvent"),
    fwd!(
        "vkSetPrivateData",
        "ptr long int64 int64 int64",
        "winevulkan.vkSetPrivateData",
    ),
    fwd!(
        "vkSignalSemaphore",
        "ptr ptr",
        "winevulkan.vkSignalSemaphore",
    ),
    fwd!(
        "vkTransitionImageLayout",
        "ptr long ptr",
        "winevulkan.vkTransitionImageLayout",
    ),
    fwd!(
        "vkTrimCommandPool",
        "ptr int64 long",
        "winevulkan.vkTrimCommandPool",
    ),
    fwd!("vkUnmapMemory", "ptr int64", "winevulkan.vkUnmapMemory"),
    fwd!("vkUnmapMemory2", "ptr ptr", "winevulkan.vkUnmapMemory2"),
    fwd!(
        "vkUpdateDescriptorSetWithTemplate",
        "ptr int64 int64 ptr",
        "winevulkan.vkUpdateDescriptorSetWithTemplate",
    ),
    fwd!(
        "vkUpdateDescriptorSets",
        "ptr long ptr long ptr",
        "winevulkan.vkUpdateDescriptorSets",
    ),
    fwd!(
        "vkWaitForFences",
        "ptr long ptr long int64",
        "winevulkan.vkWaitForFences",
    ),
    fwd!(
        "vkWaitSemaphores",
        "ptr ptr int64",
        "winevulkan.vkWaitSemaphores",
    ),
];

/// The export called `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn find_export(name: &str) -> Option<&'static Export> {
    EXPORTS.iter().find(|e| e.name == name)
}

/// The kind of `name`, or `None` when the spec does not export it.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    find_export(name).map(|e| e.kind)
}

/// The forward target of `name` (for example `winevulkan.vkCreateDevice`), or `None` when the
/// name is a stub or is not exported.
pub fn forward_target(name: &str) -> Option<&'static str> {
    match export_kind(name) {
        Some(ExportKind::Forward(target)) => Some(target),
        _ => None,
    }
}
