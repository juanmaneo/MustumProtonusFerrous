/// How `dlls/d3dx10_36/d3dx10_36.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall` with no forward. The body is in `d3dx10_36_main.c` or, for the
    /// other names, in the parent source directory `dlls/d3dx9_36`.
    Stdcall,
    /// `@ stdcall NAME(...) dll.NAME`: the export is forwarded to the named DLL.
    Forward(&'static str),
}

/// The `d3dx10_36.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 180] = [
    ("D3DX10CreateThreadPump", ExportKind::Forward("d3dx10_43")),
    ("D3DX10CheckVersion", ExportKind::Stdcall),
    ("D3DX10CompileFromFileA", ExportKind::Forward("d3dx10_43")),
    ("D3DX10CompileFromFileW", ExportKind::Forward("d3dx10_43")),
    ("D3DX10CompileFromMemory", ExportKind::Stdcall),
    (
        "D3DX10CompileFromResourceA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CompileFromResourceW",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DX10ComputeNormalMap", ExportKind::Forward("d3dx10_43")),
    (
        "D3DX10CreateAsyncCompilerProcessor",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateAsyncEffectCreateProcessor",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateAsyncEffectPoolCreateProcessor",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateAsyncFileLoaderA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateAsyncFileLoaderW",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateAsyncMemoryLoader",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateAsyncResourceLoaderA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateAsyncResourceLoaderW",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateAsyncShaderPreprocessProcessor",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateAsyncShaderResourceViewProcessor",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateAsyncTextureInfoProcessor",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateAsyncTextureProcessor",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DX10CreateDevice", ExportKind::Forward("d3dx10_43")),
    (
        "D3DX10CreateDeviceAndSwapChain",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DX10CreateEffectFromFileA", ExportKind::Stdcall),
    ("D3DX10CreateEffectFromFileW", ExportKind::Stdcall),
    ("D3DX10CreateEffectFromMemory", ExportKind::Stdcall),
    ("D3DX10CreateEffectFromResourceA", ExportKind::Stdcall),
    ("D3DX10CreateEffectFromResourceW", ExportKind::Stdcall),
    (
        "D3DX10CreateEffectPoolFromFileA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateEffectPoolFromFileW",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateEffectPoolFromMemory",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateEffectPoolFromResourceA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateEffectPoolFromResourceW",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DX10CreateFontA", ExportKind::Forward("d3dx10_43")),
    (
        "D3DX10CreateFontIndirectA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateFontIndirectW",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DX10CreateFontW", ExportKind::Forward("d3dx10_43")),
    ("D3DX10CreateMesh", ExportKind::Forward("d3dx10_43")),
    (
        "D3DX10CreateShaderResourceViewFromFileA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateShaderResourceViewFromFileW",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateShaderResourceViewFromMemory",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateShaderResourceViewFromResourceA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateShaderResourceViewFromResourceW",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DX10CreateSkinInfo", ExportKind::Forward("d3dx10_43")),
    ("D3DX10CreateSprite", ExportKind::Forward("d3dx10_43")),
    (
        "D3DX10CreateTextureFromFileA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateTextureFromFileW",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateTextureFromMemory",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateTextureFromResourceA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10CreateTextureFromResourceW",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DX10DisassembleEffect", ExportKind::Forward("d3dx10_39")),
    ("D3DX10DisassembleShader", ExportKind::Forward("d3dx10_39")),
    ("D3DX10FilterTexture", ExportKind::Forward("d3dx10_43")),
    ("D3DX10GetDriverLevel", ExportKind::Forward("d3dx10_37")),
    ("D3DX10GetFeatureLevel1", ExportKind::Forward("d3dx10_43")),
    (
        "D3DX10GetImageInfoFromFileA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10GetImageInfoFromFileW",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10GetImageInfoFromMemory",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10GetImageInfoFromResourceA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10GetImageInfoFromResourceW",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10LoadTextureFromTexture",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10PreprocessShaderFromFileA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10PreprocessShaderFromFileW",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DX10PreprocessShaderFromMemory", ExportKind::Stdcall),
    (
        "D3DX10PreprocessShaderFromResourceA",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10PreprocessShaderFromResourceW",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DX10ReflectShader", ExportKind::Forward("d3dx10_39")),
    ("D3DX10SHProjectCubeMap", ExportKind::Forward("d3dx10_43")),
    ("D3DX10SaveTextureToFileA", ExportKind::Forward("d3dx10_43")),
    ("D3DX10SaveTextureToFileW", ExportKind::Forward("d3dx10_43")),
    (
        "D3DX10SaveTextureToMemory",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DX10UnsetAllDeviceObjects",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXBoxBoundProbe", ExportKind::Forward("d3dx10_43")),
    ("D3DXColorAdjustContrast", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXColorAdjustSaturation",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXComputeBoundingBox", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXComputeBoundingSphere",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXCpuOptimizations", ExportKind::Forward("d3dx10_43")),
    ("D3DXCreateMatrixStack", ExportKind::Forward("d3dx10_43")),
    ("D3DXFloat16To32Array", ExportKind::Forward("d3dx10_43")),
    ("D3DXFloat32To16Array", ExportKind::Forward("d3dx10_43")),
    ("D3DXFresnelTerm", ExportKind::Forward("d3dx10_43")),
    ("D3DXIntersectTri", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXMatrixAffineTransformation2D",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DXMatrixAffineTransformation",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXMatrixDecompose", ExportKind::Forward("d3dx10_43")),
    ("D3DXMatrixDeterminant", ExportKind::Forward("d3dx10_43")),
    ("D3DXMatrixInverse", ExportKind::Forward("d3dx10_43")),
    ("D3DXMatrixLookAtLH", ExportKind::Forward("d3dx10_43")),
    ("D3DXMatrixLookAtRH", ExportKind::Forward("d3dx10_43")),
    ("D3DXMatrixMultiply", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXMatrixMultiplyTranspose",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXMatrixOrthoLH", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXMatrixOrthoOffCenterLH",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DXMatrixOrthoOffCenterRH",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXMatrixOrthoRH", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXMatrixPerspectiveFovLH",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DXMatrixPerspectiveFovRH",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXMatrixPerspectiveLH", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXMatrixPerspectiveOffCenterLH",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DXMatrixPerspectiveOffCenterRH",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXMatrixPerspectiveRH", ExportKind::Forward("d3dx10_43")),
    ("D3DXMatrixReflect", ExportKind::Forward("d3dx10_43")),
    ("D3DXMatrixRotationAxis", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXMatrixRotationQuaternion",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXMatrixRotationX", ExportKind::Forward("d3dx10_43")),
    ("D3DXMatrixRotationY", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXMatrixRotationYawPitchRoll",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXMatrixRotationZ", ExportKind::Forward("d3dx10_43")),
    ("D3DXMatrixScaling", ExportKind::Forward("d3dx10_43")),
    ("D3DXMatrixShadow", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXMatrixTransformation2D",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXMatrixTransformation", ExportKind::Forward("d3dx10_43")),
    ("D3DXMatrixTranslation", ExportKind::Forward("d3dx10_43")),
    ("D3DXMatrixTranspose", ExportKind::Forward("d3dx10_43")),
    ("D3DXPlaneFromPointNormal", ExportKind::Forward("d3dx10_43")),
    ("D3DXPlaneFromPoints", ExportKind::Forward("d3dx10_43")),
    ("D3DXPlaneIntersectLine", ExportKind::Forward("d3dx10_43")),
    ("D3DXPlaneNormalize", ExportKind::Forward("d3dx10_43")),
    ("D3DXPlaneTransform", ExportKind::Forward("d3dx10_43")),
    ("D3DXPlaneTransformArray", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXQuaternionBaryCentric",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXQuaternionExp", ExportKind::Forward("d3dx10_43")),
    ("D3DXQuaternionInverse", ExportKind::Forward("d3dx10_43")),
    ("D3DXQuaternionLn", ExportKind::Forward("d3dx10_43")),
    ("D3DXQuaternionMultiply", ExportKind::Forward("d3dx10_43")),
    ("D3DXQuaternionNormalize", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXQuaternionRotationAxis",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DXQuaternionRotationMatrix",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DXQuaternionRotationYawPitchRoll",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXQuaternionSlerp", ExportKind::Forward("d3dx10_43")),
    ("D3DXQuaternionSquad", ExportKind::Forward("d3dx10_43")),
    ("D3DXQuaternionSquadSetup", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXQuaternionToAxisAngle",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXSHAdd", ExportKind::Forward("d3dx10_43")),
    ("D3DXSHDot", ExportKind::Forward("d3dx10_43")),
    ("D3DXSHEvalConeLight", ExportKind::Forward("d3dx10_43")),
    ("D3DXSHEvalDirection", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXSHEvalDirectionalLight",
        ExportKind::Forward("d3dx10_43"),
    ),
    (
        "D3DXSHEvalHemisphereLight",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXSHEvalSphericalLight", ExportKind::Forward("d3dx10_43")),
    ("D3DXSHMultiply2", ExportKind::Forward("d3dx10_43")),
    ("D3DXSHMultiply3", ExportKind::Forward("d3dx10_43")),
    ("D3DXSHMultiply4", ExportKind::Forward("d3dx10_43")),
    ("D3DXSHMultiply5", ExportKind::Forward("d3dx10_43")),
    ("D3DXSHMultiply6", ExportKind::Forward("d3dx10_43")),
    ("D3DXSHRotate", ExportKind::Forward("d3dx10_43")),
    ("D3DXSHRotateZ", ExportKind::Forward("d3dx10_43")),
    ("D3DXSHScale", ExportKind::Forward("d3dx10_43")),
    ("D3DXSphereBoundProbe", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec2BaryCentric", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec2CatmullRom", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec2Hermite", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec2Normalize", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec2Transform", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec2TransformArray", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec2TransformCoord", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXVec2TransformCoordArray",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXVec2TransformNormal", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXVec2TransformNormalArray",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXVec3BaryCentric", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec3CatmullRom", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec3Hermite", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec3Normalize", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec3Project", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec3ProjectArray", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec3Transform", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec3TransformArray", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec3TransformCoord", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXVec3TransformCoordArray",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXVec3TransformNormal", ExportKind::Forward("d3dx10_43")),
    (
        "D3DXVec3TransformNormalArray",
        ExportKind::Forward("d3dx10_43"),
    ),
    ("D3DXVec3Unproject", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec3UnprojectArray", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec4BaryCentric", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec4CatmullRom", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec4Cross", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec4Hermite", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec4Normalize", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec4Transform", ExportKind::Forward("d3dx10_43")),
    ("D3DXVec4TransformArray", ExportKind::Forward("d3dx10_43")),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
