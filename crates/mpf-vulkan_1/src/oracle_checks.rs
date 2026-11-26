//! Oracle-derived test vectors for the ported vulkan-1 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DllMain`
//! (`dlls/vulkan-1/vulkan.c`) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/vulkan-1/`. The harness
//! replaces `RtlRaiseException` with a recorder, and parses `vulkan-1.spec` on its own to print
//! one row per export. It printed 268 rows: 6 `DllMain` rows, 10 stub rows and 252 forwarder
//! rows. ASan and UBSan flagged none, so no row is excluded.

use super::*;

/// `(reason, disable_thread_library_calls, get_dpi_for_system)`; every row returns `TRUE`.
const DLL_MAIN_ROWS: [(u32, bool, bool); 6] = [
    (0, false, false),
    (1, true, true),
    (2, false, false),
    (3, false, false),
    (4, false, false),
    (0xffff_ffff, false, false),
];

/// `(spec index, name, args, forward target)` for the `@ stdcall` rows.
const FORWARD_ROWS: [(usize, &str, &str, &str); 252] = [
    (
        0,
        "vkAcquireNextImage2KHR",
        "ptr ptr ptr",
        "winevulkan.vkAcquireNextImage2KHR",
    ),
    (
        1,
        "vkAcquireNextImageKHR",
        "ptr int64 int64 int64 int64 ptr",
        "winevulkan.vkAcquireNextImageKHR",
    ),
    (
        2,
        "vkAllocateCommandBuffers",
        "ptr ptr ptr",
        "winevulkan.vkAllocateCommandBuffers",
    ),
    (
        3,
        "vkAllocateDescriptorSets",
        "ptr ptr ptr",
        "winevulkan.vkAllocateDescriptorSets",
    ),
    (
        4,
        "vkAllocateMemory",
        "ptr ptr ptr ptr",
        "winevulkan.vkAllocateMemory",
    ),
    (
        5,
        "vkBeginCommandBuffer",
        "ptr ptr",
        "winevulkan.vkBeginCommandBuffer",
    ),
    (
        6,
        "vkBindBufferMemory",
        "ptr int64 int64 int64",
        "winevulkan.vkBindBufferMemory",
    ),
    (
        7,
        "vkBindBufferMemory2",
        "ptr long ptr",
        "winevulkan.vkBindBufferMemory2",
    ),
    (
        8,
        "vkBindImageMemory",
        "ptr int64 int64 int64",
        "winevulkan.vkBindImageMemory",
    ),
    (
        9,
        "vkBindImageMemory2",
        "ptr long ptr",
        "winevulkan.vkBindImageMemory2",
    ),
    (
        10,
        "vkCmdBeginQuery",
        "ptr int64 long long",
        "winevulkan.vkCmdBeginQuery",
    ),
    (
        11,
        "vkCmdBeginRenderPass",
        "ptr ptr long",
        "winevulkan.vkCmdBeginRenderPass",
    ),
    (
        12,
        "vkCmdBeginRenderPass2",
        "ptr ptr ptr",
        "winevulkan.vkCmdBeginRenderPass2",
    ),
    (
        13,
        "vkCmdBeginRendering",
        "ptr ptr",
        "winevulkan.vkCmdBeginRendering",
    ),
    (
        14,
        "vkCmdBindDescriptorSets",
        "ptr long int64 long long ptr long ptr",
        "winevulkan.vkCmdBindDescriptorSets",
    ),
    (
        15,
        "vkCmdBindDescriptorSets2",
        "ptr ptr",
        "winevulkan.vkCmdBindDescriptorSets2",
    ),
    (
        16,
        "vkCmdBindIndexBuffer",
        "ptr int64 int64 long",
        "winevulkan.vkCmdBindIndexBuffer",
    ),
    (
        17,
        "vkCmdBindIndexBuffer2",
        "ptr int64 int64 int64 long",
        "winevulkan.vkCmdBindIndexBuffer2",
    ),
    (
        18,
        "vkCmdBindPipeline",
        "ptr long int64",
        "winevulkan.vkCmdBindPipeline",
    ),
    (
        19,
        "vkCmdBindVertexBuffers",
        "ptr long long ptr ptr",
        "winevulkan.vkCmdBindVertexBuffers",
    ),
    (
        20,
        "vkCmdBindVertexBuffers2",
        "ptr long long ptr ptr ptr ptr",
        "winevulkan.vkCmdBindVertexBuffers2",
    ),
    (
        21,
        "vkCmdBlitImage",
        "ptr int64 long int64 long long ptr long",
        "winevulkan.vkCmdBlitImage",
    ),
    (
        22,
        "vkCmdBlitImage2",
        "ptr ptr",
        "winevulkan.vkCmdBlitImage2",
    ),
    (
        23,
        "vkCmdClearAttachments",
        "ptr long ptr long ptr",
        "winevulkan.vkCmdClearAttachments",
    ),
    (
        24,
        "vkCmdClearColorImage",
        "ptr int64 long ptr long ptr",
        "winevulkan.vkCmdClearColorImage",
    ),
    (
        25,
        "vkCmdClearDepthStencilImage",
        "ptr int64 long ptr long ptr",
        "winevulkan.vkCmdClearDepthStencilImage",
    ),
    (
        26,
        "vkCmdCopyBuffer",
        "ptr int64 int64 long ptr",
        "winevulkan.vkCmdCopyBuffer",
    ),
    (
        27,
        "vkCmdCopyBuffer2",
        "ptr ptr",
        "winevulkan.vkCmdCopyBuffer2",
    ),
    (
        28,
        "vkCmdCopyBufferToImage",
        "ptr int64 int64 long long ptr",
        "winevulkan.vkCmdCopyBufferToImage",
    ),
    (
        29,
        "vkCmdCopyBufferToImage2",
        "ptr ptr",
        "winevulkan.vkCmdCopyBufferToImage2",
    ),
    (
        30,
        "vkCmdCopyImage",
        "ptr int64 long int64 long long ptr",
        "winevulkan.vkCmdCopyImage",
    ),
    (
        31,
        "vkCmdCopyImage2",
        "ptr ptr",
        "winevulkan.vkCmdCopyImage2",
    ),
    (
        32,
        "vkCmdCopyImageToBuffer",
        "ptr int64 long int64 long ptr",
        "winevulkan.vkCmdCopyImageToBuffer",
    ),
    (
        33,
        "vkCmdCopyImageToBuffer2",
        "ptr ptr",
        "winevulkan.vkCmdCopyImageToBuffer2",
    ),
    (
        34,
        "vkCmdCopyQueryPoolResults",
        "ptr int64 long long int64 int64 int64 long",
        "winevulkan.vkCmdCopyQueryPoolResults",
    ),
    (
        35,
        "vkCmdDispatch",
        "ptr long long long",
        "winevulkan.vkCmdDispatch",
    ),
    (
        36,
        "vkCmdDispatchBase",
        "ptr long long long long long long",
        "winevulkan.vkCmdDispatchBase",
    ),
    (
        37,
        "vkCmdDispatchIndirect",
        "ptr int64 int64",
        "winevulkan.vkCmdDispatchIndirect",
    ),
    (
        38,
        "vkCmdDraw",
        "ptr long long long long",
        "winevulkan.vkCmdDraw",
    ),
    (
        39,
        "vkCmdDrawIndexed",
        "ptr long long long long long",
        "winevulkan.vkCmdDrawIndexed",
    ),
    (
        40,
        "vkCmdDrawIndexedIndirect",
        "ptr int64 int64 long long",
        "winevulkan.vkCmdDrawIndexedIndirect",
    ),
    (
        41,
        "vkCmdDrawIndexedIndirectCount",
        "ptr int64 int64 int64 int64 long long",
        "winevulkan.vkCmdDrawIndexedIndirectCount",
    ),
    (
        42,
        "vkCmdDrawIndirect",
        "ptr int64 int64 long long",
        "winevulkan.vkCmdDrawIndirect",
    ),
    (
        43,
        "vkCmdDrawIndirectCount",
        "ptr int64 int64 int64 int64 long long",
        "winevulkan.vkCmdDrawIndirectCount",
    ),
    (
        44,
        "vkCmdEndQuery",
        "ptr int64 long",
        "winevulkan.vkCmdEndQuery",
    ),
    (
        45,
        "vkCmdEndRenderPass",
        "ptr",
        "winevulkan.vkCmdEndRenderPass",
    ),
    (
        46,
        "vkCmdEndRenderPass2",
        "ptr ptr",
        "winevulkan.vkCmdEndRenderPass2",
    ),
    (
        47,
        "vkCmdEndRendering",
        "ptr",
        "winevulkan.vkCmdEndRendering",
    ),
    (
        48,
        "vkCmdExecuteCommands",
        "ptr long ptr",
        "winevulkan.vkCmdExecuteCommands",
    ),
    (
        49,
        "vkCmdFillBuffer",
        "ptr int64 int64 int64 long",
        "winevulkan.vkCmdFillBuffer",
    ),
    (
        50,
        "vkCmdNextSubpass",
        "ptr long",
        "winevulkan.vkCmdNextSubpass",
    ),
    (
        51,
        "vkCmdNextSubpass2",
        "ptr ptr ptr",
        "winevulkan.vkCmdNextSubpass2",
    ),
    (
        52,
        "vkCmdPipelineBarrier",
        "ptr long long long long ptr long ptr long ptr",
        "winevulkan.vkCmdPipelineBarrier",
    ),
    (
        53,
        "vkCmdPipelineBarrier2",
        "ptr ptr",
        "winevulkan.vkCmdPipelineBarrier2",
    ),
    (
        54,
        "vkCmdPushConstants",
        "ptr int64 long long long ptr",
        "winevulkan.vkCmdPushConstants",
    ),
    (
        55,
        "vkCmdPushConstants2",
        "ptr ptr",
        "winevulkan.vkCmdPushConstants2",
    ),
    (
        56,
        "vkCmdPushDescriptorSet",
        "ptr long int64 long long ptr",
        "winevulkan.vkCmdPushDescriptorSet",
    ),
    (
        57,
        "vkCmdPushDescriptorSet2",
        "ptr ptr",
        "winevulkan.vkCmdPushDescriptorSet2",
    ),
    (
        58,
        "vkCmdPushDescriptorSetWithTemplate",
        "ptr int64 int64 long ptr",
        "winevulkan.vkCmdPushDescriptorSetWithTemplate",
    ),
    (
        59,
        "vkCmdPushDescriptorSetWithTemplate2",
        "ptr ptr",
        "winevulkan.vkCmdPushDescriptorSetWithTemplate2",
    ),
    (
        60,
        "vkCmdResetEvent",
        "ptr int64 long",
        "winevulkan.vkCmdResetEvent",
    ),
    (
        61,
        "vkCmdResetEvent2",
        "ptr int64 int64",
        "winevulkan.vkCmdResetEvent2",
    ),
    (
        62,
        "vkCmdResetQueryPool",
        "ptr int64 long long",
        "winevulkan.vkCmdResetQueryPool",
    ),
    (
        63,
        "vkCmdResolveImage",
        "ptr int64 long int64 long long ptr",
        "winevulkan.vkCmdResolveImage",
    ),
    (
        64,
        "vkCmdResolveImage2",
        "ptr ptr",
        "winevulkan.vkCmdResolveImage2",
    ),
    (
        65,
        "vkCmdSetBlendConstants",
        "ptr ptr",
        "winevulkan.vkCmdSetBlendConstants",
    ),
    (
        66,
        "vkCmdSetCullMode",
        "ptr long",
        "winevulkan.vkCmdSetCullMode",
    ),
    (
        67,
        "vkCmdSetDepthBias",
        "ptr float float float",
        "winevulkan.vkCmdSetDepthBias",
    ),
    (
        68,
        "vkCmdSetDepthBiasEnable",
        "ptr long",
        "winevulkan.vkCmdSetDepthBiasEnable",
    ),
    (
        69,
        "vkCmdSetDepthBounds",
        "ptr float float",
        "winevulkan.vkCmdSetDepthBounds",
    ),
    (
        70,
        "vkCmdSetDepthBoundsTestEnable",
        "ptr long",
        "winevulkan.vkCmdSetDepthBoundsTestEnable",
    ),
    (
        71,
        "vkCmdSetDepthCompareOp",
        "ptr long",
        "winevulkan.vkCmdSetDepthCompareOp",
    ),
    (
        72,
        "vkCmdSetDepthTestEnable",
        "ptr long",
        "winevulkan.vkCmdSetDepthTestEnable",
    ),
    (
        73,
        "vkCmdSetDepthWriteEnable",
        "ptr long",
        "winevulkan.vkCmdSetDepthWriteEnable",
    ),
    (
        74,
        "vkCmdSetDeviceMask",
        "ptr long",
        "winevulkan.vkCmdSetDeviceMask",
    ),
    (
        75,
        "vkCmdSetEvent",
        "ptr int64 long",
        "winevulkan.vkCmdSetEvent",
    ),
    (
        76,
        "vkCmdSetEvent2",
        "ptr int64 ptr",
        "winevulkan.vkCmdSetEvent2",
    ),
    (
        77,
        "vkCmdSetFrontFace",
        "ptr long",
        "winevulkan.vkCmdSetFrontFace",
    ),
    (
        78,
        "vkCmdSetLineStipple",
        "ptr long long",
        "winevulkan.vkCmdSetLineStipple",
    ),
    (
        79,
        "vkCmdSetLineWidth",
        "ptr float",
        "winevulkan.vkCmdSetLineWidth",
    ),
    (
        80,
        "vkCmdSetPrimitiveRestartEnable",
        "ptr long",
        "winevulkan.vkCmdSetPrimitiveRestartEnable",
    ),
    (
        81,
        "vkCmdSetPrimitiveTopology",
        "ptr long",
        "winevulkan.vkCmdSetPrimitiveTopology",
    ),
    (
        82,
        "vkCmdSetRasterizerDiscardEnable",
        "ptr long",
        "winevulkan.vkCmdSetRasterizerDiscardEnable",
    ),
    (
        83,
        "vkCmdSetRenderingAttachmentLocations",
        "ptr ptr",
        "winevulkan.vkCmdSetRenderingAttachmentLocations",
    ),
    (
        84,
        "vkCmdSetRenderingInputAttachmentIndices",
        "ptr ptr",
        "winevulkan.vkCmdSetRenderingInputAttachmentIndices",
    ),
    (
        85,
        "vkCmdSetScissor",
        "ptr long long ptr",
        "winevulkan.vkCmdSetScissor",
    ),
    (
        86,
        "vkCmdSetScissorWithCount",
        "ptr long ptr",
        "winevulkan.vkCmdSetScissorWithCount",
    ),
    (
        87,
        "vkCmdSetStencilCompareMask",
        "ptr long long",
        "winevulkan.vkCmdSetStencilCompareMask",
    ),
    (
        88,
        "vkCmdSetStencilOp",
        "ptr long long long long long",
        "winevulkan.vkCmdSetStencilOp",
    ),
    (
        89,
        "vkCmdSetStencilReference",
        "ptr long long",
        "winevulkan.vkCmdSetStencilReference",
    ),
    (
        90,
        "vkCmdSetStencilTestEnable",
        "ptr long",
        "winevulkan.vkCmdSetStencilTestEnable",
    ),
    (
        91,
        "vkCmdSetStencilWriteMask",
        "ptr long long",
        "winevulkan.vkCmdSetStencilWriteMask",
    ),
    (
        92,
        "vkCmdSetViewport",
        "ptr long long ptr",
        "winevulkan.vkCmdSetViewport",
    ),
    (
        93,
        "vkCmdSetViewportWithCount",
        "ptr long ptr",
        "winevulkan.vkCmdSetViewportWithCount",
    ),
    (
        94,
        "vkCmdUpdateBuffer",
        "ptr int64 int64 int64 ptr",
        "winevulkan.vkCmdUpdateBuffer",
    ),
    (
        95,
        "vkCmdWaitEvents",
        "ptr long ptr long long long ptr long ptr long ptr",
        "winevulkan.vkCmdWaitEvents",
    ),
    (
        96,
        "vkCmdWaitEvents2",
        "ptr long ptr ptr",
        "winevulkan.vkCmdWaitEvents2",
    ),
    (
        97,
        "vkCmdWriteTimestamp",
        "ptr long int64 long",
        "winevulkan.vkCmdWriteTimestamp",
    ),
    (
        98,
        "vkCmdWriteTimestamp2",
        "ptr int64 int64 long",
        "winevulkan.vkCmdWriteTimestamp2",
    ),
    (
        99,
        "vkCopyImageToImage",
        "ptr ptr",
        "winevulkan.vkCopyImageToImage",
    ),
    (
        100,
        "vkCopyImageToMemory",
        "ptr ptr",
        "winevulkan.vkCopyImageToMemory",
    ),
    (
        101,
        "vkCopyMemoryToImage",
        "ptr ptr",
        "winevulkan.vkCopyMemoryToImage",
    ),
    (
        102,
        "vkCreateBuffer",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateBuffer",
    ),
    (
        103,
        "vkCreateBufferView",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateBufferView",
    ),
    (
        104,
        "vkCreateCommandPool",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateCommandPool",
    ),
    (
        105,
        "vkCreateComputePipelines",
        "ptr int64 long ptr ptr ptr",
        "winevulkan.vkCreateComputePipelines",
    ),
    (
        106,
        "vkCreateDescriptorPool",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateDescriptorPool",
    ),
    (
        107,
        "vkCreateDescriptorSetLayout",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateDescriptorSetLayout",
    ),
    (
        108,
        "vkCreateDescriptorUpdateTemplate",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateDescriptorUpdateTemplate",
    ),
    (
        109,
        "vkCreateDevice",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateDevice",
    ),
    (
        112,
        "vkCreateEvent",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateEvent",
    ),
    (
        113,
        "vkCreateFence",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateFence",
    ),
    (
        114,
        "vkCreateFramebuffer",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateFramebuffer",
    ),
    (
        115,
        "vkCreateGraphicsPipelines",
        "ptr int64 long ptr ptr ptr",
        "winevulkan.vkCreateGraphicsPipelines",
    ),
    (
        116,
        "vkCreateImage",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateImage",
    ),
    (
        117,
        "vkCreateImageView",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateImageView",
    ),
    (
        118,
        "vkCreateInstance",
        "ptr ptr ptr",
        "winevulkan.vkCreateInstance",
    ),
    (
        119,
        "vkCreatePipelineCache",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreatePipelineCache",
    ),
    (
        120,
        "vkCreatePipelineLayout",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreatePipelineLayout",
    ),
    (
        121,
        "vkCreatePrivateDataSlot",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreatePrivateDataSlot",
    ),
    (
        122,
        "vkCreateQueryPool",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateQueryPool",
    ),
    (
        123,
        "vkCreateRenderPass",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateRenderPass",
    ),
    (
        124,
        "vkCreateRenderPass2",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateRenderPass2",
    ),
    (
        125,
        "vkCreateSampler",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateSampler",
    ),
    (
        126,
        "vkCreateSamplerYcbcrConversion",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateSamplerYcbcrConversion",
    ),
    (
        127,
        "vkCreateSemaphore",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateSemaphore",
    ),
    (
        128,
        "vkCreateShaderModule",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateShaderModule",
    ),
    (
        130,
        "vkCreateSwapchainKHR",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateSwapchainKHR",
    ),
    (
        131,
        "vkCreateWin32SurfaceKHR",
        "ptr ptr ptr ptr",
        "winevulkan.vkCreateWin32SurfaceKHR",
    ),
    (
        132,
        "vkDestroyBuffer",
        "ptr int64 ptr",
        "winevulkan.vkDestroyBuffer",
    ),
    (
        133,
        "vkDestroyBufferView",
        "ptr int64 ptr",
        "winevulkan.vkDestroyBufferView",
    ),
    (
        134,
        "vkDestroyCommandPool",
        "ptr int64 ptr",
        "winevulkan.vkDestroyCommandPool",
    ),
    (
        135,
        "vkDestroyDescriptorPool",
        "ptr int64 ptr",
        "winevulkan.vkDestroyDescriptorPool",
    ),
    (
        136,
        "vkDestroyDescriptorSetLayout",
        "ptr int64 ptr",
        "winevulkan.vkDestroyDescriptorSetLayout",
    ),
    (
        137,
        "vkDestroyDescriptorUpdateTemplate",
        "ptr int64 ptr",
        "winevulkan.vkDestroyDescriptorUpdateTemplate",
    ),
    (
        138,
        "vkDestroyDevice",
        "ptr ptr",
        "winevulkan.vkDestroyDevice",
    ),
    (
        139,
        "vkDestroyEvent",
        "ptr int64 ptr",
        "winevulkan.vkDestroyEvent",
    ),
    (
        140,
        "vkDestroyFence",
        "ptr int64 ptr",
        "winevulkan.vkDestroyFence",
    ),
    (
        141,
        "vkDestroyFramebuffer",
        "ptr int64 ptr",
        "winevulkan.vkDestroyFramebuffer",
    ),
    (
        142,
        "vkDestroyImage",
        "ptr int64 ptr",
        "winevulkan.vkDestroyImage",
    ),
    (
        143,
        "vkDestroyImageView",
        "ptr int64 ptr",
        "winevulkan.vkDestroyImageView",
    ),
    (
        144,
        "vkDestroyInstance",
        "ptr ptr",
        "winevulkan.vkDestroyInstance",
    ),
    (
        145,
        "vkDestroyPipeline",
        "ptr int64 ptr",
        "winevulkan.vkDestroyPipeline",
    ),
    (
        146,
        "vkDestroyPipelineCache",
        "ptr int64 ptr",
        "winevulkan.vkDestroyPipelineCache",
    ),
    (
        147,
        "vkDestroyPipelineLayout",
        "ptr int64 ptr",
        "winevulkan.vkDestroyPipelineLayout",
    ),
    (
        148,
        "vkDestroyPrivateDataSlot",
        "ptr int64 ptr",
        "winevulkan.vkDestroyPrivateDataSlot",
    ),
    (
        149,
        "vkDestroyQueryPool",
        "ptr int64 ptr",
        "winevulkan.vkDestroyQueryPool",
    ),
    (
        150,
        "vkDestroyRenderPass",
        "ptr int64 ptr",
        "winevulkan.vkDestroyRenderPass",
    ),
    (
        151,
        "vkDestroySampler",
        "ptr int64 ptr",
        "winevulkan.vkDestroySampler",
    ),
    (
        152,
        "vkDestroySamplerYcbcrConversion",
        "ptr int64 ptr",
        "winevulkan.vkDestroySamplerYcbcrConversion",
    ),
    (
        153,
        "vkDestroySemaphore",
        "ptr int64 ptr",
        "winevulkan.vkDestroySemaphore",
    ),
    (
        154,
        "vkDestroyShaderModule",
        "ptr int64 ptr",
        "winevulkan.vkDestroyShaderModule",
    ),
    (
        155,
        "vkDestroySurfaceKHR",
        "ptr int64 ptr",
        "winevulkan.vkDestroySurfaceKHR",
    ),
    (
        156,
        "vkDestroySwapchainKHR",
        "ptr int64 ptr",
        "winevulkan.vkDestroySwapchainKHR",
    ),
    (
        157,
        "vkDeviceWaitIdle",
        "ptr",
        "winevulkan.vkDeviceWaitIdle",
    ),
    (
        158,
        "vkEndCommandBuffer",
        "ptr",
        "winevulkan.vkEndCommandBuffer",
    ),
    (
        159,
        "vkEnumerateDeviceExtensionProperties",
        "ptr str ptr ptr",
        "winevulkan.vkEnumerateDeviceExtensionProperties",
    ),
    (
        160,
        "vkEnumerateDeviceLayerProperties",
        "ptr ptr ptr",
        "winevulkan.vkEnumerateDeviceLayerProperties",
    ),
    (
        161,
        "vkEnumerateInstanceExtensionProperties",
        "str ptr ptr",
        "winevulkan.vkEnumerateInstanceExtensionProperties",
    ),
    (
        162,
        "vkEnumerateInstanceLayerProperties",
        "ptr ptr",
        "winevulkan.vkEnumerateInstanceLayerProperties",
    ),
    (
        163,
        "vkEnumerateInstanceVersion",
        "ptr",
        "winevulkan.vkEnumerateInstanceVersion",
    ),
    (
        164,
        "vkEnumeratePhysicalDeviceGroups",
        "ptr ptr ptr",
        "winevulkan.vkEnumeratePhysicalDeviceGroups",
    ),
    (
        165,
        "vkEnumeratePhysicalDevices",
        "ptr ptr ptr",
        "winevulkan.vkEnumeratePhysicalDevices",
    ),
    (
        166,
        "vkFlushMappedMemoryRanges",
        "ptr long ptr",
        "winevulkan.vkFlushMappedMemoryRanges",
    ),
    (
        167,
        "vkFreeCommandBuffers",
        "ptr int64 long ptr",
        "winevulkan.vkFreeCommandBuffers",
    ),
    (
        168,
        "vkFreeDescriptorSets",
        "ptr int64 long ptr",
        "winevulkan.vkFreeDescriptorSets",
    ),
    (
        169,
        "vkFreeMemory",
        "ptr int64 ptr",
        "winevulkan.vkFreeMemory",
    ),
    (
        170,
        "vkGetBufferDeviceAddress",
        "ptr ptr",
        "winevulkan.vkGetBufferDeviceAddress",
    ),
    (
        171,
        "vkGetBufferMemoryRequirements",
        "ptr int64 ptr",
        "winevulkan.vkGetBufferMemoryRequirements",
    ),
    (
        172,
        "vkGetBufferMemoryRequirements2",
        "ptr ptr ptr",
        "winevulkan.vkGetBufferMemoryRequirements2",
    ),
    (
        173,
        "vkGetBufferOpaqueCaptureAddress",
        "ptr ptr",
        "winevulkan.vkGetBufferOpaqueCaptureAddress",
    ),
    (
        175,
        "vkGetDescriptorSetLayoutSupport",
        "ptr ptr ptr",
        "winevulkan.vkGetDescriptorSetLayoutSupport",
    ),
    (
        176,
        "vkGetDeviceBufferMemoryRequirements",
        "ptr ptr ptr",
        "winevulkan.vkGetDeviceBufferMemoryRequirements",
    ),
    (
        177,
        "vkGetDeviceGroupPeerMemoryFeatures",
        "ptr long long long ptr",
        "winevulkan.vkGetDeviceGroupPeerMemoryFeatures",
    ),
    (
        178,
        "vkGetDeviceGroupPresentCapabilitiesKHR",
        "ptr ptr",
        "winevulkan.vkGetDeviceGroupPresentCapabilitiesKHR",
    ),
    (
        179,
        "vkGetDeviceGroupSurfacePresentModesKHR",
        "ptr int64 ptr",
        "winevulkan.vkGetDeviceGroupSurfacePresentModesKHR",
    ),
    (
        180,
        "vkGetDeviceImageMemoryRequirements",
        "ptr ptr ptr",
        "winevulkan.vkGetDeviceImageMemoryRequirements",
    ),
    (
        181,
        "vkGetDeviceImageSparseMemoryRequirements",
        "ptr ptr ptr ptr",
        "winevulkan.vkGetDeviceImageSparseMemoryRequirements",
    ),
    (
        182,
        "vkGetDeviceImageSubresourceLayout",
        "ptr ptr ptr",
        "winevulkan.vkGetDeviceImageSubresourceLayout",
    ),
    (
        183,
        "vkGetDeviceMemoryCommitment",
        "ptr int64 ptr",
        "winevulkan.vkGetDeviceMemoryCommitment",
    ),
    (
        184,
        "vkGetDeviceMemoryOpaqueCaptureAddress",
        "ptr ptr",
        "winevulkan.vkGetDeviceMemoryOpaqueCaptureAddress",
    ),
    (
        185,
        "vkGetDeviceProcAddr",
        "ptr str",
        "winevulkan.vkGetDeviceProcAddr",
    ),
    (
        186,
        "vkGetDeviceQueue",
        "ptr long long ptr",
        "winevulkan.vkGetDeviceQueue",
    ),
    (
        187,
        "vkGetDeviceQueue2",
        "ptr ptr ptr",
        "winevulkan.vkGetDeviceQueue2",
    ),
    (
        191,
        "vkGetEventStatus",
        "ptr int64",
        "winevulkan.vkGetEventStatus",
    ),
    (
        193,
        "vkGetFenceStatus",
        "ptr int64",
        "winevulkan.vkGetFenceStatus",
    ),
    (
        194,
        "vkGetImageMemoryRequirements",
        "ptr int64 ptr",
        "winevulkan.vkGetImageMemoryRequirements",
    ),
    (
        195,
        "vkGetImageMemoryRequirements2",
        "ptr ptr ptr",
        "winevulkan.vkGetImageMemoryRequirements2",
    ),
    (
        196,
        "vkGetImageSparseMemoryRequirements",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetImageSparseMemoryRequirements",
    ),
    (
        197,
        "vkGetImageSparseMemoryRequirements2",
        "ptr ptr ptr ptr",
        "winevulkan.vkGetImageSparseMemoryRequirements2",
    ),
    (
        198,
        "vkGetImageSubresourceLayout",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetImageSubresourceLayout",
    ),
    (
        199,
        "vkGetImageSubresourceLayout2",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetImageSubresourceLayout2",
    ),
    (
        200,
        "vkGetInstanceProcAddr",
        "ptr str",
        "winevulkan.vkGetInstanceProcAddr",
    ),
    (
        203,
        "vkGetPhysicalDeviceExternalBufferProperties",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceExternalBufferProperties",
    ),
    (
        204,
        "vkGetPhysicalDeviceExternalFenceProperties",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceExternalFenceProperties",
    ),
    (
        205,
        "vkGetPhysicalDeviceExternalSemaphoreProperties",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceExternalSemaphoreProperties",
    ),
    (
        206,
        "vkGetPhysicalDeviceFeatures",
        "ptr ptr",
        "winevulkan.vkGetPhysicalDeviceFeatures",
    ),
    (
        207,
        "vkGetPhysicalDeviceFeatures2",
        "ptr ptr",
        "winevulkan.vkGetPhysicalDeviceFeatures2",
    ),
    (
        208,
        "vkGetPhysicalDeviceFormatProperties",
        "ptr long ptr",
        "winevulkan.vkGetPhysicalDeviceFormatProperties",
    ),
    (
        209,
        "vkGetPhysicalDeviceFormatProperties2",
        "ptr long ptr",
        "winevulkan.vkGetPhysicalDeviceFormatProperties2",
    ),
    (
        210,
        "vkGetPhysicalDeviceImageFormatProperties",
        "ptr long long long long long ptr",
        "winevulkan.vkGetPhysicalDeviceImageFormatProperties",
    ),
    (
        211,
        "vkGetPhysicalDeviceImageFormatProperties2",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceImageFormatProperties2",
    ),
    (
        212,
        "vkGetPhysicalDeviceMemoryProperties",
        "ptr ptr",
        "winevulkan.vkGetPhysicalDeviceMemoryProperties",
    ),
    (
        213,
        "vkGetPhysicalDeviceMemoryProperties2",
        "ptr ptr",
        "winevulkan.vkGetPhysicalDeviceMemoryProperties2",
    ),
    (
        214,
        "vkGetPhysicalDevicePresentRectanglesKHR",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetPhysicalDevicePresentRectanglesKHR",
    ),
    (
        215,
        "vkGetPhysicalDeviceProperties",
        "ptr ptr",
        "winevulkan.vkGetPhysicalDeviceProperties",
    ),
    (
        216,
        "vkGetPhysicalDeviceProperties2",
        "ptr ptr",
        "winevulkan.vkGetPhysicalDeviceProperties2",
    ),
    (
        217,
        "vkGetPhysicalDeviceQueueFamilyProperties",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceQueueFamilyProperties",
    ),
    (
        218,
        "vkGetPhysicalDeviceQueueFamilyProperties2",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceQueueFamilyProperties2",
    ),
    (
        219,
        "vkGetPhysicalDeviceSparseImageFormatProperties",
        "ptr long long long long long ptr ptr",
        "winevulkan.vkGetPhysicalDeviceSparseImageFormatProperties",
    ),
    (
        220,
        "vkGetPhysicalDeviceSparseImageFormatProperties2",
        "ptr ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceSparseImageFormatProperties2",
    ),
    (
        221,
        "vkGetPhysicalDeviceSurfaceCapabilities2KHR",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceSurfaceCapabilities2KHR",
    ),
    (
        222,
        "vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
        "ptr int64 ptr",
        "winevulkan.vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
    ),
    (
        223,
        "vkGetPhysicalDeviceSurfaceFormats2KHR",
        "ptr ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceSurfaceFormats2KHR",
    ),
    (
        224,
        "vkGetPhysicalDeviceSurfaceFormatsKHR",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetPhysicalDeviceSurfaceFormatsKHR",
    ),
    (
        225,
        "vkGetPhysicalDeviceSurfacePresentModesKHR",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetPhysicalDeviceSurfacePresentModesKHR",
    ),
    (
        226,
        "vkGetPhysicalDeviceSurfaceSupportKHR",
        "ptr long int64 ptr",
        "winevulkan.vkGetPhysicalDeviceSurfaceSupportKHR",
    ),
    (
        227,
        "vkGetPhysicalDeviceToolProperties",
        "ptr ptr ptr",
        "winevulkan.vkGetPhysicalDeviceToolProperties",
    ),
    (
        228,
        "vkGetPhysicalDeviceWin32PresentationSupportKHR",
        "ptr long",
        "winevulkan.vkGetPhysicalDeviceWin32PresentationSupportKHR",
    ),
    (
        229,
        "vkGetPipelineCacheData",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetPipelineCacheData",
    ),
    (
        230,
        "vkGetPrivateData",
        "ptr long int64 int64 ptr",
        "winevulkan.vkGetPrivateData",
    ),
    (
        231,
        "vkGetQueryPoolResults",
        "ptr int64 long long long ptr int64 long",
        "winevulkan.vkGetQueryPoolResults",
    ),
    (
        232,
        "vkGetRenderAreaGranularity",
        "ptr int64 ptr",
        "winevulkan.vkGetRenderAreaGranularity",
    ),
    (
        233,
        "vkGetRenderingAreaGranularity",
        "ptr ptr ptr",
        "winevulkan.vkGetRenderingAreaGranularity",
    ),
    (
        234,
        "vkGetSemaphoreCounterValue",
        "ptr int64 ptr",
        "winevulkan.vkGetSemaphoreCounterValue",
    ),
    (
        235,
        "vkGetSwapchainImagesKHR",
        "ptr int64 ptr ptr",
        "winevulkan.vkGetSwapchainImagesKHR",
    ),
    (
        236,
        "vkInvalidateMappedMemoryRanges",
        "ptr long ptr",
        "winevulkan.vkInvalidateMappedMemoryRanges",
    ),
    (
        237,
        "vkMapMemory",
        "ptr int64 int64 int64 long ptr",
        "winevulkan.vkMapMemory",
    ),
    (
        238,
        "vkMapMemory2",
        "ptr ptr ptr",
        "winevulkan.vkMapMemory2",
    ),
    (
        239,
        "vkMergePipelineCaches",
        "ptr int64 long ptr",
        "winevulkan.vkMergePipelineCaches",
    ),
    (
        240,
        "vkQueueBindSparse",
        "ptr long ptr int64",
        "winevulkan.vkQueueBindSparse",
    ),
    (
        241,
        "vkQueuePresentKHR",
        "ptr ptr",
        "winevulkan.vkQueuePresentKHR",
    ),
    (
        242,
        "vkQueueSubmit",
        "ptr long ptr int64",
        "winevulkan.vkQueueSubmit",
    ),
    (
        243,
        "vkQueueSubmit2",
        "ptr long ptr int64",
        "winevulkan.vkQueueSubmit2",
    ),
    (244, "vkQueueWaitIdle", "ptr", "winevulkan.vkQueueWaitIdle"),
    (
        245,
        "vkResetCommandBuffer",
        "ptr long",
        "winevulkan.vkResetCommandBuffer",
    ),
    (
        246,
        "vkResetCommandPool",
        "ptr int64 long",
        "winevulkan.vkResetCommandPool",
    ),
    (
        247,
        "vkResetDescriptorPool",
        "ptr int64 long",
        "winevulkan.vkResetDescriptorPool",
    ),
    (248, "vkResetEvent", "ptr int64", "winevulkan.vkResetEvent"),
    (
        249,
        "vkResetFences",
        "ptr long ptr",
        "winevulkan.vkResetFences",
    ),
    (
        250,
        "vkResetQueryPool",
        "ptr int64 long long",
        "winevulkan.vkResetQueryPool",
    ),
    (251, "vkSetEvent", "ptr int64", "winevulkan.vkSetEvent"),
    (
        252,
        "vkSetPrivateData",
        "ptr long int64 int64 int64",
        "winevulkan.vkSetPrivateData",
    ),
    (
        253,
        "vkSignalSemaphore",
        "ptr ptr",
        "winevulkan.vkSignalSemaphore",
    ),
    (
        254,
        "vkTransitionImageLayout",
        "ptr long ptr",
        "winevulkan.vkTransitionImageLayout",
    ),
    (
        255,
        "vkTrimCommandPool",
        "ptr int64 long",
        "winevulkan.vkTrimCommandPool",
    ),
    (
        256,
        "vkUnmapMemory",
        "ptr int64",
        "winevulkan.vkUnmapMemory",
    ),
    (
        257,
        "vkUnmapMemory2",
        "ptr ptr",
        "winevulkan.vkUnmapMemory2",
    ),
    (
        258,
        "vkUpdateDescriptorSetWithTemplate",
        "ptr int64 int64 ptr",
        "winevulkan.vkUpdateDescriptorSetWithTemplate",
    ),
    (
        259,
        "vkUpdateDescriptorSets",
        "ptr long ptr long ptr",
        "winevulkan.vkUpdateDescriptorSets",
    ),
    (
        260,
        "vkWaitForFences",
        "ptr long ptr long int64",
        "winevulkan.vkWaitForFences",
    ),
    (
        261,
        "vkWaitSemaphores",
        "ptr ptr int64",
        "winevulkan.vkWaitSemaphores",
    ),
];

/// `(spec index, name)` for the `@ stub` rows. The harness printed
/// `code=0x80000100 flags=0x1 nparams=2 module=vulkan-1.dll function=<name>` for each.
const STUB_ROWS: [(usize, &str); 10] = [
    (110, "vkCreateDisplayModeKHR"),
    (111, "vkCreateDisplayPlaneSurfaceKHR"),
    (129, "vkCreateSharedSwapchainsKHR"),
    (174, "vkGetCommandPoolMemoryConsumption"),
    (188, "vkGetDisplayModePropertiesKHR"),
    (189, "vkGetDisplayPlaneCapabilitiesKHR"),
    (190, "vkGetDisplayPlaneSupportedDisplaysKHR"),
    (192, "vkGetFaultData"),
    (201, "vkGetPhysicalDeviceDisplayPlanePropertiesKHR"),
    (202, "vkGetPhysicalDeviceDisplayPropertiesKHR"),
];

#[test]
fn oracle_dll_main_rows() {
    for (reason, disable, dpi) in DLL_MAIN_ROWS {
        let (ret, effects) = dll_main(reason);
        assert!(ret, "reason {reason}");
        assert_eq!(
            effects.disable_thread_library_calls, disable,
            "reason {reason}"
        );
        assert_eq!(effects.get_dpi_for_system, dpi, "reason {reason}");
    }
}

#[test]
fn oracle_forward_rows() {
    for (index, name, args, target) in FORWARD_ROWS {
        assert_eq!(EXPORTS[index].name, name);
        assert_eq!(EXPORTS[index].args, args, "{name}");
        assert_eq!(EXPORTS[index].kind, ExportKind::Forward(target), "{name}");
        assert_eq!(export_kind(name), Some(ExportKind::Forward(target)));
        assert_eq!(forward_target(name), Some(target));
        assert_eq!(stub_exception(name), None);
    }
}

#[test]
fn oracle_stub_rows() {
    for (index, name) in STUB_ROWS {
        assert_eq!(EXPORTS[index].name, name);
        assert_eq!(EXPORTS[index].kind, ExportKind::Stub);
        assert_eq!(forward_target(name), None);
        assert_eq!(
            stub_exception(name),
            Some(StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "vulkan-1.dll",
                function: name,
            })
        );
    }
}

type StubHelper = (&'static str, fn() -> StubException);

#[test]
fn oracle_stub_helpers() {
    let helpers: [StubHelper; 10] = [
        ("vkCreateDisplayModeKHR", vk_create_display_mode_khr),
        (
            "vkCreateDisplayPlaneSurfaceKHR",
            vk_create_display_plane_surface_khr,
        ),
        (
            "vkCreateSharedSwapchainsKHR",
            vk_create_shared_swapchains_khr,
        ),
        (
            "vkGetCommandPoolMemoryConsumption",
            vk_get_command_pool_memory_consumption,
        ),
        (
            "vkGetDisplayModePropertiesKHR",
            vk_get_display_mode_properties_khr,
        ),
        (
            "vkGetDisplayPlaneCapabilitiesKHR",
            vk_get_display_plane_capabilities_khr,
        ),
        (
            "vkGetDisplayPlaneSupportedDisplaysKHR",
            vk_get_display_plane_supported_displays_khr,
        ),
        ("vkGetFaultData", vk_get_fault_data),
        (
            "vkGetPhysicalDeviceDisplayPlanePropertiesKHR",
            vk_get_physical_device_display_plane_properties_khr,
        ),
        (
            "vkGetPhysicalDeviceDisplayPropertiesKHR",
            vk_get_physical_device_display_properties_khr,
        ),
    ];
    assert_eq!(helpers.len(), STUB_ROWS.len());
    for ((name, helper), (_, row_name)) in helpers.iter().zip(STUB_ROWS) {
        assert_eq!(*name, row_name);
        assert_eq!(Some(helper()), stub_exception(name));
        assert_eq!(helper().module, "vulkan-1.dll");
        assert_eq!(helper().function, *name);
    }
}

#[test]
fn oracle_table_size() {
    assert_eq!(EXPORTS.len(), FORWARD_ROWS.len() + STUB_ROWS.len());
}
